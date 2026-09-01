# Bridge & Transport Wire Protocol Exploration

## 1. Wire Protocol & Frame Kinds

**Bridge Version:** `BRIDGE_VERSION = 1` (🧵️bridge/🦀️component.rs:29, 🧵️bridge/🟦️component.ts:11)

### ShellToGateway Frames (Shell→Gateway, tags 0-8, little-endian)
Binary framing: `tag: u8` + fields in declaration order, length-prefixed bytes/strings.

| Tag | Variant | Fields | Rust Line | TS Line |
|-----|---------|--------|-----------|---------|
| 0 | Hello | bridge_version(u16), shell_kind(tag:u8), shell_session_id(str), principal_actor(str), flags(u8 bitmask) | 288-295 | 214-221 |
| 1 | ShellState | revision(u64), state(bytes) | 296-300 | 222-226 |
| 2 | ShellStatePatch | revision(u64), base_revision(u64), patch(bytes) | 301-306 | 227-232 |
| 3 | Instances | entries(u32 count + BridgeInstanceRef[] with plugin_id, app_id, instance_id, artifact_ref, window_ids) | 307-313 | 233-237 |
| 4 | AppFrames | in_reply_to(u64), instance_id(str), frames(u32 count + bytes[]) | 314-319 | 238-243 |
| 5 | ShellCommandResult | in_reply_to(u64), ok(bool), fault(option:str) | 320-325 | 244-249 |
| 6 | Approval | approval_id(str), decision(tag:u8 {0=deny, 1=once, 2=session}), note(option:str) | 326-331 | 250-255 |
| 7 | Ping | (no fields) | 332 | 256-258 |
| 8 | Bye | (no fields) | 333 | 259-261 |

### GatewayToShell Frames (Gateway→Shell, tags 0-7, little-endian)
Same binary codec. Rust encoder/decoder at lines 1311-1420; TS at lines 321-406.

| Tag | Variant | Fields | Rust Line | TS Line |
|-----|---------|--------|-----------|---------|
| 0 | Welcome | bridge_version(u16), connection(str), principal(str) | 1314-1319 | 324-329 |
| 1 | ShellCommand | seq(u64), command(bytes) | 1320-1324 | 330-334 |
| 2 | AppCommand | seq(u64), instance_id(str), command(bytes) | 1325-1330 | 335-340 |
| 3 | ApprovalRequested | approval_id(str), summary(str) | 1331-1335 | 341-345 |
| 4 | ApprovalResolved | approval_id(str), decision(tag:u8) | 1336-1340 | 346-350 |
| 5 | AgentPresence | active(bool), label(str), invocation_id(option:str) | 1341-1346 | 351-356 |
| 6 | Pong | (no fields) | 1347 | 357-359 |
| 7 | Bye | reason(str) | 1348-1351 | 360-363 |

**Codec Sync:** ✅ Both codecs identical—same frame kinds, same tags, same wire order. Fixtures (`🧫️fixtures/frames.json`) prove byte-for-byte agreement via Rust test `bridge::quick::every_fixture_round_trips_through_the_rust_codec` (bridge/🦀️component.rs) and TS `checkFixtureParity` run via foreground `bun run` script (bridge/🟦️component.ts:6).

### Handshake & Versioning
1. **Initiation:** Shell dials loopback WebSocket with `?token=<bridge-token>` query param (🧵️bridge/🟦️component.ts:8, AgentBridge/🟦️component.tsx:433).
2. **Shell Hello:** First frame shell sends (tag 0)—versioning negotiated per frame, not handshake-level; `bridge_version` field allows future protocol evolution (line 1316 Rust, 216 TS).
3. **Gateway Welcome:** First frame gateway sends (tag 0)—echoes negotiated version, provides connection_id string, principal identifier (1314-1319 Rust, 324-329 TS).
4. **No Negotiation:** Unlike MCP's `initialize`, bridge sends/receives frames statelessly—the version is already known from `?token=` query and first frame.

### Opaque Payloads
Per design note (🧵️bridge/🦀️component.rs:8-15): `state`, `patch`, `command`, `frames` are length-prefixed byte blobs; codec does NOT interpret internals. Shell's real packed format (the "pack" from `📋️master.md` §2.2) lives in `💻️os/🔨️modules/🖥️shell`, not the bridge. AgentBridge/🟦️component.tsx encodes/decodes these as JSON (lines 222-227).

---

## 2. Shell-Side Client Connection

**Client Exists:** ✅ Yes, fully implemented in React host.

**File:** `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/AgentBridge/🟦️component.tsx`
- **Discovery Function:** `discoverAgentBridgeConfig()` (lines 119-133)—reads `VITE_SEMIO_BRIDGE_URL` and `VITE_SEMIO_BRIDGE_TOKEN` env vars; returns `null` if either missing (dev/test mode with no live gateway).
- **Hook:** `useAgentBridge(options)` (lines 302-493)—React hook that:
  - Dials WebSocket at `bridgeUrlWithToken(config)` (line 433)
  - Sends `Hello` frame on `socket.onopen` (line 444)
  - Handles inbound `GatewayToShell` frames via `handleFrame()` (line 386)
  - Maintains `ShellState` mirror via `reduce()` from `🖥️shell/🟦️component.ts` (line 259)
  - Publishes full `ShellState` snapshot on `welcome` and after `reduce()` (lines 234, 340)
  - Tracks agent presence, pending approvals, connection status
- **Reconnection:** Exponential backoff (base 1000ms, max 30000ms) on close; `status` states: disabled→connecting→open; reconnecting on socket close (lines 376-384, 460-464).
- **Ping/Heartbeat:** Sends `Ping` frame every 20s when open (line 445, `PING_INTERVAL_MS = 20000`).

**Note on "Loopback WebSocket":** The bridge is loopback-only by design (🧵️bridge/🟦️component.rs:17, AgentBridge/🟦️component.tsx:110 "port/token discovery compromise"): no filesystem bridge-token file is written anywhere; browser can only dial if dev server injects `VITE_SEMIO_BRIDGE_URL`/`VITE_SEMIO_BRIDGE_TOKEN`. Current status (AgentBridge/🟦️component.tsx:111-114): `/bridge` route is a "still-unmounted skeleton" in `🌉️mcp/🦀️component.rs`—the server side exists (tests bind `:0` ephemeral sockets at 🧵️bridge/🦀️component.rs line 3125+), but the HTTP `/bridge` route is not wired into the real `run_http()` entry point yet.

---

## 3. Transport Layer: stdio & HTTP

**StdioTransport** (🚚️transport/🦀️component.rs:45-117)
- Newline-delimited JSON-RPC over `input`/`output`, with separate `log` writer for diagnostics.
- No bridge involvement—MCP protocol only.
- Rejects malformed lines, never pollutes stdout with non-JSON.

**HttpTransport** (🚚️transport/🦀️component.rs:120+)
- Bind address: default `127.0.0.1:6300` (configurable, never `0.0.0.0`); line 140.
- **Bearer Token:** `/mcp` endpoint requires `Authorization: Bearer <token>` header (line 1721, `bearer_matches`). Constant-time comparison (line 1706, `constant_time_eq`).
- **Bridge Token:** `/bridge` WebSocket uses query-string `?token=<bridge-token>`, validated via constant-time match (🧵️bridge/🦀️component.rs:1443-1444).
- **Both Tokens Distinct:** `/mcp` bearer and `/bridge` query token are separate secrets, freshly minted per process start (🚚️transport/🦀️component.rs:127-128, `📓️sol-P1c-packet.md`).
- **Origin Checks:** Both `/mcp` and `/bridge` reject non-loopback, non-`null`, non-allowlisted Origins with 403 (🚚️transport/🦀️component.rs:1669-1727, `origin_allowed` function called at 🧵️bridge/🦀️component.rs:2550).
- **Session IDs:** HTTPTransport uses `HttpConnectionKey` (slotwise generation counter) to track connection lifetime, not protocol-level sessions (🚚️transport/🦀️component.rs:266-270).

**Streamable HTTP** (🚚️transport/🦀️component.rs:203-211)
- `/mcp` POST: every JSON-RPC request/notification → dispatched through same `McpServer` stdio uses.
- `/mcp` GET: legacy resumable stream endpoint for 2025-11-25/2025-06-18 clients (the 2026-07-28 spec removed GET entirely; this is dual-era accommodation).
- Legacy GET uses `Last-Event-ID` query param to resume from `EventLog` (bounded ring buffer, 256-entry capacity at line 164).
- **No explicit "SSE":** The legacy GET is line-by-line notification replay, not chunked `text/event-stream` (classic Server-Sent Events framing).

**Resumability:** Limited to legacy GET's `Last-Event-ID` mechanism; modern era (`/mcp` POST) is stateless per request. `/bridge` WebSocket has no resumption—connection close means reconnect from scratch.

---

## 4. TypeScript Test Files

All three are **real protocol round-trips** against spawned server, not static text assertions.

### `🧪️hygiene.test.ts` (Lines 1-78)
- **Type:** Black-box process-level proof (comments §3.4).
- **Spawns:** Real compiled `semio-os-mcp stdio` binary.
- **Asserts:**
  - Every stdout line parses as valid JSON, even after malformed input.
  - Malformed input yields PARSE_ERROR response (code -32700).
  - Process answers next request cleanly (not crashed).
  - Diagnostic text lands on stderr, never stdout.
  - Process exits cleanly (code 0) on stdin EOF.
- **NOT in Rust tests:** Differs from Rust's in-memory `Cursor` unit tests (`StdioTransport`).

### `🧪️modern-era.test.ts` (Lines 1-100+)
- **Type:** Raw JSON-RPC client (hand-rolled `spawnRawMcp`, 🟦️component.ts) because SDK is legacy-only (`LATEST_PROTOCOL_VERSION = '2025-11-25'`).
- **Proves:** 2026-07-28 spec's "no negotiation handshake" contract—per-request `_meta` versioning (key `io.modelcontextprotocol/protocolVersion`).
- **Asserts:**
  - `server/discover` with no `_meta` negotiates newest (2026-07-28).
  - Explicit supported `_meta` version negotiates that exact version.
  - Unsupported version yields -32022 error with `data.supported` array.
  - Fresh process with NO prior handshake accepts `tools/list` with `_meta` (spec compliance).

### `🧪️legacy-conformance.test.ts` (Lines 1-100+)
- **Type:** Real SDK handshake (installed `@modelcontextprotocol/sdk` 1.30.0 `Client` + `StdioClientTransport`).
- **Spawns:** Real binary with argv `["stdio"]`.
- **Asserts:**
  - `initialize` negotiates protocolVersion 2025-11-25 and returns serverInfo/capabilities.
  - `tools/list`, `resources/list`, `resources/templates/list`, `prompts/list` return schema-shaped arrays.
  - Tool names match regex `^[a-zA-Z0-9_-]{1,64}$`.
  - Tool schemas validate as JSON Schema draft 2020-12 (via `isValidJsonSchema2020_12` wrapper over `ajv/dist/2020.js`).
  - **Note:** Empty tool registry (🦀️component.rs `run_stdio()` boots `McpServer::with_defaults()`) makes `tools/call` vacuously pass; "unknown tool" protocol error only exercised by Rust unit test, not here.

---

## 5. Stub Markers (Full Inventory)

### Bridge/Transport Files
- **🧵️bridge/🦀️component.rs:** None (`todo!`, `unimplemented!` absent; "not yet" appears only in code comments referencing future phases).
- **🧵️bridge/🟦️component.ts:** None.
- **🚚️transport/🦀️component.rs:** None.

### Other MCP Module Files (Incidental)
- **🛡️policy/🦀️component.rs:line ~unknown:** "not yet decided" (comment on resubmit logic).
- **🏠️workspace/🦀️component.rs:** "not yet wired to this workspace" (3 occurrences)—P6 territory (`action.prepare`, `action.invoke`, artifact schema resolution).
- **🦀️component.rs (module root):** "not yet implemented" on a declared tool (description string, says "returns PLUGIN_UNAVAILABLE until P7/P10").

### TypeScript Packages
- **📦️packages/🟦️typescript/🧪️hygiene.test.ts, modern-era.test.ts, legacy-conformance.test.ts:** None (no stubs; clean test assertions).
- **🧬️schema-validation.ts:** None (simple Ajv wrapper).

---

## Key Findings

1. **Codec Sync:** Rust SSOT and TS twin codecs are byte-identical; fixtures prove this automatically.
2. **Frame Parity:** Both languages implement all 8 ShellToGateway + 8 GatewayToShell kinds; no missing/extra types.
3. **Client Exists:** AgentBridge hook fully implements shell-side WebSocket dial, Hello/Bye exchange, state sync, presence tracking.
4. **Server Status:** Bridge server code exists (`bridge_router`, `BridgeHandle`), but `/bridge` route not yet wired into HTTP entry point (skeleton only).
5. **Token Separation:** `/mcp` bearer and `/bridge` query tokens are distinct secrets.
6. **Origin Enforcement:** Both endpoints check Origin header; loopback/`null`/allowlisted only.
7. **No Negotiation Handshake:** Bridge frames carry version in `Hello`/`Welcome` fields; no separate negotiation phase.
8. **Tests are Real:** All TypeScript tests spawn actual binary and exercise live protocol round-trips; fixtures add anti-drift proofs for codec.

---

## References

- Wire protocol definition: 🧵️bridge/🦀️component.rs lines 1–15 (header), 28–165 (wire primitives), 268–369 (ShellToGateway), 1279–1420 (GatewayToShell).
- TS twin: 🧵️bridge/🟦️component.ts lines 1–407.
- Fixtures: 🧵️bridge/🧫️fixtures/frames.json (22 frame examples with hex encodings).
- Shell client: AgentBridge/🟦️component.tsx lines 102–493 (discovery, hook, socket lifecycle).
- HTTP transport: 🚚️transport/🦀️component.rs lines 120–1727 (endpoints, auth, origin checks, legacy GET).
- Tests: 📦️packages/🟦️typescript/ (three test files, ~200 combined assertions).
