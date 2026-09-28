#!/usr/bin/env python3
"""🔁️ H13 prepared patch (window 3; services + kernel are frozen, os-mcp lands with them so it is never red before the
chain's os-mcp build): the semio MCP gateway's hub lane wedged into permanent "hub directory is temporarily unavailable
(transport)" refusals because no native directory transport ever refilled its HttpPool byte budget.

  services  `TokioHostRuntime::worker_pool()`; `HTTP_BUCKET_REFILL_INTERVAL_MS` is the public production interval.
  kernel    `NativeDirectoryTransport::with_new_http_pool_now` starts its pool's refill driver (covers the MCP gateway,
            the wgpu shell's directory client and the renderer's socket-grant probe: all build through it); law
            `an_exhausted_directory_byte_budget_names_itself_and_refills_on_the_pools_own_turn` + fixture
            `🪪️runtime/🧫️fixtures/🔁️byte-budget-refill.json`.
  os-mcp    🔗️remote keeps a transport fault's own detail (`HubUnavailableCause::Transport { detail }`, bounded) and a
            hub-unavailable refusal carries `details.cause` (schema `HubUnavailableCauseV1`) + an en/de `summary`;
            schema defs, fixture `🔣️hub-unavailable-refusal.json` (generated here from the Rust formats), laws
            `a_hub_unavailable_refusal_names_its_typed_cause_in_english_and_german` and
            `a_transport_fault_keeps_its_cause_and_the_next_refresh_recovers_the_binding` (injected fault, then recovery),
            Ajv + independent-derivation oracle case in `🧪️tests/🔐️authenticated-hub-workspace/🟦️.ts`.

Idempotent: every edit is skipped when its result is present; `--dry-run` reports without writing.
usage: python3 h13-transport-refill-patch.py [--dry-run]"""
import json, sys
from pathlib import Path

DRY = "--dry-run" in sys.argv
ROOT = Path("/Users/ueli/Documents/semio")
MOD = "🧰️framework/🛍️products/💻️os/🔨️modules"
SERVICES = f"{MOD}/🛎️services/🦀️.rs"
CLIENT = f"{MOD}/📇️directory/🔌️client/🦀️.rs"
CLIENT_LAW = f"{MOD}/📇️directory/🔌️client/🪪️runtime/🧪️tests/🪪️runtime/🦀️.rs"
CLIENT_FIXTURE = f"{MOD}/📇️directory/🔌️client/🪪️runtime/🧫️fixtures/🔁️byte-budget-refill.json"
REMOTE = f"{MOD}/🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs"
REMOTE_LAW = f"{MOD}/🌉️mcp/🏠️workspace/🔗️remote/🧪️tests/🔬️unit/🦀️.rs"
REMOTE_SCHEMA = f"{MOD}/🌉️mcp/🏠️workspace/🔗️remote/🧬️schema/🔣️.json"
REMOTE_FIXTURE = f"{MOD}/🌉️mcp/🏠️workspace/🔗️remote/🧫️fixtures/🔣️hub-unavailable-refusal.json"
REMOTE_ORACLE = f"{MOD}/🌉️mcp/🧪️tests/🔐️authenticated-hub-workspace/🟦️.ts"
DETAIL_MAX_CHARS = 512

EDITS = [
    (SERVICES, r'''/// 🐌️ How often [`HttpPool::spawn_refill_driver`] tops every tracked package's bucket back toward
/// its `network_bytes_per_min` cap — the PRODUCTION `interval_ms` argument that fn's real caller (a
/// process bootstrap, out of this packet's boundary — see the packet report's `## honest gaps`)
/// passes; this crate's own tests pass a short interval instead, so nothing in THIS crate reads this
/// const today.
#[allow(dead_code)]
const HTTP_BUCKET_REFILL_INTERVAL_MS: u64 = 60_000;''', r'''/// 🐌️ How often [`HttpPool::spawn_refill_driver`] tops every tracked package's bucket back to its
/// `network_bytes_per_min` cap in production: `NativeDirectoryTransport::with_new_http_pool_now` starts its
/// pool's driver at this interval, so the budget is an allowance per minute, never a lifetime one. This
/// crate's own tests pass a short interval instead.
pub const HTTP_BUCKET_REFILL_INTERVAL_MS: u64 = 60_000;'''),
    (SERVICES, r'''    pub fn with_pool(pool: WorkerPool) -> TokioHostRuntime {
        TokioHostRuntime { scopes: ScopeTable::new(pool.clone()), pool }
    }
''', r'''    pub fn with_pool(pool: WorkerPool) -> TokioHostRuntime {
        TokioHostRuntime { scopes: ScopeTable::new(pool.clone()), pool }
    }

    /// 🧵️ The worker pool this runtime schedules onto, for a service that submits its own maintenance
    /// turns to the same threads (an [`HttpPool`]'s refill driver).
    pub fn worker_pool(&self) -> &WorkerPool {
        &self.pool
    }
'''),
    (CLIENT, r'''HttpResponseHead, HttpTransportStart, HttpTransportTerminalGuard, TokioHostRuntime,
    };''', r'''HttpResponseHead, HttpTransportStart, HttpTransportTerminalGuard, TokioHostRuntime,
        HTTP_BUCKET_REFILL_INTERVAL_MS,
    };'''),
    (CLIENT, r'''        pub fn with_new_http_pool_now(runtime: Arc<TokioHostRuntime>, scope: ScopeHandle, compute: Arc<ComputePool>, bytes_per_minute_cap: u64, outstanding_cap: u32, package: PackageId, actor: ActorId) -> Self {
            let transport: Arc<dyn AsyncHttpTransport> = Arc::new(UreqStreamingHttpTransport::new(compute, runtime.clone(), scope.clone()));
            Self::new_now(runtime, scope, Arc::new(HttpPool::new_with_async_transport_now(transport, bytes_per_minute_cap, outstanding_cap)), package, actor)
        }
    }
''', r'''        /// 🔁️ A transport over its own fresh [`HttpPool`] whose per-package byte budget refills every
        /// [`HTTP_BUCKET_REFILL_INTERVAL_MS`] on the runtime's worker pool. Without that driver the "per minute"
        /// budget was a lifetime one: a long-lived client (the semio MCP gateway, a wgpu shell's directory
        /// client) that had moved `bytes_per_minute_cap` bytes answered every later request with an exhausted
        /// budget, forever (ticket 26/09/23, session 14b).
        pub fn with_new_http_pool_now(runtime: Arc<TokioHostRuntime>, scope: ScopeHandle, compute: Arc<ComputePool>, bytes_per_minute_cap: u64, outstanding_cap: u32, package: PackageId, actor: ActorId) -> Self {
            let http_pool = refilled_http_pool(&runtime, &scope, compute, bytes_per_minute_cap, outstanding_cap, HTTP_BUCKET_REFILL_INTERVAL_MS);
            Self::new_now(runtime, scope, http_pool, package, actor)
        }
    }

    /// 🚰️ One fresh [`HttpPool`] over the ureq streaming transport whose refill driver already runs on
    /// `runtime`'s worker pool every `refill_interval_ms`.
    fn refilled_http_pool(runtime: &Arc<TokioHostRuntime>, scope: &ScopeHandle, compute: Arc<ComputePool>, bytes_per_minute_cap: u64, outstanding_cap: u32, refill_interval_ms: u64) -> Arc<HttpPool> {
        let transport: Arc<dyn AsyncHttpTransport> = Arc::new(UreqStreamingHttpTransport::new(compute, runtime.clone(), scope.clone()));
        let http_pool = Arc::new(HttpPool::new_with_async_transport_now(transport, bytes_per_minute_cap, outstanding_cap));
        http_pool.spawn_refill_driver(runtime.worker_pool(), refill_interval_ms);
        http_pool
    }
'''),
    (CLIENT_LAW, r'''    pool.shutdown().expect("request budget pool shuts down");
}
//#endregion ⏱️RequestBudget
''', r'''    pool.shutdown().expect("request budget pool shuts down");
}
//#endregion ⏱️RequestBudget

//#region 🔁️ByteBudgetRefill
/// 🔁️ A directory transport's byte budget is an allowance per refill turn, never a lifetime one: once exhausted,
/// a request is refused naming that cause, and the pool's own driver admits requests again on its next turn (shared
/// fixture `🔁️byte-budget-refill.json`; the semio MCP gateway wedged on the lifetime budget, ticket 26/09/23 14b).
#[semio_framework_async_macros::async_test]
async fn an_exhausted_directory_byte_budget_names_itself_and_refills_on_the_pools_own_turn() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔁️byte-budget-refill.json")).unwrap();
    assert_eq!(fixture["productionRefillIntervalMs"], HTTP_BUCKET_REFILL_INTERVAL_MS);
    let body = fixture["body"].as_str().unwrap().to_string();
    let admitted = usize::try_from(fixture["admittedRequests"].as_u64().unwrap()).unwrap();
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let url = format!("http://{address}/budget");
    let answer = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len());
    let server = std::thread::spawn(move || {
        for _ in 0..admitted {
            let (mut stream, _) = listener.accept().unwrap();
            let mut request = [0u8; 1024];
            let _ = stream.read(&mut request);
            let _ = std::io::Write::write_all(&mut stream, answer.as_bytes());
        }
    });
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 2));
    let runtime = Arc::new(TokioHostRuntime::with_pool(pool.clone()));
    let compute = Arc::new(ComputePool::with_pool(2, pool.clone()));
    let scope = runtime.open_scope_now(ScopeOwner::Service("directory-byte-budget-refill"), None);
    let budget = u64::try_from(url.len() + body.len()).unwrap();
    let http_pool = refilled_http_pool(&runtime, &scope, compute.clone(), budget, 2, fixture["refillIntervalMs"].as_u64().unwrap());
    let transport = NativeDirectoryTransport::new_now(runtime.clone(), scope.clone(), http_pool, PackageId("os.directory-client".into()), ActorId(0));
    let context = || OperationContext { actor: 0, generation: 0, trace: semio_framework_async::TraceId(0), lane: 0, deadline_ms: None, cancel: semio_framework_async::CancelToken::root_now(), capability: None };
    let first = transport.http(&context(), HttpMethod::Get, &url, None, None).await;
    let refused = transport.http(&context(), HttpMethod::Get, &url, None, None).await;
    let started = std::time::Instant::now();
    let refilled = loop {
        match transport.http(&context(), HttpMethod::Get, &url, None, None).await {
            Ok(response) => break Some(response),
            Err(_) if started.elapsed() < Duration::from_secs(fixture["refillBoundSeconds"].as_u64().unwrap()) => std::thread::sleep(Duration::from_millis(20)),
            Err(_) => break None,
        }
    };
    if refilled.is_none() {
        let _ = std::net::TcpStream::connect(address);
    }
    server.join().unwrap();
    assert_eq!(first.map(|response| (response.status, response.body)), Ok((200, body.as_bytes().to_vec())));
    assert!(matches!(&refused, Err(TransportError::Io(detail)) if detail.contains(fixture["refusalFragment"].as_str().unwrap())), "an exhausted budget names itself: {refused:?}");
    assert_eq!(refilled.map(|response| (response.status, response.body)), Some((200, body.as_bytes().to_vec())), "the pool's own refill turn admits requests again");
    drop(transport);
    drop(compute);
    let _ = runtime.cancel_scope(&scope.owner, 0).await;
    drop(scope);
    drop(runtime);
    pool.shutdown().expect("byte budget pool shuts down");
}
//#endregion 🔁️ByteBudgetRefill
'''),
    (REMOTE, r'''pub const HUB_BINDING_DIAGNOSTIC_MAX_BYTES: usize = 4_096;
''', r'''pub const HUB_BINDING_DIAGNOSTIC_MAX_BYTES: usize = 4_096;
/// ✂️ The longest transport detail a hub-unavailable refusal carries (characters), schema `HubUnavailableCauseV1`.
pub const HUB_UNAVAILABLE_DETAIL_MAX_CHARS: usize = 512;
'''),
    (REMOTE, r'''/// 🔎️ What made the hub directory unavailable, so a refusal names it instead of one opaque word.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HubUnavailableCause {
    /// 🌐️ The HTTP status and, when the hub answered its typed refusal, that refusal's `code`.
    Http { status: u16, code: Option<String> },
    Transport,
    UndecodableResponse,
}

impl HubUnavailableCause {
    /// 🌐️ An HTTP refusal, keeping the hub's typed `code` (bounded) when its body carries one.
    fn http(status: u16, body: &str) -> Self {
        let code = serde_json::from_str::<serde_json::Value>(body).ok().and_then(|value| value.get("code").and_then(serde_json::Value::as_str).map(|code| code.chars().take(64).collect()));
        Self::Http { status, code }
    }
}
''', r'''/// 🔎️ What made the hub directory unavailable, so a refusal names it instead of one opaque word
/// (schema `HubUnavailableCauseV1`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HubUnavailableCause {
    /// 🌐️ The HTTP status and, when the hub answered its typed refusal, that refusal's `code`.
    Http { status: u16, code: Option<String> },
    /// 🔌️ The transport failed before any HTTP answer; `detail` is its own bounded cause (an exhausted byte
    /// budget, a refused connection), never dropped.
    Transport { detail: String },
    UndecodableResponse,
}

impl HubUnavailableCause {
    /// 🌐️ An HTTP refusal, keeping the hub's typed `code` (bounded) when its body carries one.
    fn http(status: u16, body: &str) -> Self {
        let code = serde_json::from_str::<serde_json::Value>(body).ok().and_then(|value| value.get("code").and_then(serde_json::Value::as_str).map(|code| code.chars().take(64).collect()));
        Self::Http { status, code }
    }

    /// 🔌️ A transport failure, keeping its own detail bounded to [`HUB_UNAVAILABLE_DETAIL_MAX_CHARS`].
    fn transport(error: &semio_framework_os_kernel::os_directory::client::TransportError) -> Self {
        let detail = match error {
            semio_framework_os_kernel::os_directory::client::TransportError::Io(detail) => detail.clone(),
            other => other.to_string(),
        };
        Self::Transport { detail: detail.chars().take(HUB_UNAVAILABLE_DETAIL_MAX_CHARS).collect() }
    }

    /// 🧾️ The typed `HubUnavailableCauseV1` a refusal's `details.cause` carries.
    fn to_json(&self) -> serde_json::Value {
        match self {
            Self::Http { status, code } => serde_json::json!({ "kind": "http", "status": status, "code": code }),
            Self::Transport { detail } => serde_json::json!({ "kind": "transport", "detail": detail }),
            Self::UndecodableResponse => serde_json::json!({ "kind": "undecodable-response" }),
        }
    }

    /// 🗣️ The en + de sentence a client shows for this cause (`details.summary`).
    fn summary(&self) -> serde_json::Value {
        let (en, de) = match self {
            Self::Http { status, code: Some(code) } => (
                format!("The hub answered HTTP {status} ({code}); the request is retried once the hub recovers."),
                format!("Der Hub antwortete mit HTTP {status} ({code}); die Anfrage wird wiederholt, sobald der Hub wieder bereit ist."),
            ),
            Self::Http { status, code: None } => (
                format!("The hub answered HTTP {status}; the request is retried once the hub recovers."),
                format!("Der Hub antwortete mit HTTP {status}; die Anfrage wird wiederholt, sobald der Hub wieder bereit ist."),
            ),
            Self::Transport { detail } => (
                format!("The hub could not be reached ({detail}); the request is retried once the connection recovers."),
                format!("Der Hub war nicht erreichbar ({detail}); die Anfrage wird wiederholt, sobald die Verbindung wieder steht."),
            ),
            Self::UndecodableResponse => (
                "The hub's answer could not be read; the request is retried once the hub recovers.".to_string(),
                "Die Antwort des Hubs war nicht lesbar; die Anfrage wird wiederholt, sobald der Hub wieder bereit ist.".to_string(),
            ),
        };
        serde_json::json!({ "en": en, "de": de })
    }
}
'''),
    (REMOTE, r'''            Self::Unavailable(HubUnavailableCause::Transport) => formatter.write_str("hub directory is temporarily unavailable (transport)"),''',
     r'''            Self::Unavailable(HubUnavailableCause::Transport { detail }) => write!(formatter, "hub directory is temporarily unavailable (transport: {detail})"),'''),
    (REMOTE, r'''        DirectoryClientError::Transport(_) => HubBindingError::Unavailable(HubUnavailableCause::Transport),''',
     r'''        DirectoryClientError::Transport(error) => HubBindingError::Unavailable(HubUnavailableCause::transport(&error)),'''),
    (REMOTE, r'''    let gateway = GatewayError::new(code, error.to_string());
    if matches!(code, GatewayErrorCode::PluginUnavailable) { gateway.retryable() } else { gateway }''', r'''    let gateway = match &error {
        HubBindingError::Unavailable(cause) => GatewayError::new(code, error.to_string()).with_details(serde_json::json!({ "cause": cause.to_json(), "summary": cause.summary() })),
        _ => GatewayError::new(code, error.to_string()),
    };
    if matches!(code, GatewayErrorCode::PluginUnavailable) { gateway.retryable() } else { gateway }'''),
    (REMOTE_LAW, r'''#[derive(Clone)]
struct RecordingTransport {
    responses: Arc<Mutex<VecDeque<HttpResponse>>>,
    requests: Arc<Mutex<Vec<(HttpMethod, String, bool)>>>,
}''', r'''#[derive(Clone)]
struct RecordingTransport {
    faults: Arc<Mutex<VecDeque<TransportError>>>,
    responses: Arc<Mutex<VecDeque<HttpResponse>>>,
    requests: Arc<Mutex<Vec<(HttpMethod, String, bool)>>>,
}'''),
    (REMOTE_LAW, r'''        self.requests.lock().unwrap().push((method, url.to_string(), bearer.is_some()));
        self.responses.lock().unwrap().pop_front().ok_or_else(|| TransportError::Io("fixture response exhausted".to_string()))''', r'''        self.requests.lock().unwrap().push((method, url.to_string(), bearer.is_some()));
        if let Some(fault) = self.faults.lock().unwrap().pop_front() {
            return Err(fault);
        }
        self.responses.lock().unwrap().pop_front().ok_or_else(|| TransportError::Io("fixture response exhausted".to_string()))'''),
    (REMOTE_LAW, r'''fn client_for(case: &serde_json::Value) -> (DirectoryClient<RecordingTransport>, Arc<Mutex<Vec<(HttpMethod, String, bool)>>>) {
    let responses''', r'''fn client_for(case: &serde_json::Value) -> (DirectoryClient<RecordingTransport>, Arc<Mutex<Vec<(HttpMethod, String, bool)>>>) {
    faulted_client_for(case, Vec::new())
}

/// 💥️ [`client_for`] whose first requests fail with the injected transport `faults`, in order, before any response.
fn faulted_client_for(case: &serde_json::Value, faults: Vec<TransportError>) -> (DirectoryClient<RecordingTransport>, Arc<Mutex<Vec<(HttpMethod, String, bool)>>>) {
    let responses'''),
    (REMOTE_LAW, r'''    let transport = RecordingTransport { responses: Arc::new(Mutex::new(responses)), requests: requests.clone() };''',
     r'''    let transport = RecordingTransport { faults: Arc::new(Mutex::new(faults.into())), responses: Arc::new(Mutex::new(responses)), requests: requests.clone() };'''),
    (REMOTE_LAW, r'''    let transport = RecordingTransport {
        responses:''', r'''    let transport = RecordingTransport {
        faults: Arc::default(),
        responses:'''),
    (REMOTE_LAW, r'''#[test]
fn a_directory_dial_refusal_names_its_cause_and_stays_retryable() {''', r'''/// 🔌️ A hub-unavailable refusal keeps its cause typed and bilingual (shared fixture `🔣️hub-unavailable-refusal.json`):
/// a transport fault names its own detail (an exhausted byte budget, a refused connection) instead of the bare word
/// "transport" the semio MCP answered while its gateway was wedged (ticket 26/09/23, session 14b).
#[test]
fn a_hub_unavailable_refusal_names_its_typed_cause_in_english_and_german() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️hub-unavailable-refusal.json")).unwrap();
    assert_eq!(corpus["detailMaxChars"], HUB_UNAVAILABLE_DETAIL_MAX_CHARS);
    for case in corpus["cases"].as_array().unwrap() {
        let fault = &case["fault"];
        let error = match fault["kind"].as_str().unwrap() {
            "transport" => DirectoryClientError::Transport(TransportError::Io(fault["io"].as_str().unwrap().to_string())),
            "http" => DirectoryClientError::Http { status: u16::try_from(fault["status"].as_u64().unwrap()).unwrap(), body: fault["body"].as_str().unwrap().to_string() },
            "decode" => DirectoryClientError::Decode(fault["decode"].as_str().unwrap().to_string()),
            other => panic!("unknown fault kind {other}"),
        };
        assert_eq!(serde_json::to_value(binding_error_to_gateway(map_client_error(error))).unwrap(), case["expected"], "{}", case["id"]);
    }
}

/// 🔁️ An injected transport fault leaves the binding refreshing (never revoked) with its own cause as the last fault,
/// and the next refresh over a healthy transport binds it again: a transient transport failure is recoverable, never
/// a wedge (ticket 26/09/23, session 14b).
#[tokio::test]
async fn a_transport_fault_keeps_its_cause_and_the_next_refresh_recovers_the_binding() {
    let contract = fixture();
    let exhausted = "package \"semio-framework-os-mcp\" exhausted its network_bytes_per_min budget";
    let (client, requests) = faulted_client_for(&contract["cases"]["memberReady"], vec![TransportError::Io(exhausted.to_string())]);
    let binding = HubRemoteBinding::new("http://hub.invalid", "space-a").unwrap();
    let fault = binding.refresh(&client, &context(Some(20_000)), 1_000, 10_000).await.unwrap_err();
    assert_eq!(fault, HubBindingError::Unavailable(HubUnavailableCause::Transport { detail: exhausted.to_string() }));
    assert!(matches!(binding.state(), HubRemoteBindingState::Refreshing), "a transport fault never revokes the binding");
    let refusal = binding.ready_snapshot(1_000).unwrap_err();
    assert!(refusal.retryable);
    assert!(refusal.details["lastFault"].as_str().is_some_and(|last| last.contains(exhausted)), "the refusal names the fault: {:?}", refusal.details);
    let snapshot = binding.refresh(&client, &context(Some(20_000)), 1_000, 10_000).await.expect("the next refresh over a healthy transport binds again");
    assert_eq!(snapshot.authenticated_user_id, "user-a");
    assert!(binding.ready_snapshot(1_000).is_ok());
    assert_eq!(binding.diagnostic(), None);
    assert_eq!(requests.lock().unwrap().len(), 3, "one faulted request, then the session and the space page");
}

#[test]
fn a_directory_dial_refusal_names_its_cause_and_stays_retryable() {'''),
    (REMOTE_SCHEMA, r'''    "Generation": { "type": "integer", "minimum": 1, "maximum": 9007199254740991 },
''', r'''    "Generation": { "type": "integer", "minimum": 1, "maximum": 9007199254740991 },
    "HubUnavailableCauseV1": {
      "description": "🔎️ What made the hub directory unavailable (os-mcp 🔗️remote `HubUnavailableCause`): an HTTP answer with its typed hub code, a transport fault with its own bounded detail, or an unreadable answer.",
      "oneOf": [
        {
          "type": "object",
          "additionalProperties": false,
          "required": ["kind", "status", "code"],
          "properties": {
            "kind": { "const": "http" },
            "status": { "type": "integer", "minimum": 100, "maximum": 599 },
            "code": { "oneOf": [{ "type": "string", "minLength": 1, "maxLength": 64 }, { "type": "null" }] }
          }
        },
        {
          "type": "object",
          "additionalProperties": false,
          "required": ["kind", "detail"],
          "properties": { "kind": { "const": "transport" }, "detail": { "type": "string", "minLength": 1, "maxLength": 512 } }
        },
        { "type": "object", "additionalProperties": false, "required": ["kind"], "properties": { "kind": { "const": "undecodable-response" } } }
      ]
    },
    "HubUnavailableRefusalDetailsV1": {
      "description": "🗣️ The `details` of a retryable PLUGIN_UNAVAILABLE refusal the hub binding answers: its typed cause and the en + de sentence a client shows.",
      "type": "object",
      "additionalProperties": false,
      "required": ["cause", "summary"],
      "properties": {
        "cause": { "$ref": "#/$defs/HubUnavailableCauseV1" },
        "summary": {
          "type": "object",
          "additionalProperties": false,
          "required": ["en", "de"],
          "properties": { "en": { "type": "string", "minLength": 1 }, "de": { "type": "string", "minLength": 1 } }
        }
      }
    },
    "HubUnavailableRefusalCorpusV1": {
      "description": "🧪️ Neutral corpus: one directory-client fault per case and the exact gateway refusal it becomes.",
      "type": "object",
      "additionalProperties": false,
      "required": ["schema", "detailMaxChars", "cases"],
      "properties": {
        "schema": { "const": "semio.os-mcp.hub-unavailable-refusal-corpus/v1" },
        "detailMaxChars": { "const": 512 },
        "cases": {
          "type": "array",
          "minItems": 6,
          "items": {
            "type": "object",
            "additionalProperties": false,
            "required": ["id", "fault", "expected"],
            "properties": {
              "id": { "type": "string", "pattern": "^[a-z0-9-]+$" },
              "fault": {
                "oneOf": [
                  { "type": "object", "additionalProperties": false, "required": ["kind", "io"], "properties": { "kind": { "const": "transport" }, "io": { "type": "string", "minLength": 1 } } },
                  { "type": "object", "additionalProperties": false, "required": ["kind", "status", "body"], "properties": { "kind": { "const": "http" }, "status": { "type": "integer", "minimum": 100, "maximum": 599 }, "body": { "type": "string" } } },
                  { "type": "object", "additionalProperties": false, "required": ["kind", "decode"], "properties": { "kind": { "const": "decode" }, "decode": { "type": "string" } } }
                ]
              },
              "expected": {
                "type": "object",
                "additionalProperties": false,
                "required": ["code", "message", "details", "retryable"],
                "properties": {
                  "code": { "const": "PLUGIN_UNAVAILABLE" },
                  "message": { "type": "string", "minLength": 1 },
                  "details": { "$ref": "#/$defs/HubUnavailableRefusalDetailsV1" },
                  "retryable": { "const": true }
                }
              }
            }
          }
        }
      }
    },
'''),
    (REMOTE_ORACLE, r'''const fixture = JSON.parse(readFileSync(resolve(remoteRoot, "🧫️fixtures/🔣️authenticated-hub-descriptor-index.json"), "utf8"));
''', r'''const fixture = JSON.parse(readFileSync(resolve(remoteRoot, "🧫️fixtures/🔣️authenticated-hub-descriptor-index.json"), "utf8"));
const refusalCorpus = JSON.parse(readFileSync(resolve(remoteRoot, "🧫️fixtures/🔣️hub-unavailable-refusal.json"), "utf8"));
'''),
    (REMOTE_ORACLE, r'''  test("remote authorization stays distinct from local principal claims and bearer material", () => {''', r'''  test("a hub-unavailable refusal keeps its typed cause bounded and bilingual", () => {
    const ajv = new Ajv({ strict: true, allErrors: true });
    ajv.addSchema(schema);
    const validateCorpus = ajv.getSchema(`${schema.$id}#/$defs/HubUnavailableRefusalCorpusV1`)!;
    expect(validateCorpus(refusalCorpus), JSON.stringify(validateCorpus.errors)).toBe(true);
    const typedCode = (body: string): string | null => {
      try {
        const code = JSON.parse(body)?.code;
        return typeof code === "string" ? [...code].slice(0, 64).join("") : null;
      } catch {
        return null;
      }
    };
    const kinds = new Set<string>();
    for (const entry of refusalCorpus.cases) {
      const fault = entry.fault;
      const cause =
        fault.kind === "transport"
          ? { kind: "transport", detail: [...fault.io].slice(0, refusalCorpus.detailMaxChars).join("") }
          : fault.kind === "http"
            ? { kind: "http", status: fault.status, code: typedCode(fault.body) }
            : { kind: "undecodable-response" };
      const said = cause.kind === "transport" ? `transport: ${cause.detail}` : cause.kind === "http" ? `HTTP ${cause.status}${cause.code === null ? "" : ` ${cause.code}`}` : "undecodable response";
      expect(entry.expected.details.cause, entry.id).toEqual(cause);
      expect(entry.expected.message, entry.id).toBe(`hub directory is temporarily unavailable (${said})`);
      expect(entry.expected.details.summary.en, entry.id).not.toBe(entry.expected.details.summary.de);
      if (cause.kind === "transport") for (const text of Object.values(entry.expected.details.summary)) expect(text, entry.id).toContain(cause.detail);
      kinds.add(cause.kind);
    }
    expect([...kinds].sort()).toEqual(["http", "transport", "undecodable-response"]);
    expect(refusalCorpus.cases.some((entry: any) => entry.fault.kind === "transport" && [...entry.fault.io].length > refusalCorpus.detailMaxChars)).toBe(true);
  });

  test("remote authorization stays distinct from local principal claims and bearer material", () => {'''),
]


def summary(cause):
    if cause["kind"] == "http" and cause["code"] is not None:
        return {"en": f"The hub answered HTTP {cause['status']} ({cause['code']}); the request is retried once the hub recovers.", "de": f"Der Hub antwortete mit HTTP {cause['status']} ({cause['code']}); die Anfrage wird wiederholt, sobald der Hub wieder bereit ist."}
    if cause["kind"] == "http":
        return {"en": f"The hub answered HTTP {cause['status']}; the request is retried once the hub recovers.", "de": f"Der Hub antwortete mit HTTP {cause['status']}; die Anfrage wird wiederholt, sobald der Hub wieder bereit ist."}
    if cause["kind"] == "transport":
        return {"en": f"The hub could not be reached ({cause['detail']}); the request is retried once the connection recovers.", "de": f"Der Hub war nicht erreichbar ({cause['detail']}); die Anfrage wird wiederholt, sobald die Verbindung wieder steht."}
    return {"en": "The hub's answer could not be read; the request is retried once the hub recovers.", "de": "Die Antwort des Hubs war nicht lesbar; die Anfrage wird wiederholt, sobald der Hub wieder bereit ist."}


def refusal_corpus() -> str:
    faults = [
        ("exhausted-byte-budget", {"kind": "transport", "io": 'package "semio-framework-os-mcp" exhausted its network_bytes_per_min budget'}),
        ("refused-connection", {"kind": "transport", "io": "Connection Failed: Connect error: Connection refused (os error 61)"}),
        ("overlong-transport-detail", {"kind": "transport", "io": "é" * 600}),
        ("busy-hub-typed-503", {"kind": "http", "status": 503, "body": '{"schema":"semio.hub.error/v1","code":"deadline-exceeded"}'}),
        ("gateway-502-without-typed-code", {"kind": "http", "status": 502, "body": "bad gateway"}),
        ("undecodable-page", {"kind": "decode", "decode": "expected value at line 1 column 1"}),
    ]
    cases = []
    for case_id, fault in faults:
        if fault["kind"] == "transport":
            cause = {"kind": "transport", "detail": fault["io"][:DETAIL_MAX_CHARS]}
            said = f"transport: {cause['detail']}"
        elif fault["kind"] == "http":
            try:
                code = json.loads(fault["body"]).get("code")
            except ValueError:
                code = None
            cause = {"kind": "http", "status": fault["status"], "code": code[:64] if isinstance(code, str) else None}
            said = f"HTTP {cause['status']}" + ("" if cause["code"] is None else f" {cause['code']}")
        else:
            cause = {"kind": "undecodable-response"}
            said = "undecodable response"
        cases.append({"id": case_id, "fault": fault, "expected": {"code": "PLUGIN_UNAVAILABLE", "message": f"hub directory is temporarily unavailable ({said})", "details": {"cause": cause, "summary": summary(cause)}, "retryable": True}})
    return json.dumps({"schema": "semio.os-mcp.hub-unavailable-refusal-corpus/v1", "detailMaxChars": DETAIL_MAX_CHARS, "cases": cases}, indent=2, ensure_ascii=False) + "\n"


def refill_fixture() -> str:
    return json.dumps({
        "$comment": "🔁️ A native directory transport's per-package byte budget is an allowance per refill turn, never a lifetime one: `with_new_http_pool_now` starts its pool's refill driver at `productionRefillIntervalMs`; the law exhausts a budget of exactly one request, expects the next one refused naming `refusalFragment`, and one admitted again within `refillBoundSeconds` of a `refillIntervalMs` driver. Ticket 26/09/23 H13 session 14b.",
        "productionRefillIntervalMs": 60000,
        "refillIntervalMs": 3000,
        "refillBoundSeconds": 15,
        "body": "ok",
        "admittedRequests": 2,
        "refusalFragment": "exhausted its network_bytes_per_min budget",
    }, indent=2, ensure_ascii=False) + "\n"


def main() -> int:
    texts: dict[str, str] = {}
    pending = problems = 0
    for path, old, new in EDITS:
        text = texts.setdefault(path, (ROOT / path).read_text(encoding="utf-8"))
        if new in text:
            print(f"applied already: {Path(path).parent.name}/{Path(path).name} :: {new.strip().splitlines()[0][:70]}")
            continue
        count = text.count(old)
        if count < 1 or (count > 1 and "DirectoryClientError::Transport(_)" not in old):
            print(f"PROBLEM anchor ×{count}: {Path(path).parent.name}/{Path(path).name} :: {old.strip().splitlines()[0][:70]}")
            problems += 1
            continue
        texts[path] = text.replace(old, new)
        pending += 1
        print(f"{'would apply' if DRY else 'apply'} (×{count}): {Path(path).parent.name}/{Path(path).name} :: {old.strip().splitlines()[0][:70]}")
    fixtures = {REMOTE_FIXTURE: refusal_corpus(), CLIENT_FIXTURE: refill_fixture()}
    for path, text in fixtures.items():
        current = (ROOT / path).read_text(encoding="utf-8") if (ROOT / path).exists() else None
        if current == text:
            print(f"fixture current: {Path(path).name}")
            continue
        pending += 1
        print(f"{'would write' if DRY else 'write'} fixture: {Path(path).name}")
        texts[path] = text
    if problems:
        print(f"{problems} problem(s); nothing written")
        return 1
    if not DRY:
        for path, text in texts.items():
            (ROOT / path).write_text(text, encoding="utf-8")
    print(f"{'dry run' if DRY else 'done'}: {pending} pending")
    return 0


if __name__ == "__main__":
    sys.exit(main())
