//! 🏪 `semio-store`: HTTP GraphQL sidecar over native [`semio::worker::ParentStore`] (same schema as WASM `KitStoreHandle`). `POST /graphql` accepts JSON `{ "query", "variables?", "operationName?" }` and serves the same kit materialization fields as the golden schema (`initialKit`, `theKit.kit`, `checkpoints.node.initial` / `kit`).

//#region 🏪State

use std::io;
use std::net::SocketAddr;
use std::sync::Arc;

use async_graphql::http::GraphiQLSource;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use semio::gql;
use semio::worker::ParentStore;
use serde::Deserialize;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;

struct AppState {
    runtime: Arc<Mutex<Option<Arc<ParentStore>>>>,
    preview: Arc<ParentStore>,
}

//#endregion
//#region 🏪Install

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
#[allow(dead_code)]
struct RemoteIn {
    hub_url: String,
    session_id: String,
}

impl InstallBody {
    async fn into_runtime(self) -> std::result::Result<Arc<ParentStore>, String> {
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
            return ParentStore::spawn_from_install_json_value(c.dto).await.map_err(|e: SemioError| e.to_string());
        }
        if let Some(p) = self.import_file {
            let txt = std::fs::read_to_string(&p.path).map_err(|e| e.to_string())?;
            let v: serde_json::Value = serde_json::from_str(&txt).map_err(|e| e.to_string())?;
            return ParentStore::spawn_from_install_json_value(v).await.map_err(|e: SemioError| e.to_string());
        }
        if self.import_from_folder.is_some() {
            return Err("importFromFolder: not wired in semio-store yet".to_string());
        }
        if self.import_from_zip.is_some() {
            return Err("importFromZip: not wired in semio-store yet".to_string());
        }
        if self.import_from_remote.is_some() {
            return Err("importFromRemote: not wired in semio-store yet".to_string());
        }
        Err("no install field".to_string())
    }
}

async fn post_install(State(state): State<Arc<AppState>>, Json(body): Json<InstallBody>) -> impl IntoResponse {
    let new_rt = match body.into_runtime().await {
        Ok(x) => x,
        Err(msg) => return (StatusCode::BAD_REQUEST, msg).into_response(),
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

/// @emoji 🌐 GraphQL-over-HTTP POST: JSON body `query`, optional `variables`, optional `operationName` (same contract as `semio::gql::graphql_request_from_json_str`).
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
    match serde_json::to_value(async_graphql::Response::from(resp)) {
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
    let port: u16 = std::env::var("SEMIO_STORE_PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(4000);
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().expect("port");
    let listener: TcpListener = TcpListener::bind(&addr).await.unwrap_or_else(|e| panic!("semio-store bind {addr}: {e}"));
    let state = Arc::new(build_state().await);
    serve(listener, app_with_state(state)).await;
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

    run().await;
}

//#endregion
//#region 🏪Tests

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

        let response = client.post(format!("{base}/graphql")).json(&json!({ "query": "mutation { session { start } }" })).send().await?;
        assert_eq!(response.status(), reqwest::StatusCode::SERVICE_UNAVAILABLE);
        let body: Value = response.json().await?;
        let message = body.pointer("/errors/0/message").and_then(|value| value.as_str()).ok_or("missing no-kit GraphQL error message")?;
        assert!(message.contains("no kit: send POST /install"));

        server.abort();
        Ok(())
    }

    #[tokio::test]
    async fn sidecar_install_rename_roundtrip() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let (server, base) = spawn_server().await?;
        let client = reqwest::Client::new();
        post_install(
            &client,
            &base,
            &json!({ "create": { "dto": { "id": "00000000-0000-7000-8000-000000000001", "name": "SeedName" } } }),
        )
        .await?;

        let tx = post_gql(&client, &base, "mutation { session { store(id: \"test-store\") { theKit { startNewChange } } } }", None).await?;
        if tx.get("errors").is_some() {
            return Err(format!("startNewChange: {tx}").into());
        }
        let tx_id = tx.pointer("/data/session/store/theKit/startNewChange").and_then(|v| v.as_str()).ok_or("tx id")?;

        let m1 = post_gql(
            &client,
            &base,
            r#"mutation($tx: ID!, $n: String!) {
  session {
    store(id: "test-store") {
      theKit {
        unsavedChange(id: $tx) {
          kit { rename(newName: $n) }
        }
      }
    }
  }
}"#,
            Some(json!({ "tx": tx_id, "n": "RenamedKit" })),
        )
        .await?;
        if m1.get("errors").is_some() {
            return Err(format!("rename: {m1}").into());
        }

        tokio::time::sleep(std::time::Duration::from_millis(200)).await;

        const Q_MAT: &str = r#"{
  store {
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
}"#;
        let q2 = post_gql(&client, &base, Q_MAT, None).await?;
        if q2.get("errors").is_some() {
            return Err(format!("query: {q2}").into());
        }
        assert_eq!(q2.pointer("/data/store/wip/initialKit/name").and_then(|n| n.as_str()), Some("SeedName"), "initialKit stays install baseline");
        assert_eq!(q2.pointer("/data/store/wip/theKit/kit/name").and_then(|n| n.as_str()), Some("RenamedKit"), "theKit.kit materialized head");
        assert_eq!(
            q2.pointer("/data/store/wip/checkpoints/edges/0/node/initial/name").and_then(|n| n.as_str()),
            Some("SeedName"),
            "checkpoint.initial is graph baseline"
        );
        assert_eq!(
            q2.pointer("/data/store/wip/checkpoints/edges/0/node/kit/name").and_then(|n| n.as_str()),
            Some("RenamedKit"),
            "checkpoint.kit matches wip parent anchor materialization"
        );

        server.abort();
        Ok(())
    }

    #[tokio::test]
    async fn sidecar_install_create_accepts_full_kit_store_bundle_doc() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use semio::kit_backbone::{DevBackboneBundleDoc, KIT_BUNDLE_HASH_STUB};

        let (server, base) = spawn_server().await?;
        let client = reqwest::Client::new();

        let mut b = DevBackboneBundleDoc::initialize_with_unsaved_change(
            "00000000-0000-7000-8000-0000000000cc",
            "change-bundle-1",
            "ck-bundle-1",
        );
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
            r#"{ store { wip { initialKit { name version } theKit { kit { name version } } } } }"#,
            None,
        )
        .await?;
        if q.get("errors").is_some() {
            return Err(format!("bundle install query: {q}").into());
        }
        assert_eq!(q.pointer("/data/store/wip/initialKit/name").and_then(|n| n.as_str()), Some("BundleInstallName"));
        assert_eq!(q.pointer("/data/store/wip/initialKit/version").and_then(|n| n.as_str()), Some("v-bundle-smoke"));
        assert_eq!(q.pointer("/data/store/wip/theKit/kit/name").and_then(|n| n.as_str()), Some("BundleInstallName"));
        assert_eq!(q.pointer("/data/store/wip/theKit/kit/version").and_then(|n| n.as_str()), Some("v-bundle-smoke"));

        server.abort();
        Ok(())
    }

    #[tokio::test]
    async fn sidecar_preview_reads_wip_materialization_without_install() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let (server, base) = spawn_server().await?;
        let client = reqwest::Client::new();
        const Q_PREVIEW: &str = r#"query WipKit {
  store {
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
}"#;
        let body = json!({ "query": Q_PREVIEW, "operationName": "WipKit" });
        let r = client.post(format!("{base}/graphql")).json(&body).send().await?;
        assert_eq!(r.status(), reqwest::StatusCode::OK);
        let v: Value = r.json().await?;
        assert!(v.get("errors").is_none(), "preview wip query errors: {v:?}");
        assert_eq!(v.pointer("/data/store/wip/initialKit/name").and_then(|n| n.as_str()), Some("the kit"));
        assert_eq!(v.pointer("/data/store/wip/theKit/kit/name").and_then(|n| n.as_str()), Some("the kit"));
        assert_eq!(v.pointer("/data/store/wip/checkpoints/edges/0/node/initial/name").and_then(|n| n.as_str()), Some("the kit"));
        assert_eq!(v.pointer("/data/store/wip/checkpoints/edges/0/node/kit/name").and_then(|n| n.as_str()), Some("the kit"));
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
