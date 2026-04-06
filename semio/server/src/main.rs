// #region 🔖Header
// [👤semio📚server💻semio-session](repo://p/u/semio/b/l/server/f/main.rs)
// 2026 Ueli Saluz <ueli@semio-tech.de>
// AGPL-3.0
// Entry point for the semio session-backend service.
// #endregion 🔖Header

// #region 🔖Main
// Main MUST bootstrap tracing, database, and HTTP server.

mod api;
mod actor;
mod command;
mod directory;
mod domain;
mod error;
mod event;
mod persistence;
mod schema;
mod state;
mod ws;

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "semio_session=debug,tower_http=debug".into()),
        )
        .init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://semio:semio@localhost:5432/semio".to_string());

    let pool = persistence::create_pool(&database_url).await;
    schema::run_migrations(&pool).await;

    let app_state = api::AppState::new(pool);
    let router = api::router(app_state);

    let addr: SocketAddr = std::env::var("LISTEN_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:8080".to_string())
        .parse()
        .expect("invalid LISTEN_ADDR");

    tracing::info!("semio-session listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

// #endregion 🔖Main
