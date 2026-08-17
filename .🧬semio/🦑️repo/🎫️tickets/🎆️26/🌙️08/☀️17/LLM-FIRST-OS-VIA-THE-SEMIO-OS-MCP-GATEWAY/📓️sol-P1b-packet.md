You are "terra", an executor on ticket `26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY` in /Users/ueli/Documents/semio. Packet id: **P1b-http-handles-bridge**. Model: Sonnet 5. Coordinator ("sol") is the main chat.

## 0. First action
Read in full: `…/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY/📌️important.md`, `…/📓️design-decisions.md`, `…/📓️terra-P1a-report.md` (**the API you build on — read its "what P1b/P2/P6 can now rely on" section carefully**), `…/📓️luna-mcpspec-audit.md` (§A.7 Streamable HTTP is your transport spec), and `/Users/ueli/Documents/semio/CLAUDE.md`. Also read the existing crate you are extending: `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/{🦀️component.rs,🧭️protocol/🦀️component.rs,🚚️transport/🦀️component.rs,⚠️errors/🦀️component.rs,🧬️schema/🦀️component.rs}` and its `📦️packages/🦀️rust/{Cargo.toml,📦️glue.rs}`.
Save this brief verbatim as `…/📓️sol-P1b-packet.md`.

**Status: P1a is landed and accepted.** The crate is a workspace member, 34 tests pass, 0 warnings, and the binary demonstrably serves both MCP eras over stdio. Do not rewrite what it built; extend it.

## 1. Owned writable paths (EXCLUSIVE)
```
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🚚️transport/🦀️component.rs      (extend: add HTTP)
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🎫️handles/🦀️component.rs        (new)
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📒️audit/🦀️component.rs          (new)
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧵️bridge/🦀️component.rs         (new)
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧵️bridge/🟦️component.ts         (new — TS twin codec)
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️component.rs                  (mount new facets only)
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️bin.rs                        (add the http subcommand + flags)
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/{Cargo.toml,📦️glue.rs}
.🧬semio/…/📓️sol-P1b-packet.md, 📓️terra-P1b-report.md, 📓️lease-P1b-*.md, *.txt scratch
```
Do NOT touch `🧭️protocol`, `⚠️errors`, `🧬️schema` (P1a's, and P2/P6 build on them), nor anything outside `🌉️mcp`.

## 2. Required result

### 2.1 Streamable HTTP transport (`🚚️transport`)
Implement `HttpTransport` behind the existing `McpTransport` trait, per `📓️luna-mcpspec-audit.md` §A.7 and the spec page `https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/streamable-http` (fetch it — do not guess): single endpoint, POST for requests, GET for the server→client SSE stream, `Accept` negotiation, the `MCP-Protocol-Version` header (must agree with the body `_meta` version — mismatch is a `400`), SSE event ids + `Last-Event-ID` resumption, and correct status codes. Serve it with `axum` (already a workspace dep — copy the version/feature style from `🌎️hub/📦️packages/🦀️rust/Cargo.toml`) + `tokio`.
**Security (mandatory, spec-required):** validate the `Origin` header (reject non-loopback/non-`null` unless explicitly allowed), bind `127.0.0.1` by default, and require a bearer token. Both eras must work over HTTP exactly as they do over stdio — the era logic lives in `🧭️protocol` and must not be duplicated here.
Default port **6300** (never the catalog range 6012–6205, never the 7300+ bench pool).

### 2.2 Handle table (`🎫️handles`)
`HandleKind { Session, Prepared, Transaction, Undo, Job, Approval, Continuation }`; ids are ULID-ish strings prefixed by kind (`prep_`, `txn_`, `undo_`, `job_`, `appr_`, `cont_`, `sess_`) — generate with the crate the hub already uses for ids (`uuid` v7) or a small internal monotonic+random scheme; **do not add a new external dep just for this**. `HandleRecord { kind, owner: SessionHandle, bound_to: Attachment, expires_ms, payload }`, `HandleTable` with mint/resolve/revoke/GC-expired. TTLs: prepared 10 min, txn 30 min, undo 24 h, job terminal+1 h, approval 10 min, continuation 5 min, session sliding.
**Authorization must never be derivable from possession of a handle alone** — `resolve` takes the requesting session and returns `PERMISSION_DENIED` on owner mismatch. Test that explicitly; it is a security property, not a nicety.
Also `IdempotencyStore` keyed `(principal, idempotency_key) -> InvocationReport` with a 24 h TTL, returning a `replayed: true` marker on a hit.

### 2.3 Audit lane (`📒️audit`)
`AgentAuditEvent` with the exact field list from `📋️master.md` §3.4 (invocation_id, ts_ms, principal, session, capability, input_hash via blake3, input_redacted, decision Allowed|Denied{code}|Approved{by,mode}, preview_hash, txn_id, edit_ids, revision_before/after, outcome, error, duration_ms, undo_token, client{name,version}). Append-only writer behind a `trait AuditSink` with a JSON-lines file sink under `~/.semio/agent/audit/` (path overridable by flag) and an in-memory sink for tests. **Secrets and full sensitive args are never written** — `input_redacted` is a redacted projection and there must be a test proving a field marked sensitive does not reach the sink. (The event-sourced OS lane is a later packet; the sink trait is the seam for it.)

### 2.4 Shell bridge codec (`🧵️bridge`)
The hand-rolled binary frame codec from `📋️master.md` §2.2 — `ShellToGateway` and `GatewayToShell` with exactly the variants listed there, `BRIDGE_VERSION = 1`, `tag: u8` + fields in declaration order. Rust SSOT in `🦀️component.rs`, a **TypeScript twin** in `🟦️component.ts` (the React shell will import it later).
Ship **shared JSON fixtures** (in `🧵️bridge/🧫️fixtures/`) that both the Rust tests and a TS test load, proving the two codecs agree byte-for-byte — this is the same anti-drift mechanism P9 used for the shell reducer, and it is the deliverable that matters most here. Do not wire the WebSocket server yet if that forces you into the shell's territory; the codec + a `BridgeServer` skeleton over axum's `ws` that echoes `Hello`/`Welcome` is the scope.

## 3. Tests
Extend the existing `//#region 🧪️Tests` convention (`mod quick` / `mod long`). Cover: HTTP POST request→response, GET SSE stream + `Last-Event-ID` resumption, Origin rejection, missing/incorrect bearer token, protocol-version header/body mismatch → 400, both eras over HTTP, handle TTL expiry, cross-session handle theft → PERMISSION_DENIED, idempotent replay, audit redaction, bridge codec round-trip for every variant + the Rust↔TS fixture parity.

## 4. Acceptance (FOREGROUND ONLY — never `&`, never background, use a long timeout)
```
CARGO_TARGET_DIR=<ticket>/🎯️target cargo test -p semio-framework-os-mcp
CARGO_TARGET_DIR=<ticket>/🎯️target cargo build -p semio-framework-os-mcp 2>&1 | grep -c "^warning"    # → 0
```
Plus a **real** HTTP smoke you run and paste: start `semio-os-mcp http --port <free port ≥7400> --token <t>` in one foreground shell invocation with a timeout, `curl` a modern `tools/list` and a legacy `initialize` against it, and show an `Origin: https://evil.example` request being rejected. If a background-free way to run the server for a curl is awkward, use a Rust integration test that boots the axum app in-process instead — that is preferred and fully foreground.
`P1a`'s 34 tests must still pass. Never claim a result you did not run.

## 5. Hard rules
Everything in `📌️important.md`: no git-modifying commands, no ticket MCP write tools, nothing outside §1, **no background builds**, `.txt`/`.md`/`.json` scratch in the ticket folder only, `[DEBUG] ` prefix removed before done, no unpasted claims, no `AGENTS.md` edits, no compat shims/deprecations. Docstrings begin with a unique emoji; no comments inside definitions; `//#region` structure. Any dependency you add to `Cargo.toml` must already exist in the workspace — check the root `[workspace.dependencies]` first and use `workspace = true` where the repo does.

## 6. Report
`…/📓️terra-P1b-report.md`: baseline HEAD, SHA-256s, per-file line counts, full acceptance output with exit codes, the exact bridge frame table as implemented, the public API P6/P10 will consume, any lease-requests, and anything you deliberately deferred with the reason.
