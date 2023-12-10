use actix_web::{get, web::ServiceConfig, Responder, HttpResponse, error::InternalError};
use shuttle_actix_web::ShuttleActixWeb;

#[get("/-1/error")]
async fn fake_error() -> HttpResponse {
    HttpResponse::InternalServerError().body("")
}

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
        cfg.service(fake_error);
    };

    Ok(config.into())
}
