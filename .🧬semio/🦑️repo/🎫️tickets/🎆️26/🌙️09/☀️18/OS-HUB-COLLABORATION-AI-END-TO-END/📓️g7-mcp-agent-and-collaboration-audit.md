# 📓️ G7 — depth audit: os MCP server, agent surface, collaboration×AI

Read-only, no builds, no servers. Traced from root `📜️script.ts` and read the live tree
(`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp`, `🌎️hub`). Builds on `📓️audit-ai-mcp.md`,
`📓️m1-mcp-servers-start.md`, `📓️m2-agent-surface-and-inference.md` — does not re-litigate M3/A1
scope (unit-test greenness, catalog-compile skips, the e2e client test). All paths below are
`file:line` against the current worktree.

**Headline finding not yet in any prior report**: `.mcp.json`'s `semio` entry
(`bun ./📜️script.ts dev mcp stdio os --scopes …`) resolves to `run_stdio`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs:717-732`), which **always passes `bridge: None`**
(line 726) and **never binds `--folder`/`--hub`** (`.mcp.json` passes neither). Every "live shell"
feature this crate ships — `AgentPresence`, `AgentToolCall`/`AgentToolResult`/`AgentMessage`,
`ui_focus`/`ui_reveal`, approvals-in-the-shell — is wired only for `run_http`
(`…/🦀️.rs:758-771`, bridge `Some(bridge_slot)`), a mode `.mcp.json` never launches and this
session's own MCP connection never uses. **As actually configured, the Claude-Code-facing `semio`
server runs bare (no workspace) with no bridge, permanently**, regardless of anything upstream
(M1/M2) fixed.

---

## 1. The ~27 tools — REAL / PARTIAL / STUB, runtime driven, and behavior with no os frontend

Tool census: `GATEWAY_TOOL_NAMES` (`…/🦀️.rs:250-278`), verified live by M1's handshake probe (27
tools, `📓️m1-mcp-servers-start.md` §4).

| tool | verdict | file:line | runtime it drives | with no os frontend running |
|---|---|---|---|---|
| `capabilities_search` | REAL | `🦀️.rs:219-226,428-430`; BM25 in `🔎️search/🦀️.rs` | the compiled `Catalog` (`build_catalog`, `🦀️.rs:148-154`) — real installed-plugin discovery, no LLM | works identically — catalog is filesystem-derived, not shell-derived. (Separately, in this tree it currently answers 0 hits because catalog compile fails on a duplicate capability id — A1/M1 territory, not re-audited here.) |
| `capabilities_describe` | REAL | `🦀️.rs:228-234,432-434` | same catalog | works identically |
| `context_resolve` | REAL | `🦀️.rs:236-243,436-439`, `🧠️context/🦀️.rs` | catalog + a monotonic session counter (`mint_session_id`) | works; `DEFAULT_SESSION_ID = "sess_default"` (`🦀️.rs:291`) is process-fixed regardless — every mutation-protocol tool call runs as ONE session for the life of the process, documented as a known P1b simplification, still true |
| `action_prepare` | REAL | `🔀️dispatch/🦀️.rs:618-666`, `🦀️.rs:441-446` | `ActionAdapter::prepare` → `AppCommand::PureCommand` dry-run against `ArtifactChannel` | tier-1 gate: `require_workspace`-shaped checks upstream of this exist per-facet, but `action_prepare` itself has none — it calls `exchange_one` against whatever `channel` was built; with no `--folder`/`--hub`, `channel = ArtifactChannels::Mock(MockArtifactChannel::new())` (`🦀️.rs:690`), so it answers against a **scripted mock instance**, not an error — a caller cannot tell from the result alone that nothing real is bound |
| `action_invoke` | REAL | `🔀️dispatch/🦀️.rs:693-731` (cached) / `731-…` (`invoke_uncached`) | same channel; commits via 2-phase `TransactionPrepare`/`TransactionCommit`, tags `MutationOrigin::Agent{principal, invocation_id}` (`🔀️dispatch/🦀️.rs:792`) | same Mock-channel caveat as above |
| `action_cancel` | REAL | `🔀️dispatch/🦀️.rs:672-679` | `HandleTable` revoke, no channel I/O | works identically (pure handle-table op) |
| `transaction_begin/commit/rollback` | REAL | `🔀️dispatch/🦀️.rs` `transaction_begin`/`transaction_commit`/`transaction_rollback` (saga over `preparedHandles`) | same `ActionAdapter`/channel | same Mock-channel caveat |
| `history_undo/redo` | REAL | `🦀️.rs:386-406`; `ActionAdapter::history_undo/redo` | fans `TransactionUndo`/`Redo` out to every member of a committed invocation/saga | same Mock-channel caveat |
| `artifact_open` | REAL | `🗿️artifact/🦀️.rs:221-255` | `HeadlessWorkspace::read_artifact_bytes`/`ensure_probe_artifact` — a real, generic, plugin-agnostic "probe document", not a plugin-typed artifact | tier-1: `require_workspace` (`🗿️artifact/🦀️.rs:135-137`) → typed retryable `PLUGIN_UNAVAILABLE` naming `--folder`/`--hub` |
| `artifact_create` | REAL (generic only) | `🗿️artifact/🦀️.rs:257-285` | same probe mechanism; `kind` argument is accepted but **not wire-routed to a plugin-specific document type** (file's own module doc, `🗿️artifact/🦀️.rs:14-17`) | same tier-1 gate |
| `artifact_validate` | REAL | `🗿️artifact/🦀️.rs:287-306` | real plugin validation via `semio://artifact/{id}/validation` resource read | same tier-1 gate |
| `artifact_snapshot` | REAL | `🗿️artifact/🦀️.rs:308-336` | real content read via `semio://artifact/{id}` resource | same tier-1 gate |
| `artifact_export` | PARTIAL | `🗿️artifact/🦀️.rs:338-365` | enumerates the resolved plugin's **real, committed** `export_formats` (`resolve_plugin_export_formats`, line 205-217) but **always** ends in `GatewayError::PluginUnavailable("no live export command is wired yet")` (line 358-362) — the wire protocol has no export-query command at all | same tier-1 gate, then always the above error even fully bound |
| `inference_list` | REAL | `💡️inference/🦀️.rs:265-287` | real `contributions.inference_services` roster off committed plugin `🔣️.json` | `workspace_binding_required` (line 261-263) |
| `inference_get` | STUB (honest) | `💡️inference/🦀️.rs:289-312`; `execution_not_wired_error` (line 177-184) | discovery only — "a bare read carries no canonical request payload to run it against — call `inference_run`" | same workspace gate; the one exception is `gis_map_hub_inference_read` (line 299, `GIS_MAP_*` consts at 398-409) which answers for real for that one hub-backed service |
| `inference_run` | REAL (per-plugin) | `💡️inference/🦀️.rs:1474` `inference_run_handler`; `🔀️dispatch/🦀️.rs:505` `ActionAdapter::run_inference`; `🏠️workspace/🦀️.rs:846` `PluginArtifactChannel::infer_real` | `AppCommand::Infer` → `ArtifactInferenceRouter` (same router `🏃️run`'s live plugin reactor uses) → the plugin's own wasm guest `semio.infer` job | tier-1 gate; unit-tested against the GIS Map oracle and `wfc`'s `s.wfc.wfc3d.solve` from real descriptors (M2 §4.2) — the guest half is **not** exercised by any test (M2 §4.2/§6), so "runs any plugin's inference" is proven at the routing/command layer, not proven against a compiled `.wasm` |
| `inference_submit/events/cancel/approve` | PARTIAL (one service) | `💡️inference/🦀️.rs:1306,1364,1393,1422` | hub-backed job pipeline (`🌎️hub/💡️inference`) — real ledger/WAL/authorization, but hardcoded to `GIS_MAP_INFERENCE_SERVICE_ID = "s.gis.gismap.inference"` / `GIS_MAP_INFERENCE_ARTIFACT_SCHEMA = "gis.map"` (line 398-401); no other artifact kind can ever submit through this quartet | requires `--hub` (see §2); folder mode cannot reach this path at all |
| `job_get` | REAL | `🖥️ui/🦀️.rs:461-470` | process-wide in-memory `job_registry()` (used by `inference_run`'s minted `job_` ids) | works identically — not bridge- or workspace-dependent |
| `job_cancel` | REAL | `🖥️ui/🦀️.rs:472-481` | same registry; flips a cooperative cancel flag the guest polls | works identically |
| `ui_focus` | REAL protocol, but **dead in the shipped config** | `🖥️ui/🦀️.rs:421-435` | `dispatch_shell_command(bridge, {"type":"focusWindow",…})` over `/bridge` | **always** `bridge_not_running_error()` (line 427-429) whenever `bridge` is `None` — which is every stdio launch, i.e. every `.mcp.json` connection today (see headline) |
| `ui_reveal` | same as `ui_focus` | `🖥️ui/🦀️.rs:437-459` | same bridge dispatch (`setPanelVisible`+`setPanelPath`) | same — always `bridge_not_running_error()` under `.mcp.json` |

**With literally no os frontend/workspace running** (i.e. exactly `.mcp.json`'s current
configuration: `dev mcp stdio os --scopes …`, no `--folder`, no `--hub`): every discovery tool
(`capabilities_*`, `context_resolve`) works for real; every mutation-protocol tool
(`action_*`/`transaction_*`/`history_*`) runs but against `MockArtifactChannel` — a **scripted
in-memory stand-in with no real effect and no clear signal to the caller that it is fake**; every
`artifact_*`/`inference_list`/`inference_get` tool answers a typed, retryable
`PLUGIN_UNAVAILABLE` naming `--folder`/`--hub`; `job_get/cancel` work (nothing to act on yet);
`ui_focus`/`ui_reveal` always fail with a bridge-not-running error.

---

## 2. The bridge to a LIVE os session

- **Transport**: a loopback WebSocket at `/bridge` on the **same** Streamable HTTP listener as
  `/mcp` (`README.md:79`, `🚚️transport/🦀️.rs`), bound only in `run_http`
  (`🦀️.rs:762-770`, `HttpTransport::new(...).publishing_bridge_into(bridge_slot)`). **`run_stdio`
  never creates this listener at all** (`🦀️.rs:717-732`) — stdio mode has zero bridge capability,
  not a degraded one.
- **Discovery**: none published anywhere the OS shell would find automatically — the log line
  `[semio-os-mcp] bridge listening on ws://{bind}:{port}/bridge` (`🦀️.rs:767`) is the only surfaced
  address; a live shell must be told this URL out of band. No mDNS/registry/handshake exists.
- **Auth/scopes enforcement**: `HttpTransportOptions::new(credential)` (`🦀️.rs:768`) requires
  `claimed_local_hub_credential("mcp")` (line 765-766) — i.e. HTTP/bridge mode **cannot start at
  all** without an inherited hub-issued fd-3 credential (`🏗️bootstrap/🦀️.rs`'s
  `S_LOCAL_CREDENTIAL_FD=3` claim path, also gated by the credential-seal M1 fixed). This means
  bridge-capable mode is only reachable as a child process the hub itself spawned with a delegated
  credential — an ad hoc `bun ./📜️script.ts dev mcp http os` run by a developer has no such fd and
  errors before binding a socket. Per-call scope enforcement for mutation-protocol tools is real:
  `PolicyEngine::authorize_scopes` (`🛡️policy/🦀️.rs:208-215`) checks `capability.policy.scopes` as a
  subset of the principal's granted scopes (parsed from `--scopes`,
  `AgentPrincipal::from_scope_names`), denying with `PERMISSION_DENIED` + an audit row on the first
  missing scope — genuinely wired, not decorative.
- **Approval flow for destructive actions — the one clear dead end found in this slice**:
  `ActionAdapter::invoke_uncached` gates via `PolicyEngine::gate_approval`
  (`🔀️dispatch/🦀️.rs:753`), which for a capability with `policy.approval = WhenDestructive|Always`
  (73 real occurrences across shipped plugin `🔣️.json` manifests, e.g.
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/release/🔌️plugin-modules/📐️cad/🔣️.json`)
  returns `ApprovalGate::Required{approval_handle}` → `GatewayErrorCode::ApprovalRequired`
  (`🛡️policy/🦀️.rs:753-767`). Three things the README's Safety section claims do NOT exist in code:
  1. **No MCP elicitation is ever sent.** `grep -rn elicitation` across `🧭️protocol`/`🔀️dispatch`/`🛡️policy`
     finds exactly one hit — a comment (`🛡️policy/🦀️.rs:121`) — never an actual
     `elicitation/create` request construction or dispatch.
  2. **No bridge frame is ever published.** `GatewayToShell::ApprovalRequested`/`ApprovalResolved`
     are defined (`🧵️bridge/🦀️.rs:1344-1345`) and the wgpu shell target can *decode and render* them
     (`📺️renderer/…/🔗️AgentBridge/🎯️targets/🧊️wgpu/🦀️.rs:359,395,636`), but `ActionAdapter` holds no
     bridge handle at all (`🔀️dispatch/🦀️.rs:467-476`) — nothing on the gateway side ever
     constructs or sends `ApprovalRequested`.
  3. **No tool resolves an approval.** `PolicyEngine::resolve_approval`/`ActionAdapter::resolve_approval`
     exist (`🛡️policy/🦀️.rs:259-266`, `🔀️dispatch/🦀️.rs:857-858`) but are **not** one of the 27
     `GATEWAY_TOOL_NAMES` — there is no `action_approve`/`approval_resolve` tool, and
     `action_invoke`'s own `approvalHandle` input field can only replay an *already-decided* handle,
     never decide one.
  4. `AutoApprovePolicy` (`never|readonly|all`) exists as a type (`🛡️policy/🦀️.rs:126-149`) but the
     CLI never parses an `--auto-approve` flag (confirmed absent from `parse_args` in
     `🏗️bootstrap/🦀️.rs`; `🦀️.rs:715-716`'s own comment: "`--auto-approve` has no CLI flag yet") —
     every real server construction hardcodes `AutoApprovePolicy::Never`
     (`🦀️.rs:520,544,690`/`server_for_workspace_options`).
  **Net effect: any capability a plugin marks destructive is permanently unapprovable through the
  shipped binary in any configuration — stdio or HTTP** — `action_invoke` returns
  `ApprovalRequired` and nothing in the system can ever answer yes.
- **How agent actions appear in the user's UI**: real, but bridge-gated. `McpServer::handle_tools_call`
  (`🧭️protocol/🦀️.rs:1023-1051`) is the single dispatch choke point; it calls
  `AgentConversation::begin_tool_call`/`finish_tool_call` (`🧵️bridge/🦀️.rs:2628`) around every
  `tools/call`, publishing `AgentToolCall`/`AgentToolResult` (tags 8/9,
  `🧵️bridge/🦀️.rs:1335`) — genuinely sourced from the real dispatch, not a second bookkeeping path
  (M2 §2 verified this by test). But `publishing_agent_conversation`
  (`🦀️.rs:531-536`) is a no-op when `bridge: None`, which — per the headline finding — is every
  `.mcp.json` connection. So the mechanism is real and tested (M2 §4.2), and **unreachable from the
  one client `.mcp.json` actually configures**.
- **Undo attribution**: real and distinct from a human edit. `invoke_uncached` tags every commit
  `MutationOrigin::Agent{principal: principal.id.clone(), invocation_id}`
  (`🔀️dispatch/🦀️.rs:792`), and `HeadlessWorkspace::actor_label()` = `format!("agent:{principal}#{session_id}")`
  (`🏠️workspace/🦀️.rs:1553-1555`) flows into the store `Mutation.actor` field (same file, lines
  575/941/1644) — so an event-log/VCS reader (human or another collaborator) can distinguish an
  MCP-agent edit from a human one by actor string, and `history_undo`/`redo` fan out
  `TransactionUndo`/`Redo` by the same `undo_token` the commit minted, not by re-deriving "whose
  edit was this."

---

## 3. Collaboration × AI

- **Does an MCP mutation in a hub space travel the same event-sourced command path?** For
  `--hub` mode: yes, architecturally. `HeadlessWorkspace::open_hub`
  (`🏠️workspace/🦀️.rs:1474-1494`) uses `NativeHubBindingDriver::connect` and
  `artifact_host.set_hub_socket_grant_source` — the **same native hub-socket binding machinery**
  (`store::sync::ArtifactHost`) the OS shell itself uses, not a parallel HTTP client — so a
  commit's `PureCommand`/`TransactionPrepare`/`TransactionCommit` frames genuinely replicate through
  the real backbone. **But `--hub` is unreachable from `.mcp.json`**: `open_hub` requires
  `claimed_local_hub_credential("mcp")` (line 1474, `1486`), which requires a `S_LOCAL_CREDENTIAL_FD=3`
  inherited fd (`🏗️bootstrap/🦀️.rs:182-192`) — only present when the process is spawned as a
  child of an already-authenticated hub session, never when a developer or Claude Code launches
  `.mcp.json`'s bare `dev mcp stdio os` directly. In the tree today there is also no wiring anywhere
  that spawns `semio-os-mcp --hub …` FROM an authenticated hub/shell session — `grep -rn "semio-os-mcp"
  🌎️hub 🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell --include=*.rs --include=*.ts` returns no hits — so
  the credential-delegation path this code is built for has no caller anywhere in the product yet.
- **Does the agent have an identity/principal in the hub directory, and show up in presence?**
  No. The hub's `LocalBootstrapClientClass::Mcp` (`🌎️hub/📇️directory/🦀️.rs:929-944`) is a **device/channel
  classification of an already-authenticated human's own session** (like `Native`/`ReactRelay`), not
  a distinct agent principal — `McpCredentialEnvelopeDelivery::deliver_mcp`
  (line 999-1001) delivers the *human's* session capability to the MCP-launched child. The MCP
  gateway's own `AgentPrincipal` (`agent:local` or `--principal <id>`,
  `🛡️policy/🦀️.rs`) is a **local, in-process policy identity** — `grep -rln AgentPrincipal 🌎️hub`
  returns nothing; the hub's presence/directory system has no concept of it. A hub-space edit an
  MCP agent makes (in the unreachable `--hub` path above) would replicate under the *delegating
  human's* directory identity with an `agent:<principal>#<session>` actor tag on the mutation
  itself (§2) — collaborators would see "a change happened," attributable by actor string if the UI
  ever surfaces it, but presence (who's online) would not show "an AI agent," only the human whose
  credential it borrowed, if presence tracks MCP sessions at all (unverified — out of this slice's
  budget to trace the presence-lease fixture, `🌎️hub/🧫️fixtures/👥️presence-lease-v1`, end to end).
- **Hub-side MCP endpoint for remote agents?** None. `find 🌎️hub -iname "*mcp*"` and
  `grep -rli mcp 🌎️hub` (§ above) turn up only the local-bootstrap client-class plumbing — no
  HTTP/SSE/streamable MCP surface is exposed BY the hub itself. MCP is strictly a **local
  process** the OS product ships (`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp`); a "remote agent"
  concept (an MCP client connecting to a hub-hosted space over the network, as opposed to a
  human's own machine running `semio-os-mcp --hub` with a locally-delegated credential) does not
  exist in this codebase.
- **What is missing for "AI agent as a collaborator in a shared space"** (concrete, not
  rhetorical):
  1. A caller that ever launches `semio-os-mcp --hub … --scopes …` with a real delegated
     credential — the credential-claim code exists, nothing invokes it end-to-end.
  2. `.mcp.json`/`run_stdio` never accepting `--folder`/`--hub` at all today (it's launched with
     neither) means the "agent as collaborator" story cannot start from the one config that
     actually ships.
  3. A resolvable approval path (§2) — a hub-space collaborator's own approval dialog would be the
     natural place to resolve an agent's destructive request, and no wiring reaches it.
  4. A bridge that binds in stdio mode too (or a documented, supported way to run HTTP mode without
     a hub-delegated fd) — otherwise "watch the agent work in your OS" requires infrastructure
     nothing in the product currently assembles.
  5. A hub-directory concept of "agent principal" distinct from "human's device," if the product
     ever wants presence/attribution to say "Agent (on Ueli's behalf)" rather than showing nothing
     or showing the human.

---

## 4. Inference

- **Provider abstraction / API keys**: none exist. `README.md:44-55` ("No model provider") is
  accurate: `grep -rn "anthropic|openai|gemini|model_provider|api_key" 🌉️mcp 🌎️hub/💡️inference` (this
  slice's own re-check) returns zero hits beyond the prior audit's findings. There is no
  credential storage, no provider enum, nothing to configure — by design, per `CLAUDE.md`'s
  "no runtime dependencies on external libraries."
- **Providers**: none (Anthropic/OpenAI/local model providers are all absent). "Inference" in this
  codebase means a **plugin's own declared native/wasm computation** (`contributions.inference_services`
  in a plugin's `🔣️.json`), never an LLM call.
- **Queue/approval/cancel/progress**: real for the ONE wired hub-backed service.
  `🌎️hub/💡️inference/🪶️sqlite` is a durable idempotent job ledger
  (`state IN ('accepted','running','succeeded','failed','cancelled')`, per the prior audit, unchanged
  here); `inference_submit/events/cancel/approve` (`💡️inference/🦀️.rs:1306-1470`) are real MCP-side
  handlers over that pipeline — but only for `GIS_MAP_INFERENCE_SERVICE_ID` (§1 table). For
  `inference_run` (any plugin), progress/cancel is coarser: `job_registry()` progress checkpoints at
  dispatch and hand-off only, and cancellation is cooperative (`cancellationId` polled by the
  guest) — M2 §4.2/§6 already states honestly that "the synchronous call itself cannot be
  interrupted mid-flight," confirmed again by reading `inference_run_handler`
  (`💡️inference/🦀️.rs:1474+`) and `run_inference` (`🔀️dispatch/🦀️.rs:505`): both are ordinary
  synchronous calls into `ArtifactInferenceRouter::infer`, wrapped in job bookkeeping, not an
  actually-interruptible async job.
- **Artifact kinds wired beyond GIS**: `inference_run` (not `inference_submit`) is the general
  route, and per M2's own tests it was proven to dispatch `wfc`'s `s.wfc.wfc3d.solve` identically to
  GIS Map (M2 §4.2) — i.e. **any plugin that declares an `inference_services` contribution and
  ships a `semio.infer` guest job** is reachable through `inference_run`, at the
  command/routing layer (the guest execution itself is untested here, per M2's own caveat). The
  `execution_not_wired_error` this crate used to return for every non-GIS service
  (`💡️inference/🦀️.rs:177-184`) now fires only from `inference_get` (a bare read with no payload to
  run against), not from `inference_run`.
- **Can the in-app agent chat panel drive a model that calls MCP tools?** No, and it is not meant
  to. `AgentChatPanel` (`📺️renderer/…/💬️AgentChatPanel/🟦️.tsx`, rewritten by M2) **displays** the
  real `tools/call` traffic (§2) and lets a human type a turn back via `sendAgentMessage` →
  `AgentMessage` bridge frame → `semio://ui/agent-messages` resource an external agent polls
  (`🖥️ui/🦀️.rs:700`) — it is a view/steering surface for an EXTERNAL agent (Claude Code, Codex,
  etc.), never a client that itself calls a model. This is by explicit design
  (`README.md:44-55`, M2 §3) — there is no loop where the in-shell panel both talks to a model AND
  drives MCP tools; it can only ever show/relay what an already-connected external MCP client does.
  Per the headline finding, even that display path is unreachable while the client uses
  `.mcp.json`'s stdio launch, since there is no bridge in that mode.

---

## 5. MCP protocol conformance

- **Protocol version**: dual-era, `SUPPORTED_PROTOCOL_VERSIONS = ["2026-07-28","2025-11-25","2025-06-18"]`
  negotiated per `handle_initialize`/legacy detection (`🧭️protocol/🦀️.rs:1005-1015`) — genuinely
  implemented, not aspirational (M1 verified `2025-06-18` live).
- **Resources**: real, listed + read + subscribe/unsubscribe endpoints exist
  (`🧭️protocol/🦀️.rs:1053-1098`). **`resources/subscribe`/`unsubscribe` are unconditional no-ops**
  (`🧠️context/🦀️.rs:265-271`, `return Ok(())` for any URI) — the server advertises
  `"resources": {"subscribe": true}` in `server_capabilities()` (`🧭️protocol/🦀️.rs:836-842`) but
  never emits `notifications/resources/updated` anywhere in production code (only the constant
  `NOTIFICATION_RESOURCES_UPDATED` at line 283 exists; zero call sites outside `🧪️tests`). A client
  that subscribes to `semio://artifact/{id}` to watch a live edit will wait forever. Likewise
  `tools`/`prompts` `listChanged: true` is declared but `NOTIFICATION_TOOLS_LIST_CHANGED`/
  `NOTIFICATION_RESOURCES_LIST_CHANGED` have zero non-test call sites — the tool/prompt set is
  static per process anyway, so this is lower-stakes than the resource case but still an
  over-declared capability.
- **Prompts**: real, 5 bilingual (EN/DE) prompts (`💬️prompts/🦀️.rs:74-153`:
  `explore_workspace`, `safe_mutation`, `inspect_artifact`, `drive_the_ui`, `undo_last_change`),
  replacing the previously-empty registry per the AI-MCP-END-TO-END closing summary.
- **Structured tool output schemas**: real. Every tool response carries `structuredContent`
  (`🧭️protocol/🦀️.rs:1040-1045`, `CallToolResult::ok(content, Some(structured))` throughout
  `🗿️artifact`/`💡️inference`/`🖥️ui`/`🔀️dispatch`), and every tool declares a named `outputSchema`
  sourced from the single `🧬️schema/🦀️.rs` registry (`tool_from_capability`, `🦀️.rs:211-217`) — not
  ad hoc per-handler shapes.
- **Error mapping**: real, structured JSON-RPC mapping. `GatewayError::to_json_rpc_parts`
  (`⚠️errors/🦀️.rs:131+`) maps `GatewayErrorCode` variants to `-32602` (input/precondition/revision
  classes, line 56) or `-32603` (permission/approval/unavailable/side-effect/cancelled/
  compensation/budget/internal classes, lines 57-64), with a `data` payload carrying the semantic
  code, `retryable` flag and `details` — richer than a bare JSON-RPC error, and consistently used
  by every handler in this audit (`CallToolResult::tool_error`, `DispatchOutcome::Error`).
- **Cancellation/progress notifications**: **not implemented as native MCP primitives.**
  `notifications/cancelled` is explicitly documented as a recognized no-op
  (`💡️inference/🦀️.rs:963,1075`: "cancels a REQUEST and not a job") — the real cancellation surface
  is the `job_cancel` tool against `job_registry()` (§1/§4), a poll-based side channel, not the
  spec's request-scoped cancellation. **No `notifications/progress` support exists at all** —
  `grep -rn "progressToken|notifications/progress"` across `🧭️protocol`/`🔀️dispatch`/`💡️inference`
  returns zero hits; progress is exposed only by polling `job_get`.
- **Pagination**: **absent.** `handle_tools_list`/`handle_resources_list`
  (`🧭️protocol/🦀️.rs:1017-1055`) never read an incoming `cursor` param and never emit `nextCursor` —
  contrast with the `repo` server, whose Go implementation genuinely supports `tools/list` cursors
  (`📓️audit-ai-mcp.md` §2). At 27 tools this is not yet operationally painful, but it is a real
  spec-conformance gap if the tool count grows past one page for a client that enforces page sizes.
- **Minor hygiene bug spotted in passing**: M1's own handshake capture
  (`📓️m1-mcp-servers-start.md` §4) lists `resources/list` returning `semio://workspace/artifacts`
  **twice** in an 8-entry list ("… `semio://window`, `semio://workspace`,
  `semio://workspace/artifacts`, `semio://workspace/artifacts`") — a duplicate resource URI in the
  static resource registry, not re-diagnosed to a line here (small, cosmetic, but a client that
  keys off resource URIs could double-count).

---

## 6. Ranked work list (excludes M3/A1 scope: unit-test greenness, catalog-compile skips, the e2e client test)

### P0 — the shipped configuration cannot do what the README promises

1. **Wire a bridge into stdio mode, or change `.mcp.json`/the default binding so the configured
   client gets one.** Today `run_stdio` (`🦀️.rs:717-732`) hardcodes `bridge: None`; every
   AgentPresence/AgentToolCall/AgentToolResult/ui_focus/ui_reveal feature (M2's whole slice) is
   unreachable from the one client `.mcp.json` launches. Minimal fix: give `StdioOptions` an
   opt-in `--bridge-port` that starts the same loopback listener `run_http` does, alongside stdio
   serving; or document/require HTTP mode for anyone who wants the live-shell experience and make
   `.mcp.json` launch that instead (needs the credential-delegation gap in item 3 solved first).
   Files: `🦀️.rs:698-772` (`StdioOptions`, `run_stdio`), `.mcp.json`.
2. **Make destructive-capability approval resolvable at all.** Currently: no elicitation is ever
   sent (`🛡️policy/🦀️.rs:121` comment only), no `ApprovalRequested` bridge frame is ever published
   (`ActionAdapter` holds no bridge, `🔀️dispatch/🦀️.rs:467-476`), no tool exists to resolve an
   approval handle (`resolve_approval` at `🛡️policy/🦀️.rs:259-266`/`🔀️dispatch/🦀️.rs:857-858` is
   unreachable from `tools/list`), and `--auto-approve` has no CLI flag
   (`🦀️.rs:715-716`). Any of the 73 shipped plugin capabilities marked `whenDestructive`/`always`
   permanently deadlocks `action_invoke`. Minimal fix: add an `action_approve`/`approval_resolve`
   MCP tool wrapping `ActionAdapter::resolve_approval`, OR parse `--auto-approve` off argv and pass
   it through `server_for_workspace_options` (already accepts an `AutoApprovePolicy`, just never
   receives anything but `Never`). Files: `🏗️bootstrap/🦀️.rs` (`parse_args`), `🦀️.rs:419-499`
   (`build_tool_registry`), `🛡️policy/🦀️.rs`.
3. **`.mcp.json`'s `semio` entry binds neither `--folder` nor `--hub`.** Every artifact/inference
   tool call from this exact session answers a typed `PLUGIN_UNAVAILABLE`, and every
   mutation-protocol tool call silently runs against `MockArtifactChannel`
   (`🦀️.rs:690`) with **no signal in the result that it's fake** — a caller cannot distinguish "ran
   for real" from "ran against a scripted mock" without reading source. Minimal fix: default
   `.mcp.json` to `--folder .` (repo root) as M1's own probe did manually, or make
   `action_prepare`/`action_invoke` responses carry an explicit `"backend": "mock"|"real"` field
   when no workspace is bound, so silent fakery is never possible. Files: `.mcp.json`,
   `🔀️dispatch/🦀️.rs` (`PreparedActionReport`/`InvocationReport` shapes).

### P1 — real functionality with a named, closable gap

4. **`artifact_export` never executes** (`🗿️artifact/🦀️.rs:338-365`) — enumerates real formats,
   always errors on the actual export. Needs a wire-protocol export-query command on
   `ArtifactChannel`, mirroring how M2 added `AppCommand::Infer` for inference.
5. **`artifact_create`'s `kind` is accepted but not routed** to a plugin-specific document type
   (`🗿️artifact/🦀️.rs:14-17,257-285`) — every created artifact is the same generic probe document
   regardless of declared kind.
6. **`resources/subscribe` is a permanent no-op** while `server_capabilities()` advertises
   `subscribe: true` (`🧠️context/🦀️.rs:265-271`, `🧭️protocol/🦀️.rs:838-840`) — either implement real
   `notifications/resources/updated` push on artifact mutation, or stop advertising the capability.
7. **No MCP-native progress/cancellation** (`notifications/progress`/`notifications/cancelled` are
   absent/no-op; `job_get`/`job_cancel` polling is the only surface) — acceptable as a stated
   design choice, but worth a README line since the spec's own progress mechanism is silently
   unsupported.
8. **No pagination on `tools/list`/`resources/list`** (`🧭️protocol/🦀️.rs:1017-1055`) — low urgency
   at 27 tools, but add a `cursor` no-op (single page, `nextCursor: null`) now so growth doesn't
   silently break spec-strict clients later.
9. **`inference_submit/events/cancel/approve` are GIS-Map-only** by hardcoded service id
   (`💡️inference/🦀️.rs:398-401`) — if the hub job quartet is meant to generalize the way
   `inference_run` did, it needs the same per-plugin-descriptor routing `inference_run` already
   proved out.
10. **Collaboration×AI has no live caller of `--hub`'s credential-delegation path** (§3.1) —
    `open_hub`/`claimed_local_hub_credential` code exists with zero production call sites that
    spawn `semio-os-mcp --hub` from an authenticated session. Until something calls it, "an MCP
    agent editing a hub space" is unreachable in practice, independent of every other fix above.

### P2 — polish / hygiene

11. Duplicate `semio://workspace/artifacts` entry in `resources/list` (M1's own capture, §5) —
    small registry-dedup fix in the resource registry.
12. Document, in `🌉️mcp/README.md`'s Safety section, that elicitation/shell-approval/`--auto-approve`
    are **not yet implemented** rather than describing them as already-working alternatives — the
    current prose reads as done; P0.2 above shows none of the three paths function today.
13. Hub directory has no "AI agent principal" concept distinct from the delegating human
    (`🌎️hub/📇️directory/🦀️.rs:929-944`) — worth a design note if presence/attribution is ever meant
    to show "Agent (on X's behalf)" rather than nothing or the human's own identity.
