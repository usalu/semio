# MCP Module Protocol & Dispatch Exploration Report

## 1. MCP Tool Registration (20 tools total)

### Real Implementations (11 tools)

**Core Gateway Tools (3):**
- `capabilities_search` — BM25 search over compiled catalog. Handler: `capabilities_search_handler()`. Lines 211-218 in 🦀️component.rs. Real implementation with live catalog search.
- `capabilities_describe` — Returns full CapabilityDefinition. Handler: `capabilities_describe_handler()`. Lines 220-226. Real implementation; queries catalog by id.
- `context_resolve` — Opens/refreshes session, returns ContextSummary. Handler: `context_resolve_handler()`. Lines 228-235. Real implementation; mints session id, resolves context.

**Mutation Protocol Tools (8):**
- `action_prepare` — Validates input, checks policy, dry-runs, captures baseline revision, returns PreparedActionReport. Handler: `action_prepare_handler()`. Lines 338-348. Real implementation via `ActionAdapter::prepare()`.
- `action_invoke` — Commits prepared action through 2-phase protocol, returns InvocationReport. Handler: `action_invoke_handler()`. Lines 350-363. Real implementation via `ActionAdapter::invoke()`.
- `action_cancel` — Drops prepared-action handle. Handler: `action_cancel_handler()`. Lines 365-374. Real implementation via `ActionAdapter::cancel()`.
- `transaction_begin` — Binds prepared handles into saga transaction. Handler: `transaction_begin_handler()`. Lines 376-382. Real implementation via `ActionAdapter::transaction_begin()`.
- `transaction_commit` — Commits saga (2-phase, reverse-order, compensating undo on failure). Handler: `transaction_commit_handler()`. Lines 384-393. Real implementation via `ActionAdapter::transaction_commit()`.
- `transaction_rollback` — Abandons saga before commit. Handler: `transaction_rollback_handler()`. Lines 395-404. Real implementation via `ActionAdapter::transaction_rollback()`.
- `history_undo` — Fans TransactionUndo to every member a committed invocation touched. Handler: `history_undo_handler()`. Lines 406-415. Real implementation via `ActionAdapter::history_undo()`.
- `history_redo` — Fans TransactionRedo to every member. Handler: `history_redo_handler()`. Lines 417-426. Real implementation via `ActionAdapter::history_redo()`.

### Stub Implementations (9 tools)

All declared at line 244 in 🦀️component.rs as `DECLARED_STUB_TOOL_NAMES`:
- `artifact_create`, `artifact_open`, `artifact_validate`, `artifact_export`, `artifact_snapshot` — P7 headless workspace
- `job_get`, `job_cancel` — P7 headless workspace  
- `ui_focus`, `ui_reveal` — P10 shell

All use stub handler `stub_tool_unavailable()` at line 246. Returns `PLUGIN_UNAVAILABLE` error (line 247): *"not implemented yet — lands with a later packet (P7 headless workspace or P10 shell)"*.

## 2. MCP Resource URI Templates

Three resource templates registered in CatalogResourceRegistry (🧠️context/🦀️component.rs lines 150-172):

- `semio://capability` — Catalog list endpoint. Handler reads all capabilities from compiled catalog. Returns JSON array of capability summaries.
- `semio://capability/{id}` — Single capability read. Handler strips `semio://capability/` prefix, queries catalog by id, returns full CapabilityDefinition as JSON.
- `semio://workspace` — Workspace metadata (with NullBackend, returns empty summary; real backend from P7's HeadlessWorkspace provides open artifacts, active_artifact_id, catalog_hash).

No subscription handlers (stubs return success but no-op).

## 3. MCP Prompts Registered

**Zero prompts.**

Every server instantiation uses `InMemoryPromptRegistry::new()` (lines 521, 554, 708 in 🦀️component.rs). No `register()` calls anywhere. Prompt support is reserved for a future packet.

## 4. Handler Implementation Status

### Real Implementations (11 mutation-protocol + 3 core):
- All 14 real tools have genuine business logic: `ActionAdapter` calls, catalog queries, policy checks, transaction state management.
- Every tool in `build_tool_registry()` (lines 431-504) wires a closure that calls either a handler function or the ActionAdapter directly.
- Example: `action_prepare_handler()` (line 338) calls `actions.prepare(catalog, principal, &default_session(), capability_id, input, 0, now_ms)` which executes the full prepare/preview lifecycle.

### Stub Implementations (9 artifact/job/ui tools):
- Line 246: `stub_tool_unavailable()` returns fixed `PLUGIN_UNAVAILABLE` error.
- Line 499: Every stub tool registration says *"Declared, not yet implemented — returns a PLUGIN_UNAVAILABLE tool-error until P7/P10 land."*
- No `todo!()`, `unimplemented!()`, or panic paths in stubs — structured error response only.

### Other Stubs/Placeholders Found via Grep:
- Line 349 (🛡️policy/🦀️component.rs): Comment *"not yet decided: resubmitting the same (undecided) handle must still be Required."* — Policy approval gate logic, not a tool handler.
- Lines 926, 930 (🏠️workspace/🦀️component.rs): `GatewayBackend::Err(PluginUnavailable)` for `prepare_action`/`invoke_action` — P7 workspace's own methods not wired to mutation protocol yet (those are P6's ActionAdapter territory).
- Line 1033 (🏠️workspace): `PluginUnavailable` for artifact schema resolution — needs live plugin instance.

## 5. Dual-Era Protocol Negotiation

**Protocol Versions Supported (line 37, 🧭️protocol/🦀️component.rs):**
```rust
["2026-07-28", "2025-11-25", "2025-06-18"]
```
Newest first (modern era is index 0).

### Modern Era (2026-07-28+)
- Entry point: `server/discover` method (line 670).
- Version signal: `params._meta."io.modelcontextprotocol/protocolVersion"` (line 40, extracted by `extract_meta_protocol_version()` line 44).
- Handler: `handle_server_discover()` (lines 714-732).
  - Reads version from `_meta` (line 715).
  - Rejects unsupported versions (line 717).
  - Sets `era = Some(ProtocolEra::Modern)` (line 724).
  - Returns `{resultType, protocolVersion, capabilities, serverInfo}` (line 726).
- Every later request on that connection re-validates per-request `_meta` version (line 686: `dispatch_versioned()` re-checks and re-sets `era`).

### Legacy Era (2025-11-25, 2025-06-18)
- Entry point: `initialize` method (line 671).
- Version signal: `params.protocolVersion` (line 735).
- Handler: `handle_initialize()` (lines 734-744).
  - Reads version from params (line 735).
  - Falls back to newest if unsupported (line 736).
  - Sets `era = Some(ProtocolEra::Legacy)` (line 737).
  - Returns same shape as modern (line 739).
- Handshake completed by `notifications/initialized` notification (line 672): sets `initialized = true`, no response.

### Dispatcher State Machine
- `McpServer` fields (lines 616-626): `era: Option<ProtocolEra>`, `negotiated_version: Option<String>`, `initialized: bool`.
- Era decided once, from opening request (dispatch_inner line 668).
- Every versioned operation after handshake routes through same handlers regardless of era (line 693+).

### Unsupported Version Handling
- Modern era: `reject_unsupported_version()` (lines 707-712) returns JSON-RPC error code `-32022` with `{supported, requested}` data.
- Legacy era: Falls back to newest supported (line 736), never errors.

## 6. `📦️bin.rs` Argument Parsing

**Modes:** `stdio` or `http` (mutually exclusive, lines 110-122).

### Both Modes Support:
- `--folder <dir>` — Local workspace binding (P7 HeadlessWorkspace::open_folder).
- `--hub <url> --space <id>` — Hub-bound workspace. Optional `--token <t>` (hub auth token, distinct from bearer).
- `--principal <id>` — AgentPrincipal id (default "agent:local").
- `--scopes <a,b,c>` — Comma-separated scope names for principal.

Mutual exclusion (line 63-64): `--folder` and `--hub` cannot both be set.

### `stdio` Mode (lines 48-66):
- `--folder <dir>` → `StdioOptions.folder`
- Hub tuple → `StdioOptions.hub: Option<HubOptions>`
- `--principal <id>` → `StdioOptions.principal`
- `--scopes <csv>` → `StdioOptions.scopes: Vec<String>`
- Invokes `run_stdio(options)` (line 134).

### `http` Mode (lines 69-108):
- `--port <p>` (default 6300, line 70).
- `--bind <addr>` (default "127.0.0.1", line 71).
- `--token <t>` (FIRST occurrence → `/mcp` bearer token, REQUIRED line 102).
- Subsequent `--token` → hub auth token (line 90).
- `--folder <dir>`, hub tuple (same as stdio).
- `--audit-dir <dir>` — Audit log directory (default `~/.semio/agent/audit`).
- `--allow-origin <origin>` — CORS allowed origins (repeatable, line 97).
- `--bridge-token-file <path>` — Where to write `/bridge` websocket secret (default `~/.semio/agent/bridge-token`).
- Invokes `run_http(options)` (line 135).

**Bridge Token (P1c):**
- NEVER taken from argv.
- Freshly minted at startup by `mint_bridge_token()` (called in `run_http()`, line 679 in 🦀️component.rs).
- Written to `bridge_token_file` with mode 0600 (line 681).
- Printed to stderr for dev consumption (line 682).

