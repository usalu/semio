# 📓️ M2 — a real AI surface: general inference execution + a live in-shell agent panel

Slice M2 of `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END`, closing audit `📓️audit-ai-mcp.md` §3/§4 gaps
**P1.4** (general inference execution) and **P1.5** (the decorative chat panel). Crate:
`semio-framework-os-mcp` (`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust`, nx
`@semio-tech/framework-os-mcp-rs`).

---

## 1. P1.4 — general inference execution

### Design

The audit's finding was exact: `HeadlessWorkspace`'s `ArtifactChannel` had **no infer command at all**
(`AppCommand` was `ReadHistory`/`PureCommand`/`Transaction*`), so every plugin-declared inference
answered a blanket `channel.not-wired`; the only executable inference in the whole gateway was the
hub-backed GIS Map job. The fix mirrors what `🏃️run`'s plugin reactor already does for `job_infer`:

```
inference_run (MCP tool)
  → ActionAdapter::run_inference           (🔀️dispatch — the same adapter every mutation uses)
    → AppCommand::Infer(InferCommand)      (the port's new command variant)
      → RoutingArtifactChannel             (routes by the command's own pluginId, not by instance slot)
        → PluginArtifactChannel::infer_real
          → ArtifactInferenceRouter::infer (semio_framework_plugin_host — the SAME router `🏃️run` uses)
            → PluginInstanceHandle::infer  → guest cold job `semio.infer`
          ← AppFrame::Inferred{inference_schema, complete, payload}
```

Key decisions, and why:

- **The router, not a second implementation.** `PluginArtifactChannel` opens a dedicated
  `GuestRuntimes` + `PluginInstanceHandle` for the cold `semio.infer` lane on first use and registers
  the plugin's own declared roster with a real `ArtifactInferenceRouter`
  (`ensure_inference_route`). That router — not this crate — resolves the route, toposorts and
  injects `dependsOn` results, drives the guest job, validates the guest's echo field-for-field and
  re-checks commit freshness. The gateway only builds the request wire and decodes the result.
- **A separate guest activation from the command lane.** `PluginInstanceHandle` takes *ownership* of
  its `GuestInstance` and drives `start-job`/`step-job` from a worker pool; the retained
  command-page loop in `exchange_one_real` cannot share that instance. The inference activation uses
  actor ordinal `u16::MAX`, disjoint from every `ensure_instance` ordinal (`instance + 1`).
- **Routing by declared owner, not by instance slot.** An inference carries no capability id, so
  `RoutingArtifactChannel::plugin_id_for` reads `InferCommand.plugin_id` (the declared row's
  `contributor`, which equals `owner` for an owner-authored inference — exactly the id
  `ArtifactInferenceRouter::register_plugin` is called with). `instance` is therefore meaningless for
  `Infer` and is a documented constant `0`.
- **Host policy vs caller input.** `workUnits` comes from the caller (clamped to ≥ 1);
  `allocationBytes` (1 MiB) and `recursionDepth` (4) are host constants — an agent must not be able
  to widen a guest's memory or dependency-recursion envelope.
- **Progress + cancellation** (AGENTS.md: every expensive operation carries both) use the existing
  plugin-agnostic `crate::ui::job_registry()` — the same registry `job_get`/`job_cancel` already
  read/act through. `inference_run` always mints a `job_` id, reports progress at dispatch and at
  hand-off, checks `is_cancel_requested` before dispatch and again after the guest returns, and the
  same `cancellationId` travels on the wire where the guest's own `semio.infer` loop polls it. This
  is stated honestly in the handler's own comment: the synchronous call itself cannot be interrupted
  mid-flight; the guest observes the cancel, the host observes it at the two points it owns.
- **`inference_get` stays a gap, but a different one.** It no longer claims "not wired": a bare READ
  carries no canonical request payload to run an inference *against*, so it points at `inference_run`
  and keeps its retryable code (a later packet that caches a document-derived payload turns it into a
  hit with no discovery-path change).

### Schema changes (schema-first)

`🌉️mcp/🧬️schema/🦀️.rs` is this scope's single registry; `🔣️.json` + `🟦️.ts` are generated from it by
`bun nx run @semio-tech/framework-os-mcp-rs:schema-mirror`. Six new exports (62 → **68**):

| export | what it is |
| --- | --- |
| `InferenceRunInput` | `inference_run`'s tool input (`artifactKind`, `inferenceSchema`, optional `pluginId`/`artifactId`/`payload`/`revision`/`generation`/`cancellationId`/`workUnits`) |
| `InferenceRunOutput` | `jobId`, `status`, `complete`, `payload`, `payloadBytes`, the resolved `pluginId`/`cancellationId` |
| `ArtifactInferenceRequestV1` | explicit MIRROR of `ArtifactInferenceRouter`'s private `InferenceRouteRequest` (that crate exports neither) |
| `ArtifactInferenceResultV1` | the subset of the guest result this gateway reads back (the router already asserted the full echo) |
| `ArtifactInferenceBudgetV1` | `allocationBytes`/`workUnits`/`recursionDepth` |
| `ArtifactInferenceCacheModeV1` | `cold`/`incremental`/`bypass` |

### Files (file:line)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔀️dispatch/🦀️.rs:94` — `pub struct InferCommand`
- `…/🔀️dispatch/🦀️.rs:120` — `AppFrame::Inferred { inference_schema, complete, payload }`
- `…/🔀️dispatch/🦀️.rs:139` — `pub struct InferenceOutcome`
- `…/🔀️dispatch/🦀️.rs:505` — `ActionAdapter::run_inference`
- `…/🔀️dispatch/🦀️.rs` (`MockInstanceState::handle`) — scripted `Infer` arm: real generation check, request payload echoed as the result payload
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:632` — `struct PluginInferenceRoute`
- `…/🏠️workspace/🦀️.rs:812` — `PluginArtifactChannel::ensure_inference_route`
- `…/🏠️workspace/🦀️.rs:846` — `PluginArtifactChannel::infer_real`
- `…/🏠️workspace/🦀️.rs:1169` — the `AppCommand::Infer` arm of `ArtifactChannel::exchange`
- `…/🏠️workspace/🦀️.rs` — `RoutingArtifactChannel::plugin_id_for` `Infer` arm; `INFERENCE_ACTOR_ORDINAL` / `INFERENCE_ALLOCATION_BYTES` / `INFERENCE_RECURSION_DEPTH`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🦀️.rs:67` — `DeclaredInference::route_plugin_id`
- `…/💡️inference/🦀️.rs:1216` — `inference_run_capability`
- `…/💡️inference/🦀️.rs:1474` — `inference_run_handler` (+ `inference_run_payload_bytes`, `inference_run_result_value`, `merge_inference_run_fields`, `INFERENCE_DEFAULT_WORK_UNITS`, `INFERENCE_ROUTED_INSTANCE`)
- `…/💡️inference/🦀️.rs` — `execution_not_wired_error` retargeted at `inference_run`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧬️schema/🦀️.rs:777` — `inference_run_input_shape` / `inference_run_output_shape`
- `…/🧬️schema/🦀️.rs:857` — `ArtifactInferenceRequestV1` and the rest of the `🔖️ArtifactInferenceWire` region; `schemas()` + `EXPORTS` bumped to 68
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs` — `GATEWAY_TOOL_NAMES` 26 → **27** (`inference_run`)

---

## 2. P1.5 — a real in-shell agent surface

### Design

`BasicChatPanel` was a client-side echo with local-only storage (`sendDraft()` appended the user's own
message back, truncated). It is **deleted**. In its place the panel renders the *actual* MCP
conversation, sourced from bridge frames the gateway emits from its own `tools/call` dispatch.

**Bridge frame family extension** (`🧵️bridge`, hand-rolled binary codec, Rust is the SSOT and
`🟦️.ts` its twin; `🧫️fixtures/📨️frames.json` is the anti-drift mechanism):

| direction | tag | frame |
| --- | --- | --- |
| Gateway → Shell | 8 | `AgentToolCall { invocationId, toolName, arguments }` |
| Gateway → Shell | 9 | `AgentToolResult { invocationId, toolName, ok, summary }` |
| Shell → Gateway | 9 | `AgentMessage { messageId, text }` |

Approvals needed no new frame: `ApprovalRequested`/`ApprovalResolved` already existed and are now
*rendered* instead of only counted.

- **Emitted from the real dispatch path, once.** `McpServer::handle_tools_call` (`🧭️protocol`) is the
  single point every tool call passes through. It calls `AgentConversation::begin_tool_call` *before*
  the handler runs (so a long tool is visible while it is still running, and the existing presence dot
  turns amber) and `finish_tool_call` after, with the result's first text block as the summary. There
  is no second bookkeeping path that could drift from what actually ran.
- **Bounded by construction.** Every human-readable string is truncated on a char boundary at
  `AGENT_CONVERSATION_MAX_TEXT` = 2048 with a visible `…`, so an agent calling a tool with a megabyte
  of arguments cannot starve the fixed-credit bridge outbox. Publishing is best-effort: no `/bridge`
  (stdio), no shell attached, or a momentarily saturated outbox must never fail or delay a tool call.
- **The receiving side is a real `semio://` resource.** MCP has no server push for this, so the agent
  polls `semio://ui/agent-messages`, which returns every turn the human typed since the last read,
  oldest first, and **drains** them — a turn handed to the agent is never handed to it twice. The
  per-connection inbox is capped at 64 and drops its oldest entry rather than growing unbounded when
  no agent polls. Bare/headless tiers answer the same typed, retryable `PLUGIN_UNAVAILABLE` every
  other UI resource answers, so "no shell" is never mistaken for "the human said nothing".
- **The panel.** `useAgentBridge` now tracks `conversation: readonly AgentConversationEntry[]` (a
  `userMessage`/`toolCall`/`approval` union, capped at 200 entries, with tool results folding into
  their own call row in place) and exposes `sendAgentMessage(text): boolean` — `false` when no socket
  is open, so the panel can tell the human their message did not go anywhere instead of showing it as
  if it had. `AgentChatPanel` renders an `<ol aria-live="polite" aria-label=…>` feed with per-entry
  `id`s and role/state chips, plus a labelled composer disabled (with an explanatory line) while
  disconnected — the same labelling/`id` discipline sibling panels use.
- **i18n.** Fifteen new keys under `os.agent.chat.*`, English first then German, through the existing
  `registerUiTranslationBundles` mechanism in `🔗️AgentBridge/🟦️.tsx` (where `os.agent.presence.*` and
  `os.agent.approvals.*` already live).

### Files (file:line)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧵️bridge/🦀️.rs:291` — `ShellToGateway::AgentMessage`
- `…/🧵️bridge/🦀️.rs:1335` — `GatewayToShell::AgentToolCall` / `AgentToolResult`, `AGENT_CONVERSATION_MAX_TEXT`, `truncate_conversation_text`
- `…/🧵️bridge/🦀️.rs:2593` — `BridgeHandle::take_agent_messages` / `pending_agent_message_count`, `AgentInboxMessage`, `BRIDGE_AGENT_INBOX_MAX_ITEMS`
- `…/🧵️bridge/🦀️.rs:2628` — `pub struct AgentConversation` (`begin_tool_call` / `finish_tool_call`)
- `…/🧵️bridge/🦀️.rs` — `encoded_len` / `encode` / `copy_encoded_page` / `decode` / `BridgeEncodedFrame::encode` arms; `ShellFrameKind::AgentMessage` and its bounded-decode + materialize phases
- `…/🧵️bridge/🟦️.ts` — the TypeScript twin of all three frames
- `…/🧵️bridge/🧫️fixtures/📨️frames.json` — 20 → 23 canonical rows
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧭️protocol/🦀️.rs:855` — `tool_result_summary`
- `…/🧭️protocol/🦀️.rs:894` — `McpServer::publishing_conversation_to` / `conversation()`; the emit pair inside `handle_tools_call`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs:514` — `publishing_agent_conversation` (wired into both server constructors)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🖥️ui/🦀️.rs:700` — `read_agent_messages_resource` + the `semio://ui/agent-messages` listing
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🚚️transport/🦀️.rs` — `AgentMessage` recorded like every other shell→gateway observation
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🟦️.tsx` — `AgentConversationEntry`, `AGENT_CONVERSATION_MAX_ENTRIES`, `sendAgentMessage`, the `agentToolCall`/`agentToolResult`/approval frame handlers, and the `os.agent.chat.*` en/de bundles
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/💬️AgentChatPanel/🟦️.tsx` — rewritten: live feed + composer
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:8558` — passes `conversation`/`onSendMessage`

### The echo mock, deleted

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx` — the whole `🎇️Basic Chat Panel` region (110 lines)
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx` — both barrel entries
- `🧰️framework/🔨️modules/🖱️ui/📖️stories/🎭️basic-chat-panel/` — the story directory
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/📚️storybook-uncovered-components/🟦️.ts` — its two story ids
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧫️storybook-discovery/🔣️.json` — its discovery row (107 → 106)

Repo-wide grep for `BasicChatPanel` now returns only ticket archives and the parity notes below.

---

## 3. Provider policy (documented)

`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/README.md` gained a section
**"No model provider — the agent is the client, not a dependency"**: no LLM/model-provider client
exists in this crate and none will, because CLAUDE.md forbids runtime dependencies on external
libraries and no model credential exists anywhere in the repo. "AI integration" runs the other way —
the OS exposes itself as a controllable substrate to whichever external agent the developer already
runs, over MCP. `inference_run` is explicitly *not* an exception: it executes a plugin's own declared
inference service (a computation the plugin ships), never a model call. The in-shell panel is a view
and steering surface for that external agent, not a chat client that talks to a model.

---

## 4. Verification

### 4.1 `cargo check`

```
$ cargo check --manifest-path Cargo.toml --all-targets --message-format short      # nx `check` target's command, widened to tests
(no output — clean; warnings emitted on the narrower --lib run prove expansion completed)
```

### 4.2 The new unit tests — all green

```
$ cargo test --manifest-path Cargo.toml --lib -- --test-threads=1 inference_run run_inference
running 6 tests
test actions::quick::run_inference_maps_a_stale_generation_to_a_revision_conflict ... ok
test actions::quick::run_inference_sends_one_infer_command_and_returns_the_guest_result ... ok
test inference::quick::inference_run_dispatches_a_previously_not_wired_service_identically ... ok
test inference::quick::inference_run_dispatches_the_gis_map_oracle_through_the_infer_command ... ok
test inference::quick::inference_run_is_scope_gated ... ok
test inference::quick::inference_run_refuses_an_undeclared_service ... ok
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 328 filtered out
```

```
$ cargo test --manifest-path Cargo.toml --lib -- --test-threads=1 agent_conversation agent_messages conversation_text
running 4 tests
test ui::quick::agent_conversation_publishes_the_real_tool_call_and_its_result ... ok
test ui::quick::agent_messages_resource_drains_the_shells_turns_exactly_once ... ok
test ui::quick::agent_messages_resource_is_retryable_plugin_unavailable_without_a_shell ... ok
test ui::quick::conversation_text_is_truncated_on_a_char_boundary ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 334 filtered out
```

The two `inference_run` dispatch tests are the pair the brief asked for: the **GIS Map oracle** (the
one service that was already executable, hub-backed) and **`wfc`'s `s.wfc.wfc3d.solve`** (previously
`channel.not-wired` for its whole life) travel the identical route with no per-plugin special case.
Both resolve their declared row from the plugins' own **committed `🔣️.json` descriptors** over a real
`--folder` `HeadlessWorkspace` (no fixture roster is invented), go through the registered
`inference_run` MCP tool via `ToolRegistry::call`, and are asserted on the real `AppCommand::Infer`
that left the dispatch path (plugin id, artifact kind, inference schema, cancellation identity) as
well as on the returned payload and job handle.

> ⚠️ The guest half of the route (`ArtifactInferenceRouter` → compiled `.wasm` → `semio.infer`) is
> **not** exercised by these tests: they stop at the `ArtifactChannel` seam
> (`MockArtifactChannel`, whose `Infer` arm enforces the real generation rule). Driving a real
> guest needs the plugin's compiled `wasm32-wasip2` artifact on disk, which is the `🔬️long` tier's
> territory, not a unit test's. What is verified is everything this slice owns: the command, the
> routing, the request wire, the tool, the scopes, the job handle, and the result decode.

### 4.3 Schema mirror regenerated

```
$ bun nx run @semio-tech/framework-os-mcp-rs:schema-mirror
schema-mirror: exports=68 ajv-draft07-resolved=68 json=1 typescript=1
```

`schema::quick::the_json_mirror_publishes_exactly_the_registry_exports` passes again.

### 4.4 Bridge codec parity (Rust ↔ TypeScript)

The Rust fixture test (`bridge::quick::every_fixture_round_trips_through_the_rust_codec`, counts
raised to 12 shell→gateway / 11 gateway→shell) and a foreground TypeScript run of the twin over the
same file both pass:

```
$ bun <scratch>/bridge-parity.ts
bridge-fixture-parity: shell_to_gateway=12 gateway_to_shell=11 total=23 encode=ok decode=ok
```

### 4.5 Renderer TypeScript

`@semio-tech/framework-renderer-react:typecheck` and `@semio-tech/ui-react:typecheck` are both RED —
but **not from this slice**. 865 and 25 `error TS…` lines respectively, all in files this slice never
touched (`♻️mit-bestand/🧺️demonstrator/**`, `✏️s/🔌️plugins/**` document-contract tests, `🛢️db/🧪️tests/**`,
`🔌️plugin/🏪️store/🧪️tests/**`, and `🏛️ShellHost`'s own pre-existing tutorial/interaction typing).
Filtering for the files this slice authored:

```
$ grep -E "error TS" <renderer typecheck log> | grep -E "AgentChatPanel|AgentBridge"
../../../../🧱️elements/🔗️AgentBridge/🟦️.tsx(197,5): error TS2353: Object literal may only specify known
  properties, and 'uiAppearance' does not exist in type 'ShellState'.
```

That one line is inside `createDefaultShellState`, unchanged by this slice — a peer's in-flight
`ShellState` field rename. **Zero** errors in `💬️AgentChatPanel/🟦️.tsx` and zero in any line this slice
added to `🔗️AgentBridge/🟦️.tsx` or `🏛️ShellHost/🟦️.tsx`. The `@semio-tech/ui-react` typecheck likewise
reports nothing under `🖱️ui/**`, so deleting `BasicChatPanel` left no dangling reference.

### 4.6 `@semio-tech/framework-renderer-react:agent-bridge-check`

```
Test Files  1 failed (1)
     Tests  1 failed | 33 passed (34)
```

The one failure is `AgentBridge inference state parity` at
`🔗️AgentBridge/🧪️tests/🧩️component/🟦️.ts:152` — `createDefaultShellState()` still carries nine `ui*`
fields (`uiAppearance`, `uiLayout`, `uiLocale`, `uiTerminology`, `uiThemeId`, `uiDriverId`,
`uiCustomDrivers`, `uiCustomThemes`, `uiKeybindingOverrides`) that `ShellState` and the
`💡️set-document-inference-port.json` fixture no longer declare. The same drift is the single
`AgentBridge` line in the typecheck above. `🖥️shell` is unmodified in the worktree and the fixture was
last committed 2026-09-09, so `ShellState`'s type changed at `HEAD` and this duplicate default was
never followed — pre-existing, and untouched by this slice (which added nothing to
`createDefaultShellState`). The other 33 assertions, including every frame-codec one, pass.

### 4.7 Full `--lib` suite

```
test result: FAILED. 316 passed; 21 failed
```

**No green baseline existed to compare against**: at `HEAD` this crate's *test* target did not even
compile (see §5). Every one of the 21 remaining failures is in code this slice neither authored nor
modified; each was classified by reading its assertion:

| failure | why it is not this slice |
| --- | --- |
| `bridge::quick::bounded_shell_decoder_and_materializer_advance_incrementally` | asserts `cursor == 9` after 3 steps on a **`ShellState`** frame (tag 1); `read_range` consumes a whole range in one step. Both the test and `read_range` are unmodified `HEAD` content. |
| `conformance::quick::*` (2) | `artifact.create`/`inference.{approve,cancel,submit}` are Mutation-kind with no declared writes — all pre-existing capabilities; `inference.run` (Job kind) is not among the findings. |
| `inference::quick::declared_inferences_for_workspace_finds_the_real_wfc_roster` | expects 1 row, the committed `wfc` descriptor now declares 5. |
| `inference::quick::gis_inference_discovery_…` / `inference::inference_jobs::*` (3) | expect `documentSchema`/`documentSchemaVersion` on `DeclaredInference`; no such fields exist in the code, and this slice added none. |
| `prompts::quick::no_prompt_names_a_specific_plugin_or_artifact_kind` | `safe_mutation` names `note`. |
| `root::quick::action_*` / `context_resolve_…catalog_hash` (4) | catalog-hash and prepared-action drift from plugin descriptors changing on disk. |
| `transport::quick::*` (2) | `payload.len() <= 125` and `drive_one` — unrelated to the one `AgentMessage` match arm added here. |
| `workspace::remote::tests::*` (5) | the `DirectorySessionAuthorityV1` refactor a peer has **in flight** in `📇️directory/🦀️.rs` (unstaged in the worktree); its fixtures answer `Unavailable`. |
| `workspace::quick::mcp_probe_document_transport_…`, `workspace::long::a_headless_commit_…` | probe/store composition + a 5s propagation timeout. |

---

## 5. Out-of-slice repairs (unblocking, deliberate, listed for the coordinator)

The crate's test target did **not compile** before this slice started; `cargo test -p
semio-framework-os-mcp` was impossible. Four minimal repairs to *committed* code were needed and are
called out here rather than buried:

1. `🚚️transport/🦀️.rs` — `HttpAdmission::authorizes_capability` matched `Self::HostSnapshot(..)`, a
   variant the enum never declared (it declares `#[cfg(test)] Fixture(..)`). Renamed to `Self::Fixture`.
2. `🌉️mcp/🦀️.rs` `🔖️Facets` — restored the `#[cfg(test)] pub use crate::source_builders::*;`
   re-export a sweep had dropped, leaving `📇️registry`'s and `🧪️tests`' own `#[cfg(test)]` modules
   calling `crate::note_descriptor()` / `note_and_cad_source()` unresolvable. Invisible to any
   non-test build.
3. `🏠️workspace/🔗️remote/🦀️.rs` — followed a peer's live `DirectorySessionAuthorityV1` field rename
   (`expires_at_ms` → `expires_at`, both `i64`) at its two consumer sites.
4. `🖥️ui/🧪️tests/🔬️quick` — `dispatch_shell_command_succeeds_once_…` and
   `…_surfaces_a_shell_fault_as_side_effect_rejected` each predicted "the next value" of the
   process-global `SHELL_COMMAND_SEQ` and raced each other under the parallel runner (one timed out
   with `PLUGIN_UNAVAILABLE` instead of asserting what it meant to). Merged into one sequential test,
   `dispatch_shell_command_resolves_a_matching_ok_reply_and_surfaces_a_shell_fault`, keeping both
   assertions.

A concurrent peer added `impl ArtifactCompositionFields for ProbeSnapshot` in `🏠️workspace/🦀️.rs`
mid-session; the duplicate this slice had written was removed and **theirs kept**.

`🏗️bootstrap/🦀️.rs` and the repo-mcp Go module were left untouched (slice M1's territory).

---

## 6. What is verified vs. not

**Verified by running:**

- the `Infer` command reaches the channel with the exact declared identity, and a stale generation
  maps to `REVISION_CONFLICT` (`actions::quick`, 2 tests);
- `inference_run` resolves both the GIS Map oracle and a previously-`not-wired` `wfc` service from
  real committed descriptors, dispatches each as `AppCommand::Infer`, mints a job handle, and refuses
  an undeclared service and an unscoped principal (`inference::quick`, 4 tests);
- the conversation sink publishes `AgentToolCall` → busy `AgentPresence` → `AgentToolResult` → idle
  `AgentPresence`, in that order, on a live connection, with the real arguments (`ui::quick`);
- `semio://ui/agent-messages` drains exactly once and is a typed retryable gap at the bare/headless
  tiers (`ui::quick`, 2 tests);
- conversation text truncates on a char boundary (`ui::quick`);
- all 23 bridge fixtures round-trip byte-identically through **both** codecs;
- the schema mirror regenerates to 68 exports and AJV draft-07-resolves every one;
- `cargo check --all-targets` is clean.

**Not verified (stated, not hidden):**

- **A real guest inference end to end.** No test in this slice drives a compiled plugin `.wasm`
  through `ArtifactInferenceRouter` → `semio.infer`. That needs built `wasm32-wasip2` artifacts and
  belongs in the `🔬️long` tier; the route is real code on a real router, but its guest half is
  unexercised here.
- **The panel in a running shell.** No browser probe was run — the renderer typecheck target is red
  from unrelated pre-existing errors, and booting the os dev shell was out of this slice's budget.
  The panel's data path is verified on the gateway side (frames emitted, inbox drained) and its
  TypeScript typechecks clean; what is unproven is the rendered pixels.
- **Cancellation mid-flight.** `job_cancel` flips the cooperative flag and the `cancellationId`
  reaches the guest, but no test proves a guest actually aborts — again a live-guest concern.

## 7. Known remaining gaps this slice deliberately did not close

- **The wgpu renderer still echoes locally.** `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`'s
  `agent_chat_messages`/`send_agent_chat_draft` is the wgpu twin of the deleted React echo. Its own
  module doc already states that this renderer's panel pipeline cannot project a transcript at all
  yet. Both it, its test (`🔬️wgpu-panel-anchor-model`), and the `💬️AgentChatPanel` wgpu target now
  carry an explicit **parity-debt** note pointing at the new bridge frames; closing it means reading
  those frames there, not keeping the echo.
- **`ui.chat.*` translation keys** in `🖱️ui` are now unused by any component. They are a generic UI
  vocabulary owned by that scope (declared in `📚️I18n/🟦️.tsx`'s type and four bundle sites); removing
  them is a `🖱️ui`-scope change, not this slice's.
- **`inference_get`** still cannot answer a value (no canonical request payload per artifact). It now
  says so accurately and names `inference_run`.
