# 📓️ G19 — the AI integration as a USER experiences it (outcome 4 re-audit)

Read-only, no sub-agents, no builds, no servers, no source edits. Repo root
`/Users/ueli/Documents/semio`. Predecessor: `📓️audit-ai-mcp.md` (phase 0, 2026-09-21 AM — "chat panel
is an echo mock; inference not-wired except GIS; env-guard false positive"). This pass re-reads that
audit plus `📓️m8-mcp-agent-third-participant.md`, `📓️m9-agent-edits-hub-document.md`,
`📓️ce1-client-e2e-pinning-and-puzzle-bound.md`, `📓️ce2-mcp-gates-green.md`,
`📓️jb1-builtin-jobs-in-production.md`, `📓️rb1-release-builds-and-production-posture.md` §2,
`📓️g15-production-readiness-reaudit.md`, `📓️u1-progress-cancel-and-connection-status.md`, then reads
the CODE each claim rests on, as of **2026-09-22, session 8 in flight** (CE3/TC3e/CA1/HT16/FP11 are
running concurrently with this audit and are named where they overlap).

## 0. Headline

**Most of what phase 0 called broken is now fixed, live, and — for two of the P0 blockers — provable
from inside this very auditing session.** The picture outcome 4 needs is no longer "does MCP work at
all" but "how far does one real agent turn reach, and where does it still stop."

1. **Both P0 blockers phase 0 found are fixed in source, and this session is direct evidence for
   both.** `.mcp.json`'s `semio` entry's credential-seal guard (`🏗️bootstrap/🦀️.rs:104-118` in phase
   0) is narrowed to the `S_`/`VITE_S_` namespaces (`🏗️bootstrap/🦀️.rs:160-236`,
   `PROTECTED_CREDENTIAL_NAMES`/`_NAMESPACES`/`_MARKERS`) instead of a bare substring scan, so a
   Claude-Code-shaped `SESSION`/`TOKEN` variable no longer trips it. The `repo` entry's binary-path
   mismatch is fixed too: `defaultMcpBin()` (`📚️library/🟦️.ts:153`) now points at
   `.🧬semio/🦑️repo/⚡️cache/🗃️bin/semio-repo-mcp` — the path `@semio-tech/repo-mcp-go:build` actually
   produces — the file exists on disk (12.8 MB, 2026-09-18 22:14), the monolithic duplicate Go module
   `🔌️mcp/📦️packages/🐹️go` is gone, and `go.work` now lists `💻️client/🔌️mcp` (the modular, 9-tool,
   tested implementation) directly. **Direct observation, not a capture file**: this G19 session's own
   deferred-tool listing carries `mcp__repo__{ticket_open,ticket_close,ticket_reopen,section_move,
   file_integrate,section_extract,goal_open,goal_close,goal_reopen}` (9 tools) and
   `mcp__semio__{action_cancel,action_invoke,action_prepare,artifact_create,artifact_export,
   artifact_open,artifact_snapshot,artifact_validate,capabilities_describe,capabilities_search,
   context_resolve,history_redo,history_undo,inference_approve,inference_cancel,inference_events,
   inference_get,inference_list,inference_run,inference_submit,job_cancel,job_get,transaction_begin,
   transaction_commit,transaction_rollback,ui_focus,ui_reveal}` (27 tools, exactly RB1's measured
   census). Those names only appear when Claude Code's own `.mcp.json`-launched child processes
   completed `initialize`/`tools/list` at THIS session's start. I did not invoke any of them (the
   ticket's read-only mandate), but their presence is itself the strongest available evidence that
   both P0 blockers are cleared on the tree I am reading.
2. **The in-shell chat panel is no longer a mock.** `BasicChatPanel` (the phase-0 finding) is
   deleted; `AgentChatPanel` (`💬️AgentChatPanel/🟦️.tsx:1-13`) now renders the live MCP conversation —
   real `AgentToolCall`/`AgentToolResult`/`ApprovalRequested`/`ApprovalResolved` bridge frames, a
   working cancel control, and inline approve/deny. But it is a **one-way** surface for the human:
   there is no assistant-text reply frame anywhere in the wire (`GatewayToShell`,
   `🧵️bridge/🦀️.rs:1363-1381`, has no such variant), so a human's typed turn is parked in a
   drain-on-read inbox (`semio://ui/agent-messages`, `🖥️ui/🦀️.rs:727-736`) that the connected agent
   must poll and can only "answer" through tool calls, never through prose. §1b below.
3. **Inference is still one real end-to-end path (GIS, native, non-LLM) plus one framework-level fix
   (JB1) that unblocks every OTHER declared inference from a dead route, unproven live because the
   component that would prove it is still mid-rebuild in session 8 (CE3/TC3e).** §1c has the
   per-plugin table. A large, genuinely real inference surface — 68 geometric algorithms on `stdio`'s
   `gltf` artifact — is completely unreachable because `stdio` has no committed catalog descriptor at
   all.
4. **The agent as a third hub participant can now read a real hub document (M8/M9) and gets all the
   way to `wasmtime` instantiate on the write path, but has never actually committed an edit
   (`Commands` frame) to a hub document.** The remaining blocker (a component-world ABI boundary from
   an in-flight peer change) is a tree-wide fault, not specific to the agent path, and session 8's
   TC3e is actively rebuilding the hub catalog that would clear it.

---

## 1. User journeys

### a) A user installs `.mcp.json` and Claude drives a live `s` shell

**`.mcp.json`, read directly:**

```json
"repo": { "command": "bun", "args": ["./📜️script.ts", "dev", "mcp", "stdio", "client"] },
"semio": { "command": "bun", "args": ["./📜️script.ts", "dev", "mcp", "stdio", "os", "--folder", ".",
  "--scopes", "workspace.read,artifact.write,inference.execute,ui.observe,ui.control"] }
```

The `--scopes` string is RB1's **corrected** one — `artifact.open`/`artifact.create`/`inference.run`
(three of the seven names the README used to hand out) are not real scope names in
`MCP_SCOPE_TABLE` (`🛡️policy/🦀️.rs:26-43`) and pass through silently granting nothing
(`expand_scope`); the shipped config now uses only real names. **Verified**: every name in the
current `.mcp.json` string appears as a key in `MCP_SCOPE_TABLE`.

**Gate state, most-recent measured numbers (CE2, session 7, 2026-09-22 02:45; CE3 is re-measuring
these right now in session 8):**

| gate | what it proves | last measured | blocker |
|---|---|---|---|
| `client-e2e` (folder lane, real stdio, two real MCP-SDK clients) | discovery → prepare → invoke → snapshot → undo/redo → transaction → inference → job, on a real `note` artifact | **5/6** — CE1's fail-closed freshness step is the only red, naming `wfc`'s descriptor as stale | one plugin's descriptor lags its rebuilt component; CE3 (session 8) is re-describing it now |
| `capability-audit-check` | no undeclared-destructive / undeclared-audience gesture routes reach an agent | **29 findings / 1 diagnostic**, down from 111 (CE1) | needs ~14 plugins' source edited + re-described; CA1 (session 8) owns it |
| `live-agent-loop-check` | a **real** `s` shell (CE2's own serve, `:6196`) driven end-to-end: spawn → tool call → cancel → `ui_reveal`/`ui_focus` → the **whole agent journey** (create/open/prepare/invoke/snapshot/undo/redo/transaction/export) **visible in the live DOM** → Approve Once / Deny / an elicitation timeout refusal | **21/21, 0 failed** | none — this is the folder-lane proof that "Claude Code ↔ semio MCP ↔ running `s` shell ↔ verified state change" already works, on a live shell, today |
| `hub-agent-participant-check` (hub 7621) | the same journey against a HUB-hosted document | **14/17**, at its ceiling on 7621 | the remaining 3 reds are a stale hub binary + a stale hub catalog (pre-dates a codec-export ABI change), not repo code (CE2 §4); TC3d/TC3e own the rebuild |

**What this means for a real install today**: a user who runs `bun nx run
@semio-tech/framework-os-mcp-rs:build-release` once (RB1 did this — 362 s, 32.3 MiB binary, first
time ever executed) and points Claude Desktop or Claude Code at either the release binary or
`.mcp.json`'s dev command gets a server that **connects** (both P0 blockers are fixed), lists **27
real tools** (RB1's probe, `initialize`→`tools/list`→4 policy-gated `tools/call`s, 20/21 PASS with
one deliberate negative control), and can drive a **real, local `s` shell** end to end — proven live
by `live-agent-loop-check` 21/21, not merely claimed. Driving a **hub-hosted** document the same way
gets to 14/17 today, capped by infrastructure staleness the coordinator's fleet is actively rebuilding
(TC3e), not by a design gap. The weakest link for a brand-new user specifically is the `repo` server's
own tool-surface history — three earlier ticket rounds (§6 of the phase-0 audit) show it has broken
and been re-fixed multiple times in two months; I did not re-run it live (no servers), only confirmed
the binary/module-path fix is present in source and that this very session's tool list is consistent
with it working.

### b) A user inside the `s` shell asks the AI panel something

**Yes, there is a real panel, and it does call something real — but not a model, and not
bidirectionally.**

- `AgentChatPanel` (`💬️AgentChatPanel/🟦️.tsx`) is mounted in `ShellHost`
  (`🏛️ShellHost/🟦️.tsx:9291`, `createFrameworkChatPanelTab`), wired to `useAgentBridge()`'s real
  `status`/`presence`/`conversation`/`sendAgentMessage`/`cancelToolCall`/`resolveApproval`. Its own
  header comment states the contract precisely: "Nothing on this surface is generated locally: with
  no bridge attached the transcript is empty and the composer is disabled" (`:3-9`).
- Typing a message calls `sendAgentMessage` (`🔗️AgentBridge/🟦️.tsx:560-570`), which sends a real
  `{variant:"agentMessage", messageId, text}` shell→gateway frame (`ShellToGateway::AgentMessage`,
  `🧵️bridge/🦀️.rs:291,349`). The gateway does **not** answer it, run it through any model, or echo
  it: it parks it in a per-connection, capacity-bounded `agent_inbox`
  (`🧵️bridge/🦀️.rs:1634,2513-2522`, `BRIDGE_AGENT_INBOX_MAX_ITEMS`), read (and **drained**) only by
  the `semio://ui/agent-messages` MCP resource (`🖥️ui/🦀️.rs:727-736`) — i.e. the connected MCP client
  (Claude Code, Claude Desktop, or whatever else is attached) has to itself decide to poll that
  resource and act on it.
- The agent's only observable "response" is what it does through tool calls: `GatewayToShell`
  (`🧵️bridge/🦀️.rs:1363-1381`) has exactly `Welcome`, `ShellCommand`, `AppCommand`,
  `ApprovalRequested`, `ApprovalResolved`, `AgentPresence`, `Pong`, `Bye`, `AgentToolCall`,
  `AgentToolResult` — **no free-text assistant-reply frame at all**. `AgentConversationEntry`
  (`🔗️AgentBridge/🟦️.tsx:391-397`) mirrors this exactly: `userMessage | toolCall | approval`, no
  `agentMessage`/assistant-text variant. So a user who types "widen that wall to 300" and the agent
  never calls a tool (e.g. it's thinking, or asking a clarifying question) sees **nothing at all**
  appear in the panel — not even "thinking…" — because there is no channel for it.
- What IS real and working: `onCancelToolCall` → a real `AgentCancel` bridge frame (U1, tag 10) that
  reaches the gateway's own process-wide `job_registry()` and settles the SAME job `job_cancel`/
  `semio://job/{id}` would report (`🧵️bridge/🦀️.rs:2501-2504`, `AgentConversation::begin_tool_call`);
  `onResolveApproval` → a real inline Approve-Once/Deny/Approve-Session control, proven live by CE2's
  (e1)/(e2)/(e3) rows against the elicitation gate. Both are genuinely load-bearing UI, not decoration.
- **What it would take to close the gap**: the gateway needs a frame family for the agent's own
  free-text turns — the natural place is next to `AgentToolCall`/`AgentToolResult` in
  `GatewayToShell`, sourced from wherever an attached MCP client is expected to publish prose (there
  is no such publishing tool today: an agent has `action_*`/`artifact_*`/`inference_*` tools, but
  nothing like `chat.reply`). This is a **medium** scope change (a new frame variant, both codec
  twins, a fixture row, a new MCP tool for the agent side, a new `AgentConversationEntry` kind, one
  more `AgentChatEntry` render branch) — not found done anywhere in the reports read for this audit.

### c) A user runs an inference on an artifact of each plugin

Grepping for the actual registration call (`.inference_service(`/`.inference_services(` — the
`ArtifactInferenceService` builder, not the string "inference" which also matches doc comments and IO
grammar directories) plus `wfc`'s separate ActionBus job-factory route:

| plugin | declared inferences | real algorithm? | reachable via `inference_list`? | executes today? |
|---|---|---|---|---|
| `🌍️gis` (`🗿️artifacts/🗺️gismap/🦀️.rs:162`) | 1 (`gis_map_inference_service`) | **yes** — bounded geometry computation (feature/route/region counting, budget-checked, canonical payload) | yes | **the only inference wired end-to-end through the hub, non-LLM** (`audit-ai-mcp.md` §3, `GIS_MAP_NATIVE_EXECUTABLE`). Locally, CE1 measured its wasm ABI broken (stale component, `failed to convert function to given type`); not rebuilt since (TC3e, session 8, owns the `gis` wasm-release rebuild) |
| `🀄️wfc` (5 artifact kinds: `◻️2d`,`🔲️grid2d`,`🖼️bitmap`,`🧊️3d`,`🧱️grid3d`) | 5, one `inference_descriptors()` each, `inference_services: Vec::new()` at the artifact-schema level | **yes** — a real, shared WFC (wave-function-collapse) solver (`🀄️wfc/🦀️.rs:30-57`, "installs the five resumable solve jobs on the production action bus", `register_wfc{2d,3d,bitmap,grid2d,grid3d}_inference_factory`) reached through `ToolFactoryKey::new("semio.infer", …)`, not `ArtifactInferenceService` | yes (`inference_list`'s roster) | **was dead in every production build** — `spawn_job` dropped every builtin `JobFn` under `#[cfg(not(test))]` and answered `job.explicit-state-machine-required` (PZ1, confirmed by JB1). **JB1 rewrote the whole builtin-job path as a cfg-free bounded state machine** (`⚛️reactor/💼️jobs/🦀️.rs`, confirmed in source: `BUILTIN_JOB_KINDS`, no `cfg(test)` outside the `mod tests` mount) — proven natively (39/39 laws) and to compile into `wasm32-wasip2`, but **not yet proven live**: the component `describe` that would let the gate reach `inference_run` died on a 1800 s wall-clock deadline (later found to be starvation under fleet load, not a runaway guest — CE2 §5 fixed the deadline shape to a no-fuel-progress bound); CE3 (session 8) is the slice actually re-measuring this |
| `cad-extension-aec-building` (`🧩️extensions/🏢️aec-building/🦀️.rs:137-161`) | 1 (`building_structure_summary_service`, storey count / building-model presence) | **yes**, small but real | **no** — `inference_list`'s published roster is `gis ×1 + wfc ×5` only; CE1 measured a contributed extension's own inference as `NOT_FOUND … is declared for artifact kind s.cad.cad` in 1s. An extension's contribution never reaches the roster | no |
| `🗄️stdio` / `gltf` (`🗿️artifacts/🧊️gltf/🦀️.rs:96,105-165`) | **68** distinct `ArtifactInferenceService`s (size, bounds, volume, curvature, symmetry, thickness, topology — genuine geometric algorithms, each with its own `infer_gltf_leaf_*` function) | **yes, all real** | **no** — `stdio` has **no committed catalog descriptor at all** (`🔣️.json did not decode`/`NotFound: no committed descriptor`, the persistent 2nd catalog diagnostic every gate in this ticket reports); the whole plugin, not just its inferences, is invisible to `capabilities_search`/`inference_list` | no |
| every other plugin (≈56 of 60 registry rows) | 0 | — | — | — |

**What a user experiences today, concretely**: asking an agent to "run the solver on this" only ever
has one artifact kind that has ever worked end to end (GIS, and only through the hub's dedicated
native pipeline, currently ABI-stale locally); the WFC constraint solver is real and now framework-
admitted in production but its live proof is mid-flight in session 8; a real, sizeable inference
surface (the `cad-extension`'s building summary, and especially `stdio`'s 68 geometric measures) is
built and unreachable for two structurally different reasons (extension-contribution roster gap;
missing catalog descriptor). Progress/cancel wiring for an inference job IS real —
`inference_submit`/`inference_events`/`inference_cancel`/`inference_approve` are 4 of the 27 live
tools (RB1), and JB1's rewrite makes cancellation of a builtin job genuinely cooperative (retire
through the same bounded close protocol `job_cancel` uses) rather than a `Running` spin.

### d) The agent as a third hub participant

M8 named three gates between "agent principal" (can authenticate) and "agent participant" (can act
like a collaborator); M9 closed the second.

| gate | M8 (2026-09-21 AM) | M9 (2026-09-21 PM) | still open? |
|---|---|---|---|
| D1 — read a hub document by kind | refused: "not the authenticated MCP probe schema" | ✅ fixed, live-proven on hub 7621 (`artifact_open` answers `kind=gis.map`) | no |
| D3 — read a hub document's real bytes | refused: "canonical artifact bodies remain unavailable until P4-B" | ✅ fixed, live-proven (`artifact_snapshot` answers 80 801 real, digest-verified pack bytes) | no |
| D2 — dispatch (execute a mutation against a hub document) | refused: "repo root not found" (a hub binding had `repo_root = None`, so the gateway had no wasm to run at all, for any plugin) | ✅ the `repo root not found` refusal itself is gone — M9 built a real `DirectoryClient::document_execution_target_component` route, fetched **47 466 541** authorized component bytes from the hub, SHA-256-verified them against the manifest, and compiled them — **live-proven as far as `wasmtime` instantiate** | **yes, differently**: instantiate now fails on `no exported instance named semio:framework/codec@1.0.0` — every component published before peer TC3b's `export codec;` addition to `world actor` lacks it, including the one hub 7621 served. This is a **tree-wide** boundary (it also blocks `client-e2e`'s `artifact_create` on the FOLDER lane, per M9/CE1/CE2/JB1 all independently hitting it) and clears only when the trusted catalog is republished from a post-TC3b tree — **TC3e (session 8) is doing exactly this against hub 7651 right now** |

**No agent has ever committed an edit to a hub document (a `Commands` frame in the hub ledger) as of
the last report I read.** Every slice that reached this boundary (M9, CE2's `hub-agent-participant`
row 11/12) explicitly declined to fabricate a screenshot or a "proven" claim past it — M9: "No agent
edit crossed, so no human saw one, and no screenshot is offered." I found nothing in the reports read
for this audit, nor in session 8's in-progress files (all "(filling)" as of this read), that closes
this. The two remaining steps M8/M9 name after the codec boundary clears are (1) `LoadDocument` of the
canonical pair so the guest's session document IS the hub document, and (2) opening the
`…/socket-grants` document socket so a committed op leaves as a `Commands` frame and the presence beat
carries `principalKind: agent` — both unstarted, both named precisely (`📓️m9-…md` §8.1).

---

## 2. Gaps — file:line, root cause, fix scope, ownership

| # | gap | file:line | root cause | fix scope | session-8 ownership |
|---|---|---|---|---|---|
| 1 | Agent has no free-text reply channel in the in-shell chat panel (§1b) | `🧵️bridge/🦀️.rs:1363-1381` (`GatewayToShell`, no assistant-text variant); `🔗️AgentBridge/🟦️.tsx:391-397` (`AgentConversationEntry`, no matching kind); no MCP tool exists for an agent to publish prose | the panel was built to show tool-call traffic (M2), never designed for two-way conversation; there is no product concept of "the agent talks back" yet | **medium** — new bridge frame + codec twins + fixture row, a `chat.reply`-shaped MCP tool, a new conversation-entry kind, one render branch | **UNOWNED** — not in scope for C8, S11, TC3e, CE3, CA1, HT16, or FP11 (all are gate/build/hub-catalog focused) |
| 2 | `stdio`'s 68 real geometric inference algorithms are entirely unreachable (no catalog descriptor for the plugin at all) | `✏️s/🔌️plugins/🗄️stdio/🔣️.json` — described by CE1/CE2/PZ1 as failing to decode / `NotFound: no committed descriptor`; the 68 services themselves are real at `🧊️gltf/🦀️.rs:96-165` | `stdio` is a special-cased plugin (GM1's) whose descriptor pipeline was never completed; unrelated to this ticket's slices | **unknown without reading GM1's own scope** — likely **medium/large** given it is a whole-plugin descriptor gap, not a one-line fix (CE1 §8 gap 2 explicitly says "GM1's") | **UNOWNED** by this ticket's session-8 roster; explicitly flagged in CE1/CE2/PZ1 as belonging to a different owner (GM1) outside this ticket |
| 3 | An extension's contributed inference (`cad-extension-aec-building`) never reaches `inference_list`'s roster | `🌉️mcp/💡️inference/🦀️.rs` roster builder (CE1 §3b measured: roster is `gis ×1 + wfc ×5` only) | the roster compiler walks the plugin registry, not the extension-contribution registry, for inference services | **small/medium** — extend the roster builder to also walk registered extension contributions, the same seam `capabilities_search` presumably already uses for extension-contributed mutations | **UNOWNED** |
| 4 | `wfc`'s inference cannot yet be proven live end-to-end: the JB1 framework fix is real (native 39/39, compiles to wasm) but the gate that would exercise `inference_run` is still blocked on a fresh, current descriptor for the rebuilt component | `⚛️reactor/💼️jobs/🦀️.rs` (fixed); `✏️s/🔌️plugins/🀄️wfc/🔣️.json` (stale relative to its own component, per CE2 §1) | JB1's own component rebuild made the descriptor stale faster than the fleet wasm mutex let it re-describe; CE2's `OwnedDeadline` fix addresses the starvation that killed the earlier re-describe attempts, but is itself unverified live (compile only, per CE2 §5.4) | **small** remaining (run the queued re-describe once the mutex frees) but **status unknown to me** — I only have CE2's snapshot; CE3 owns exactly this | **CE3 (session 8)** — actively "filling" its own report on this right now |
| 5 | `gis`'s local wasm ABI is stale (`inference instantiate: wasmtime: failed to convert function to given type`) and was deliberately left unrebuilt by CE1 | `✏️s/🔌️plugins/🌍️gis/🔣️.json` / staged component | component age vs. host ABI drift, same class as gap 4 | **small** (a rebuild + re-describe) | **TC3e (session 8)** — its own scope line names "stdio/gis/note wasm-release" |
| 6 | Component-world ABI boundary (`no exported instance named semio:framework/codec@1.0.0`) blocks the agent's write path to a hub document (D2, §1d) and blocks `client-e2e`'s `artifact_create` on the folder lane too | `🔌️plugin/🧬️schema/📜️.wit` (`export codec;` added to `world actor`, uncommitted per M9 §2.4); every component published before it | peer TC3b widened the component world mid-session; every previously-built component (including hub 7621's catalog) lacks the new export | **large** — every plugin component must be rebuilt from a post-TC3b tree, and for the hub lane the trusted catalog must be republished (~45 min materialize, per G15's own JC1 cross-reference) | **TC3e (session 8)** — "three-package hub and note creation" on hub 7651 is exactly this rebuild+republish |
| 7 | No agent has ever committed a `Commands` frame to a hub document — steps 3–4 of M8/M9's D2 chain (`LoadDocument` of the canonical pair; opening the document socket) | `🌉️mcp/🏠️workspace/🦀️.rs:501` (`PersistenceBinding::Hub { surface: Some(PROBE_SURFACE_ID) }` pinned, not the document's real surface); `ArtifactHost::open` needs a registered native codec for `gis.map`, which this process does not have (M9 §8.1) | gated behind gap 6 clearing first; genuinely unstarted work after that, with a named unknown (no native `gis.map` codec in this process) | **large** — two non-trivial steps plus an unknown (whether a native codec for `gis.map` needs writing) | **UNOWNED** — not named in any session-8 slice title; closest is TC3e (rebuilds the catalog gap 6 needs) but TC3e's own scope line ("three-package hub and note creation") does not claim the write-path steps |
| 8 | `capability-audit-check`'s 29 findings (25 `WhenDestructive never fires`, 4 `declares no audience`) mean an agent-facing capability can be destructive or ungoverned without the audit catching it | ~14 plugin `🔣️.json` sources (architect, energy, gis, layout, lowpoly, mathematical, norm, playbook, sequence, trinity, vcs, wfc, writer, demonstrator per CE2 §2) | plugin authors declared verbs without `effects.destructive`/`audience` metadata; the audit is real, the plugins are not yet compliant | **medium/large** (14 plugins × source edit × re-describe through the serialized fleet mutex — CE2 measured this as "hours of serialized mutex time") | **CA1 (session 8)** — "capability-audit-zero" is exactly this |
| 9 | The MCP README's own "no descriptor declares `destructive: true` yet" caveat is now false (82 declared as of CE1, down to a smaller true count as gap 8 clears) — a user reading the README today is told the approval gate has nothing to fire on, which is no longer accurate | `🌉️mcp/README.md` (the "Honest limit as of today" paragraph, per G15 #10) | doc not updated as the underlying capability-audit sweeps landed | **small** — one paragraph edit, but should wait until gap 8's count stabilizes | **UNOWNED** |
| 10 | `.mcp.json` cannot express a hub-bound agent at all — it hard-codes `--folder .`, and `--folder`/`--hub` are mutually exclusive (`🏗️bootstrap/🦀️.rs:108`-equivalent parse check) | `.mcp.json` (repo root) | the config format assumes a folder-mode agent; a hub-bound agent needs a delegation UI to print a whole different argv (M8 §8.6, M9 §8.8, unchanged) | **small** (docs/example) to **medium** (an in-product "copy my MCP config" button off the AgentDelegations panel, which RB1 partly built the doc side of) | **UNOWNED** |
| 11 | `repo` MCP server's own tool-surface history is volatile — three prior repair tickets in 2 months (phase-0 §6); this pass could not verify live behavior (no servers permitted) beyond the deferred-tool listing evidence in §0.1 | n/a (historical pattern, not a single site) | recurring regression class, not a single defect | n/a | **UNOWNED** — worth CI (G15 ranked-item 7: no GitHub Actions workflow exists at all) |
| 12 | Hub startup can ABORT (not gracefully report `not-ready`) on its 30 s trusted-catalog wall-clock budget under fleet load — makes a busy machine look like a corrupt data root | `🌎️hub/🏗️bootstrap/🦀️.rs:586` (`TRUSTED_CATALOG_STARTUP_BUDGET_MS`) — M8/CE2 both measured this live, twice | a fixed wall-clock budget instead of the no-fuel-progress-style bound CE2 already applied to plugin `describe` | **small** (same shape as CE2's `OwnedDeadline` fix, applied to the hub's own boot path) — M9 §5.3 says it is written (`StartupArtifactAuthority`, compiled, `cargo check` 0 errors) but **never observed live** | **HT16 (session 8)** — title is literally "hub-startup-stall-bound-and-frontier-identity" |

---

## 3. Ranked list — what the coordinator should launch next for outcome 4

1. **Let CE3/TC3e/CA1/HT16 finish** (session 8, already in flight, already scoped to gaps 4/5/6/8/12
   above) before launching anything new against the same files — a fresh slice today would collide
   with the wasm mutex queue these four are already holding places in (`c8→s11→tc3e→ce3→ca1`,
   `📓️status.md:688`).
2. **A slice to give the agent a free-text reply channel in `AgentChatPanel`** (gap 1) — this is the
   single most user-visible remaining hole in outcome 4's own "working AI integration for users"
   framing: today a user can watch an agent act, and can type at it, but can never see it explain
   itself in the one surface built for exactly that. Medium scope, self-contained to
   `🧵️bridge`/`AgentBridge`/`AgentChatPanel`, and does not depend on any session-8 slice landing
   first.
3. **A slice to complete M8/M9's D2 chain** (gap 7: `LoadDocument` of the canonical pair + opening the
   document socket) the moment TC3e's catalog republish (gap 6) lands — this is the one step between
   "agent reads a hub document" (proven, M8/M9) and "a human sees an agent's edit land in a hub
   document" (the ticket's own stated outcome-4 bar, never yet crossed). Currently unowned; should be
   claimed explicitly rather than assumed to be inside TC3e's scope, since TC3e's own title names only
   the catalog rebuild.
4. **Extend the inference roster to include extension-contributed services** (gap 3) — small/medium,
   unblocks `cad-extension-aec-building`'s real (if modest) inference and is a template for any future
   extension that contributes one.
5. **Hand `stdio`'s descriptor gap (gap 2) to whoever owns GM1's line** — 68 real geometric algorithms
   sitting completely dark in the catalog is disproportionate to how little attention it is getting;
   this ticket's own slices have correctly treated it as out of scope every time it surfaced (CE1,
   CE2, PZ1 all name it and move on), so it needs an owner named outside this roster.
6. **A one-paragraph README correction** (gap 9) once CA1 (gap 8) lands and the destructive-declaration
   count stabilizes — cheap, and currently actively misleading a first-time reader about what the
   approval gate protects.
7. **A worked `--hub`/`--credential-file` live demonstration** (gap 10, = G15's own ranked item #9) —
   the pieces are all real and committed (M6's delegation UI, M8/M9's gateway-side wiring), but nobody
   has driven the whole loop with a real external MCP client the way `live-agent-loop-check` already
   does for the folder lane. Natural follow-on once gap 7 (item 3 above) closes, since it would be the
   first thing worth demonstrating.
8. **CI for the `repo`/`semio` MCP surfaces** (gap 11) — `.github/workflows/` is still empty (G15,
   re-confirmed nothing in the reports read here contradicts it); the `repo` server's three-times-
   broken history is exactly the class of regression a PR gate would catch before it reaches a ticket
   audit.
