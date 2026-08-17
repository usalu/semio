# 📓️ sol → terra packet brief (verbatim) — P1a-protocol-core

You are "terra", an executor on ticket `26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY` in /Users/ueli/Documents/semio. Packet id: **P1a-protocol-core**. Model: Sonnet 5. Coordinator ("sol") is the main chat.

## 0. First action
Read these, in order, in full:
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY/📌️important.md` (binding rules)
- `…/📓️design-decisions.md` (W0 deltas — **these override the master plan**)
- `…/📓️luna-mcpspec-audit.md` (the exact MCP wire shapes; §C is your checklist)
- `…/📓️luna-testinfra-audit.md` §"Cookbook for the new 🌉️mcp module" (exact boilerplate: Cargo.toml keys, project.json's four required test levels, script.ts shape, root wiring)
- `/Users/ueli/Documents/semio/CLAUDE.md`
Then save this entire brief verbatim as `…/📓️sol-P1a-protocol-core-packet.md`.

## 1. Owned writable paths (EXCLUSIVE — nothing outside this list, ever)
```
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️component.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧭️protocol/🦀️component.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🚚️transport/🦀️component.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/⚠️errors/🦀️component.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧬️schema/🦀️component.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️bin.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/{Cargo.toml,📦️glue.rs,📜️script.ts,📋️project.json}
.🧬semio/…/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY/📓️sol-P1a-protocol-core-packet.md
.🧬semio/…/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY/📓️terra-P1a-report.md  (+ .txt scratch in that folder)
```
The root `Cargo.toml` member line is **sol's** job — you emit a `lease-request` block for it (see §6) and do NOT edit root files.

## 2. Required result
A compiling, tested Rust crate `semio-framework-os-mcp` (bin `semio-os-mcp`) implementing the **dual-era MCP protocol core over stdio**, with all OS/plugin integration behind a trait so this packet has ZERO dependency on the in-flight microkernel work.

### 2.1 Crate skeleton
Follow the cookbook exactly. `Cargo.toml`: `[package] name = "semio-framework-os-mcp"`, `[lib] path = "📦️glue.rs"`, `[[bin]] name = "semio-os-mcp", path = "../../📦️bin.rs"` (verify the relative depth against the `🏃️run` exemplar), `[package.metadata.semio] role = "product"`. Dependencies for THIS packet only: `serde`, `serde_json`, `thiserror`, `schemars`, `jsonschema` (default-features=false if that is how the repo uses it — check `🧬️schema`'s Cargo.toml), `blake3`. **No tokio, no axum in P1a** (they arrive with P1b's HTTP transport) — stdio is a blocking loop. `📦️glue.rs` mounts the facet files with `#[path]` exactly as the `🏃️run`/`💻️os` glue files do. `📋️project.json` MUST declare all four test levels (`test`, `test-quick`, `test-long`, `test-exhaustive`) or `verify` fails. `📜️script.ts` follows the sibling modules' shape and supports at least `test`, `dev`.

### 2.2 `🧭️protocol` — dual-era JSON-RPC + MCP
Read `📓️design-decisions.md` D1 first; it is the controlling requirement.
- JSON-RPC 2.0 framing: request/response/notification/batch, id types, the standard error codes, plus MCP's `UnsupportedProtocolVersionError` = **-32022** with `data: {supported, requested}`.
- `SUPPORTED_PROTOCOL_VERSIONS = ["2026-07-28", "2025-11-25", "2025-06-18"]`, newest first.
- **Modern era** (`2026-07-28`): stateless; every request carries its version in `_meta` under the key `io.modelcontextprotocol/protocolVersion`; implement `server/discover` (MUST) returning server identity + supported versions + capabilities; no `initialize`, no session state.
- **Legacy era** (`2025-11-25`, `2025-06-18`): the `initialize` request/`notifications/initialized` handshake, capability negotiation, and version echo (respond with the client's version when supported, else your latest).
- A `ProtocolEra` is decided per connection by how the client opens (modern `_meta` vs `initialize`) and recorded; both eras share ONE handler layer underneath — do not fork the tool/resource implementations.
- Methods to route (return "method not found" for ones later packets own, but the routing table and the request/response types must exist now): `server/discover`, `initialize`, `notifications/initialized`, `tools/list`, `tools/call`, `resources/list`, `resources/templates/list`, `resources/read`, `resources/subscribe`, `resources/unsubscribe`, `prompts/list`, `prompts/get`, `ping`, `notifications/cancelled`, and the server→client notifications `notifications/tools/list_changed`, `notifications/resources/list_changed`, `notifications/resources/updated`.
- Registries: `ToolRegistry`, `ResourceRegistry`, `PromptRegistry` — each a trait + an in-memory default, so P1b/P2/P6 plug real providers in without touching this file. Tool objects carry `name`, `title`, `description`, `inputSchema`, `outputSchema`, `annotations`, `_meta`; **enforce the tool-name charset `^[a-zA-Z0-9_-]{1,64}$` at registration time with a test.**
- `CallToolResult { content, structuredContent, isError }`; a **tool** failure is `isError: true` + structured error payload, a **protocol** failure is a JSON-RPC error. Get this distinction right; it is tested.

### 2.3 `⚠️errors`
`GatewayError { code: GatewayErrorCode, message, details: serde_json::Value, retryable: bool }` with the exact code set from `📋️master.md` §3.3 (`INPUT_INVALID, PRECONDITION_FAILED, REVISION_CONFLICT, PERMISSION_DENIED, APPROVAL_REQUIRED, PLUGIN_UNAVAILABLE, SIDE_EFFECT_REJECTED, CANCELLED, COMPENSATION_FAILED, NOT_FOUND, BUDGET_EXCEEDED, INTERNAL`) + conversion to both MCP shapes.

### 2.4 `🧬️schema`
schemars-derived gateway wire types: `InvocationReport`, `PreparedActionReport`, `RevisionStamp`, `SearchHit`, `JobStatus`, `ContextSummary`, `GatewayError`. Emit their JSON Schemas (2020-12) via a `schemas()` function; a test asserts each compiles under `jsonschema::Validator::new`.

### 2.5 `🚚️transport` + `📦️bin.rs`
`trait McpTransport` (serve a request/response loop, emit server-initiated notifications) + `StdioTransport` (newline-delimited JSON-RPC on stdin/stdout; **all logging to stderr — a stray stdout byte corrupts the protocol**; add a test for that discipline). `📦️bin.rs`: `semio-os-mcp stdio [--folder <dir>] [--principal <id>] [--scopes a,b]` — parse and store the flags; unknown/unimplemented modes exit with a clear message. Keep `main` thin, all logic in the lib.

### 2.6 Backend seam (critical for parallelism)
Define `trait GatewayBackend` with the operations later packets implement (`resolve_context`, `search_capabilities`, `describe_capabilities`, `prepare_action`, `invoke_action`, `read_resource`, `list_resources`, …) returning the `🧬️schema` types, plus a `NullBackend` that answers `PLUGIN_UNAVAILABLE`/empty for everything. **This packet must not reference `semio-framework`, `semio-framework-os-kernel`, the plugin host, the channel, or the actor crate at all** — they are mid-rewrite by a peer ticket. Zero dependency on them is a hard requirement and is verified by inspecting your Cargo.toml.

## 3. Tests (in-file `//#region 🧪️Tests`, `mod quick` / `mod long` per repo convention)
Cover at minimum: JSON-RPC framing round-trips incl. batch and null-id notifications; modern request routing via `_meta` version; legacy `initialize` handshake + version echo; unsupported version → -32022 with the supported list; `server/discover` shape; era detection from the opening request; tool-name charset rejection; tool-error vs protocol-error mapping; every `🧬️schema` type's JSON Schema validating its own example; stdio transport never writes to stdout except responses.

## 4. Acceptance commands (run in the FOREGROUND, paste full output + exit code into your report)
```
CARGO_TARGET_DIR=.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY/🎯️target cargo test -p semio-framework-os-mcp
CARGO_TARGET_DIR=.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY/🎯️target cargo build -p semio-framework-os-mcp --bin semio-os-mcp 2>&1 | tee <ticket>/🧪️p1a-build.txt
```
plus a manual smoke proving both eras, e.g.
```
printf '{"jsonrpc":"2.0","id":1,"method":"server/discover","params":{"_meta":{"io.modelcontextprotocol/protocolVersion":"2026-07-28"}}}\n' | CARGO_TARGET_DIR=<same> cargo run -q -p semio-framework-os-mcp --bin semio-os-mcp -- stdio
printf '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25","capabilities":{},"clientInfo":{"name":"t","version":"0"}}}\n' | … -- stdio
```
Zero warnings is required (`cargo build … 2>&1 | grep -c "^warning"` → 0). The crate is not yet a workspace member, so until sol applies your lease it will not build via `-p`; **first emit the lease-request (§6), then continue working, and run acceptance after sol confirms** — if sol has not confirmed by the time you are otherwise done, say so plainly in the report and paste whatever you could run. Never invent results.

## 5. Hard rules (from `📌️important.md`)
No git-modifying commands. No `ticket_close`/`ticket_reopen`/any repo-MCP write tool. Never edit outside §1. **Never run builds in the background** (no `&`, no `run_in_background`, no polling loop — use a long foreground timeout, e.g. 900000 ms). Scratch files `.txt`/`.md`/`.json` in the ticket folder only, never `.log`. `[DEBUG] ` prefix on temp logs, removed before done. Never claim a test passed without pasted output + exit code. Never edit `AGENTS.md`. No legacy/compat shims (D1's dual-era support is a spec requirement, not a shim — implement it as first-class). Docstrings start with a unique emoji; no comments inside definitions; use `//#region` structure.

## 6. Lease-request (emit EARLY, in your first few minutes, as a fenced block in your report AND as `…/📓️lease-P1a-cargo-member.md`)
Name: root `Cargo.toml`. Exact text to insert (workspace member line in correct sorted position + `[workspace.dependencies]` alias), plus why. Then continue with the rest of the work while sol applies it.

## 7. Report
Write `…/📓️terra-P1a-report.md`: preconditions (baseline `git rev-parse HEAD`, SHA-256 of files you created), what you built per file with line counts, the acceptance output verbatim with exit codes, the lease-request, any deviation from this brief with justification, and a "what P1b/P2/P6 can now rely on" section listing the exact public API (trait + type signatures) you published.
