# 📓️ M4 — os MCP: stdio bridge, resolvable approval chain, no silent mock, workspace binding

Slice M4 of ticket 26/09/18. Source memos: `📓️g7-mcp-agent-and-collaboration-audit.md` §2/§6 P0 1–3 + P2 12,
`📓️m2-agent-surface-and-inference.md`. Crate: `semio-framework-os-mcp`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp`). Peers M3/A1/A2/U1 edited the same crate throughout.

---

## 1. Design decisions

### 1.1 Why stdio gets its own bridge listener (P0.1)

The topology is not negotiable: the **gateway is the `/bridge` WebSocket server and a shell dials it**
(`🧵️bridge`, `🚚️transport`). A browser-hosted React shell cannot host a socket, and there is no other
listener anywhere in a `dev s` session, so "stdio connects out to the session" is not implementable —
the attachment must stay "stdio listens, session dials".

What blocked that in stdio mode was not the listener but **admission**: `HttpTransportOptions::new`
takes an `Arc<LocalHubCredential>`, claimed from the inherited `S_LOCAL_CREDENTIAL_FD=3`. A bare
`.mcp.json` launch inherits no such fd, and fabricating one would defeat the process-entry seal
`🏗️bootstrap` enforces.

Decision: a **per-process admission proof** published through an **owner-only rendezvous file**, and a
**bridge-only** listener.

- `HttpAdmission::LocalProof(Arc<[u8]>)` is a new production variant checked by the same
  constant-time comparison the credential path uses, against the same second websocket subprotocol.
- `HttpState.server` became `Option<…>`: a bridge-only listener carries no `McpServer`, so `/mcp` on
  that socket is genuinely **absent (404)**, not present-but-empty. This process's MCP surface is
  stdin/stdout and nothing else.
- The proof and url live in `~/.semio/agent/bridge/offers/<pid>.json`, mode `0600`, written
  atomically (temp + rename) and **removed on drop** — same trust boundary the agent audit lane
  (`~/.semio/agent/audit`) already stands on, and strictly stronger than an argv/env carrier.

### 1.2 What "discovery of a running `dev s` session" means concretely

A new facet `🛰️rendezvous` owns one directory with two halves:

| path | written by | read by |
|---|---|---|
| `~/.semio/agent/bridge/sessions/<pid>.json` | the live os session (the `dev s` Vite server) | the gateway, to decide whether to offer a bridge at all |
| `~/.semio/agent/bridge/offers/<pid>.json` | the gateway | the live os session, to learn where to dial |

Liveness is the **pid**, not a timestamp: `kill(pid, 0)` on unix (a three-day-old session is still a
session; ageing it out would blind discovery for exactly the long-running sessions an agent most wants).
The age cap survives only as the fallback on a platform with no probe. Malformed and dead records are
swept on read.

`run_stdio` therefore: finds no live session → binds nothing, and every bridge-dependent surface
answers a typed, retryable `PLUGIN_UNAVAILABLE` naming the sessions directory and the live-session
count **as of that call** (so an agent can tell "start `dev s`" apart from "the session started after I
did"). Finds one → binds `127.0.0.1:0`, publishes the offer, logs the url, and holds both alive for
exactly as long as stdio serving runs. `--no-bridge` opts out.

### 1.3 The approval chain, and why there is no `approval_resolve` tool (P0.2)

**There is deliberately no `approval_resolve`/`action_approve` MCP tool, and this is the security
decision of the slice.** Every MCP tool is callable by the agent and by nobody else. A tool that
resolves an approval handle would let the agent approve its own destructive action — precisely what the
gate exists to prevent. No argument reachable by the agent can distinguish "the agent called this" from
"a human called this", because there is only one caller on that transport.

The three actors that may decide are all outside the agent's reach, and `ApprovalCoordinator`
(`🛡️policy`) offers a parked handle to them in this order:

1. **`--auto-approve never|readonly|all`** — a launch-time human decision, now parsed off argv for both
   transports. Applied one layer up in `PolicyEngine::requires_approval`, so an auto-approved
   capability never parks a handle at all. `never` is the default; an unknown value is a hard argv
   error, never a silent downgrade.
2. **MCP elicitation** — a real `elicitation/create` server→client request, sent only when the client
   advertised `capabilities.elicitation` at `initialize`. `accept` + `content.approve == true`
   approves; `decline`, `cancel`, an error response, and EOF are all refusals. Never an approval by
   default.
3. **The live OS shell** — `GatewayToShell::ApprovalRequested` over `/bridge`, decided by the human in
   `🤖️AgentApprovals` or inline in `💬️AgentChatPanel`, answered by `ShellToGateway::Approval`,
   bounded by `SHELL_APPROVAL_TIMEOUT_MS` (120 s). A silent shell times out into a refusal.

With none available, `action_invoke` answers `APPROVAL_REQUIRED` whose `details` name every lane it
tried, why each was closed, and the remedy. `action_invoke`'s `approvalHandle` input keeps its original
meaning: replay a decision one of the three already made.

**Elicitation forced a real transport change.** `elicitation/create` is a server→client request issued
from *inside* `tools/call`, while the serve loop is suspended in `dispatch`. `StdioTransport` owned its
streams by value, so nothing could reach the descriptors from in there. The streams now live in one
shared `StdioLines` owner that both the serve loop and the elicitation wait read/write through; a line
the wait reads that is not its own answer is **deferred back onto the serve loop in arrival order**, so
a client that keeps working while a human decides loses nothing. The wait itself reads the raw stream
(`read_line_direct`) and never the deferred queue, or it would re-consume its own deferrals forever
(a real bug this caught — §3).

### 1.4 No silent mock (P0.3)

`dyn_enum_close!` carries no per-variant attributes, so `ArtifactChannels` is spelled twice under
complementary `cfg`s: the production enum has `Unbound`/`Plugin`/`Routing`, the test enum adds `Mock`.
`MockArtifactChannel` and every item in its region are now `#[cfg(test)]` — reachable from **no**
production construction path. `build_server()` (the last production constructor that built one) had
zero call sites and is deleted.

Unbound sessions run on the new `UnboundArtifactChannel`, whose every `exchange` returns the typed
`workspace.unbound` fault → retryable `PLUGIN_UNAVAILABLE` naming `--folder` and `--hub`, matching what
`🗿️artifact`'s tier-1 gate already answered. A caller can no longer mistake a scripted commit for a
real one.

### 1.5 Constructor shape

`bridge: Option<BridgeSlot>` on the three `build_server*` constructors became one `GatewayRuntime`
{ `bridge`, `elicitation`, `auto_approve` } — the three things a live gateway binds late, all absent in
the ordinary test tier (`GatewayRuntime::default()`).

---

## 2. Fixes (file:line at time of writing; peers move lines)

| # | file | what changed |
|---|---|---|
| 1 | `🌉️mcp/🛰️rendezvous/🦀️.rs` (new, 250 lines) | the whole rendezvous: paths, `OsSessionRecord`/`BridgeOffer`, pid liveness + sweep, atomic `0600` offer publication with `Drop` cleanup, OS-seeded `mint_admission_proof` |
| 2 | `🌉️mcp/📦️packages/🦀️rust/🦀️.rs:39` | mounts `pub mod rendezvous` |
| 3 | `🌉️mcp/🦀️.rs:498-520` | new `GatewayRuntime` + `approval_coordinator()` |
| 4 | `🌉️mcp/🦀️.rs:527-590` | `build_server_with_principal`/`from_catalog`/`with_workspace` take `GatewayRuntime`, pass `auto_approve` into `ActionAdapter::new`, bind the coordinator |
| 5 | `🌉️mcp/🦀️.rs:~560` | `build_server()` deleted (dead, and the last production mock constructor) |
| 6 | `🌉️mcp/🦀️.rs:706` | unbound tier is `ArtifactChannels::Unbound(UnboundArtifactChannel)`, was `Mock` |
| 7 | `🌉️mcp/🦀️.rs:717-795` | `StdioOptions{auto_approve,no_bridge}`, `attach_stdio_bridge`, `StdioBridgeAttachment` (Drop-cancels the listener) |
| 8 | `🌉️mcp/🦀️.rs:~820` | `run_stdio` builds the runtime, attaches the bridge, publishes the elicitation channel, serves, then drops the attachment |
| 9 | `🌉️mcp/🦀️.rs:~845` | `HttpOptions.auto_approve`; `run_http` builds a `GatewayRuntime` |
| 10 | `🌉️mcp/🚚️transport/🦀️.rs:51-160` | `StdioLines` (shared duplex owner, deferred queue, `read_line_direct`), `StdioTransport<L>` over it, `publishing_elicitation_into` |
| 11 | `🌉️mcp/🚚️transport/🦀️.rs:~215-300` | `ElicitationSlot`/`ElicitationChannel`/`ElicitationAction`/`ElicitationUnavailable`, `request_boolean`, `elicitation_action` |
| 12 | `🌉️mcp/🚚️transport/🦀️.rs:130-190` | `HttpAdmission::LocalProof`, `HttpTransportOptions::with_local_proof` |
| 13 | `🌉️mcp/🚚️transport/🦀️.rs:426-482` | `start_bridge_only`, `start_with(Option<McpServer>)`, `HttpTransportRun::local_addr` |
| 14 | `🌉️mcp/🚚️transport/🦀️.rs:~1620` | `/mcp` 404s when no server is mounted |
| 15 | `🌉️mcp/🧭️protocol/🦀️.rs:836-858` | `ClientFeatures` + `record()`; `McpServer.client_features` filled by `handle_initialize` and `handle_server_discover` |
| 16 | `🌉️mcp/🛡️policy/🦀️.rs:273-430` | `ApprovalChannel`, `ApprovalResolution`, `ApprovalRequest` (+ the structured `shell_summary` `🤖️AgentApprovals` already parses), `ApprovalCoordinator` with both lanes |
| 17 | `🌉️mcp/🔀️dispatch/🦀️.rs:189-224` | `UnboundArtifactChannel`, `WORKSPACE_UNBOUND_FAULT_CODE`, `workspace.unbound` → retryable `PLUGIN_UNAVAILABLE` |
| 18 | `🌉️mcp/🔀️dispatch/🦀️.rs:226-400` | every `MockArtifactChannel` item is `#[cfg(test)]` |
| 19 | `🌉️mcp/🔀️dispatch/🦀️.rs:~536,~590,~884` | `ActionAdapter.approvals`, `bind_approval_coordinator`, `settle_approval`; `invoke_uncached`'s gate now resolves instead of only reporting |
| 20 | `🌉️mcp/🏠️workspace/🦀️.rs:1419-1441` | `ArtifactChannels` spelled twice under `cfg(test)`/`cfg(not(test))` |
| 21 | `🌉️mcp/🖥️ui/🦀️.rs:69-85` | `bridge_not_running_error` names the rendezvous state and the live-session count |
| 22 | `🌉️mcp/🏗️bootstrap/🦀️.rs:29-102` | `--auto-approve never\|readonly\|all` (both modes) + `--no-bridge` (stdio) |
| 23 | `🌉️mcp/README.md:71-130` | Safety section rewritten (P2.12): the approval chain as implemented, the no-`approval_resolve` decision, workspace binding, stdio bridge attachment |
| 24 | `.mcp.json`, `.cursor/mcp.json`, `.vscode/mcp.json`, `.windsurf/mcp.json`, `.kiro/settings/mcp.json`, `.codex/config.toml` | every `semio` entry binds `--folder .` and the same five scopes |
| 25 | `📺️renderer/…/💬️AgentChatPanel/🟦️.tsx` | inline Deny / Approve Once / Approve for Session on a pending approval row, `role="group"` + `aria-label`, reusing the existing en+de decision labels |
| 26 | `📺️renderer/…/🔗️AgentBridge/🟦️.tsx` | new `os.agent.chat.approvalActionsLabel` en + de |
| 27 | `📺️renderer/…/🏛️ShellHost/🟦️.tsx:8693` | passes `onResolveApproval={agentBridge.resolveApproval}` |
| 28 | `🧑‍💻dev/🔌️vite-plugins/🟦️.ts` (new region) | `semioAgentBridgeRendezvousVitePlugin` — publishes this dev session's `sessions/<pid>.json` (0600, removed on close) and serves the gateway's offer at `/__semio/agent-bridge` |
| 29 | `🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts` | mounts it |

---

## 3. Tests — real counts, all run

`cargo test -p semio-framework-os-mcp` (lib + bin), filtered. Capture:
`🗑️generated/m4-unit-tests.txt`.

**39 lib tests matched the M4 filters, 0 failed; 5 bin tests, 0 failed.** Of those, **32 are new in
this slice**:

| facet | new tests | what they pin |
|---|---|---|
| `rendezvous::quick` | 6 | offer round-trip + `0600` + no leftover `.partial`; drop removes the file; a dead pid is swept and a live one survives; no directory is an empty discovery, not an error; a malformed record is swept; proofs are fresh per mint and 32 hex chars |
| `transport::quick` | 5 | the elicitation writes a real `elicitation/create` with the right schema and reads its own answer; **an unrelated client request arriving mid-elicitation is deferred back, never dropped**; an unadvertised client is never written to at all; an `error` response is a cancel; EOF is `ClientClosed`; the client-feature mirror records exactly what was advertised and clears on re-handshake |
| `policy::quick` | 12 | elicitation accept/decline/cancel/accept-without-`approve`/unadvertised/closed; shell publishes the structured summary and honours yes; shell deny carries the note; a silent shell **times out** (asserted against real wall-clock); a stale decision for another handle is never mistaken for this one; an empty bridge and a no-lane-at-all answer name every closed lane and the remedy |
| `actions::quick` | 6 | an unbound gateway answers retryable `PLUGIN_UNAVAILABLE` naming both flags (prepare **and** invoke); the unbound channel answers no frame for any command; no coordinator bound → the error says nobody could be asked; a bound coordinator's yes commits **in one call**; its no is `PERMISSION_DENIED` tagged with the channel; `--auto-approve all` needs no human |
| bin `quick` | 3 | `--auto-approve` reaches both transports; defaults to `never` and rejects an unknown policy (and a missing value); `--no-bridge` is off by default and stdio-only |

Two real bugs the tests caught before any of this shipped:

1. **Elicitation re-read its own deferred lines**, spinning forever — the wait must read the raw stream,
   not the deferred queue (`read_line_direct`, `🚚️transport/🦀️.rs`). Caught by the interleaved-request
   test hanging.
2. **The rendezvous age cap swept live sessions** on unix, where the pid probe is authoritative. Caught
   by `live_sessions_keep_this_process_and_sweep_a_dead_pid`.

---

## 4. Runtime evidence

`🐍️m4-bridge-approval-probe.ts` drives the real `semio-os-mcp` binary over stdio exactly as a client
launches it (`.mcp.json` args verbatim), answers the server's own `elicitation/create` requests, and
prints: (a) the unbound launch's typed refusal, (b) the `--folder`-bound launch's real tool results,
(c) the approval round trip. Capture: `🗑️generated/m4-probe.txt`.

### (a) unbound — `.mcp.json` args with `--folder` stripped

```
action_prepare  isError=true :: {"code":"PLUGIN_UNAVAILABLE","message":"no workspace is bound to this
  session — start the gateway with --folder <dir> or --hub <url> --space <id> before invoking a
  capability","retryable":true}
artifact_open   isError=true :: {"code":"PLUGIN_UNAVAILABLE", … "retryable":true}
```

`action_prepare` is the call that used to answer from `MockArtifactChannel` with a plausible-looking
success. It now refuses, with the same typed shape the artifact tools already used.

### (b) bound — `.mcp.json` args verbatim (now carrying `--folder .`)

```
capabilities_search isError=false :: {"results":[{"capabilityId":"artifact.open",…},
  {"capabilityId":"space.s.space.space@1/*#editor.openArtifact",…}
artifact_create isError=true :: {"code":"INPUT_INVALID","message":"artifactId is required"}
ui_focus        isError=true :: {"code":"PLUGIN_UNAVAILABLE","message":"no shell is attached to
  `/bridge` yet — this is expected until a shell connects; retry once one does","retryable":true}
live os session records: 4
```

Three things this proves at runtime:

- the workspace binding is real — `capabilities_search` answers off the live installed catalog, and
  `artifact_create` reaches real input validation instead of a mock;
- **the stdio bridge attached**: `ui_focus` answers `no shell is attached to /bridge yet`, which is
  only reachable once the bridge slot is filled. Before this slice that call answered
  `bridge_not_running` unconditionally in stdio mode;
- **the discovery is real, not hypothetical**: four live `dev s` sessions (peers' own dev servers,
  restarted after the new vite plugin landed) had published records into
  `~/.semio/agent/bridge/sessions/`, and the gateway found them and bound. After the probe's gateways
  exited, `~/.semio/agent/bridge/offers/` was empty again — the `Drop` cleanup, observed.

### (c) approval round trip — **not reachable at runtime today**

Eight real destructive capabilities were discovered from the live catalog
(`flow/note/procedural/space…deleteSelection`, `lowpoly.clearSeam`, three `remodel.clear*`). Every one
of them fails **before** the approval gate, inside `prepare`'s first `InstanceOpen`:

```
{"code":"INTERNAL","message":"InstanceOpen: guest trapped: wasm trap: memory write is out of bounds
  … the guest's shadow stack underflowed at call depth 26 … link the component with a larger
  `-zstack-size`"}
{"code":"INTERNAL","message":"InstanceOpen: guest fault plugin.reactor-turn-deadline: guest lifecycle
  turn exceeded strict time authority; receipt retained"}
```

That is the `InstanceOpen` guest trap **slice A2 is fixing in this same crate** — no plugin instance
can be opened at all right now, so no capability can be prepared, so nothing reaches the gate. The
probe retries `BUDGET_EXCEEDED` up to 400 times (that code is the guest's own 8 ms interpreter slice
and is explicitly retryable); the two candidates that got that far then hit the reactor-turn deadline.
The approval chain is therefore proven **at the unit level only** (§3: 18 tests across `policy` and
`actions`, including the full `ActionAdapter::invoke` path with a bound coordinator saying yes and
saying no), and the probe is left in place to re-run the moment A2 lands.

Captures: `🗑️generated/m4-unit-tests.txt`, `🗑️generated/m4-probe.txt`.

---

## 5. Honest gaps

1. **The browser shell still does not dial.** `discoverAgentBridgeConfig` in
   `📺️renderer/…/🔗️AgentBridge/🟦️.tsx` returns `null` unconditionally, so `useAgentBridge` stays
   `status: "disabled"` and no React shell has ever connected to `/bridge` — a gap that predates this
   slice and blocks the live-shell approval lane end to end in the React host. This slice closed both
   sides around it: the gateway now offers a bridge, and the dev server now publishes the session
   record **and serves the offer at `GET /__semio/agent-bridge`**. The remaining step is exactly one
   seam: make `useAgentBridge` acquire its config asynchronously from that endpoint (it is a loopback
   request to the local supervisor, which is what the hook's own comment says the proof must come
   from — not an environment carrier). Left undone deliberately: `🔗️AgentBridge/🟦️.tsx` was being
   edited by peers U1/M3 throughout this slice and has live component tests.
2. **The wgpu shell** has `semio_wgpu_set_agent_bridge_config`, which nothing calls. Same remaining
   seam, different host; M3 owns that panel.
3. **Elicitation has no wall-clock timeout.** A blocking `read_line` cannot be deadline-bounded without
   a reader thread. The wait ends on the client's answer or on EOF. In practice an MCP client always
   answers an `elicitation/create` (accept/decline/cancel), and a client that hangs has hung the whole
   connection anyway. The shell lane *is* deadline-bounded and tested.
4. **The offer is not re-published if a `dev s` session starts later.** Discovery runs once, at gateway
   start. Reconnecting the MCP server after starting `dev s` is the documented remedy, and the typed
   error says so with a live count. A watch-and-attach loop is the clean follow-up.
5. **`--hub` is still unreachable from any client config** (audit §3.1 item 10): it needs the fd-3
   credential, and nothing in the product spawns `semio-os-mcp --hub` from an authenticated session.
   Out of this slice; unchanged.
6. **`artifact_create`'s `kind` routing and `artifact_export` execution** are A1's items, untouched here.
6b. **The runtime approval round trip is blocked on A2's `InstanceOpen` guest trap** (§4c). This is the
   single claim in this slice that rests on unit tests alone rather than on observed runtime behaviour.
7. **The renderer's own vitest suite was not run** for the `AgentChatPanel`/`AgentBridge` edits: the
   renderer carries ~865 pre-existing TS errors (M2 §6) and the component suite is being edited by
   peers. The panel change is additive and type-checked only by inspection — this is the weakest
   verified claim in the slice.

---

## 6. Files changed

Rust (crate `semio-framework-os-mcp`):
`🛰️rendezvous/🦀️.rs` (new), `🛰️rendezvous/🧪️tests/🔬️quick/🦀️.rs` (new), `📦️packages/🦀️rust/🦀️.rs`,
`🦀️.rs`, `🚚️transport/🦀️.rs`, `🚚️transport/🧪️tests/🔬️quick/🦀️.rs`,
`🚚️transport/🧪️tests/🔬️long/🦀️.rs`, `🧭️protocol/🦀️.rs`, `🛡️policy/🦀️.rs`,
`🛡️policy/🧪️tests/🔬️quick/🦀️.rs`, `🔀️dispatch/🦀️.rs`, `🔀️dispatch/🧪️tests/🔬️quick/🦀️.rs`,
`🏠️workspace/🦀️.rs`, `🏠️workspace/🧪️tests/🔬️quick/🦀️.rs`, `🖥️ui/🦀️.rs`, `🏗️bootstrap/🦀️.rs`,
`🧪️tests/🔬️quick/🦀️.rs`, `🧪️tests/🔬️bin-quick/🦀️.rs`, `README.md`.

TypeScript: `📺️renderer/🧑‍🎨engine/🧱️elements/💬️AgentChatPanel/🟦️.tsx`,
`📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🟦️.tsx`,
`📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx`,
`🧑‍💻dev/🔌️vite-plugins/🟦️.ts`, `🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts`.

Configs: `.mcp.json`, `.cursor/mcp.json`, `.vscode/mcp.json`, `.windsurf/mcp.json`,
`.kiro/settings/mcp.json`, `.codex/config.toml`.

Ticket: `🐍️m4-bridge-approval-probe.ts`, `🗑️generated/m4-unit-tests.txt`, `🗑️generated/m4-probe.txt`,
this report.
