//! 🏪️ `semio_compose_rs-gql`: HTTP GraphQL sidecar over native [`semio_compose_rs::worker::ParentStore`] (same schema as WASM `KitStoreHandle`). `POST /graphql` accepts JSON `{ "query", "variables?", "operationName?" }` and serves the same kit materialization fields as the golden schema (`initialKit`, `theKit.kit`, `checkpoints.node.initial` / `kit`).

//#region 🏪️State

use std::io;
use std::net::SocketAddr;
use std::sync::Arc;

use async_graphql::http::GraphiQLSource;
use axum::extract::{DefaultBodyLimit, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use semio_compose_rs::gql;
use semio_compose_rs::worker::ParentStore;
use serde::Deserialize;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;

struct AppState {
    runtime: Arc<Mutex<Option<Arc<ParentStore>>>>,
    preview: Arc<ParentStore>,
}

//#endregion
//#region ⚠️ Errors

mod errors {
    use std::net::SocketAddr;
    use thiserror::Error;

    /// ⚠️ semio_compose_rs-gql install/request/serve failure.
    #[derive(Debug, Error)]
    pub enum StoreError {
        #[error("expected exactly one of: create, importFile, importFromFolder, importFromZip, importFromRemote")]
        AmbiguousInstallSource,
        #[error("{0}: not wired in semio_compose_rs-gql yet")]
        NotWired(&'static str),
        #[error("no install field")]
        NoInstallField,
        #[error("read install file: {0}")]
        ReadInstallFile(std::io::Error),
        #[error("parse install file: {0}")]
        ParseInstallFile(serde_json::Error),
        #[error("invalid graphql json: {0}")]
        InvalidGraphqlJson(serde_json::Error),
        #[error(transparent)]
        Kit(#[from] semio_compose_rs::error::ComposeError),
        #[error("bind {addr}: {source}")]
        BindFailed {
            addr: SocketAddr,
            #[source]
            source: std::io::Error,
        },
    }
}
use errors::StoreError;

//#endregion
//#region 🏪️Install

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
    dto: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct PathOnly {
    path: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code, reason = "hub_url/session_id parsed for schema compat with importFromRemote; unread until that path is wired")]
struct RemoteIn {
    hub_url: String,
    session_id: String,
}

impl InstallBody {
    async fn into_runtime(self) -> Result<Arc<ParentStore>, StoreError> {
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
            return Err(StoreError::AmbiguousInstallSource);
        }
        if let Some(c) = self.create {
            return Ok(ParentStore::spawn_from_install_json_value(c.dto).await?);
        }
        if let Some(p) = self.import_file {
            let txt = std::fs::read_to_string(&p.path).map_err(StoreError::ReadInstallFile)?;
            let v: serde_json::Value = serde_json::from_str(&txt).map_err(StoreError::ParseInstallFile)?;
            return Ok(ParentStore::spawn_from_install_json_value(v).await?);
        }
        if self.import_from_folder.is_some() {
            return Err(StoreError::NotWired("importFromFolder"));
        }
        if self.import_from_zip.is_some() {
            return Err(StoreError::NotWired("importFromZip"));
        }
        if self.import_from_remote.is_some() {
            return Err(StoreError::NotWired("importFromRemote"));
        }
        Err(StoreError::NoInstallField)
    }
}

async fn post_install(State(state): State<Arc<AppState>>, Json(body): Json<InstallBody>) -> impl IntoResponse {
    let new_rt = match body.into_runtime().await {
        Ok(x) => x,
        Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    };
    let mut lock = state.runtime.lock().await;
    if lock.is_some() {
        return (StatusCode::CONFLICT, "kit already installed").into_response();
    }
    *lock = Some(new_rt);
    (StatusCode::CREATED, "ok").into_response()
}

//#endregion
//#region 🏪️Graphql

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

fn is_mutation_request(body: &str) -> Result<bool, StoreError> {
    let parsed: GraphqlRequestBody = serde_json::from_str(body).map_err(StoreError::InvalidGraphqlJson)?;
    for line in parsed.query.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let head = t.split_whitespace().next().unwrap_or("");
        if head.eq_ignore_ascii_case("fragment") {
            continue;
        }
        return Ok(head.eq_ignore_ascii_case("mutation") || head.eq_ignore_ascii_case("subscription"));
    }
    Ok(false)
}

/// @emoji 🌐️ GraphQL-over-HTTP POST: JSON body `query`, optional `variables`, optional `operationName` (same contract as `semio_compose_rs::gql::graphql_request_from_json_str`).
async fn post_graphql(State(state): State<Arc<AppState>>, body: String) -> impl IntoResponse {
    let (rt, installed): (Arc<ParentStore>, bool) = {
        let l = state.runtime.lock().await;
        match l.as_ref() {
            None => (state.preview.clone(), false),
            Some(r) => (r.clone(), true),
        }
    };
    if !installed && matches!(is_mutation_request(&body), Ok(true)) {
        return graphql_error(StatusCode::SERVICE_UNAVAILABLE, "no kit: send POST /install with { \"create\": { \"dto\": { ... } } } first");
    }
    let mut req = match gql::graphql_request_from_json_str(&body) {
        Ok(r) => r,
        Err(e) => return bad_request(e.to_string()),
    };
    req = req.data(rt.clone()).data(rt.bus.clone());
    let schema = gql::build_schema_for(rt);
    let resp = schema.execute(req).await;
    match serde_json::to_value(resp) {
        Ok(v) => Json(v).into_response(),
        Err(e) => graphql_error(StatusCode::INTERNAL_SERVER_ERROR, e),
    }
}

async fn get_graphiql() -> Html<String> {
    let html: String = GraphiQLSource::build().title("semio_compose_rs-gql GraphiQL").endpoint("/graphql").finish();
    Html(html)
}

async fn get_health() -> impl IntoResponse {
    "semio_compose_rs-gql\n"
}

async fn post_shutdown() -> StatusCode {
    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_millis(50));
        std::process::exit(0);
    });
    StatusCode::ACCEPTED
}

//#endregion
//#region 🏪️Serve

async fn build_state() -> AppState {
    let preview = ParentStore::spawn().await;
    AppState { runtime: Arc::new(Mutex::new(None)), preview }
}

fn app_with_state(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/healthz", get(get_health))
        .route("/graphiql", get(get_graphiql))
        .route("/graphql", post(post_graphql).get(get_graphiql))
        .route("/install", post(post_install))
        .route("/server/shutdown", post(post_shutdown))
        .layer(DefaultBodyLimit::max(512 * 1024 * 1024))
        .with_state(state)
        .layer(CorsLayer::permissive())
}

async fn serve(listener: TcpListener, app: Router) {
    let local = listener.local_addr().expect("listener is already bound, so its local addr is always resolvable");
    let actual_port: u16 = local.port();
    {
        use std::io::Write;
        let ready: serde_json::Value = serde_json::json!({
            "composeGqlReady": true,
            "port": actual_port,
            "graphiql": format!("http://127.0.0.1:{}/graphiql", actual_port),
        });
        println!("{}", ready);
        let _ = io::stdout().lock().flush();
    }
    let base = format!("http://127.0.0.1:{}", actual_port);
    tracing::info!(target: "semio_compose_gql", "┌️ post /install, post /graphql, get /graphiql, get /healthz, post /server/shutdown");
    tracing::info!(target: "semio_compose_gql", "└️ {base}/graphiql  (GraphiQL)  →  POST {base}/graphql", base = base);

    if let Err(e) = axum::serve(listener, app.into_make_service()).with_graceful_shutdown(shutdown_signal()).await {
        tracing::error!(target: "semio_compose_gql", "server: {e}");
    }
}

/// 🌐️ Axum + GraphQL; binds `0.0.0.0` on `COMPOSE_GQL_PORT` (default `4000`).
async fn run() -> Result<(), StoreError> {
    let port: u16 = std::env::var("COMPOSE_GQL_PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(4000);
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().expect("a u16 port always parses into a valid 0.0.0.0:<port> socket addr");
    let listener: TcpListener = TcpListener::bind(&addr).await.map_err(|source| StoreError::BindFailed { addr, source })?;
    let state = Arc::new(build_state().await);
    serve(listener, app_with_state(state)).await;
    Ok(())
}

async fn shutdown_signal() {
    use tokio::signal;
    let _ = signal::ctrl_c().await;
}

#[tokio::main]
async fn main() -> std::process::ExitCode {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(std::env::var("RUST_LOG").or_else(|_| std::env::var("RUST_TRACING")).unwrap_or_else(|_| "error,semio_compose_gql=info,compose_gql_event=off".to_string()))
        .with_target(false)
        .with_writer(io::stderr)
        .try_init();

    match run().await {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(e) => {
            tracing::error!(target: "semio_compose_gql", "{e}");
            std::process::ExitCode::FAILURE
        }
    }
}

//#endregion
//#region 🏪️Tests

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};
    use tokio::task::JoinHandle;

    async fn spawn_server() -> Result<(JoinHandle<()>, String), Box<dyn std::error::Error + Send + Sync>> {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let port = listener.local_addr()?.port();
        let state = Arc::new(build_state().await);
        let handle = tokio::spawn(async move {
            if let Err(e) = axum::serve(listener, app_with_state(state).into_make_service()).await {
                tracing::error!(target: "semio_compose_gql", "test server: {e}");
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
        let v: Value = serde_json::from_str(&t).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("{e}, body: {t}")))?;
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

        assert!(html.contains("semio_compose_rs-gql GraphiQL"));
        assert!(html.contains("/graphql"));
        assert!(html.contains("graphiql@4"));
        assert!(!html.contains("catch(() => response.text())"));

        let response = client.post(format!("{base}/graphql")).json(&json!({ "query": "{ __typename }" })).send().await?;
        assert_eq!(response.status(), StatusCode::OK);
        let body: Value = response.json().await?;
        assert_eq!(body.pointer("/data/__typename"), Some(&json!("Query")));

        let response = client.post(format!("{base}/graphql")).json(&json!({ "query": "mutation { session { start } }" })).send().await?;
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
        let body: Value = response.json().await?;
        let message = body.pointer("/errors/0/message").and_then(|value| value.as_str()).ok_or("missing no-kit GraphQL error message")?;
        assert!(message.contains("no kit: send POST /install"));

        server.abort();
        Ok(())
    }

    const GQL_RESPONSE: &str = "ok errors { kind message requestId } result { ... on IdResult { value } }";
    const STORE_ID: &str = "e0";

    #[tokio::test]
    async fn sidecar_install_rename_roundtrip() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let (server, base) = spawn_server().await?;
        let client = reqwest::Client::new();
        post_install(&client, &base, &json!({ "create": { "dto": { "id": "00000000-0000-7000-8000-000000000001", "name": "SeedName" } } })).await?;

        let tx = post_gql(&client, &base, &format!("mutation {{ session {{ store(id: \"{STORE_ID}\") {{ theKit {{ startNewChange {{ {GQL_RESPONSE} }} }} }} }} }}"), None).await?;
        if tx.get("errors").is_some() {
            return Err(format!("startNewChange: {tx}").into());
        }
        let tx_id = tx.pointer("/data/session/store/theKit/startNewChange/result/value").and_then(|v| v.as_str()).ok_or("tx id")?;

        let m1 = post_gql(
            &client,
            &base,
            &format!(
                r#"mutation($tx: ID!, $n: String!) {{
  session {{
    store(id: "{STORE_ID}") {{
      theKit {{
        unsavedChange(id: $tx) {{
          kit {{ rename(newName: $n) {{ {GQL_RESPONSE} }} }}
        }}
      }}
    }}
  }}
}}"#
            ),
            Some(json!({ "tx": tx_id, "n": "RenamedKit" })),
        )
        .await?;
        if m1.get("errors").is_some() {
            return Err(format!("rename: {m1}").into());
        }

        tokio::time::sleep(std::time::Duration::from_millis(200)).await;

        const Q_MAT: &str = r#"query KitMaterialization {
  session {
    stores {
      edges {
        node {
          wip {
            initialKit { name }
            theKit { kit { name } }
            checkpoints {
              edges {
                node {
                  initial { name }
                  kit { name }
                }
              }
            }
          }
        }
      }
    }
  }
}"#;
        let q2 = post_gql(&client, &base, Q_MAT, None).await?;
        if q2.get("errors").is_some() {
            return Err(format!("query: {q2}").into());
        }
        let wip = "/data/session/stores/edges/0/node/wip";
        assert_eq!(q2.pointer(&format!("{wip}/initialKit/name")).and_then(|n| n.as_str()), Some("SeedName"), "initialKit stays install baseline");
        assert_eq!(q2.pointer(&format!("{wip}/theKit/kit/name")).and_then(|n| n.as_str()), Some("RenamedKit"), "theKit.kit materialized head");
        assert_eq!(q2.pointer(&format!("{wip}/checkpoints/edges/0/node/initial/name")).and_then(|n| n.as_str()), Some("SeedName"), "checkpoint.initial is graph baseline");
        assert_eq!(q2.pointer(&format!("{wip}/checkpoints/edges/0/node/kit/name")).and_then(|n| n.as_str()), Some("RenamedKit"), "checkpoint.kit matches wip parent anchor materialization");

        server.abort();
        Ok(())
    }

    #[tokio::test]
    async fn sidecar_install_create_accepts_full_kit_store_bundle_doc() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use semio_compose_rs::kit_backbone::{DevBackboneBundleDoc, KIT_BUNDLE_HASH_STUB};

        let (server, base) = spawn_server().await?;
        let client = reqwest::Client::new();

        let mut b = DevBackboneBundleDoc::initialize_with_unsaved_change("00000000-0000-7000-8000-0000000000cc", "change-bundle-1", "ck-bundle-1");
        b.wip.initial_kit = serde_json::json!({
            "id": "00000000-0000-7000-8000-0000000000cc",
            "name": "BundleInstallName",
            "version": "v-bundle-smoke",
            "types": { "hash": KIT_BUNDLE_HASH_STUB, "items": [] },
            "designs": { "hash": KIT_BUNDLE_HASH_STUB, "items": [] },
        });
        let dto = serde_json::to_value(&b)?;
        post_install(&client, &base, &json!({ "create": { "dto": dto } })).await?;

        let q = post_gql(
            &client,
            &base,
            r#"query BundleKit {
  session {
    stores {
      edges {
        node {
          wip {
            initialKit { name version }
            theKit { kit { name version } }
          }
        }
      }
    }
  }
}"#,
            None,
        )
        .await?;
        if q.get("errors").is_some() {
            return Err(format!("bundle install query: {q}").into());
        }
        let wip = "/data/session/stores/edges/0/node/wip";
        assert_eq!(q.pointer(&format!("{wip}/initialKit/name")).and_then(|n| n.as_str()), Some("BundleInstallName"));
        assert_eq!(q.pointer(&format!("{wip}/initialKit/version")).and_then(|n| n.as_str()), Some("v-bundle-smoke"));
        assert_eq!(q.pointer(&format!("{wip}/theKit/kit/name")).and_then(|n| n.as_str()), Some("BundleInstallName"));
        assert_eq!(q.pointer(&format!("{wip}/theKit/kit/version")).and_then(|n| n.as_str()), Some("v-bundle-smoke"));

        server.abort();
        Ok(())
    }

    #[tokio::test]
    async fn sidecar_preview_reads_wip_materialization_without_install() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let (server, base) = spawn_server().await?;
        let client = reqwest::Client::new();
        const Q_PREVIEW: &str = r#"query WipKit {
  session {
    stores {
      edges {
        node {
          wip {
            initialKit { name }
            theKit { kit { name } }
            checkpoints {
              edges {
                node {
                  initial { name }
                  kit { name }
                }
              }
            }
          }
        }
      }
    }
  }
}"#;
        let body = json!({ "query": Q_PREVIEW, "operationName": "WipKit" });
        let r = client.post(format!("{base}/graphql")).json(&body).send().await?;
        assert_eq!(r.status(), StatusCode::OK);
        let v: Value = r.json().await?;
        assert!(v.get("errors").is_none(), "preview wip query errors: {v:?}");
        let wip = "/data/session/stores/edges/0/node/wip";
        assert_eq!(v.pointer(&format!("{wip}/initialKit/name")).and_then(|n| n.as_str()), Some("the kit"));
        assert_eq!(v.pointer(&format!("{wip}/theKit/kit/name")).and_then(|n| n.as_str()), Some("the kit"));
        assert_eq!(v.pointer(&format!("{wip}/checkpoints/edges/0/node/initial/name")).and_then(|n| n.as_str()), Some("the kit"));
        assert_eq!(v.pointer(&format!("{wip}/checkpoints/edges/0/node/kit/name")).and_then(|n| n.as_str()), Some("the kit"));
        server.abort();
        Ok(())
    }

    fn comprehensive_fixture_path() -> Option<std::path::PathBuf> {
        semio_compose_rs::kit_store_comprehensive_e2e::kit_store_comprehensive_fixture_path()
    }

    async fn run_comprehensive_fixture_sidecar_steps(fixture: &Value, client: &reqwest::Client, base: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let steps = fixture["sidecarSteps"].as_array().ok_or("sidecarSteps array")?;
        for step in steps {
            match step["kind"].as_str().ok_or("kind")? {
                "graphql" => {
                    let q = step["query"].as_str().ok_or("query")?;
                    let v = post_gql(client, base, q, None).await?;
                    assert!(v.get("errors").is_none(), "step {:?}: {v}", step["id"]);
                    if let Some(expect) = step.get("expect").and_then(|e| e.as_object()) {
                        for (pointer, want) in expect {
                            assert_eq!(v.pointer(pointer), Some(want), "step {:?}", step["id"]);
                        }
                    }
                }
                "sidecarInstallRename" => {
                    let seed = step["installName"].as_str().unwrap_or("SeedName");
                    let renamed = step["renamedName"].as_str().unwrap_or("SidecarComprehensiveRenamed");
                    post_install(client, base, &json!({ "create": { "dto": { "id": "00000000-0000-7000-8000-000000000001", "name": seed } } })).await?;
                    let tx = post_gql(client, base, &format!("mutation {{ session {{ store(id: \"{STORE_ID}\") {{ theKit {{ startNewChange {{ {GQL_RESPONSE} }} }} }} }} }}"), None).await?;
                    if tx.get("errors").is_some() {
                        return Err(format!("startNewChange: {tx}").into());
                    }
                    let tx_id = tx.pointer("/data/session/store/theKit/startNewChange/result/value").and_then(|v| v.as_str()).ok_or("tx id")?;
                    let m1 = post_gql(
                        client,
                        base,
                        &format!(
                            r#"mutation($tx: ID!, $n: String!) {{
  session {{
    store(id: "{STORE_ID}") {{
      theKit {{
        unsavedChange(id: $tx) {{
          kit {{ rename(newName: $n) {{ {GQL_RESPONSE} }} }}
        }}
      }}
    }}
  }}
}}"#
                        ),
                        Some(json!({ "tx": tx_id, "n": renamed })),
                    )
                    .await?;
                    if m1.get("errors").is_some() {
                        return Err(format!("rename: {m1}").into());
                    }
                    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
                    let q2 = post_gql(client, base, r#"query { session { stores { edges { node { wip { theKit { kit { name } } } } } } } }"#, None).await?;
                    if q2.get("errors").is_some() {
                        return Err(format!("materialization query: {q2}").into());
                    }
                    assert_eq!(q2.pointer("/data/session/stores/edges/0/node/wip/theKit/kit/name").and_then(|n| n.as_str()), Some(renamed));
                }
                other => return Err(format!("unknown sidecar step kind: {other}").into()),
            }
        }
        Ok(())
    }

    #[tokio::test]
    async fn sidecar_comprehensive_fixture_sidecar_steps() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let Some(path) = comprehensive_fixture_path() else {
            eprintln!("[DEBUG] skip sidecar_comprehensive_fixture_sidecar_steps: missing kit-store.comprehensive.semio.json");
            return Ok(());
        };
        let fixture: Value = serde_json::from_str(&std::fs::read_to_string(path)?)?;
        let (server, base) = spawn_server().await?;
        let client = reqwest::Client::new();
        run_comprehensive_fixture_sidecar_steps(&fixture, &client, &base).await?;
        server.abort();
        Ok(())
    }

    /// @emoji 🧪️ Full catalog E2E: in-process GraphQL + backbone replay, then live semio_compose_rs-gql HTTP sidecar steps.
    #[tokio::test]
    async fn comprehensive_fixture_end_to_end() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let Some(path) = comprehensive_fixture_path() else {
            eprintln!("[DEBUG] skip comprehensive_fixture_end_to_end: missing kit-store.comprehensive.semio.json");
            return Ok(());
        };
        let fixture: Value = serde_json::from_str(&std::fs::read_to_string(path)?)?;
        semio_compose_rs::kit_store_comprehensive_e2e::run_in_process(&fixture).await;
        let (server, base) = spawn_server().await?;
        let client = reqwest::Client::new();
        run_comprehensive_fixture_sidecar_steps(&fixture, &client, &base).await?;
        server.abort();
        Ok(())
    }

    #[tokio::test]
    async fn sidecar_graphql_detects_mutation_after_leading_comment_and_fragment() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let q = "# prelude\nfragment X on Query { __typename }\nmutation { session { start } }";
        let body = serde_json::to_string(&json!({ "query": q }))?;
        assert!(is_mutation_request(&body)?);
        let q2 = "# only\nquery { __typename }";
        let body2 = serde_json::to_string(&json!({ "query": q2 }))?;
        assert!(!is_mutation_request(&body2)?);
        Ok(())
    }
}

//#endregion
