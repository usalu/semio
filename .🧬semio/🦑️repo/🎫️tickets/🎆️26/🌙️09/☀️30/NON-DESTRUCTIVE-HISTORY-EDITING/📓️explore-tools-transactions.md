# 🔎 Explore: tools, interactions, actions, commands and transactions today

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`. Read-only audit (no source edited, no cargo/bun run, no git write).
Requirement under test (dev): *"Tools must be state machines that yield mutations within a transaction. Tools are
interactive and hence can't be altered by history but the mutations can be changed."* Example: puzzle 2d, dragging a
selection triggers a drag mutation; selection and drag offset stay editable afterwards in the history.

Sources were re-read on 2026-09-30. The runtime files churn under concurrent edits (P grew ~15 lines while I read), so
anchor on symbol names; line numbers are "as read". Static counts come from `git grep` / a file-list walk (tracked +
untracked, `git ls-files -co --exclude-standard -- "✏️s/🔌️plugins"`, script kept as `🗑️generated/explore-tools-census.py` after re-pointing its input path), not from a build.

Abbreviations used below:

- `FM` = `🧰️framework/🔨️modules`
- `OS` = `🧰️framework/🛍️products/💻️os/🔨️modules`
- `P` = `OS/🔌️plugin/🦀️.rs` (≈45.3k lines, the plugin runtime: `VcsArtifactApp`, `Emit`, dispatch, command log)
- `PT` = `OS/🔌️plugin/⏯️tool-run/🦀️.rs` (the `ToolRunLedger` driver)
- `S` = `OS/🏪️store/🦀️.rs` (`ArtifactStore`, ≈23.6k lines)
- `PL` = `✏️s/🔌️plugins`

---

## 0. Headline findings

1. **No generic "tool = state machine yielding mutations inside a transaction" contract exists.** Three unrelated
   things carry the word *tool* (§1): a static manifest row, the command job bus, and the algorithmic `ToolRun`.
2. **`ToolRun` is the closest thing to the requirement** (`FM/⏯️tool-run` + `PT`): a lifecycle state machine, a
   provisional-mutation ledger rendered as an overlay, zero-trace abort, and finalize as ONE `Edit` stamped
   `group_id = "toolRun:<run>"`. But it is scoped to long algorithmic runs with a panel (17 declaring tools in 8
   plugins), yields plain ops (no parametric inputs) and forgets the run after finalize except for that group id.
3. **Interactive pointer tools are not framework state machines.** They are (a) host-local engines/refs (puzzle 2d
   `BoardHost::InteractionMode`, `World3dHost` gumball refs) that flush one batch on release, (b) app scratch
   (lowpoly `LowpolyScratch`, draw `DrawingSession`), or (c) per-tick `Emit::amend` into a coalesced edit (20 plugins).
   Only draw (a private copy of the `machine` crate) and CAD (64 declarative `spatial.interaction` JSON assets) use a
   real statechart. The framework `🔄️machine` crate has **no Rust consumer** besides its own tests.
4. **Almost no drag becomes a drag mutation.** Puzzle 2d turns `nodeDragEnd.moves` / `translateSelection` into a scratch
   fixture, diffs it (`puzzle2d_document_delta_operations`) and stores N absolute `MoveNode{id,newX,newY}` ops ("absolute
   FINAL-state position"). Selection ids and offset are gone from history. CAD `MoveObjects{pane,placements}`, flow
   `MoveWidgets{entries}` and puzzle 3d `move-object` are absolute too. 2691 of 2946 mutation leaves repo-wide are
   `diffParticipation: "detect"` (snapshot-diff-derived). **The one precedent of the requested shape is 🎥️shooting:**
   `DragAssets{asset_ids, dx, dy, dz}`, `RotateAssets{asset_ids, ax, ay, az, angle}`, `ScaleAssets{asset_ids, sx, sy,
   sz}` (relative, selection-scoped leaves emitted per gumball tick via `Emit::amend`, `PL/🎥️shooting/.../🎮️commands/🧭️gumball`),
   but amended per tick they store N per-tick deltas, not one editable offset.
5. **A parametric-mutation mechanism already exists and is unused for gestures:** `CompositeMutationKind::plan`
   (`OS/📡️spr/🎮️command/🦀️.rs:789`). The composite op with its inputs is what `Edit.forwards` stores; diff/inverse fold
   from its plan. 5 composite leaves exist (flow `duplicate-widget`, sequence `delete-step`, cad
   `create-building-storey`, plus 2 law fixtures). This is the natural carrier for `MoveSelection{targets, offset}`.
6. **The history substrate is already event-sourced and non-tail-capable:** `HistoryTransition::{Revert, Reinstate,
   Commit, Branch, Checkout, Repin}` (`FM/📡️replication/🔗️causal/🔀️transition/🦀️.rs:54`); undo is `Revert{mutation_ids}`
   of any local edit (`S:undo_lane_position`). Nothing can change the *payload* of an existing edit today.
7. **Nothing on an `Edit` remembers tool, command, transaction or inputs.** `Edit` = `{id, actor, forwards, inverse,
   mutation_meta, description, coalesce_key, sequence_number, started_at, finished_at}`. `MutationMeta.semantic_kind`,
   `.label`, `.group_id`, `.origin` are always defaulted by `replay_mutations` (`S:18473-18510`); `group_id` is stamped
   afterwards only for ToolRun finalize and cross-artifact composite gestures. `CommandLogEntry` (the only place with
   the command/tool id) is runtime-only and rebuilt as `action_id:"apply"` on reload (`P:26683`).
8. **The gesture boundary is a coalesce key, not an identity.** `amend_command` (`S:18335`) merges into the last applied,
   not-checkpointed edit whose `coalesce_key` is equal; two back-to-back gestures with the same key merge into one
   undo step (no test covers it; tests cover only different key and committed edit). Every tick's ops are appended
   (`edit.forwards.extend`), so a 60-tick drag stores 60 batches.
9. **Cancel semantics differ per pattern.** Host-local drags cancel with zero trace (`BoardHost::pointer_cancel_screen`
   restores start positions and clears events; World3d relocate drops on Escape/`pointercancel`). The amend pattern has
   no abort (undo leaves a redo entry; each tick was already announced as a `MutationEnvelope`). ToolRun abort is zero
   trace by construction.
10. **Do not call it bare "transaction":** four meanings already exist (§1). Suggested name: `ToolTransaction`.
11. **Recommendation (detail in §9):** a schema-first `ToolMachine` (generalizing CAD's `spatial.interaction` and the
    `machine` crate) that yields `ToolYield::{Upsert(MutationInputs), Commit, Abort}` inside a `ToolTransaction`;
    commit publishes ONE edit carrying a persisted `TransactionRef{id, tool_id, machine fingerprint}`; mutations are
    parametric composites; a history edit is a new event-sourced `HistoryTransition::Amend`. Reuse `ToolRunLedger`
    (overlay, staleness, finalize), `CompositeMutationKind`, `machine`, `HistoryTransition`.

---

## 1. Vocabulary: what the words mean here (and where they collide)

| Word | Meaning today | Where |
|---|---|---|
| **Tool** (manifest) | `ToolDefinition{id,label,icon_id,keys,run}`: a mode-level *armed/not-armed* row; activation is `ViewModel.active_tool_id`, never a document field. No state, no lifecycle beyond arm/disarm (except `run`). | `FM/🛂️manifest/🦀️.rs:1940`, `active_tool_id` `:2855` |
| **Utility** | `UtilityDefinition{id,label,icon_id,group,keys,cursor,category,allows_actions_while_active,run}`: a per-window live pointer mode; exactly one active per window (`active_utility_id`). The `UtilityPreviewContract` doc (`P:13140`) names two preview patterns (amend per tick / scratch+commit). | `FM/🛂️manifest/🦀️.rs:1720`, `:4953`; `P:13140` |
| **Tool** (job bus) | Every UI-reachable command enters as a `ToolOperationSpec{controller_id, tool_id, payload, operation}`, resolved by a `ToolJobFactory` into a resumable `InteractiveJob`. Here `tool_id` == the command id (`translateSelection`, `engagementInput`, ...). Plugins declare `ArtifactToolPublicationContract{tool_id, lanes}` per command. | `FM/🎯️action-bus/🦀️.rs:20-332`; `P:15167` |
| **Tool run** | Long algorithmic run with lifecycle start/pause/step/abort/finalize/dismiss and a panel. Declared by `ToolDefinition.run` / `UtilityDefinition.run` = `ToolRunDefinition`. | `FM/⏯️tool-run/🦀️.rs`, `PT` |
| **Action** | `ActionDefinition{id,label,kind: ActionKind,args,keys,in_palette,semantics}`. `ActionKind` = Mutation, View, History, Clipboard, Shell, Interaction. `CapabilityAudience` = Agent / **Input** (raw pointer/keyboard events, hidden from agents) / Chrome. | `FM/🛂️manifest/🦀️.rs:86, 833, 1054` |
| **Command** | Two senses. (1) `CommandDefinition` = footer-catalogue verb (same `ActionKind`/`semantics`), `:1799`. (2) The typed `🎮️commands/<cmd>/🦀️.rs` payload record + `handle(payload, doc, cfg[, ctx]) -> Result<Emit,Fault>`; `app_commands!` (`P:12816`) generates the per-app `Command` enum, its `OpText`/`OpBinary` codecs and `dispatch`. `ArtifactApp::Command` is the wire-level dispatch surface. | `P:12816, 13487` |
| **Engagement** | The tool's live prompt/inputs bar (`WindowEngagement`) plus `engagementInput/Submit/Abort/ControlSelect/RepeatLast` verbs. | `FM/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs:1342`; puzzle 2d `engagement_*` |
| **Session / scratch** | Plugin-invented tool state: `DrawingSession`, `LowpolyScratch`, `CadEngagementScratch`, `Puzzle3dSessionRegistry`. Not one of the four state classes unless routed through a lane. | draw `🖱️canvas-pointer-down:993`; lowpoly `🖌️session`; cad `⚙️engine/🕹️interaction:65` |
| **Transaction** | (1) `FrameTransaction`, a per-frame UI render pipeline (`FM/🖱️ui/🧠️runtime/🔄️transaction`); (2) `PendingTransaction`/`transaction_prepare|commit|rollback`, synchronous cross-artifact 2PC with a freeze guard (`P:14122-14135, 23488`); (3) CAD `openTransaction/commitTransaction/rollbackTransaction` effect kinds: declared in the TS type only, no implementation found (`✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️.ts:565`); (4) ToolRun finalize stamping `group_id`. | see cells |
| **HostEffect** | No type of that name. The host-effect vocabulary is the kernel `Effect` enum (`LoadDocument`, `SetActiveUtility`, `SetActiveTool`, `DispatchAction`, `ReplayShellCommand`, `SpawnJob`, `CancelJob`, `SetTimer`, `Notify`... ≈45 variants). | `FM/🎠️kernel/🦀️.rs:359` |
| **InverseAction** | Memory-only replayable inverse of a `View`/`Shell` command row; the old ad hoc undo, kept only for `noteShellCommand`. | `P:11938` |

---

## 2. Framework modules (task 1)

| Module | Path / size | What it is | Relevance to the requirement |
|---|---|---|---|
| **tool-run** | `FM/⏯️tool-run/🦀️.rs` 2274 L, `🟦️.ts` 1275 L, `🧬️schema/🔣️.json` 2234 L, `🧫️fixtures/⚖️lifecycle-law.json` 2628 L | Pure, schema-first ToolRun contract. Identity `ToolRunId/Identity/Freshness` `:54-99`; lifecycle: `ToolRunState` 9 states `:105`, `ToolRunEvent` 17 events `:178`, reducer `ToolRunMachine::apply` `:471` (staleness guard: `toolRun.stale/busy/illegal`); `ToolRunLane` `:438` (all mutating runs share ONE lane per instance; read-only runs keyed by tool+window); progress, step ring, trace pages; `ToolRunTick` `:1066` (`append_ops`, `retract_to`, `append_entities`, `payload`); `ToolRunDefinition` `:1884`; 7 reserved actions + chords `:2017-2044`; `TOOL_RUN_GROUP_ID_PREFIX` `:48`. | Best existing state machine + transaction shape. Lifecycle-law fixture is the language-agnostic test; TS twin byte-parity. |
| **machine** | `FM/🔄️machine/🦀️.rs` 1783 L + `🟦️.ts` 1156 L; derive `✨️derive` | XState-parity statechart kernel: `Machine` trait `:1590`, `statechart!`, `Host` (effects/timers/invoke), `macrostep`, `persist/restore` by stable ids `:904`, inspectors. | Right base for a generic tool FSM. **Rust consumers: none outside its tests.** draw vendors a near-copy (`PL/🖍️draw/.../🖱️canvas-pointer-down/🔄️fsm`, crate `semio-s-plugin-draw-fsm`, 1554 L, ~297 differing lines). TS consumer: CAD `🎰️stately`. `ToolRunMachine` does not use it (hand-written reducer). |
| **interaction** | `FM/🕹️interaction/🦀️.rs` 119 L, `🧬️schema`, `👆️gesture/🦀️.rs` 273 L | Declarative hover/selection (`InteractionDefinition`) and pure pinch/camera gesture math. The pure selection/hover FSMs live in replication: `next_selection` `FM/📡️replication/📡️wire/🦀️.rs:2714`, `next_hover` `:2830`. | Selection is a framework-owned lane, not a tool. |
| **action-bus** | `FM/🎯️action-bus/🦀️.rs` 764 L (+ `🧹️wire-retirement`) | `ToolJobBus`: `ToolFactoryKey` `:20`, `ToolOperationSpec` `:46`, `ToolJobDispatch` `:63`, fixed 4096-byte `ToolWirePage` `:80`, `RetainedToolWireInput` `:108` (the "retained tool wire": the `OpBinary`-encoded command paged with exact declared extent), `ToolExecutionContract` `:216` (`resumable` / `bounded_first_step`, step < 8000 us), `ToolJobFactory` `:332`. Factories are accepted only for manifest `InteractiveJobClassification::Migrated`. | Admission/transport of every command. Not a lifecycle. |
| **job** | `FM/🧵️job/🦀️.rs` 3663 L | `InteractiveJob` `:1307`: synchronous `step(cx) -> StepOutcome{Yield, PreviewReady, CheckpointReady(Checkpoint), Complete(CommitCandidate), Cancelled, Fault}` `:1241`; `drive_step` `:1330` under an 8 ms watchdog. Module doc: "every interactive operation is a persistent state machine". | The step protocol a ToolMachine job would ride on. |
| **dispatch** | `FM/🔀️dispatch/🦀️.rs` 562 L | `#[dyn_enum]` / `dyn_enum_close!` macros (dyn-free trait dispatch). | Name clash only; unrelated to action dispatch. |
| **action-argument-resolution** | `FM/🧩️action-argument-resolution/🟦️.ts` 51 L | TS-only staged/default/choice resolution of `ActionArgDef`s for dialogs and forms (`effectiveActionArgs`, `missingRequiredArgs`). | Shell-side arg staging for one-shot actions; not gesture-related. |
| **editor** | `FM/✍️editor/🦀️.rs` 1665 L | Text editor engine on the infinite canvas (`EditorHost`). | Name clash with `ArtifactEditor`; unrelated. |
| **manifest** | `FM/🛂️manifest/🦀️.rs` 6162 L | `ActionKind :86`, `CapabilityAudience :833`, `GUMBALL_GESTURE_BRACKET_ACTION_IDS = [transformBegin, transformEnd] :873`, `ActionDefinition :1054`, `tool_run_action_definitions :1530`, `UtilityDefinition :1720`, `CommandDefinition :1799`, `ToolDefinition :1940`, `ViewModel.active_tool_id/active_utility_id`. | Declaration layer. |
| **replication/mutation** | `FM/📡️replication/🎮️mutation/🦀️.rs` | `Mutation<P>` trait `:174`, `MutationMeta :1362`, `Edit<Op> :1488`, `MutationOrigin :1584`. | The unit history stores. |
| **spr/command** | `OS/📡️spr/🎮️command/🦀️.rs` | `MutationKind<P,Op>` `:219` (`SEMANTICS`, `diff`, `inverse`, `label`, `target`), `SemanticMutation`, `Planner`, `PlanStep`, `CompositeMutationKind :789`, `fold_plan_diff/inverse`, `plan_foreign_steps`. | Parametric-mutation carrier. |

### Plugin runtime pieces asked about

- **`Emit`** (`P:12242`): `artifact_mutations, config_mutations, window_config_mutations, draft_mutations, description,
  coalesce_key, effects (kernel Effect), extension_invocations, events, ui_scope, child_emits, interaction_writes,
  tasks`. Constructors `Emit::mutations` / `amend(ops, key)` `:12642` (per-tick coalesced) / `commit(ops, description)`
  `:12649` (gesture-end) / `amend_config` / `commit_config`. **There is no tool id, transaction id or inputs field.**
- **`EphemeralEmit`** (`P:12446`): `presence` (ephemeral shared), `transient`, `window_transient` (ephemeral local).
  Documented as having no op log, no undo, no command-log row. Applied before `handle` (`A::ephemeral`, `P:13456`).
- **`ActionEmit`**: obsolete name (renamed `Emit`); only survives in doc comments (`P:12237, 13136`).
- **`InverseAction`** (`P:11938`), **`CommandLogEntry`** (`P:11964`), **`CommandView`** (`P:12006`), **`record_command`**
  (`P:26651`), **`backfill_command_log`** (`P:26683`), **`build_history_view`** (`P:26827`): see §6.
- **`dispatch_action`** (`P:29411`), **`dispatch_emit`** (`P:27216`), **`dispatch_emit_inner`** (`P:27299`),
  **`dispatch_emit_group`** (`P:27642`), **`dispatch_typed_command_inner`** (`P:31104`),
  **`start_typed_command_operation`** (`P:31218`), **`publish_mounted_typed_operation_unit`** (`P:30670`).
- **Transient/coalesced amends**: document (`ArtifactCommand::AmendLast`, `P:27430`, or batch `set_coalesce_key`
  `P:30893`), config (`format!("config:{key}")`, `P:27379`), per-window config (`window_config_store.dispatch(..,
  coalesce_key)`, `P:27395`; e.g. camera drags), window transient (`EphemeralEmit.window_transient`).
- **Retained tool wire**: command → `OpBinary` bytes → `ToolWirePage` pages (`FM/🎯️action-bus:80-108`), admitted by
  `admit_command_wire` against the factory's `ToolExecutionContract`.

---

## 3. Lifecycle: pointer input to history row (task 1)

```
DOM pointer/key ─▶ (A) host-local machine ─┐
                 ─▶ (B) per-event Input actions (canvasPointer*, worldPointer*, paintStroke*)
                 ─▶ (C) engagement REPL (engagementInput/Submit)
                                             │  dispatch(action, args)
                                             ▼
PluginApp::handle_action (impl P:33488; trait P:14046) ─▶ VcsArtifactApp::dispatch_action (P:29411)
   ├─ reserved: tool-run (PT:1167) / undo, redo, checkpoint, clipboard, revertToCommand, interaction*, setActive{Tool,Utility}
   └─ app action: A::command_from_action (P:13509) ─▶ typed A::Command ─▶ admit_command_wire (retained tool wire)
        ─▶ dispatch_typed_command_inner (P:31104): typed-operation slot, optional latest-wins queue (latest_wins_target P:15197)
        ─▶ start_typed_command_operation (P:31218): capture immutable roots, A::ephemeral (P:13456), spawn retained job
        ─▶ A::handle(command, doc, cfg, interaction, view_state, draft, engines) -> Emit   (P:13487, pure)
        ─▶ publish_mounted_typed_operation_unit (P:30670): lane contract check (ArtifactToolPublicationLane P:15167),
              freeze guards (tool_runs.freezes_local_emits, pending_transaction),
              artifact lane: store.begin_outbound_apply_batch (P:30878) + set_coalesce_key (P:30893)
        ─▶ ArtifactStore: apply_command / amend_command (S:18335) ─▶ Edit ─▶ announce_operations (S:17933) ─▶ backbone Mutations
        ─▶ record_typed_operation_lane (P:26065) ─▶ record_command (P:26651) ─▶ CommandLogEntry ─▶ CommandView (P:12006)
```

Notes on the path:

1. **`handle` is a pure reducer** from `(command, doc, cfg, interaction, view_state, draft, engines)` to `Emit`
   (`P:13487`). The command payload is therefore the natural "mutation input" (§7), but it is never persisted.
2. **The unmigrated route** (`dispatch_emit_inner`, `P:27299`) still exists for host-configuration verbs,
   `setActiveTool/Utility`, catalogue examples and the cross-artifact commit; it issues `Apply` or, when `coalesce_key`
   is `Some`, `AmendLast` (`P:27430`).
3. **Store level**: `apply_command` (`S:18286`) builds `Edit{coalesce_key: None, ...}`; `amend_command` (`S:18335`)
   appends `new_forwards/new_inverse/new_mutation_meta` into the target edit and re-announces only the new ops
   (`operation_envelopes_since`, `flush_apply_outbound S:19353`: a hub once refused a replay of already-accepted ops).
4. **Undo/redo** (`S:18238-18256`): `Undo` = `undo_with_policy(TransformAgainstConcurrent)` → `undo_lane_position` →
   `HistoryTransition::Revert{mutation_ids}`; `Redo` = `Reinstate{mutation_ids}` "landing back at its own HLC
   position". `HistoryLane::Interaction` (`S:2640`) edits (selection) are excluded from plain undo/redo.
5. **History row** = one `CommandLogEntry` per edit; a folded gesture finds `logged_seq` and adds no row
   (`record_typed_operation_lane`, `P:26065`). `CommandView` shows the last `HISTORY_ROW_OPERATION_PREVIEW = 8` op lines
   (`P:12040`). `revertToCommand` walks undo one unit per step (`FrameworkRevertToCommandJob`).

### State classes and where tool state may live

| StateClass (`FM/📡️replication/🧾️wire/🦀️.rs:294`) | Store | Tool-relevant content today |
|---|---|---|
| Artifact (persisted shared) | `ArtifactStore` (edits, transitions) | document mutations |
| Config (persisted local-only) | `ConfigStore`; `interaction_store` (selection, `HistoryLane::Interaction`, `P:23447`); per-window config (camera, tool options) | selection, camera, tool options; CAD `engagement_session_json` |
| Presence (ephemeral shared) | `PresenceStore` | cursors, `PresenceToolRun` summary |
| Transient (ephemeral local-only) | `TransientStore`, `WindowTransientStore` | in-flight gesture, hover, app scratch |

---

## 4. Taxonomy directories and census (task 2)

### 4.1 What the four directories are

| Directory | What a leaf contains | Behavior? |
|---|---|---|
| `🛠️tools/<slug>/🦀️.rs` | a mode-level `ToolDefinition` (`definition(label)`), usually with `run: Some(ToolRunDefinition)` and its `WindowMeasure` options (e.g. puzzle 2d `🪣️fill`, 99 L) | declaration only; the run job lives in `⏳️precompute/…` or `🧵️…-session/` |
| `🪛️utilities/<slug>/🦀️.rs` | a window-scoped `UtilityDefinition` + option measures (puzzle 2d `select` = 12 L: id/label/icon/category only; 5d `transform` = gumball flags `setTransformGumballFlag`) | declaration only |
| `🎬️actions/<slug>/🦀️.rs` | an 8-line `ActionRef` binding an app action name to a window kind (e.g. `"translateSelection".into()`) | none |
| `🎮️commands/<slug>/🦀️.rs` | a typed payload record (`dsl::DslRecord`, `#[dsl(keyword=..)]`) and `handle(payload, doc, cfg[, ctx]) -> Result<Emit,Fault>` | **the only place behavior lives** (with the app's `ArtifactApp::handle` dispatch enum) |

Declaration style: `ToolRunDefinition` is schema-first (`FM/⏯️tool-run/🧬️schema/🔣️.json`, 49 `$defs`, Rust/TS mirrors,
fixtures). `ToolDefinition`/`UtilityDefinition`/`ActionDefinition` are Rust structs with `ToValue/FromValue`. **Commands
are code-first: 964 of 968 `🎮️commands/*` dirs hold only a `🦀️.rs`** (2 hold a JSON), unlike mutation leaves which have a
`🔣️.json` descriptor (`semanticKind`, `binaryTag`, `invertibility`, `diffParticipation`, `composition`).

### 4.2 Census (walk of `PL/**`, 2026-09-30)

| Directory kind | Containers (dirs of that name) | Leaf dirs with ≥1 `.rs` | Empty scaffolding containers (`📌️.empty.md`) | Plugins with ≥1 leaf |
|---|---:|---:|---:|---|
| `🛠️tools` | 13 | 13 | 0 | 7: wfc 5, puzzle 3, reasoning 1, remodel 1, energy 1, trinity 1, dag 1 |
| `🪛️utilities` | 349 | 14 | ≈344 | 2: puzzle 12, layout 2 |
| `🎬️actions` | 358 | 7 | ≈356 | 1: puzzle (5d) 7 |
| `🎮️commands` | 640 | 964 | 577 | 33 of 34 (energy 0); puzzle 170, norm 120, procedural 79, space 72, fem 65, block 38, flow 37, remodel 35, note 33, draw 30 |

Declarations in code (not directory-bound): `UtilityDefinition::new` 44 sites in 14 plugins (puzzle 12, wfc 6, remodel 4,
process 4, fem 3, shooting 3, procedural 3, block 2, layout 2, note/raster/draw/cad/lowpoly 1 each);
`ToolDefinition::new` 15 sites in 8 plugins (wfc 5, puzzle 3, procedural 2, dag/trinity/energy/remodel/reasoning 1 each).

### 4.3 Are tools already state machines? Pattern census

| # | Pattern | Who | Count | State machine? | Preview during gesture | Yields |
|---|---|---|---|---|---|---|
| A | **Host-local engine / refs** flush one batch on release | puzzle 2d/trinity `♾️infinite/🎲️board` `InteractionMode` (`DragNode{node_id,offset}`, `DragNodes`, `DrawEdge`, `AreaSelect`, `Pan`, `Idle`; `♾️infinite/🎲️board/🦀️.rs:276`); World3d gumball/relocate refs (`World3dHost/🟦️.tsx:6017-6030, 7018-7070`) | 2 board engines (`InteractionMode`; directed-port `Interaction` in `♾️infinite/🎲️board/🔌️ports/➡️directed:615`), ≥2 React hosts (World3dHost, Board2dHost/Canvas2dHost gumball) | hand-rolled enums / refs | local only (engine moves nodes, buffers `NodeMoved`) | one `applyBoardEvents` / one absolute delta |
| B | **Per-tick amend** (`Emit::amend` / `amend_config` / `coalesce_key: Some`) | forms, fem, procedural, puzzle, remodel, flow, writer, note, draw, dag, energy(camera), shooting, vcs (typing), gis(camera), space, trinity (typing), playbook, cad (engagement session in config), layout, wfc (bitmap stroke on window config) | 20 plugins, ≈50 files (static grep; a few are camera/typing/config amends rather than pointer drags) | no (stateless reducer per tick) | ticks written into the live document | ops appended into one edit |
| C | **Scratch + commit** (`Emit::commit`) | draw 11 files, lowpoly 3, mathematical 2, norm 1; lowpoly `transformBegin/End`, `paintStrokeBegin/End` brackets | 4 plugins | scratch structs (`DrawingSession`, `LowpolyScratch`) | app scratch, overlay | one described edit at end |
| D | **Statechart** | draw `drawing_gesture` (`fsm::statechart!`, states idle/moving_layer/marqueeing/shape_dragging/drafting; events PointerDown/Move/Up, CommitDraft, Escape, UtilityChanged; effects `CommitMarquee/Shape/Draft/Trace`, `PickPoint`) | 1 plugin (private copy of `machine`) | yes | `DrawingGesturePreview` | `GestureEffect` then `Emit::commit` |
| E | **Declarative interaction spec** `spatial.interaction/v1` | CAD: 64 JSON assets (`📚️examples/🖼️assets/🏗️modelDefinitions/*/🕹️interactions/*.json`) + Rust interpreter (`✏️editor/⚙️engine/🕹️interaction/🦀️.rs`, 1050 L) + TS interpreter (`applyTransition`, `⚙️engine/🎬️actions/🟦️.ts:1190`) | 1 plugin | yes: `machine{initial,states[on[event→transitions{guard,effects,target}]]}`, `context`, `guards`, `display` per state, `commit{fromStates, operation{action, params←context}}` | display items + `requestPreview` | `CommitOutcome{Objects,Move,Copy,Rotate,Scale}` then per-object ops |
| F | **ToolRun** (algorithmic) | wfc fill 5, puzzle fill 3 (+ brush utilities 2, read-only), generation2d/3d `previewEval` 2 (read-only), reasoning/trinity/dag `reorganize` 3, remodel reconstruction, energy simulation | 17 tools in 8 plugins (policy table requires 28, `📜️script.ts:10377`) | yes (`ToolRunMachine`) | overlay of provisional ops + trace lane | ONE finalize edit, `group_id=toolRun:<n>` |
| G | **Composite mutation** | flow `duplicate-widget`, sequence `delete-step`, cad `create-building-storey` (+2 law fixtures) | 3 real leaves | n/a (mutation, not tool) | n/a | inputs stored, plan folds |
| H | **Host-only brackets, empty handlers** | puzzle 3d, fem 3d `transformBegin/End` (`HostOnly` lane, `Emit::default()`) | 2 | no | none | none |

Only patterns D, E, F are state machines in the requirement's sense; only F has a transaction; none stores inputs.

---

## 5. One gesture, one transaction? (task 3)

| Pattern / example | Preview while dragging | Coalescing | Commit on release | Cancel (Escape) | History rows |
|---|---|---|---|---|---|
| **A: puzzle 2d board drag** (`BoardHost`) | engine mutates `node.center`, buffers `NodeMoved` (`♾️infinite/🎲️board:1411-1436`); document untouched | none; `applyBoardEvents` carries `coalesce_key: None` (doc `puzzle2d_gesture_coalesce_key`, puzzle 2d `✏️editor/🦀️.rs:2858`) | one buffered `applyBoardEvents{eventsJson}`; `nodeDragEnd.moves=[{id,x,y}]` folded into a scratch fixture, diffed into N `MoveNode` (`apply_board_events`, `puzzle2d_dispatch_emit :2877-3012`); auto-connect rides the same edit | `pointer_cancel_screen` (`:1139`) restores `drag_start_positions`, clears events: zero trace | 1 edit, 1 row |
| **A: puzzle 3d gumball / relocate** | React refs + imperative preview (`gumballDragStartPoseRef`, live-dispatch option `gumballLiveDispatch`) | key `gumball-translate/rotate/scale` (`puzzle 3d ✏️editor/🦀️.rs:3576`) | ONE absolute start→end delta (`translateSelection{ids,dx,dy,dz}`); `transformBegin/End` are host-only empty handlers (`:3655`, lane `HostOnly :7214`) | relocate: Escape/`pointercancel` drop with no dispatch (`World3dHost:7133-7160`); gumball cancel path not located | 1 edit (2 gestures back to back with the same key would merge, see §8) |
| **B: per-tick amend** (fem gumball, generation slider) | ticks visible in the document | `AmendLast` on equal key | implicit: last tick is the commit; no end event | none; undo leaves a redo entry, ticks already announced | 1 row, edit grows |
| **C: lowpoly transform** | `LowpolyScratch.transform` doc, `preview_seq` | n/a | `transformEnd` → `end_transform_drag` → one `Objects(Patch)` (`🖌️session:383, 508`) | begin clears stale scratch; no explicit cancel verb found | 1 edit |
| **C/D: draw layer move** | `DrawingGesturePreview{transformation,node_translation}` | n/a | `finish_layer_move` → `Emit::commit(update_layer_transform×N, "Move selection")` (`🖱️canvas-pointer-down:1365`) | `Event::Escape`/`UtilityChanged` → idle, `layer_move` dropped | 1 edit |
| **E: CAD interaction** | display items per state, `requestPreview` | n/a | `runCommit` → action `transform.move{from,to,targets,moveMode}` → `CommitOutcome::Move` → `translate_objects_mutations` (`✏️editor/🦀️.rs:909`) | tool-local `snapUndoStack`/`rollback{state,context}`; abort clears session | 1 edit (TS engine also keeps its own `DocumentHistory` of `Modification{interactionId,label,archiveContext,backwardsDiff}`) |
| **F: ToolRun** | provisional ops folded into an overlay, never the store (`ToolRunEntry.provisional/overlay`, `PT:399, 564`) | n/a | `toolRunFinalize` → `publish_tool_run` (`PT:1836`) → ONE edit, `stamp_tail_group_id("toolRun:<n>")`, command row `action_id = tool_id` | `toolRunAbort`: store untouched, no edit/redo/command row/backbone frame | 1 edit, 1 row |

---

## 6. What a mutation / history row remembers (task 4)

| Datum | On `Edit` / `MutationMeta` (persisted in `.spr`) | On `CommandLogEntry` (runtime only) |
|---|---|---|
| tool id | no (ToolRun only: as `action_id`, runtime) | `action_id` = command/tool id |
| command id / args | no; args never stored | `action_id`; `label`; no args |
| transaction / gesture id | only `MutationMeta.group_id` for ToolRun (`toolRun:<n>`) and cross-artifact `dispatch_group` (`invocation_id`); `coalesce_key` on `Edit` is a *kind*, not an instance | none |
| provenance | `MutationMeta.origin` = Owner / Contributed / Transaction(initiator) (`mutation:1584`) | none |
| semantic kind / label | fields exist (`semantic_kind`, `label`) but `replay_mutations` writes `None` (`S:18473-18510`); the label is recomputed from the op via `MutationKind::label()` | `label` (from `Emit.description` or registry label) |
| per-op identity | `mutation_id` (content-hash minted), `dependencies`, `payload_hash`, `undo_policy`, HLC `timestamp`, `author_id` | none |
| edit-level | `id, actor, forwards, inverse, mutation_meta, description, coalesce_key, sequence_number, started_at, finished_at` (`mutation:1488`) | `edit_id`, `config_edit_ids`, `child_edit_ids`, `count`, `inverse: InverseAction` |
| lane | `envelope.lanes[edit_id]` = Document / Interaction | `kind: ActionKind` |

- **Grouping**: a history row is one `CommandLogEntry` per edit. Rows never group several edits by transaction. Only
  `TransactionUndo/Redo{group_id}` (`P:27551`, `transaction_group_history`) and `CompositionCoordinator::undo_group`
  move several members by `group_id`.
- **Persistence gap**: after reload `backfill_command_log` (`P:26683`) recreates one row per edit with
  `action_id:"apply"` and label = `edit.description` or the first op's text. The tool/command identity is lost.
- `ToolRun` description is persisted as the tool label resolved to English only (`PT:1844`).

---

## 7. Existing "mutation inputs vs tool state" separations (task 5)

| Concept | Where | Separation it gives |
|---|---|---|
| Pure reducer `handle(command, doc, cfg, interaction, view_state, draft, engines) -> Emit` | `P:13487` | command payload = input; doc/cfg/selection/transient = state; result = ops. Inputs not persisted. |
| `Emit` (durable lanes) vs `EphemeralEmit` (presence/transient/window_transient) | `P:12242, 12446` | durable mutation vs ephemeral tool state |
| `StateClass` (4 classes) + `HistoryLane::{Document, Interaction}` | `FM/📡️replication/🧾️wire:294`, `S:2640` | where each kind of state may live; selection excluded from undo |
| `ToolRunTick`: `append_ops`/`retract_to` vs `progress/steps/trace/payload` | `FM/⏯️tool-run:1066` | yielded mutations vs process display; run identity `(run, generation, baseRevision)` |
| CAD `InteractionSpec`: `context` + `machine` (tool state) vs `commit.operation{action, params←context}` (mutation inputs) | `spatial-kernel/⚙️engine/📐️geometry/🟦️.ts:638` | the cleanest existing split; but the commit collapses to absolute per-object ops and the TS `Modification.archiveContext` is memory-only |
| `CompositeMutationKind::plan` | `OS/📡️spr/🎮️command:789` | stored inputs vs planned primitive steps (`fold_plan_diff`, `fold_plan_inverse`); replayed against the current base |
| `MutationLeafDescriptor{diffParticipation: detect|apply-only|plan|none, composition: atomic|composite}` | mutation leaf `🔣️.json` (e.g. puzzle 2d `📍move-node`) | vocabulary already distinguishes diff-detected leaves from planned composites |
| Relative, selection-scoped drag leaves | 🎥️shooting `DragAssets{asset_ids,dx,dy,dz}` / `RotateAssets` / `ScaleAssets` (atomic leaves whose `target()` = the ids) | the only production leaf that stores selection + offset as inputs; per-tick amend still stores N deltas |
| gesture brackets `transformBegin/transformEnd` (`Input` audience) | `FM/🛂️manifest:873` | host-declared begin/end of a gesture; only lowpoly gives them meaning |

---

## 8. Defects and gaps observed

1. **Absolute ops erase intent.** Puzzle 2d/3d/5d, CAD and draw all store absolute final-state ops per entity; selection
   and offset cannot be recovered or edited (`move-node` doc: "absolute FINAL-state position").
2. **Coalescing has no gesture identity.** `amend_command`/`batch_amend_target` (`S:17484`) key on the last edit + equal
   key + not checkpointed. Two consecutive drags (same key, nothing in between) merge into one edit. Tests:
   `amend_last_absorbs_into_matching_coalesce_key`, `..._starts_new_edit_when_coalesce_key_differs`,
   `..._does_not_absorb_into_committed_edit` (`S/🧪️tests/🔬️unit/🦀️.rs:6270-6347`); none for back-to-back same-key
   gestures. By code reading only, not run.
3. **Amend ticks are durable and shared immediately** (`announce_operations` per tick); nothing marks them provisional;
   abort of an amended edit is impossible without a redo trace (also concluded in
   `☀️13/INTERACTIVE-TOOLS-VISIBLE-PROCESS/📋️tool-run-contract.md` §0.1, which additionally claims a full-snapshot
   broadcast per tick: not re-verified, my read of `amend_command` shows per-op envelopes).
4. **Edit growth.** Each tick appends its ops (`edit.forwards.extend`), unbounded per gesture.
5. **Three transaction-like mechanisms, none general:** ToolRun ledger (algorithmic only), `PendingTransaction`
   (synchronous 2PC, freezes other emits), amend (no abort).
6. **`machine` crate unused; draw vendors a diverged copy** (1554 L, ~297 differing lines). CAD's TS twin is the only
   consumer of the framework statechart. `ToolRunMachine` and `BoardHost::InteractionMode` are hand-rolled.
7. **Commands are code-first** (964/968), against the schema-first rule; `Emit` has no place for tool/transaction metadata.
8. **History identity lost on reload** (§6).
9. **CAD `openTransaction/commitTransaction/rollbackTransaction`** effects are declared in the spec type and not
   interpreted (`applyEffectAsync` handles assign/clear/append/kernel.query/action only).
10. **Bracket verbs inconsistent:** `transformBegin/End` are real in lowpoly, empty `HostOnly` in puzzle 3d/fem 3d.
11. **Name collisions** (tool ×3, transaction ×4, `editor`, `dispatch`) will confuse the new contract unless named apart.

---

## 9. Recommendations: a generic `ToolMachine` yielding mutations in a `ToolTransaction`

### 9.1 Contract (artifact-agnostic, schema-first)

Source of record: one JSON Schema (`🧬️schema/🔣️.json`) with Rust + TS mirrors and shared fixtures, like `⏯️tool-run`.
Placement: a new sibling module next to `⏯️tool-run` and `🔄️machine` (name/emoji to be chosen by the coordinator).

```text
ToolMachineDefinition                       (static, manifest; localized EN then DE, no default)
  id, label, icon, keys, scope: mode|window-utility
  machine: statechart { initial, states[ on[event -> transitions{guard, effects, target}] ] }   // = CAD spatial.interaction/v1 generalized
  context: typed record                      // tool state: StateClass::Transient (window-scoped), never in history
  events: ToolEvent union                    // pointerDown/Move/Up/Cancel, key, value(name,number), confirm, abort, utilityChanged
  yields: [ MutationKindRef{ kind, inputSchema } ]   // composite mutation kinds this tool may yield
  profile: gesture | run                     // gesture: begin=first Upsert/pointerDown, commit=pointerUp/confirm, abort=Escape/pointercancel
                                             // run: = ToolRunDefinition (pause/step/panel/trace)

ToolYield  (what a tick of the machine returns; pure)
  Upsert { key, kind, inputs }               // keyed by (kind,target); last write wins WITHIN the transaction
  Retract { key }
  Preview { display }                        // ephemeral only
  Commit | Abort

ToolTransaction
  id: { instance, seq, hlc }                 // globally unique, minted at begin (NOT a coalesce key)
  tool_id, machine_fingerprint, actor, base_revision, generation, state: open|committed|aborted|faulted
  provisional: overlay fold of Upsert inputs (ToolRunLedger overlay reused)   // visible locally, zero-trace on abort
  commit: publishes ONE Edit; every MutationMeta.group_id = transaction id; Edit.transaction = TransactionRef{id, tool_id, machine_fingerprint}
```

Semantics to specify and test:

- Tool state (context, machine configuration) is ephemeral local; it is never editable through history. Only the
  yielded, parametric mutations and the `TransactionRef` are durable.
- One logical operation = one mutation with inputs, not N tick batches: the drag is
  `MoveSelection{targets:[ids], offset:[dx,dy], snap}` (composite, `plan` expands to per-node `MoveNode`), the rotate is
  `RotateSelection{targets, pivot, radians}`, and so on.
- Provisional visibility through the ToolRun overlay (not `AmendLast`), so peers see nothing until commit
  (ephemeral-shared preview may ride presence); abort leaves the store byte-identical (already a ToolRun law).
- Commit is exactly one edit, one undo step, one `Mutations` batch; staleness via `(transaction, generation)` like
  `ToolRunIdentity`; rebase policy `revalidate|restart|freeze` reused.
- Editing history: a new event-sourced transition, e.g. `HistoryTransition::Amend{transaction_id, replaces:[mutation_ids],
  with:[MutationInputs]}`. Original mutation ids stay in the log (superseded, not deleted); the fold replans the composite
  against the current base; undo of the amend reinstates the original. Conflicts use the existing
  `MutationMessage`/`MergePolicy` severity path and `Degraded`/`Quarantined` conflicts.

### 9.2 What must change (ordered)

1. **Mutation vocabulary (per artifact).** Replace snapshot-diff-derived absolute ops for gestures with parametric
   `CompositeMutationKind`s; start with puzzle 2d translate/rotate/scale/relocate-region and the puzzle 3d gumball
   verbs (ids and offsets are already in `nodeDragEnd.moves`, `nodeRotate{ids,radians}`, `translateSelection{ids,dx,dy,dz}`).
   Model them on 🎥️shooting `DragAssets/RotateAssets/ScaleAssets`, but keep the *net* offset of a transaction as one
   upserted input (replace-by-key), not one leaf per tick. Require `diffParticipation: "plan"` (or an authored diff) for
   tool-yielded kinds; forbid snapshot-diff derivation for them.
2. **Store.** Add `Edit.transaction: Option<TransactionRef>` (persisted in `.spr` `HistoryEdit`/`HistoryOpMeta`, wire, GraphQL/proto
   schemas); make `amend_command` target by transaction id and bound the edit (replace-by-key inside a transaction instead of
   `forwards.extend`); retire `coalesce_key` as gesture boundary; populate `MutationMeta.semantic_kind/label`;
   add `HistoryTransition::Amend` and fold/undo semantics.
3. **Plugin runtime.** `Emit` gains `transaction: Option<ToolStep>` (or the machine host yields into the ledger directly);
   `ToolRunLedger` becomes the transaction ledger with `profile: gesture` (no panel/pause/step); `CommandLogEntry` derives
   from `Edit.transaction` (persisted identity) instead of runtime-only; `record_command` carries tool id.
4. **Machine hosting.** A generic `ToolMachineHost` in the runtime keeps `context` in `WindowTransient` and persists via
   `machine::persist`. Migrate onto it: draw `drawing_gesture` (delete the vendored `fsm` copy), lowpoly
   `LowpolyScratch`, CAD `CadEngagementScratch`/`spatial.interaction` (make the JSON spec the schema of record for all
   tools, generating Rust tables and the TS twin), puzzle `InteractionMode` and World3d gumball refs (host-local machines
   emitting `ToolEvent`s or yielding directly).
5. **Consolidate the framework state-machine crates:** `🔄️machine` becomes the single kernel; `ToolRunMachine` either
   expressed as a `machine` chart or kept as a documented reducer with a shared fixture.
6. **Actions/commands.** Keep pointer verbs as `CapabilityAudience::Input`; give brackets one meaning (begin/commit/abort of
   a `ToolTransaction`) and delete the empty `HostOnly` handlers. Make command payloads schema-first (JSON schema of
   record) so history can render input editors from `MutationKind` input schemas.
7. **History UI.** Rows per transaction with expandable member mutations and a schema-generated inputs editor; a
   superseded marker; EN/DE labels from `MutationKind::label`.
8. **Naming.** `ToolTransaction` / `ToolTransactionId`, not "transaction"; keep `PendingTransaction` for cross-artifact 2PC.

### 9.3 Tests (language-agnostic first, third-party oracle)

- Fixture `tool-machine-trace.json`: event sequence → yielded inputs → final snapshot; Rust and TS must agree byte-for-byte
  (pattern of `⚖️lifecycle-law.json` and `🎞️ticks.json`); XState as the machine oracle, existing geometry oracles
  (`geo`, `parry3d`, `fdg-sim`) for the plan expansion.
- Laws: abort leaves generation/edits/redo/command log/backbone outbox unchanged; commit = one edit + one undo;
  two back-to-back gestures = two transactions; amend then undo restores original; replan determinism; concurrent remote
  edit during an open transaction triggers `revalidate`.
- Gate: extend the `⏯️ToolRunPolicy` predicates in `📜️script.ts:10335` to forbid `Emit::amend` gesture streaming and
  absolute-diff gesture ops once a tool is converted.

---

## 10. Verification status and unverified points

- Nothing here was run (read-only). Counts are static greps; line numbers are as read on 2026-09-30.
- Verified by reading: `Emit` shape; `dispatch_emit_inner`, typed publication, `record_command`, `amend_command`,
  `replay_mutations`, `undo_lane_position`, `HistoryTransition`, `CompositeMutationKind`, `ToolRunLedger::publish_tool_run`,
  `puzzle2d_dispatch_emit`, `apply_board_events`, `BoardHost::pointer_move_screen/pointer_cancel_screen`, lowpoly
  scratch, draw statechart, CAD runtime and commit mapping.
- **Not verified:** (a) back-to-back same-key gestures merging (code reading only); (b) that the React gumball has no cancel
  path (not located, not proven absent); (c) whether the hub/backbone treats amend ticks as provisional (the older contract's
  full-snapshot claim was not re-checked); (d) BoardHost flush cadence relies on the `puzzle2d_gesture_coalesce_key` doc
  comment ("a board drag arrives as one buffered `applyBoardEvents`"); (e) CAD transaction effects unimplemented (grep of
  `openTransaction|commitTransaction` in cad and spatial-kernel found only the type declaration); (f) the 5 composite leaves
  include 2 law fixtures under `OS/📡️spr/🎮️command/🧫️fixtures`, not production kinds.
- Prior art worth reading before designing: `☀️13/INTERACTIVE-TOOLS-VISIBLE-PROCESS/📋️tool-run-contract.md` (binding ToolRun
  contract, rejects `Emit::amend`/DraftStore), `📓️audit-p4-transaction-primitives.md`, `📓️wave-W0-D.md` (ledger + driver),
  `📓️wave-W1-D.md` (policy predicates).
