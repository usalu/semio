# 📓️ terra report — packet P1c-bridge-live

## 1. Preconditions

- Baseline `git rev-parse HEAD` at session start of THIS packet: `830f2a4269320c0c71ff5d4fea344b4be865ffa0`
  (already carries P1b, P2-catalog, P6-actions-policy, P7-headless-workspace, and other concurrent
  packets' landed work — confirmed by reading the live `🌉️mcp/🦀️component.rs`/`bin.rs` before touching
  anything, matching sol's brief: `run_http` existed but never mounted `/bridge`, and root
  `mod quick`'s `_bridge_frame = ShellToGateway::Ping;` was exactly as sol described).
- Verified sol's two specific claims directly before writing anything: `grep -n "run_http\|bridge_router\|ShellToGateway::Ping"`
  on `🦀️component.rs` confirmed `run_http` (then ~L664) built an `HttpTransport` and called
  `.serve(server)` with no `/bridge` route anywhere, and the placeholder line was exactly at L705.
  `🧵️bridge/🦀️component.rs`/`🚚️transport/🦀️component.rs` hashes matched byte-for-byte what P1b's own
  report recorded (`3cbcec0a…`/`4ee19ca5…`) — confirming no other session had touched my exclusive
  territory in between.
- Mid-session sol flagged an upstream `semio-framework-ui` compile error (peer presence-ticket churn,
  not mine to fix). By the time my own `cargo check` ran, it had already resolved (that session
  finished its edit) — the full build/test run below is clean, no upstream error encountered.
- SHA-256 (`shasum -a 256`) and line count (`wc -l`) of every file touched, taken after the final edit:

| file | lines | sha256 |
|---|---:|---|
| `🧵️bridge/🦀️component.rs` (extended: `BridgeHandle`/`BridgeToken`/real `BridgeServer`) | 982 | `b9e28f8e60df0a99fe4ba395b42fe80d6f7062e60b89806bae00e4821bdeabed` |
| `🚚️transport/🦀️component.rs` (extended: bridge mount, `pub(crate)` seams, merged-app test) | 772 | `dc3c290a1e2423a6b63f42a3e25d934185d70e7159573161c8267cd50d3506c1` |
| `🦀️component.rs` (root: `run_http` mints/writes/prints the bridge token) | 834 | `dd5cd2e55159199c1d0db4fb3ef2afb829942993a91ee907bd799508da191b09` |
| `📦️bin.rs` (`--bridge-token-file` flag) | 139 | `ed5dcd0caded50048f5abff602474caaef2a4610e19897de1b4959b0bb4343ea` |
| `📦️packages/🦀️rust/Cargo.toml` (`futures` promoted dev→real dep) | 90 | `f7ddb533c5c75301d674a92c4cddc6c19d5bd83d9e54f6cc5caa7117c206bd44` |
| `📦️packages/🦀️rust/📦️glue.rs` (checked — no change needed, no new top-level facet) | 63 | `a2f6ba1bcb00bb47b5fe78d641331ee8bd3882dbcd844b42f58541fd30488696` |

`🧵️bridge/🟦️component.ts` deliberately UNTOUCHED (`e0c357db…`, byte-identical to P1b) — the wire
format did not change, so the TS twin and its 22 conformance tests needed no edits.

## 2. Design — the real `/bridge` route

### 2.1 Mounting (`🚚️transport/🦀️component.rs`)

`HttpTransport::router(server)` now returns a 3-tuple `(Router, HttpEventPublisher, BridgeHandle)`:
it builds the `/mcp` sub-router exactly as P1b did, calls `crate::bridge::server::bridge_router(bridge_token,
allowed_origins)` for the `/bridge` sub-router, and `.merge()`s them into ONE `axum::Router` — the
literal "same axum app" sol's brief asked for, proven by a test that binds ONE socket and drives both
`/mcp` (a raw HTTP/1.1 POST) and `/bridge` (a real websocket) against it
(`transport::long::bridge_is_live_on_the_same_merged_app_run_http_builds`).

`HttpTransportOptions::new` now takes TWO tokens: `bearer_token` (`/mcp`, chosen by whoever starts the
process, unchanged from P1b) and `bridge_token` (`/bridge`, always freshly minted — see §2.2). This
forced `McpTransport::serve`'s only other caller (`StdioTransport`) to keep working unchanged (it never
touches `HttpTransportOptions`) and forced every existing P1b test's `HttpTransportOptions::new("test-token")`
call site to add a second literal argument — mechanical, no behavior change.

`origin_allowed`/`constant_time_eq` (private in P1b) are now `pub(crate)` so `🧵️bridge`'s websocket
upgrade handler reuses the SAME Origin policy and the SAME constant-time comparison `/mcp` uses,
rather than a second, potentially-drifting copy.

### 2.2 Token minting (`🧵️bridge::mint_bridge_token`/`write_bridge_token_file`, wired in root `run_http`)

`run_http` now, before binding any socket:
1. `bridge::mint_bridge_token()` — a fresh 64-hex-char blake3-mixed secret (process id + monotonic
   counter + a stack-pointer entropy marker + wall-clock nanos hashed together), the SAME
   dependency-free scheme `🎫️handles::mint_id` already used in P1b (no `rand`/`uuid` added).
2. `bridge::write_bridge_token_file(path, token)` — creates the parent directory if missing, writes the
   token verbatim, and (unix only — no POSIX mode bits on Windows, documented rather than silently
   claimed) `chmod 0600`s it. Default path `~/.semio/agent/bridge-token` (`bridge::default_bridge_token_path`,
   same cross-platform `HOME`/`USERPROFILE` lookup pattern `📒️audit::default_audit_dir` already uses,
   duplicated locally rather than importing `audit` — `📒️audit/**` is outside this packet's owned
   paths this time).
3. `eprintln!` ONCE: `[semio-os-mcp] bridge listening on ws://<bind>:<port>/bridge?token=<token>  (also
   written to <path>)` — a dev server/shell process can read either the file or stderr.
4. Passes the minted token into `HttpTransportOptions::new(options.token, bridge_token)`.

Verified live (not just unit-tested) — see §4's real-process smoke: the token file is created with
mode `0600`, its content is a 64-char token, and stderr carries the exact line above.

### 2.3 The connection lifecycle (`🧵️bridge::server::handle_socket`)

Per-connection, over `axum::extract::ws::WebSocket::split()` (a sender half + receiver half, `futures`
crate — promoted from P1b's dev-only dependency to a real one now that PRODUCTION code, not just
tests, calls `.split()`/`.send()`/`.next()`):

1. **Auth, before the upgrade completes** (`upgrade` handler, not `handle_socket`): `Origin` checked
   via `crate::transport::origin_allowed` (`403` if it fails), then `?token=` checked via
   `crate::transport::constant_time_eq` against the minted bridge token (`401` if it fails). A rejected
   client gets a plain HTTP error status and the websocket handshake never completes — proven directly
   (`tokio_tungstenite::connect_async` returns `Err` for both cases, not a socket that opens then
   closes).
2. **Opening frame must be `Hello`** — anything else, or a closed/errored socket, ends the connection
   immediately without ever registering it in `BridgeHandle`.
3. **`Welcome` reply**, `BridgeHandle::register` mints a `ShellConnectionId` and an `mpsc` outbox; the
   `Welcome.connection` field IS that id's `Display` string (`conn_<n>`).
4. **The read/write loop** (`tokio::select!`, so incoming client frames and outgoing pushed frames are
   both serviced without either starving the other):
   - `Ping` → immediate inline `Pong` reply (never reaches `BridgeHandle::record`).
   - `Bye` or a websocket close/error → loop ends.
   - `ShellState`/`ShellStatePatch`/`Instances`/`ShellCommandResult`/`Approval` → `BridgeHandle::record`
     (see §3).
   - `AppFrames`/`Hello` (a second one) → silently ignored (not in sol's "accept" list; `AppFrames` is
     acknowledged as unhandled in §6).
   - A frame that fails to DECODE → skipped, does not kill the connection (one malformed frame from a
     buggy/mid-upgrade shell should not drop a live session).
   - Anything pushed into this connection's `mpsc` outbox (via `BridgeHandle::send_to`/`::broadcast`)
     is forwarded onto the socket as a binary frame.
5. **`BridgeHandle::unregister`** on any exit path.

## 3. The published `BridgeHandle` API (`🧵️bridge::{BridgeHandle, ShellConnectionId}`)

```rust
pub struct ShellConnectionId(/* opaque */);           // Copy, Eq, Hash, Display ("conn_<n>")

pub struct BridgeHandle { /* Clone, cheap — Arc internally */ }
impl BridgeHandle {
    pub fn new() -> Self;
    pub fn connections(&self) -> Vec<ShellConnectionId>;
    pub fn send_to(&self, id: ShellConnectionId, frame: GatewayToShell) -> bool;   // false = not live
    pub fn broadcast(&self, frame: GatewayToShell) -> usize;                       // count reached
    pub fn last_shell_state(&self, id: ShellConnectionId) -> Option<ShellToGateway>;      // ShellState | ShellStatePatch
    pub fn last_instances(&self, id: ShellConnectionId) -> Option<Vec<BridgeInstanceRef>>;
    pub fn last_command_result(&self, id: ShellConnectionId) -> Option<(u64, bool, Option<String>)>;
    pub fn last_approval(&self, id: ShellConnectionId) -> Option<(String, ApprovalDecision, Option<String>)>;
}
```

Obtained from `crate::bridge::server::bridge_router(token, allowed_origins) -> (Router, BridgeHandle)`
directly, or — the path a real process takes — from `HttpTransport::router(server) -> (Router,
HttpEventPublisher, BridgeHandle)`. **Not wired into any other facet's file** (`🎬️actions`, `🛡️policy`,
`🏠️workspace`, `🗂️catalog` are untouched, exactly as instructed): a future packet that needs P6's
policy engine to route a parked approval, or a `ui.*` tool to push a `ShellCommand`, obtains a
`BridgeHandle` the same way this packet's own tests do and calls `send_to`/`broadcast` directly.

**`last_shell_state` returns the raw last-received frame, not a merged/applied state.** Reconstructing
canonical `ShellState` from a `ShellState` frame followed by zero-or-more `ShellStatePatch` frames is
`💻️os/🔨️modules/🖥️shell`'s reducer's job (P9), not this facet's — a caller (e.g. a future `semio://ui/shell`
resource handler) feeds whatever `last_shell_state` returns through that reducer.

**One current limitation, stated plainly**: `run_http`'s production path calls
`HttpTransport::serve(server)` (via the `McpTransport` trait, unchanged from P1b), which internally
calls `.router(server)` and then blocks forever inside `axum::serve` — the `BridgeHandle` it builds is
dropped, unreachable from outside the serving future. This is fine for TODAY (nothing in-process
consumes it yet), but a future packet that wants `run_http`-launched, in-process access (e.g., P6's
policy engine running in the SAME process) will need a small, mechanical change: call
`HttpTransport::router()` directly instead of `.serve()`, keep the returned `BridgeHandle`, and drive
`axum::serve` itself — the exact pattern this packet's own integration tests already use. Not done
here because it has no consumer yet and doing it speculatively would mean guessing at an API shape
before the real caller exists.

## 4. Tests — 169 total, up from sol's reported 160, all green

```
$ CARGO_TARGET_DIR=.🧬semio/…/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY/🎯️target cargo test -p semio-framework-os-mcp
...
test result: ok. 169 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 158.69s
     Running unittests ../../📦️bin.rs …
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
exit code: 0
```
Full verbatim transcript at `🧪️p1c-cargo-test-1.txt` (238 lines). 9 net-new tests, all in
`bridge::long`/`transport::long`:
- `bridge::long::bridge_websocket_replies_welcome_to_hello` (updated for the new token/origin args)
- `bridge::long::wrong_token_is_rejected_before_the_websocket_upgrade`
- `bridge::long::missing_token_is_rejected`
- `bridge::long::an_evil_origin_is_rejected_before_the_websocket_upgrade`
- `bridge::long::full_bridge_lifecycle_hello_state_push_and_command_result` (Hello→Welcome, `ShellState`
  publish visible via `last_shell_state`, a pushed `ShellCommand` answered by `ShellCommandResult`
  visible via `last_command_result`, a `broadcast` reaching exactly the one live connection)
- `bridge::long::send_to_an_unknown_connection_returns_false`
- `bridge::long::mint_bridge_token_produces_distinct_high_entropy_tokens`
- `bridge::long::write_bridge_token_file_creates_parents_and_is_readable_back` (asserts the real
  `0600` mode on unix)
- `bridge::long::default_bridge_token_path_ends_with_the_frozen_suffix`
- `transport::long::bridge_is_live_on_the_same_merged_app_run_http_builds` — THE acceptance test sol
  asked for verbatim: real ephemeral socket, the actual `HttpTransport::router()` output (not a
  bridge-only router), a real `tokio-tungstenite` client, drives `/mcp` still answering + `Hello`→
  `Welcome` + `ShellState` publish + pushed `ShellCommand`/`ShellCommandResult` round trip + wrong-token
  rejection + bad-Origin rejection, all in one foreground `#[tokio::test]`.

```
$ CARGO_TARGET_DIR=.🧬semio/…/🎯️target cargo build -p semio-framework-os-mcp
... 2 warning lines total, BOTH from the upstream `semio-framework-os-kernel` dependency
    (`🔨️modules/📡️spr/📡️wire/🦀️component.rs:448`, "value assigned to `pos` is never read" — pre-
    existing, not this crate's code, not touched by this packet)
$ … | grep -c "^warning"
2
```
**Zero warnings from `semio-framework-os-mcp` itself** — confirmed by reading the full transcript
(`🧪️p1c-cargo-build.txt`): every `warning:` line names `🔨️modules/📡️spr/📡️wire/🦀️component.rs`, none
name any file under `🌉️mcp/`. Sol's acceptance bar was "no warning from our code," which this meets;
the raw `grep -c` number is 2 only because it also counts the upstream dependency's own warning, which
is outside this ticket entirely.

### Real-process smoke (not just in-process — the actual `semio-os-mcp http` binary)

```
$ semio-os-mcp http --port 7403 --token mcp-secret --bridge-token-file <path>
[semio-os-mcp] bridge listening on ws://127.0.0.1:7403/bridge?token=3851ce8300c961a2d148fda53cf718239c5eb0a893218104ce02f86fcc339f19  (also written to <path>)

$ ls -l <path>
-rw-------@ 1 ueli  staff  64 … <path>      # 0600, confirmed, not just claimed

$ curl -sS -i -X POST http://127.0.0.1:7403/mcp -H "Authorization: Bearer mcp-secret" -d '{"jsonrpc":"2.0","id":1,"method":"ping"}'
HTTP/1.1 200 OK
content-type: application/json
{"jsonrpc":"2.0","id":1,"result":{}}
```
Full transcripts at `🧪️p1c-http-smoke.txt` / `🧪️p1c-http-server.txt`. Run in one bounded foreground
Bash invocation (server backgrounded and killed within the SAME command, never left running past the
tool call — no `run_in_background: true` was ever set on the Bash tool itself).

## 5. Deviations / design choices, with justification

1. **`HttpTransportOptions::new` now takes two tokens** (`bearer_token`, `bridge_token`) instead of
   one — the bridge secret and the `/mcp` bearer are different secrets with different lifecycles (the
   bearer is operator-chosen and stable across restarts if they want; the bridge token is ALWAYS
   freshly minted). Making this explicit in the constructor signature (rather than, say, overloading
   `bearer_token` for both) makes "these are two different secrets" impossible to get wrong at the call
   site — every P1b test call site needed one mechanical extra argument.
2. **`AppFrames` is received but not recorded anywhere** — not in sol's explicit "accept" list
   (`ShellState`/`ShellStatePatch`/`Instances`/`ShellCommandResult`/`Approval`/`Ping`) and no consumer
   exists yet for per-instance embedded app frames. It decodes successfully and is silently dropped
   rather than causing a "non-exhaustive match" compile error or an incorrectly-generic catch-all that
   would also swallow a real bug. A later packet that needs it adds one `ConnectionEntry` field and one
   `match` arm.
3. **`futures` moved from `[dev-dependencies]` to `[dependencies]`** — P1b only used `WebSocket::split()`
   in test code; P1c's real `handle_socket` needs it in production. Same crate/version, just a real
   dependency now instead of a test-only one.
4. **The real-process smoke test speaks raw HTTP/1.1 by hand** in `transport::long`'s merged-app test
   (`reqwest_free_post`) for the ONE `/mcp`-still-works assertion, rather than adding a `reqwest`
   dependency — that test already needs a real bound socket for the websocket half, so one more
   fire-and-forget request over the same `TcpStream` primitive costs nothing extra; `curl` (external
   process) does the equivalent job for the real-binary smoke in §4.
5. **`run_http`'s `BridgeHandle` is currently unreachable from outside the serving future** — see §3's
   explicit callout. Not a gap in what THIS packet delivers (the handle exists, is fully tested, is
   public), just an honest note that wiring it into a live process for another packet's in-process
   consumption is a few-line follow-up when that consumer exists, deliberately not built speculatively.
6. **Did not touch `🎬️actions`, `🛡️policy`, `🏠️workspace`, `🗂️catalog`, `📒️audit`** — exactly as
   instructed. `bridge::default_bridge_token_path`'s `HOME`/`USERPROFILE` lookup duplicates
   `📒️audit::default_audit_dir`'s pattern (5 lines) rather than importing/exporting it from `audit`,
   since `📒️audit/**` is outside this packet's owned paths this time (P1b owned it, P1c does not).

No other deviations. Every numbered item in sol's task (mount `/bridge`, mint+write+print the token,
reject wrong token/bad Origin, serve `Hello`/`Welcome`/`ShellState`/`ShellStatePatch`/`Instances`/
`ShellCommandResult`/`Approval`/`Ping`, push `ShellCommand`/`ApprovalRequested`/`AgentPresence`/`Pong`,
keep last-known `ShellState` per connection, publish `BridgeHandle` without wiring it into other
packets' files) is implemented and tested.

## 6. Files touched (for sol's own `ticket_close` call — not mine)

Extended: `🧵️bridge/🦀️component.rs`, `🚚️transport/🦀️component.rs`, `🦀️component.rs` (root), `📦️bin.rs`,
`📦️packages/🦀️rust/Cargo.toml`. Checked, unchanged: `📦️packages/🦀️rust/📦️glue.rs`,
`🧵️bridge/🟦️component.ts`. Created: this report, scratch `.txt` evidence files
(`🧪️p1c-cargo-test-1.txt`, `🧪️p1c-cargo-build.txt`, `🧪️p1c-http-smoke.txt`, `🧪️p1c-http-server.txt`) in
this ticket folder. Nothing outside `path_scope` was touched; no git-modifying command was run.
