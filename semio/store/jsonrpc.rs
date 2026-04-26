//! `semio-store`: HTTP GraphQL (same schema as `semio::kit_graphql` + `semio/js`), optional GraphiQL.
//! `POST /install` bootstraps the process-local [`kit_store::KitStore`], then `POST /graphql` accepts
//! a standard GraphQL JSON body (`{ "query", "variables", "operationName" }`).

//#region 🏪State

use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;

use async_graphql::http::GraphiQLSource;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use semio::id::Id;
use semio::kit_graph::{KitFullDto, KitGraph, KitGraphRef};
use semio::kit_graphql::{GraphQlVcsOverride, GraphWork};
use semio::kit_store::KitStore;
use serde::Deserialize;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;

struct AppState {
    runtime: Arc<Mutex<Option<KitRuntime>>>,
    preview: KitRuntime,
}

struct KitRuntime {
    store: Arc<KitStore>,
    work_tx: async_channel::Sender<GraphWork>,
}

impl Clone for KitRuntime {
    fn clone(&self) -> Self {
        KitRuntime { store: self.store.clone(), work_tx: self.work_tx.clone() }
    }
}

//#endregion
//#region 🏪Install

fn install_k(graph: KitGraphRef) {
    use async_broadcast::RecvError;
    use futures_lite::future::block_on;
    use std::sync::Once;
    use std::thread;

    static EVENTS: Once = Once::new();
    if matches!(std::env::var("SEMIO_STORE_NO_EVENTS").as_deref(), Ok("1" | "true" | "yes")) {
        return;
    }
    let g2: KitGraphRef = graph.clone();
    EVENTS.call_once(move || {
        let _ = thread::Builder::new().name("semio-store-event-log".to_string()).spawn(move || {
            let mut rx = match g2.read() {
                Ok(gg) => gg.subscribe(),
                Err(_) => return,
            };
            drop(g2);
            loop {
                match block_on(async { rx.recv().await }) {
                    Ok(ev) => {
                        if let Ok(line) = serde_json::to_string(&ev) {
                            tracing::info!(target: "semio_store_event", "{}", line);
                        }
                    }
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Overflowed(_)) => {}
                }
            }
        });
    });
}

fn runtime_from_kit_graph(graph: KitGraphRef, log_events: bool) -> std::result::Result<KitRuntime, String> {
    let store = Arc::new(KitStore::from_graph(graph));
    let (work_tx, work_rx) = async_channel::unbounded();
    semio::kit_graphql::spawn_actor(store.graph().clone(), work_rx);
    let rt = KitRuntime { store, work_tx };
    if log_events {
        install_k(rt.store.graph().clone());
    }
    Ok(rt)
}

/// 🌱 First successful install sets the sole control-plane; further installs return 409.
fn install_from_kit_graph(graph: KitGraphRef) -> std::result::Result<KitRuntime, String> {
    runtime_from_kit_graph(graph, true)
}

fn preview_runtime() -> KitRuntime {
    runtime_from_kit_graph(KitGraph::from_full_dto(KitFullDto { id: Id::new_v7(), name: "GraphiQL Preview".to_string(), ..Default::default() }), false).expect("preview runtime")
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct InstallBody {
    create: Option<CreateInstall>,
    import_file: Option<PathOnly>,
    import_from_folder: Option<PathOnly>,
    import_from_zip: Option<PathOnly>,
    import_from_remote: Option<RemoteIn>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateInstall {
    dto: KitFullDto,
}

#[derive(Debug, Deserialize)]
struct PathOnly {
    path: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RemoteIn {
    hub_url: String,
    session_id: String,
}

impl InstallBody {
    fn into_graph(self) -> std::result::Result<KitGraphRef, String> {
        use semio::error::SemioError;
        let mut n = 0u8;
        if self.create.is_some() {
            n += 1;
        }
        if self.import_file.is_some() {
            n += 1;
        }
        if self.import_from_folder.is_some() {
            n += 1;
        }
        if self.import_from_zip.is_some() {
            n += 1;
        }
        if self.import_from_remote.is_some() {
            n += 1;
        }
        if n != 1 {
            return Err("expected exactly one of: create, importFile, importFromFolder, importFromZip, importFromRemote".to_string());
        }
        if let Some(c) = self.create {
            return Ok(KitGraph::from_full_dto(c.dto));
        }
        if let Some(p) = self.import_file {
            return KitGraph::load_json_file(Path::new(&p.path)).map_err(|e: SemioError| e.to_string());
        }
        if let Some(p) = self.import_from_folder {
            return KitGraph::load_local_folder(Path::new(&p.path)).map_err(|e: SemioError| e.to_string());
        }
        if let Some(p) = self.import_from_zip {
            return KitGraph::load_zip(Path::new(&p.path)).map_err(|e: SemioError| e.to_string());
        }
        if let Some(r) = self.import_from_remote {
            return KitGraph::load_remote_session(&r.hub_url, &r.session_id).map_err(|e: SemioError| e.to_string());
        }
        Err("no install field".to_string())
    }
}

async fn post_install(State(state): State<Arc<AppState>>, Json(body): Json<InstallBody>) -> impl IntoResponse {
    let graph: KitGraphRef = match body.into_graph() {
        Ok(g) => g,
        Err(msg) => return (StatusCode::BAD_REQUEST, msg).into_response(),
    };
    let new_rt = match install_from_kit_graph(graph) {
        Ok(x) => x,
        Err(msg) => return (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response(),
    };
    let mut lock = state.runtime.lock().await;
    if lock.is_some() {
        return (StatusCode::CONFLICT, "kit already installed").into_response();
    }
    *lock = Some(new_rt);
    (StatusCode::CREATED, "ok").into_response()
}

//#endregion
//#region 🏪Graphql

fn graphql_error(status: StatusCode, msg: impl std::fmt::Display) -> Response {
    (
        status,
        Json(serde_json::json!({
            "errors": [{ "message": msg.to_string() }]
        })),
    )
        .into_response()
}

fn bad_request(msg: impl std::fmt::Display) -> Response {
    graphql_error(StatusCode::BAD_REQUEST, msg)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GraphqlRequestBody {
    query: String,
}

fn is_mutation_request(body: &str) -> std::result::Result<bool, String> {
    let parsed: GraphqlRequestBody = serde_json::from_str(body).map_err(|e| format!("invalid graphql json: {e}"))?;
    let operation = parsed.query.lines().map(str::trim).find(|line| !line.is_empty() && !line.starts_with('#')).unwrap_or_default();
    Ok(operation.starts_with("mutation") || operation.starts_with("subscription"))
}

async fn post_graphql(State(state): State<Arc<AppState>>, body: String) -> impl IntoResponse {
    let (rt, installed): (KitRuntime, bool) = {
        let l = state.runtime.lock().await;
        match l.as_ref() {
            None => (state.preview.clone(), false),
            Some(r) => (r.clone(), true),
        }
    };
    if !installed && matches!(is_mutation_request(&body), Ok(true)) {
        return graphql_error(StatusCode::SERVICE_UNAVAILABLE, "no kit: send POST /install with { \"create\": { \"dto\": { ... } } } first");
    }
    let vcs = GraphQlVcsOverride { native: Some(rt.store.clone()) };
    let resp = match semio::kit_graphql::execute_with_control_plane(&body, rt.store.graph().clone(), rt.work_tx, vcs).await {
        Ok(r) => r,
        Err(e) => return bad_request(format!("{e:?}")),
    };
    match serde_json::to_value(&resp) {
        Ok(v) => axum::Json(v).into_response(),
        Err(e) => graphql_error(StatusCode::INTERNAL_SERVER_ERROR, e),
    }
}

async fn get_graphiql() -> Html<String> {
    let html: String = GraphiQLSource::build().title("semio-store GraphiQL").endpoint("/graphql").finish();
    Html(html)
}

async fn get_health() -> impl IntoResponse {
    "semio-store\n"
}

async fn post_shutdown() -> StatusCode {
    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_millis(50));
        std::process::exit(0);
    });
    StatusCode::ACCEPTED
}

//#endregion
//#region 🏪Serve

/// 🌐 Axum + GraphQL; binds `0.0.0.0` on `SEMIO_STORE_PORT` (default `4000`).
pub async fn run() {
    let state: Arc<AppState> = Arc::new(AppState { runtime: Arc::new(Mutex::new(None)), preview: preview_runtime() });
    let app: Router = Router::new()
        .route("/healthz", get(get_health))
        .route("/graphiql", get(get_graphiql))
        .route("/graphql", post(post_graphql).get(get_graphiql))
        .route("/install", post(post_install))
        .route("/server/shutdown", post(post_shutdown))
        .with_state(state.clone())
        .layer(CorsLayer::permissive());

    // `0` = free port (emits actual port in the first JSON line to stdout). Default `4000` for local dev.
    let port: u16 = std::env::var("SEMIO_STORE_PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(4000);
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().expect("port");
    let listener: TcpListener = TcpListener::bind(&addr).await.unwrap_or_else(|e| panic!("semio-store bind {addr}: {e}"));
    let local = listener.local_addr().expect("local_addr");
    let actual_port: u16 = local.port();
    {
        use std::io::Write;
        let ready: serde_json::Value = serde_json::json!({
            "semioStoreReady": true,
            "port": actual_port,
            "graphiql": format!("http://127.0.0.1:{}/graphiql", actual_port),
        });
        println!("{}", ready);
        let _ = std::io::stdout().lock().flush();
    }
    let base = format!("http://127.0.0.1:{}", actual_port);
    tracing::info!(target: "semio_store", "┌ post /install, post /graphql, get /graphiql, get /healthz, post /server/shutdown");
    tracing::info!(target: "semio_store", "└ {base}/graphiql  (GraphiQL)  →  POST {base}/graphql", base = base);

    if let Err(e) = axum::serve(listener, app.into_make_service()).with_graceful_shutdown(shutdown_signal()).await {
        tracing::error!(target: "semio_store", "server: {e}");
    }
}

async fn shutdown_signal() {
    use tokio::signal;
    let _ = signal::ctrl_c().await;
}

//#endregion
