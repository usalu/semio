# Audit P4 — Tool/Job Lifecycle Primitives, Wave F Landed State, Gap To Start/Abort/Finalize

Read-only audit for phase 4 of ticket `26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS`. No source touched, no git
command run. Repo MCP unavailable this session (`invalid initialize params`/`CONNECTION_CLOSED`) — findings below
come from direct `grep -rn`/`rg`/`Read`, all with `cd /Users/ueli/Documents/semio` per call, since the Grep tool is
broken. Every claim below cites `file:line` from a fresh read in this session; anything not independently
re-verified this pass is marked **[from audit-progress-primitives.md / audit-tool-inventory.md, not re-checked]**.

Dev's phase-4 requirement under test: *"All tools must be interactive and the user must see how the algorithms
think by seeing the process. Every tool can show progress (different steps, percentage, status, etc). A tool that
is mutating the artifact is wrapped inside a transaction. A tool can be started, aborted and finalized (only when
it is complete)."*

---

## 1. What a "tool" is today — no lifecycle state exists on the declaration

### 1.1 `ToolDefinition` — purely static, no run state

`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:1488-1513`:

```rust
pub struct ToolDefinition {
    pub id: String,
    pub label: LocalizedLabel,
    pub icon_id: IconName,
    pub keys: Option<String>,
}
```

Four fields, all static declaration — **no state field whatsoever** (no `Idle/Running/Complete`, no `mutating:
bool`, no `transactional: bool`, no progress handle). The docstring at `🦀️.rs:1488-1493` states the actual model:
*"exactly one tool is active per app at a time, and activation is host-owned session view state
(`ViewModel.active_tool_id`), never a document field or VCS operation. A tool's live options are supplied
dynamically via `ArtifactApp::tool_measures`, keyed by tool id — not part of this static declaration."*

- Activation state: `ViewModel.active_tool_id: Option<String>` (`🦀️.rs:2408` and the windowed-view-context copy at
  `🦀️.rs:4376`) — a single global "which tool id is armed", binary (armed/not), **not** a per-run lifecycle. There
  is exactly one bit of state per app: which tool id (if any) is the active one. Nothing tracks whether that tool
  is mid-operation, has a pending mutation, or can be finalized.
- Arming action: `SET_ACTIVE_TOOL_ACTION_ID = "setActiveTool"` (`🦀️.rs:1205`), a framework-injected, palette-hidden
  `ActionDefinition` (`🦀️.rs:1207-1216`) that takes one required `toolId` arg. Its effect-layer twin is
  `Effect::SetActiveTool { tool_id: String }` (`🧰️framework/🔨️modules/🎠️kernel/🦀️.rs:488-493`, doc: *"empty
  `tool_id` deactivates the current tool"*). Arming/disarming is the entire lifecycle surface: **on** or **off**,
  nothing in between is modeled.
- **Live options while armed**: `ArtifactApp::tool_measures` / `PluginApp::tool_measures` — e.g. object-safe
  signature at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:12058-12059` (`async fn tool_measures(&mut
  self, _view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>>`) — this is where a tool COULD surface a
  `WindowMeasure::Progress`, but the framework itself imposes no requirement that it does, and nothing ties a
  `tool_measures` entry back to the tool's own start/abort/finalize.

### 1.2 `UtilityDefinition` — the sibling concept, same shape, same gap

`🦀️.rs:1288-1327` (`UtilityDefinition { id, label, icon_id, group, keys, cursor, category,
allows_actions_while_active }`) — "exactly one utility is active per window kind at a time … host-owned session
view state (`ViewModel.active_utility_id`)" (`🦀️.rs:1289-1291`). Same binary armed/unarmed model, same absence of
any run-state field. `SET_ACTIVE_UTILITY_ACTION_ID`/`set_active_utility_action_definition` at `🦀️.rs:1195-1201`
mirror the tool action exactly.

### 1.3 `🛠️tools/` as a literal directory barely exists

**[from audit-tool-inventory.md §0, re-confirmed]**: `grep -rl "ToolDefinition::new(" ✏️s/🔌️plugins` returns
exactly two files, both puzzle fill tools (`◻️2d` and `🧊️3d` `…/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs`). Every other
plugin's algorithmic surface is a `🪛️utilities/*` (window-scoped `UtilityDefinition`) or a bare `🎮️commands/*`
(`ActionDefinition`, fire-once). Neither `ToolDefinition` nor `UtilityDefinition` nor `ActionDefinition` carries a
lifecycle field — **the closest thing to a per-tool lifecycle anywhere in the framework layer is the single
`active_tool_id` / `active_utility_id` arm/disarm bit.** Everything past "armed" (running, checkpointed, done,
abortable, finalizable) is plugin-invented, ad hoc, and inconsistent (see §4).

### 1.4 Conclusion for Q1

**The framework has no `ToolRun`/lifecycle concept today.** A tool is a static id+label+icon+keybinding row plus
one global "is this the active tool" bit. Whether a tool mutates the document, runs a background job, needs a
transaction, or can be cancelled is entirely undeclared at the `ToolDefinition`/`UtilityDefinition` level — it is
implicit in what the plugin's `handle_action`/`tool_measures` happen to do. This is the structural gap the phase-4
requirement (start/abort/finalize, transaction-wrapped mutation) must close: today there is no schema type to hang
those states on.

---

## 2. Wave F landed code — exact inventory (no report exists; this is a fresh read)

Per `📓️status.md:59` ("wave F code landed … without a report"), the master plan §2.4 contract
(`📋️master-plan.md`) specified `WindowMeasure::Number`, `WindowMeasure::Progress`, `MeasureProgressStep`. What
actually landed, verified this session:

### 2.1 Rust schema definition — landed exactly as speced

`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs:1040-1158`:

- `MeasureProgressStepKind` (`:1042-1050`): `Info | Success | Warning | Danger`, wire tags via `as_str()`
  (`:1215-1224`) — `"info"|"success"|"warning"|"danger"`.
- `MeasureProgressStep` (`:1052-1059`): `{ kind: MeasureProgressStepKind, text: String }`.
- `WindowMeasure::Number` (`:1098-1133`): `{ id, label, value, min: Option<f64>, max: Option<f64>, step, ready,
  loading, waiting, disabled, on_change }` — `max: None` is genuinely unbounded per the doc comment (`:1109`,
  "renderers must not clamp").
- `WindowMeasure::Progress` (`:1134-1158`): `{ id, label, stage: Option<String>, completed: f64, total:
  Option<f64>, steps: Vec<MeasureProgressStep>, cancel: Option<ActionDescriptor>, loading: Option<bool> }`. **No
  `start`/`abort`/`finalize` field — `cancel` is the only lifecycle-adjacent field, and it is a plain cancel, not a
  distinguished abort-vs-finalize pair.**
- `measure_progress_cancel_label()` (`:1211-1213`): `LocalizedLabel::native("Cancel", "Abbrechen")` — the one
  framework-owned EN/DE string pair for this whole feature.

Verdict: **matches the master-plan §2.4 contract byte-for-byte** for the two variants; ships no lifecycle beyond a
single optional cancel action.

### 2.2 wgpu widget rendering — defined but never called (dead code, applies to ALL measure kinds, not just the new ones)

`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🪀️widgets/🦀️.rs`:

- `render_window_measure_number` (`:461-488`) — real paint code: `render_number_stepper` for the entry
  (`:479`, reusing the existing `NumberStepper` widget), plus a soft "ready extent" bar under it when `max`/`ready`
  are both `Some` (`:481-487`).
- `render_window_measure_progress` (`:490-551`) — real paint code: label, stage caption, determinate bar
  (`completed/total`, `:524-532`) or indeterminate sweep (`:533-537`, `MEASURE_PROGRESS_INDETERMINATE_SHARE`),
  the last `MEASURE_PROGRESS_STEPS_SHOWN` (`:203`, `= 4`) step lines tinted by `measure_progress_step_color`
  (`:554-562`, mapping `Info/Success/Warning/Danger` → `theme.progress/success/warning/error` — **never a literal
  color**), and a cancel `render_button` (`:546-550`) when `cancel` is `Some`.

**Verified dead code**: `grep -rn "render_window_measure_number\|render_window_measure_progress" --include="*.rs"
.` from repo root returns **only the two definition sites** (`🪀️widgets/🦀️.rs:475,496`) — zero callers anywhere in
the monorepo. The same is true for the pre-existing `render_window_measure_select`/`_slider`/`_toggle`
(`:457-468`) — **this whole `//#region 🔖️WindowMeasureBorrowedControls` block (`:456-551`) has no caller in the
repo**, for any `WindowMeasure` variant, old or new. I could not locate any function anywhere under
`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu` or `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer` that converts a
`WindowMeasure` tree into the native `WidgetNode`/`ControlNode` retained-widget tree those render functions would
need to be reached from (`WidgetNode`/`ControlNode` enums at `🪀️widgets/🦀️.rs:170-186`; neither carries a
`Progress` variant at all — no place to put a determinate/indeterminate bar or a step log in the retained tree
even if a converter existed). **Unverified**: whether a native (non-React) desktop wgpu consumer of the Measures
overlay exists anywhere outside this repo scan or is simply not built yet — flagged, not proven absent beyond this
grep's reach; but for `Number`/`Progress` specifically, wave F added an immediate-draw paint routine with no
integration point, so it cannot currently be exercised by any live app regardless.

Two hits for `WindowMeasure::Slider`/`::Progress` inside
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (e.g. `:36505-36755`) are test/example `ArtifactApp`
fixtures constructing measures, not a render dispatcher.

### 2.3 Projection TS mirror — landed, matches the Rust shape

`🧰️framework/🔨️modules/🧬️schema/📽️projection/🦀️.rs:1817-1885` (`WindowMeasure` entry in `SchemaMetadata::TYPES`):
adds `{ "kind": "number", … min?, max?, … }` and `{ "kind": "progress", … stage?, completed, total?,
steps: Array<MeasureProgressStep>, cancel?: ActionDescriptor, loading? }` arms to the discriminated union, with the
same doc comments carried through as TSDoc. `MeasureProgressStep`/`MeasureProgressStepKind` TS types at
`:870-884`. This is the hand-mirrored "JSON Schema of record" per the existing convention (no real
`semio_framework_schema_registry` entry — **[from audit-progress-primitives.md §2.1(3), not re-checked this
pass]**).

### 2.4 Generated manifest — landed

`🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts:1451,1475` carry the same `"kind": "number"` /
`"kind": "progress"` arms verbatim — confirms `bun nx run @semio-tech/framework-rs:generate` was actually run
(this file is machine-written, not hand-edited) and the manifest mirror is currently in sync with the Rust source.

### 2.5 React renderer — fully wired, the most complete part of wave F

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🎚️measure-controls/🟦️.tsx`
(209 lines):

- `WindowMeasureNumber` (`:114-142`): a `Stepper` bound to a draft-until-published value
  (`useWindowMeasureDraft`, `:33-38`, dedupes the "control reads a value the program already left behind"
  problem measured at 0.7s idle / several seconds busy round trip, per its own doc comment `:9-27`), dispatches
  `onAction` once on pointer-up/blur (not per keystroke — avoids a "1/2/25/250" dispatch storm, `:118-121` doc),
  plus a `ready` extent bar (`:135-140`).
- `WindowMeasureProgress` (`:161-209`): stage caption (`:184-188`), a bar with real accessibility markup —
  `role={determinate ? "progressbar" : undefined}` (`:192`), `aria-label` from `uiDataLabel(measure.label ??
  measure.stage ?? measure.id)` (`:193`, `:181`), `aria-valuenow`/`aria-valuemin`/`aria-valuemax` when determinate
  (`:194-196`), `aria-busy` when indeterminate (`:197`) — plus `data-completed`/`data-total`/`data-measure-id`
  attributes for probes. Step log rendered as an unordered list (`:200-207`) tinted via
  `MEASURE_PROGRESS_STEP_CLASS` (`:96-101`, same four semantic tokens as wgpu). Cancel is a real `<button
  type="button" id="{measure.id}.cancel">` (`:202-208`, cancel-labeled from the framework's own `ui.common.cancel`
  bundle via `useLabel`, `:163,166`) — **a real focusable, tab-reachable, Enter/Space-activatable DOM button**,
  not a synthetic click target.
- **Wired into the actual render path**: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx:211`
  imports all four control components; `windowMeasuresToTreeItems` (`:3309-3362`) handles `measure.kind ===
  "number"` (`:3337-3345`) and `"progress"` (`:3346-3353`); `renderWindowMeasure` (`:3364-3409`) handles the same
  two kinds again (`:3387-3400`) for the non-tree render path. **Confirmed live, not dead code** — this is a
  genuine dispatch switch with two call sites per kind.

### 2.6 Tests — wire-format/round-trip only, no rendering/interaction/keyboard test

- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-component-layout-layout-wire-format/🦀️.rs:103-173` — JSON
  wire-format assertions for `Number` (bounded/unbounded, `:103-135`) and `Progress`
  (determinate/indeterminate, step kinds, `:140-173`).
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-component-layout-value-round-trip/🦀️.rs:58-92` — `ToValue`/
  `FromValue` round trips for the same two variants plus `MeasureProgressStep`/`Kind` (`:89-92`).
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-accessibility-projection/🦀️.rs` — grepped for
  `Number|Progress|role|aria`: **zero hits beyond the generic `node.role` assertion at `:74`** — the wgpu
  accessibility-projection suite does not exercise either new variant at all.
- No test anywhere asserts keyboard reachability of the React cancel button, the wgpu paint routine's actual pixel
  output, or that `render_window_measure_progress`/`_number` are reachable from any real render path (they are
  not — §2.2).

### 2.7 Wave F landed-state table

| Layer | State | Evidence |
|---|---|---|
| Rust enum (`WindowMeasure::Number`/`::Progress`, `MeasureProgressStep(Kind)`) | **Landed, matches spec** | `🧩️component/🦀️.rs:1098-1158` |
| wgpu widget paint functions | **Landed but unreachable — zero callers repo-wide** | `🪀️widgets/🦀️.rs:475,496`; grep confirms no caller |
| wgpu retained tree (`WidgetNode`/`ControlNode`) | **Not extended** — no `Progress` variant exists in the retained tree at all | `🪀️widgets/🦀️.rs:170-186` |
| TS projection mirror | **Landed, matches Rust shape** | `📽️projection/🦀️.rs:1817-1885` |
| Generated manifest `🟦️.ts` | **Landed, in sync** | `🤖️generated/🪪️manifest/🟦️.ts:1451,1475` |
| React components (`WindowMeasureNumber`/`Progress`) | **Landed, fully wired, with real a11y** | `🎚️measure-controls/🟦️.tsx:114-209`; wired at `🛠️ShellHelpers/🟦️.tsx:211,3337-3400` |
| wire/round-trip tests | **Landed** | layout-wire-format & value-round-trip test dirs |
| a11y / interaction / keyboard tests | **Not landed** | `targets-wgpu-accessibility-projection` has zero Number/Progress coverage; no React keyboard test found |
| puzzle3d fill actually using `Number`/`Progress` | **Not landed** — `count_measure()`/`cancel_measure()` in `🪣️fill/🦀️.rs` still build `Slider`/`Toggle` **[per audit-progress-primitives.md §1.3, matches this session's independent reading of the wave-F diff scope — fill tool file was not touched by wave F]** | see §3 below |
| start/abort/finalize actions on `Progress` | **Not landed** — only `cancel: Option<ActionDescriptor>` exists; no distinguished abort vs finalize, no "start" affordance on the measure itself (start is presumed to be a separate `ActionDefinition`, per-tool, outside this schema) | `🧩️component/🦀️.rs:1150-1153` |

### 2.8 Answering the audit brief's specific questions on wave F

- **Renders on both React and wgpu targets?** React: yes, fully. wgpu (native): no — code exists but is not
  reachable from any render path (§2.2). So today it is **React-only in practice**, despite the wgpu-named schema
  crate being the "canonical" definition site.
- **Accessibility**: React side has real `role="progressbar"`, `aria-valuenow/min/max`, `aria-busy`, `aria-label`,
  a genuine `<button>` for cancel (tab/Enter/Space reachable via native DOM semantics — no custom keyboard handler
  needed, browser default suffices). wgpu side: the paint code has no keyboard/focus/AT concept visible in the
  function itself (it is immediate-mode `ctx.draw`/`draw_text` calls, no `interaction_maps`/hit-target
  registration comparable to `register_toggle_meta` at `🪀️widgets/🦀️.rs:441-445`), and since it's never called,
  the question is moot for now.
- **What's missing for start/abort/finalize buttons**: nothing in `WindowMeasure::Progress` models three distinct
  actions — only `cancel`. There is no schema field for "finalize" (commit a completed run) or a way to
  distinguish "abort mid-run" from "the run already finished, dismiss/finalize it." A generic `ToolRun` contract
  needs at minimum a `state` field the renderer can branch on (to show Start when Idle, Cancel when Running,
  Finalize when Complete) plus up to three distinct `ActionDescriptor`s — today only one optional action exists.

---

## 3. Job primitives

### 3.1 `InteractiveJob` step protocol — `🧰️framework/🔨️modules/🧵️job/🦀️.rs`

- `Checkpoint { state: RetainedJobPayload, applied_progress: u64 }` (`:1072-1075`) — doc: *"`applied_progress` is
  the Puzzle 3D `FillBuilder.applied_count` pattern generalized: how much of `state` is COMMITTED versus merely
  planned"* (`:1063-1066`). This is the closest existing primitive to "provisional mutation visible while running,
  committed on finalize."
- `CommitCandidate { state: RetainedJobPayload, output: RetainedJobPayload }` (`:1078-1082`) — terminal success
  payload.
- `JobFault { detail: RetainedJobPayload }` (`:1085-1088`).
- `StepOutcome` (`:1092-1103`): `Yield | PreviewReady(payload) | CheckpointReady(Checkpoint) |
  Complete(CommitCandidate) | Cancelled | Fault(JobFault)`; `is_terminal()` (`:1106-1108`) = `Complete | Cancelled
  | Fault`.
- `InteractiveJob` trait (`:1163-1172`): `step(&mut self, cx) -> StepOutcome`, `begin_close`, `close_step`,
  `register_close_wake`, `terminal_is_empty`. One call per slice; job-owned state carries everything between
  calls — no run-to-completion method exists on the trait.
- `drive_step` (`:1186-1199` signature, body follows) — runs exactly one `step` under a `Watchdog`, pre-checks
  cancellation, is "the ONE place a returned `StepOutcome` becomes a `semio_framework_trace::record_*` call."
  Takes `operation: OperationId`, `generation: Generation`, `cancel: CancelToken` as explicit parameters — i.e.
  **the `(operation, generation)` identity pair is already a first-class part of every single interactive step**,
  which is exactly the identity a `ToolRun` needs to make a stale abort/finalize a safe no-op.

### 3.2 Reactor-level bounded jobs — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🦀️.rs`

- `JobBudget { fuel: u64, deadline_ms: u32 }` (`:98-101`).
- `JobStep::{Running(Option<Vec<u8>>), Done(Vec<u8>), Failed(Vec<u8>)}` (`:106-110`) — the `Option<Vec<u8>>` on
  `Running` is the opaque progress-bytes channel (job-defined encoding, no shared schema at this layer).
- `BoundedJob` trait (`:122-127`): `step(&mut self, budget) -> JobStep`, `cancel(&mut self)`, `checkpoint(&self)
  -> Option<Vec<u8>>`, `terminal_drop_is_shallow(&self)`.
- `register_bounded_job_kind(kind, factory)` (`:169`) — registers into a thread-local `BOUNDED_KIND_REGISTRY`.
- `STALL_LIMIT: u32 = 3` (`:324`) — a job reporting `Running` with unchanged progress+budget for 3 consecutive
  `step_job` calls fails as `job.stalled` (`:530`). This is an existing, generic "the tool looks stuck" detector
  that a `ToolRun` UI could surface as a distinguished state (e.g. `Stalled`) rather than leaving it silent until
  a hard fault.

### 3.3 Kernel effects — `🧰️framework/🔨️modules/🎠️kernel/🦀️.rs`

- `Effect::SpawnJob { job: u64, kind: String, input: Vec<u8>, placement: JobPlacement }` (`:632-637`).
- `Effect::CancelJob { job: u64 }` (`:638-640`).
- `Effect::DispatchAction { req, action, args, delay_ms: u64 }` (`:508-514`) — doc: lets `handle_action` advance
  staged work "over several ticks without blocking the host," self-chaining via `requestedEffects`.
- `Effect::SetTimer { id: u64, after_ms: u64, repeat: bool }` (`:625-630`) — "replaces self-tick loops and
  `pending_effects()` polling."
- `JobPlacement` enum at `:712` (`Inline`/`Isolated`/`Exclusive`, per `🦀️.rs:712-719` — **[not re-read
  variant-by-variant this pass, matches audit-progress-primitives.md §3.3]**).

### 3.4 `(job, operation, generation)` identity and adopt pattern — confirmed live in the energy exemplar

`✏️s/🔌️plugins/🔋️energy/…/⚡️simulation/🦀️.rs:50-58` (`request_identity_args()`): every non-start verb
(cancel/retry/discard/adopt) carries `request, operation, generation, configDigest` — *"so a cancel/retry/discard/
adopt can never be applied to a run other than the one the user is looking at"* (`:50-51`). Actions declared at
`:70-74`: `START_ACTION_ID`/`CANCEL_ACTION_ID`/`RETRY_ACTION_ID`/`DISCARD_ACTION_ID`/`ADOPT_ACTION_ID` — **five
distinct verbs**, i.e. this exemplar already distinguishes "abort" (`cancel`/`discard`) from "finalize"
(`adopt` — only reachable from `FinalReady`, see §4.1) even though the framework schema (`WindowMeasure::Progress`)
only has room for one (`cancel`).

No checkpoint/preview/adopt primitive at the reactor `BoundedJob` layer beyond `checkpoint(&self) ->
Option<Vec<u8>>` (§3.2) — "adopt" is entirely a plugin-invented convention (persisted document state transition
gated by a `FinalReady`/`Adopted` status enum), not a framework primitive.

---

## 4. Exemplar comparison

### 4.1 Energy simulation — richest lifecycle vocabulary in the repo

- **States** (`EnergySimulationStatus`, referenced at `⚡️simulation/🦀️.rs:108-120`, 9 variants): `Idle →
  Admitting → Queued → Running → {Cancelled | Faulted | FinalReady} → Adopted`, plus `Closing`. This is the
  richest state machine found anywhere in the audited surface, and it already separates "the run finished"
  (`FinalReady`) from "the user committed it" (`Adopted`) — i.e. **the exact Complete-vs-Finalized split the
  dev's requirement asks for, already built, just not generalized.**
- **Actions**: `start-energy-simulation` / `cancel-energy-simulation` / `retry-energy-simulation` /
  `discard-energy-simulation` / `adopt-energy-simulation` (`:15-19`, declared `:70-74`), each a
  `bounded_catalog`/`InteractiveJobClassification::Migrated` `ActionDefinition` (`:27-31`).
- **Keyboard**: `.keybinding("mod+enter", simulation::START_ACTION_ID)`, `.keybinding("mod+.",
  simulation::CANCEL_ACTION_ID)`, `.keybinding("mod+shift+enter", simulation::ADOPT_ACTION_ID)` —
  `✏️s/🔌️plugins/🔋️energy/…/✏️editor/🦀️.rs:1343-1345`. A repo-wide law test
  (`✏️s/🔌️plugins/🔋️energy/…/🧪️tests/🔬️unit/🦀️.rs:335-349`,
  `every_keybinding_uses_canonical_punctuation_key_tokens`) asserts every declared chord's last segment is a
  canonical key token (forbids the English word `"period"` etc. — `event.key` for `.` is literally `"."`) and
  pins the cancel chord to exactly `"mod+."` — i.e. this exemplar's keyboard bindings are test-guarded, not just
  documented.
- **Progress**: **no `WindowMeasure` used at all** — entirely a `TreeView`/`TreeNodeView` body. `status_text()`
  (`:108-120`) and `stage_text()` (`:124-149`, 22-variant `EnergyJobStage`) are hand-authored `say(german, en,
  de)` EN/DE pairs, not `LocalizedLabel::native`. Percent-complete is hand-computed per-tier string math
  **[from audit-progress-primitives.md §1.3, not independently re-derived this pass — file not re-opened past
  line 161]**.
- **i18n**: every label EN+DE via the local `say()` helper (`:100-106`), no default language, matching the
  ticket's requirement — but via a bespoke local convention, not `LocalizedLabel`/terminology-macro.
- **Transaction semantics**: `Adopted` is reached only from `FinalReady` (`adopt-energy-simulation`) — the
  document mutation is deferred until the user explicitly commits, i.e. **this IS the transaction pattern the
  dev wants**, already implemented once, informally, per-plugin.

### 4.2 Remodel reconstruction

`✏️s/🔌️plugins/📸️remodel/…/🎮️commands/🏗️run-reconstruction/🦀️.rs:20-21`: `RECONSTRUCTION_STEP_BUDGET: usize = 1`,
`MAX_RECONSTRUCTION_TICKS: u32 = 200_000`; a `RequestedStage` enum at `:35` (ten named stages **[count not
re-verified this pass — from audit-tool-inventory.md §1.2]**). Status readout:
`📌️panels/🗿️artifact/🦀️.rs:26-38` — `"Reconstruction: <Stage> (NN%)"` derived from `job.stage`. Own doc comment
(`:26-30`) admits a known limitation: *"`running` is derived from the persisted job stage … a synchronous run
never leaves the document in a non-terminal stage … effectively always Idle once a run finishes"* — i.e. a fast
run never visibly shows Running even momentarily, a real regression risk for "show the process" if reconstruction
ever gets fast enough to race the UI's own poll.
Keyboard bindings for this exemplar were not located this pass (no `.keybinding(` hits found in
`✏️editor/🦀️.rs` scoped to remodel's reconstruction actions) — **flagged as unconfirmed, not proven absent.**

### 4.3 Procedural preview eval (`cancel-preview-eval`)

`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/…/✏️editor/🦀️.rs:92` maps the DSL action name
`"cancelPreviewEval"` to `"cancel-preview-eval"`; keyboard binding `.keybinding("mod+.", "cancelPreviewEval")`
(`:2496`) — **the same `mod+.` chord energy uses for its own cancel**, suggesting an (informal, non-enforced)
repo-wide convention: `mod+.` = cancel/abort the running tool. No `start`/`finalize` binding was found for this
tool in the same scope (`mod+enter` is bound to `"undo"` in this file's context at a different line — not a
preview-eval start chord); flagged unconfirmed rather than assumed absent.

### 4.4 Puzzle 2d fill lifecycle — the richest generic state machine in the repo (best model for `ToolRun`)

`Puzzle2dFillLifecycle` (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/…/🎚️config/🦀️.rs:42-60`), doc: *"Event-sourced
public lifecycle for the transient mounted fill owner"*:

```rust
pub enum Puzzle2dFillLifecycle {
    Idle, Capturing, Queued, Running, CheckpointReady, Applying,
    AwaitingAdoption, Closing, Completed, Cancelled, Faulted, Discarded,
}
```

Twelve states, `#[default] Idle`, `ToValue`/`FromValue` derived (wire-safe). This is materially more granular
than the energy exemplar's 9 states and already separates **Running → CheckpointReady → Applying →
AwaitingAdoption → Completed** (a genuine "provisional, then adopted" pipeline) from the abort paths
(**Cancelled/Faulted/Discarded**), plus a **Closing** transitional state neither other exemplar has explicitly
named. **This enum is the strongest existing candidate to generalize into a framework `ToolRunState`.**

### 4.5 Assembly WFC engine

`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/…/🧬️schema/💡️inferences/🧩️wfc-engine/💼️job/🦀️.rs` — full
`StepOutcome`-driven engine, staged (`WfcStage`), `emit_preview` publishing bounded batches
**[from audit-tool-inventory.md §1, not re-derived this pass]**. Verified this session: `grep -rln
"WfcPreview|WfcStage" ✏️s/🔌️plugins/🌀️procedural` returns **only the engine's own job file and its own unit test**
— **zero UI consumer exists anywhere in the plugin tree.** The engine already emits everything a `ToolRun`
progress view would need; nothing renders it.

### 4.6 Comparison table

| Exemplar | States (count) | Start | Abort | Finalize | Progress UI | Keyboard | i18n | a11y |
|---|---|---|---|---|---|---|---|---|
| Energy simulation | `EnergySimulationStatus`, 9 | `start-energy-simulation` | `cancel-energy-simulation` | `adopt-energy-simulation` (from `FinalReady`); `discard-energy-simulation` also exists as a non-adopt terminal | Hand-rolled `TreeView`, 4-tier % math, `aria-live=polite role=status busy=…` **[not re-verified this pass]** | `mod+enter` start / `mod+.` cancel / `mod+shift+enter` adopt, test-guarded (`🧪️tests/🔬️unit/🦀️.rs:335-349`) | Local `say(german,en,de)`, no default | Hand-encoded ARIA strings in tree labels, not first-class elements |
| Remodel reconstruction | `RequestedStage`, ~10 named stages | `run-reconstruction` | `cancel-reconstruction` | none named — no distinct finalize verb found | `"Stage (NN%)"` string in panel; known "never shows Running on a fast run" bug (`📌️panels/🗿️artifact/🦀️.rs:26-30`) | not located this pass | not verified this pass | not verified this pass |
| Procedural preview eval | phase/progress object, informal | implicit (self re-arms) | `cancel-preview-eval`, `mod+.` (`✏️editor/🦀️.rs:2496`) | none — self-settles | `preview_window_status_json` **[not re-derived]** | `mod+.` cancel only found | not verified this pass | not verified this pass |
| Puzzle 2d fill | `Puzzle2dFillLifecycle`, 12 | `fill-session-begin` **[name from audit-tool-inventory.md, not re-verified]** | implicit via `Cancelled`/`Faulted`/`Discarded` states | implicit via `AwaitingAdoption → Completed` | Slider label string `"Progress: accepted/count"` **[from audit-progress-primitives.md, not re-verified]** | not verified this pass | not verified this pass | not verified this pass |
| Assembly WFC | `WfcStage` + `StepOutcome`, engine-only | engine `step()` only | `StepOutcome::Cancelled` | `StepOutcome::Complete` | `emit_preview`, **no consumer** | n/a — no UI | n/a | n/a |

**No exemplar implements all three of {start, abort, finalize} as three textually-distinct, keyboard-bound,
generically-typed actions with a shared state enum.** Energy comes closest (5 distinct actions, 9 states,
keyboard-bound, test-guarded) but hand-rolls its own progress UI outside `WindowMeasure` entirely. Puzzle 2d fill
has the richest state enum but the audit found no independent confirmation of its keyboard/a11y story this pass.

---

## 5. Generic operation/task/activity panel — does not exist

Searched for a shell-level, cross-plugin surface that could host a generic "tool is running" readout:

- **`TransientNotice`** (`🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/🧬️schema/🦀️.rs:418-435`): `{ message:
  String, kind: NoticeKind, expiresAtMs: Option<i64> }`. `ShellState.transient_notice: Option<TransientNotice>`
  (`:662`) — **exactly one slot**, singular, auto-dismissing, no id, no progress payload, no cancel affordance.
  Actions `ShowTransientNotice`/`DismissTransientNotice` (`:821-822`). This is a toast, not an activity log; it
  cannot host a running tool's state.
- **`Effect::Notify { message: String }`** (plugin-authority layer) **[from audit-progress-primitives.md §1.4,
  not re-checked this pass]** — same shape problem, one level lower (kernel effect vs shell command).
- **No `ActivityPanel`/`JobsPanel`/`OperationsPanel`/`TaskPanel`/`CommandPalette`-with-job-list type exists**:
  `grep -rln "ActivityPanel\|JobsPanel\|OperationsPanel\|TaskPanel\|CommandPalette" --include="*.tsx"
  --include="*.rs" 🧰️framework` (excluding tests) returns only unrelated hits (events dispatch, server contract,
  the shell schema file already covered above) — no such type.
- **Footer** — `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔚️Footer/🟦️.tsx` **[from audit-progress-primitives.md §1.4,
  not re-opened this pass]**: generic `NavbarItem[]` host, no built-in status/progress semantics; the one
  "status" convention there is a hand-built sync-status pill, unrelated to tool runs.
- **`WindowMeasure` overlay itself** is the closest candidate to a generic host, but it is scoped per-window
  (`tool_measures`/`window_measures`, keyed by tool/window id) — there is no aggregate view across all
  currently-running tools/jobs in an app, let alone across the whole shell.

**Conclusion**: there is no existing generic activity/operations surface anywhere in the framework. A `ToolRun`
contract needs to either (a) stay scoped to the tool's own `WindowMeasure::Progress` leaf (what wave F already
built the rendering half of), or (b) if the dev wants a cross-tool "what's running right now" view, that is new
framework surface with no precedent to reuse — it would have to be built from scratch, most plausibly as a new
`Footer`/shell-chrome element that reads the same per-tool `ToolRun` records rather than a bespoke second copy of
the state.

---

## 6. Proposed `ToolRun` contract

Domain-neutral, schema-first, one new manifest type plus the state it puts on top of what already exists. Kept
deliberately close to `Puzzle2dFillLifecycle` (§4.4, the richest state enum already proven in production) and the
energy five-verb action set (§4.1), rather than inventing new vocabulary.

### 6.1 State enum — lives beside `ToolDefinition` in `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs`

```rust
/// 🔄️ @emoji Generic lifecycle every interactive tool run passes through. Framework-owned so every
/// renderer (React, wgpu) can branch on ONE shared enum instead of each plugin inventing its own
/// (compare `Puzzle2dFillLifecycle`, `EnergySimulationStatus` — this generalizes both).
pub enum ToolRunState {
    Idle,                 // armed, not yet started (mirrors ViewModel.active_tool_id being set with no run)
    Running,              // step() is being driven; provisional mutations may be visible (transaction open)
    CheckpointReady,       // a resumable pause point exists (Checkpoint.applied_progress) but run continues
    Complete,             // StepOutcome::Complete reached; transaction NOT yet committed — awaiting finalize
    Aborted,              // user- or system-cancelled before Complete; transaction rolled back
    Faulted,              // StepOutcome::Fault; transaction rolled back
    Finalized,            // user committed a Complete run; transaction committed, terminal
}
```

`Finalize` is only a legal transition from `Complete` — mirrors energy's `FinalReady → Adopted` split (§4.1) and
matches the dev's explicit requirement ("finalized only when complete"). `Aborted` is legal from `Idle | Running |
CheckpointReady | Complete` (an already-complete-but-not-yet-finalized run can still be discarded, matching
energy's `discard-energy-simulation`, §4.1). This directly encodes requirement 2's transaction discipline: the
document only ever sees a mutation applied at the `Finalized` transition; `Aborted`/`Faulted` are rollback points.

### 6.2 Generic actions — three framework-injected, palette-hidden actions per tool, mirroring `setActiveTool`

```rust
pub const START_TOOL_RUN_ACTION_ID: &str = "startToolRun";
pub const ABORT_TOOL_RUN_ACTION_ID: &str = "abortToolRun";
pub const FINALIZE_TOOL_RUN_ACTION_ID: &str = "finalizeToolRun";
```

Each takes the identity args already proven by energy (`request, operation, generation` — §3.4/§4.1), so a stale
abort/finalize dispatched after the run already moved on is a safe no-op, exactly like
`cancel-energy-simulation`'s existing guard. `toolId` is implied by `ViewModel.active_tool_id` the same way
`setActiveTool` needs no `windowKindId` (tools are windowless per `🦀️.rs:1207-1210`).

### 6.3 Progress payload — extend `WindowMeasure::Progress`, do not replace it

Wave F's `WindowMeasure::Progress` (`🧩️component/🦀️.rs:1134-1158`) already carries `stage`, `completed`, `total`,
`steps: Vec<MeasureProgressStep>`. Add:

```rust
Progress {
    // ...existing fields unchanged...
    state: ToolRunState,                    // NEW — drives which of start/abort/finalize renders
    start: Option<ActionDescriptor>,         // NEW — replaces "presumed separate start action"; None while Running
    abort: Option<ActionDescriptor>,         // RENAME of `cancel` — same shape, clearer name matching the dev's word
    finalize: Option<ActionDescriptor>,      // NEW — only Some when state == Complete
}
```

This is additive to the wire shape (new fields, old `cancel` field can alias-deprecate to `abort` in the same
pass since this is a greenfield repo — CLAUDE.md forbids compatibility layers, so `cancel` should be renamed, not
kept alongside `abort`). The renderer branches purely on `state`: show `start` when `Idle`, the progress bar +
`abort` when `Running`/`CheckpointReady`, `finalize` + `abort`(=discard) when `Complete`, a terminal readout with
no actions when `Aborted`/`Faulted`/`Finalized`.

### 6.4 Where it lives in the taxonomy

- **Definition**: `ToolRunState` beside `ToolDefinition` in `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` (manifest
  layer, like `ToolDefinition`/`UtilityDefinition` themselves — not the wgpu-schema crate, since this is
  framework-wide session-state vocabulary, not a rendering primitive).
- **Wire-format leaf** (`Progress`'s new fields): stays in
  `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs` beside the rest of `WindowMeasure`, mirroring
  wave F's own placement decision.
  - Add `ToolRunState`'s own wire mirror there too (`#[serde(rename_all = "camelCase")]`, `#[value(rename_all =
    "camelCase")]`), matching `MeasureProgressStepKind`'s existing convention.
- **TS projection**: `🧰️framework/🔨️modules/🧬️schema/📽️projection/🦀️.rs`, alongside the existing `WindowMeasure`
  entry (§2.3) and a new `ToolRunState` entry beside `MeasureProgressStepKind` (§2.3, `:878-884`).
- **Generated manifest**: regenerate via `bun nx run @semio-tech/framework-rs:generate` exactly as wave F already
  exercised (§2.4) — this pipeline is proven working.
- **wgpu widget**: extend `render_window_measure_progress` (`🪀️widgets/🦀️.rs:496-551`) to take the three new
  action slots and branch its button row on `state`; **and, this time, actually wire it into a real call site** —
  before this can render anywhere native, the missing "`WindowMeasure` tree → retained `WidgetNode`/`ControlNode`
  tree" converter needs to exist (§2.2 gap) and `WidgetNode`/`ControlNode` need a `Progress` variant added
  (`🪀️widgets/🦀️.rs:170-186`), since neither enum has one today. This is real, unscoped follow-up work — wave F
  did not just leave the widget partially wired, it left the entire measures-overlay native rendering path
  without an entry point for ANY measure kind (§2.2), and that gap must close for `Number`/`Progress` (and their
  siblings) to ever paint on a native target.
- **React**: extend `WindowMeasureProgress` (`🎚️measure-controls/🟦️.tsx:161-209`) with the `start`/`finalize`
  buttons (same pattern as the existing `cancel` button at `:202-208`, renamed `abort`), gated on `measure.state`.
  No new wiring needed at the `ShellHelpers/🟦️.tsx` dispatch level (§2.5) — the existing `case "progress"` sites
  (`:3337-3400`+ its `switch` at `:3580`) already route the whole measure object through.

### 6.5 How plugins extend it (domain-specific extensions over the domain-neutral core)

- `ToolRunState`/the three generic actions are the domain-neutral core — every plugin's tool run reports through
  this ONE enum + three verbs.
- Domain-specific detail (energy's 22-variant `EnergyJobStage`, puzzle3d's `stall_reason`, WFC's `WfcStage`) stays
  exactly where it already lives: as the `stage: Option<String>` free-text caption and the `steps:
  Vec<MeasureProgressStep>` log — both already string/enum-tagged, already localized per-plugin via each
  plugin's own `🗣️terminology/🦀️.rs` (e.g. `fill_progress`/`fill_cancel` at
  `✏️s/🔌️plugins/🧩️puzzle/…/🗣️terminology/🦀️.rs:25-28`, per audit-progress-primitives.md §2.2 item 7, not
  re-verified this pass). Plugins never need their own `Running`/`Complete`/`Aborted` vocabulary — that
  fragmentation (9 states here, 12 there, none matching) is exactly what today's per-plugin `EnergySimulationStatus`
  / `Puzzle2dFillLifecycle` duplication produces, and what this proposal collapses into one enum.
- A plugin that needs extra terminal states (e.g. puzzle3d fill's document-capacity stall,
  `master-plan.md §2.1(4)`'s `stallReason`) reports them through the existing free-text `stage`/`steps` channel
  while `state` stays `Running` (a stall is not a new lifecycle state, it's a Running tool that isn't currently
  making progress — the reactor's own `STALL_LIMIT`/`job.stalled` detector, §3.2, already treats it this way at
  the job layer).

### 6.6 EN/DE labels and keyboard shortcuts

- Three framework-owned label pairs, `LocalizedLabel::native(en, de)`, alongside the existing
  `measure_progress_cancel_label()` (`🧩️component/🦀️.rs:1211-1213`) — rename that function
  `measure_progress_abort_label()` returning `("Abort", "Abbrechen")`, add `measure_progress_start_label()` →
  `("Start", "Starten")` and `measure_progress_finalize_label()` → `("Finalize", "Abschließen")`. No default
  locale — an exhaustive match on `Locale`, per `LocalizedLabel::native`'s existing contract
  (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🏷️label/🦀️.rs:123-131`, **[from
  audit-progress-primitives.md §2.2 item 7, not re-verified this pass]**).
- Keyboard: standardize on the pattern already used twice independently in this repo (energy §4.1, procedural
  preview-eval §4.3) — `mod+enter` = start, `mod+.` = abort, `mod+shift+enter` = finalize (reusing energy's own
  adopt chord verbatim, since "finalize" and "adopt" are the same concept under different plugin-chosen names).
  Every plugin's `.keybinding(...)` declarations for these three actions should register at the
  `AppDefinition`/window-kind level exactly as energy already does (`✏️editor/🦀️.rs:1343-1345`), and should be
  covered by the same kind of law test energy already has
  (`🧪️tests/🔬️unit/🦀️.rs:335-349`, generalized to a framework-level test asserting the three canonical chords
  are used consistently wherever `START_TOOL_RUN_ACTION_ID`/`ABORT_TOOL_RUN_ACTION_ID`/
  `FINALIZE_TOOL_RUN_ACTION_ID` are bound, rather than re-deriving the rule per plugin).

---

## 7. Summary of unverified/flagged points (do not treat as fact without a follow-up read)

1. Remodel reconstruction's and puzzle 2d fill's exact keyboard bindings — not located this pass.
2. Energy's `aria-live=polite role=status busy=…` claim and its per-tier percent math — carried from
   audit-progress-primitives.md, not re-opened this pass past `⚡️simulation/🦀️.rs:161`.
3. Whether a native (non-React) desktop consumer of the wgpu Measures overlay exists anywhere outside this
   session's grep reach — the dead-code finding in §2.2 is a strong, repo-wide-grep-backed signal, not an
   absolute proof of non-existence (a macro-generated or dynamically-dispatched call site would not show up in a
   literal-string grep, though no evidence of such indirection was found either).
4. `JobPlacement` variant docstrings (`Inline`/`Isolated`/`Exclusive`) — line range cited from the earlier audit,
   not re-read line-by-line this pass.
5. Puzzle 2d fill's action names (`fill-session-begin` etc.) — carried from audit-tool-inventory.md §2, not
   independently re-grepped this pass.
