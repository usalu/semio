//! 🏪 `semio-store`: single-file HTTP GraphQL sidecar for native `KitStore`.

//#region 🏪State

use std::io;
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

fn app() -> Router {
    let state: Arc<AppState> = Arc::new(AppState { runtime: Arc::new(Mutex::new(None)), preview: preview_runtime() });
    Router::new()
        .route("/healthz", get(get_health))
        .route("/graphiql", get(get_graphiql))
        .route("/graphql", post(post_graphql).get(get_graphiql))
        .route("/install", post(post_install))
        .route("/server/shutdown", post(post_shutdown))
        .with_state(state)
        .layer(CorsLayer::permissive())
}

async fn serve(listener: TcpListener, app: Router) {
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

/// 🌐 Axum + GraphQL; binds `0.0.0.0` on `SEMIO_STORE_PORT` (default `4000`).
async fn run() {
    // `0` = free port (emits actual port in the first JSON line to stdout). Default `4000` for local dev.
    let port: u16 = std::env::var("SEMIO_STORE_PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(4000);
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().expect("port");
    let listener: TcpListener = TcpListener::bind(&addr).await.unwrap_or_else(|e| panic!("semio-store bind {addr}: {e}"));
    serve(listener, app()).await;
}

async fn shutdown_signal() {
    use tokio::signal;
    let _ = signal::ctrl_c().await;
}

#[tokio::main]
async fn main() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(std::env::var("RUST_LOG").or_else(|_| std::env::var("RUST_TRACING")).unwrap_or_else(|_| "error,semio_store=info,semio_store_event=off".to_string()))
        .with_target(false)
        .with_writer(io::stderr)
        .try_init();

    run().await
}

//#endregion
//#region 🏪Tests

#[cfg(test)]
mod tests {
    use super::*;
    use semio::kit::KitFullDto;
    use serde_json::{json, Value};
    use tokio::task::JoinHandle;

    fn batch_mutation_with_fields(extra: &str) -> String {
        format!(
            r"mutation Batch($input: KitStoreBatchInput!) {{
  kitStore {{
    batch(input: $input) {{
      clientMutationId
      results {{
        kind
        ok
        count
        backbone {{ attached kind tip }}
        conflicts {{ id backboneTip reason createdAt }}
        {extra}
      }}
    }}
  }}
}}"
        )
    }

    async fn spawn_server() -> Result<(JoinHandle<()>, String), Box<dyn std::error::Error + Send + Sync>> {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let port = listener.local_addr()?.port();
        let handle = tokio::spawn(async move {
            if let Err(e) = axum::serve(listener, app().into_make_service()).await {
                tracing::error!(target: "semio_store", "test server: {e}");
            }
        });
        Ok((handle, format!("http://127.0.0.1:{port}")))
    }

    async fn post_gql(client: &reqwest::Client, base: &str, query: &str, variables: Option<Value>) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        let mut body = json!({ "query": query });
        if let Some(v) = variables {
            body["variables"] = v;
        }
        let r = client.post(format!("{base}/graphql")).json(&body).send().await?;
        let t = r.text().await?;
        let v: Value = serde_json::from_str(&t).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("{e}, body: {t}")))?;
        Ok(v)
    }

    async fn post_install(client: &reqwest::Client, base: &str, body: &Value) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let st = client.post(format!("{base}/install")).json(body).send().await?;
        if !st.status().is_success() {
            return Err(format!("install {}: {}", st.status(), st.text().await?).into());
        }
        Ok(())
    }

    #[tokio::test]
    async fn sidecar_graphiql_serves_working_fetcher() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let (server, base) = spawn_server().await?;
        let client = reqwest::Client::new();
        let html = client.get(format!("{base}/graphiql")).send().await?.error_for_status()?.text().await?;

        assert!(html.contains("semio-store GraphiQL"));
        assert!(html.contains("/graphql"));
        assert!(html.contains("graphiql@4"));
        assert!(!html.contains("catch(() => response.text())"));

        let response = client.post(format!("{base}/graphql")).json(&json!({ "query": "{ __typename }" })).send().await?;
        assert_eq!(response.status(), reqwest::StatusCode::OK);
        let body: Value = response.json().await?;
        assert_eq!(body.pointer("/data/__typename"), Some(&json!("Query")));

        let response = client.post(format!("{base}/graphql")).json(&json!({ "query": "mutation { kitStore { batch(input: { commands: [] }) { clientMutationId } } }" })).send().await?;
        assert_eq!(response.status(), reqwest::StatusCode::SERVICE_UNAVAILABLE);
        let body: Value = response.json().await?;
        let message = body.pointer("/errors/0/message").and_then(|value| value.as_str()).ok_or("missing no-kit GraphQL error message")?;
        assert!(message.contains("no kit: send POST /install"));

        server.abort();
        Ok(())
    }

    #[tokio::test]
    async fn sidecar_create_snapshot_name_change_undo() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let (server, base) = spawn_server().await?;
        let client = reqwest::Client::new();
        let kid = Id::new_v7();
        let dto = KitFullDto { id: kid, name: "A".to_string(), ..Default::default() };
        post_install(&client, &base, &json!({ "create": { "dto": serde_json::to_value(&dto)? } })).await?;

        let qm = batch_mutation_with_fields("");
        let m1 = post_gql(
            &client,
            &base,
            &qm,
            Some(json!({
                "input": {
                    "commands": [{
                        "live": {
                            "commands": [{
                                "changeKitCommands": {
                                    "commands": [{ "name": { "name": "Renamed" } }]
                                }
                            }]
                        }
                    }]
                }
            })),
        )
        .await?;
        if m1.get("errors").is_some() {
            return Err(format!("graphql m1: {m1}").into());
        }

        let q2 = post_gql(&client, &base, "query { kitStore { liveFullDto } }", None).await?;
        let name = q2.pointer("/data/kitStore/liveFullDto/name").and_then(|n| n.as_str()).ok_or("liveFullDto.name")?;
        assert_eq!(name, "Renamed");

        let m3 = post_gql(
            &client,
            &base,
            &qm,
            Some(json!({
                "input": {
                    "commands": [{
                        "live": {
                            "commands": [{ "undo": { "confirm": true } }]
                        }
                    }]
                }
            })),
        )
        .await?;
        if m3.get("errors").is_some() {
            return Err(format!("graphql m3: {m3}").into());
        }

        let q4 = post_gql(&client, &base, "query { kitStore { liveFullDto } }", None).await?;
        let name2 = q4.pointer("/data/kitStore/liveFullDto/name").and_then(|n| n.as_str()).ok_or("name2")?;
        assert_eq!(name2, "A");

        server.abort();
        Ok(())
    }

    #[tokio::test]
    async fn sidecar_batch_rename_to_q() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let (server, base) = spawn_server().await?;
        let client = reqwest::Client::new();
        let kid = Id::new_v7();
        let dto = KitFullDto { id: kid.clone(), name: "P".to_string(), ..Default::default() };
        post_install(&client, &base, &json!({ "create": { "dto": serde_json::to_value(&dto)? } })).await?;

        let qm = batch_mutation_with_fields("");
        let m3 = post_gql(
            &client,
            &base,
            &qm,
            Some(json!({
                "input": {
                    "commands": [{
                        "live": {
                            "commands": [{
                                "changeKitCommands": {
                                    "commands": [{ "name": { "name": "Q" } }]
                                }
                            }]
                        }
                    }]
                }
            })),
        )
        .await?;
        if m3.get("errors").is_some() {
            return Err(format!("execute: {m3}").into());
        }

        let q4 = post_gql(&client, &base, "query { kitStore { liveFullDto } }", None).await?;
        let name = q4.pointer("/data/kitStore/liveFullDto/name").and_then(|n| n.as_str());
        assert_eq!(name, Some("Q"));

        server.abort();
        Ok(())
    }

    #[tokio::test]
    async fn sidecar_backbone_attach_status_conflicts_sync_detach() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let (server, base) = spawn_server().await?;
        let client = reqwest::Client::new();
        let kid = Id::new_v7();
        let dto = KitFullDto { id: kid, name: "Bb".to_string(), ..Default::default() };
        post_install(&client, &base, &json!({ "create": { "dto": serde_json::to_value(&dto)? } })).await?;

        let qm = batch_mutation_with_fields("");

        let m2 = post_gql(
            &client,
            &base,
            &qm,
            Some(json!({
                "input": {
                    "commands": [{
                        "backbone": {
                            "commands": [{ "listConflicts": { "confirm": true } }]
                        }
                    }]
                }
            })),
        )
        .await?;
        if m2.get("errors").is_some() {
            return Err(format!("m2: {m2}").into());
        }
        let c0 = m2.pointer("/data/kitStore/batch/results/0/conflicts").and_then(|x| x.as_array()).ok_or("listConflicts.conflicts")?;
        assert!(c0.is_empty(), "expected no conflicts");

        let mut bb_path = std::env::temp_dir();
        bb_path.push(format!("semio-store-dev-backbone-{}.json", Id::new_v7().as_str()));

        let m3 = post_gql(
            &client,
            &base,
            &qm,
            Some(json!({
                "input": {
                    "commands": [{
                        "backbone": {
                            "commands": [{
                                "attachBackbone": { "dev": { "path": bb_path.to_string_lossy() } }
                            }]
                        }
                    }]
                }
            })),
        )
        .await?;
        if m3.get("errors").is_some() {
            return Err(format!("m3: {m3}").into());
        }
        assert_eq!(m3.pointer("/data/kitStore/batch/results/0/ok"), Some(&json!(true)));

        let m4 = post_gql(
            &client,
            &base,
            &qm,
            Some(json!({
                "input": {
                    "commands": [{
                        "backbone": {
                            "commands": [{ "backboneStatus": { "confirm": true } }]
                        }
                    }]
                }
            })),
        )
        .await?;
        if m4.get("errors").is_some() {
            return Err(format!("m4: {m4}").into());
        }
        assert_eq!(m4.pointer("/data/kitStore/batch/results/0/backbone/attached"), Some(&json!(true)));
        assert_eq!(m4.pointer("/data/kitStore/batch/results/0/backbone/kind"), Some(&json!("dev")));

        let m5 = post_gql(
            &client,
            &base,
            &qm,
            Some(json!({
                "input": {
                    "commands": [{
                        "backbone": { "commands": [{ "syncNow": { "confirm": true } }] }
                    }]
                }
            })),
        )
        .await?;
        if m5.get("errors").is_some() {
            return Err(format!("m5: {m5}").into());
        }
        assert_eq!(m5.pointer("/data/kitStore/batch/results/0/ok"), Some(&json!(true)));

        let m6 = post_gql(
            &client,
            &base,
            &qm,
            Some(json!({
                "input": {
                    "commands": [{
                        "backbone": { "commands": [{ "detachBackbone": { "confirm": true } }] }
                    }]
                }
            })),
        )
        .await?;
        if m6.get("errors").is_some() {
            return Err(format!("m6: {m6}").into());
        }
        assert_eq!(m6.pointer("/data/kitStore/batch/results/0/ok"), Some(&json!(true)));

        let m7 = post_gql(
            &client,
            &base,
            &qm,
            Some(json!({
                "input": {
                    "commands": [{
                        "backbone": {
                            "commands": [{ "backboneStatus": { "confirm": true } }]
                        }
                    }]
                }
            })),
        )
        .await?;
        if m7.get("errors").is_some() {
            return Err(format!("m7: {m7}").into());
        }
        assert_eq!(m7.pointer("/data/kitStore/batch/results/0/backbone/attached"), Some(&json!(false)));

        let _ = std::fs::remove_file(&bb_path);
        server.abort();
        Ok(())
    }
}

//#endregion
