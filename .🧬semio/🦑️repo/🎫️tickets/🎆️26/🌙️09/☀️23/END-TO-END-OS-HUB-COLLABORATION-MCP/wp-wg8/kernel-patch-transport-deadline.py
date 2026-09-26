#!/usr/bin/env python3
"""WG8 s12 PREPARED KERNEL PATCH — written under the guest freeze, NOT compiled yet (first build after applying may need
import/path touch-ups in the test: `PackageId`/`ActorId` come from `semio_framework_actor`, `TraceId`/`CancelToken` from
`semio_framework_async`).
 (apply only after W2's `--packages all` publish, rule 20): the native directory transport
honours the caller's deadline instead of a fixed 15 s overall cap.

Measured (hub 7800, load ~50, 16:2x): mint `POST /auth/sessions` 9–46 s, `GET /directory/spaces` 25–58 s. The ureq agent's
fixed `.timeout(15 s)` + `.timeout_read(15 s)` cut every such request at 15 s, so the shell's own 30 s sign-in deadline
(`HUB_SIGN_IN_DEADLINE_MS`) never applied, sign-in answered `Unreachable`, and a create-space's list reload failed.

Change: the agent bounds only the TCP connect; every request carries its own ureq overall timeout = the time left on the
caller's `OperationContext` deadline (measured on the runtime clock `run_io` compares against), or
`UREQ_HTTP_UNBOUNDED_REQUEST_MS` (120 s) for a request with no deadline — a backstop that frees the blocking IO thread,
since `ComputePool::run_in_lane` cannot interrupt a blocking call. The body keeps its between-page stall bound
(`UREQ_HTTP_READ_TIMEOUT_MS`, the pool's `read_deadline` handle). Laws: the pure budget is held to a shared fixture
(`🪪️runtime/🧫️fixtures/⏱️request-budget.json`), and a slow local server proves a head that takes longer than the old
cap's analogue arrives inside a longer caller deadline and is refused past a shorter one.

Usage: python3 kernel-patch-transport-deadline.py   (idempotence guarded by exact-match asserts)
"""
import json
import pathlib

CLIENT = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client")
SOURCE = CLIENT / "🦀️.rs"
RUNTIME_TESTS = CLIENT / "🪪️runtime/🧪️tests/🪪️runtime/🦀️.rs"
FIXTURE = CLIENT / "🪪️runtime/🧫️fixtures/⏱️request-budget.json"

text = SOURCE.read_text()


def swap(old, new):
    global text
    assert text.count(old) == 1, old[:100]
    text = text.replace(old, new)


swap(
    "    const UREQ_HTTP_READ_TIMEOUT_MS: u64 = 15_000;\n",
    "    const UREQ_HTTP_READ_TIMEOUT_MS: u64 = 15_000;\n"
    "    const UREQ_HTTP_CONNECT_TIMEOUT_MS: u64 = 15_000;\n"
    "    /// ⏱️ The overall bound of a request whose caller named no deadline: long enough for a loaded hub's slowest\n"
    "    /// answer and a 64 MiB execution-target body, finite so the blocking IO thread it occupies is freed.\n"
    "    pub const UREQ_HTTP_UNBOUNDED_REQUEST_MS: u64 = 120_000;\n"
    "\n"
    "    /// ⏱️ One request's ureq overall timeout: what is left of the caller's deadline (at least 1 ms), else\n"
    "    /// [`UREQ_HTTP_UNBOUNDED_REQUEST_MS`]. `now_ms` is the runtime clock `ComputePool::run_io` compares the\n"
    "    /// deadline against, so the transport never cuts a request the caller still waits for.\n"
    "    pub fn ureq_request_budget_ms(deadline_ms: Option<u64>, now_ms: u64) -> u64 {\n"
    "        deadline_ms.map_or(UREQ_HTTP_UNBOUNDED_REQUEST_MS, |deadline| deadline.saturating_sub(now_ms).max(1))\n"
    "    }\n",
)
swap(
    "            Self { agent: ureq::AgentBuilder::new().timeout(Duration::from_millis(UREQ_HTTP_READ_TIMEOUT_MS)).timeout_read(Duration::from_millis(UREQ_HTTP_READ_TIMEOUT_MS)).build(), compute, runtime, scope }",
    "            Self { agent: ureq::AgentBuilder::new().timeout_connect(Duration::from_millis(UREQ_HTTP_CONNECT_TIMEOUT_MS)).build(), compute, runtime, scope }",
)
swap(
    "                let (head, reader) = connect_compute\n"
    "                    .run_io(connect_runtime.as_ref(), &connect_scope, connect_ctx, move || {\n"
    "                        let _terminal = terminal_guard;\n"
    "                        ureq_stream_start(&agent, request)\n"
    "                    })",
    "                let budget_ms = ureq_request_budget_ms(connect_ctx.deadline_ms, connect_runtime.now_ms().await);\n"
    "                let (head, reader) = connect_compute\n"
    "                    .run_io(connect_runtime.as_ref(), &connect_scope, connect_ctx, move || {\n"
    "                        let _terminal = terminal_guard;\n"
    "                        ureq_stream_start(&agent, request, Duration::from_millis(budget_ms))\n"
    "                    })",
)
swap(
    "    fn ureq_stream_start(agent: &ureq::Agent, request: PoolHttpRequest) -> Result<(HttpResponseHead, UreqBodyReader), HttpPoolError> {",
    "    fn ureq_stream_start(agent: &ureq::Agent, request: PoolHttpRequest, budget: Duration) -> Result<(HttpResponseHead, UreqBodyReader), HttpPoolError> {",
)
swap(
    "            for (name, value) in &request.headers {\n                builder = builder.set(name, value);\n            }\n            let outcome =",
    "            builder = builder.timeout(budget);\n            for (name, value) in &request.headers {\n                builder = builder.set(name, value);\n            }\n            let outcome =",
)
SOURCE.write_text(text)

FIXTURE.write_text(
    json.dumps(
        {
            "$comment": "⏱️ The native directory transport's per-request budget (`ureq_request_budget_ms`): the caller's deadline governs, a request without one gets the unbounded backstop. Ticket 26/09/23 WG8.",
            "unboundedRequestMs": 120000,
            "cases": [
                {"id": "no-deadline-takes-the-backstop", "deadlineMs": None, "nowMs": 5000, "budgetMs": 120000},
                {"id": "sign-in-deadline-outlives-the-old-15s-cap", "deadlineMs": 35000, "nowMs": 5000, "budgetMs": 30000},
                {"id": "command-deadline", "deadlineMs": 10000, "nowMs": 5000, "budgetMs": 5000},
                {"id": "a-passed-deadline-still-asks-for-one-millisecond", "deadlineMs": 4000, "nowMs": 5000, "budgetMs": 1},
                {"id": "the-deadline-instant", "deadlineMs": 5000, "nowMs": 5000, "budgetMs": 1},
            ],
        },
        indent=2,
    )
    + "\n"
)

tests = RUNTIME_TESTS.read_text()
LAWS = r'''

//#region ⏱️RequestBudget
/// ⏱️ The per-request budget follows the caller's deadline (shared fixture `⏱️request-budget.json`).
#[test]
fn every_request_budget_is_what_the_callers_deadline_leaves() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/⏱️request-budget.json")).unwrap();
    assert_eq!(fixture["unboundedRequestMs"], UREQ_HTTP_UNBOUNDED_REQUEST_MS);
    for case in fixture["cases"].as_array().unwrap() {
        assert_eq!(ureq_request_budget_ms(case["deadlineMs"].as_u64(), case["nowMs"].as_u64().unwrap()), case["budgetMs"].as_u64().unwrap(), "{case}");
    }
}

/// 🐢️ A hub whose answer head takes `delay` arrives inside a longer caller deadline and is refused past a
/// shorter one — the transport's bound is the caller's, never a fixed cap of its own.
#[semio_framework_async_macros::async_test]
async fn a_slow_answer_arrives_inside_the_callers_deadline_and_not_after_it() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}/slow", listener.local_addr().unwrap());
    let server = std::thread::spawn(move || {
        for _ in 0..2 {
            let (mut stream, _) = listener.accept().unwrap();
            let mut request = [0u8; 1024];
            let _ = std::io::Read::read(&mut stream, &mut request);
            std::thread::sleep(std::time::Duration::from_millis(600));
            let _ = std::io::Write::write_all(&mut stream, b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nok");
        }
    });
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 2));
    let runtime = Arc::new(TokioHostRuntime::with_pool(pool.clone()));
    let compute = Arc::new(ComputePool::with_pool(2, pool.clone()));
    let scope = runtime.open_scope_now(ScopeOwner::Service("directory-request-budget"), None);
    let transport = NativeDirectoryTransport::with_new_http_pool_now(runtime.clone(), scope.clone(), compute.clone(), 1 << 20, 2, PackageId("os.directory-client".into()), ActorId(0));
    let context = |budget_ms: u64, now_ms: u64| OperationContext { actor: 0, generation: 0, trace: semio_framework_async::TraceId(0), lane: 0, deadline_ms: Some(now_ms + budget_ms), cancel: semio_framework_async::CancelToken::root_now(), capability: None };
    let now = runtime.now_ms().await;
    let inside = transport.http(&context(5_000, now), HttpMethod::Get, &url, None, None).await;
    let now = runtime.now_ms().await;
    let past = transport.http(&context(200, now), HttpMethod::Get, &url, None, None).await;
    server.join().unwrap();
    assert_eq!(inside.map(|response| (response.status, response.body)), Ok((200, b"ok".to_vec())));
    assert!(past.is_err(), "a 600 ms head past a 200 ms deadline is refused");
    drop(transport);
    drop(compute);
    let _ = runtime.cancel_scope(&scope.owner, 0).await;
    drop(scope);
    drop(runtime);
    pool.shutdown().expect("request budget pool shuts down");
}
//#endregion ⏱️RequestBudget
'''
if "every_request_budget_is_what_the_callers_deadline_leaves" not in tests:
    RUNTIME_TESTS.write_text(tests.rstrip("\n") + LAWS)
print("kernel transport deadline patch applied — now: cargo test -p semio-framework-os-kernel --lib --features native-directory -- runtime_identity_tests (and wasm32 checks)")
