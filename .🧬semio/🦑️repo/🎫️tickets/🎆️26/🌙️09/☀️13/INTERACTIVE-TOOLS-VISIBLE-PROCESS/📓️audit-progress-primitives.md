# Audit: Progress/Process Primitives For Interactive Tools

Read-only audit. No source touched. MCP servers `repo`/`semio` failed to connect this session
("invalid initialize params" / "CONNECTION_CLOSED") — ticket folder managed on disk per
`project-repo-mcp-may-fail-to-connect.md`; this ticket folder already existed
(`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️13/INTERACTIVE-TOOLS-VISIBLE-PROCESS/`), so no
open/reopen/close was attempted, per instructions.

---

## 1. Inventory: existing progress/status affordances

### 1.1 `WindowMeasure` (the canonical, plugin-facing enum)

Single definition, `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs:1043-1131`
(re-exported to plugins as `semio_framework_plugin::WindowMeasure` — grep for
`pub enum WindowMeasure` across `🧰️framework` returns exactly this one hit; there is no separate
"plugin-facing" copy, only the one canonical enum, re-exported through the plugin crate's prelude).

Four variants, `#[serde(tag = "kind")]`, camelCase on the wire:

- **`Select { id, label, value, items: Vec<MeasureSelectItem>, on_change }`** — plain dropdown, no progress semantics.
- **`Slider { id, label, value, min, max, step, ready, loading, waiting, disabled, reveal, on_change }`** (line 1050-1077):
  - `ready: Option<f64>` — "absolute value on the fixed `[min,max]` range that is already preloaded/ready"; renderer keeps `max` stable and draws a highlight from the knob to this extent. **This is the closest thing to a progress bar the schema has** — but it rides on a slider, not a dedicated progress widget, and both `min`/`max` are mandatory plain `f64` (no `Option`), so a slider can never express "no upper bound."
  - `loading: Option<bool>` — shows a spinning ring on the leaf while true.
  - `waiting: Option<bool>` — shows a dashed, slower ring; `loading` wins if both set.
  - `disabled: Option<bool>` — inert (e.g. parent weight is zero).
  - `reveal: Option<String>` — reveal-group id; while dragging, the host only mutates a client-side cutoff set (`WorldInstancesLayer`), not on every drag tick — commits on pointer-up only.
- **`Toggle { id, icon_id, label, pressed, text, on_change }`** (1078-1085) — `text: Option<String>` is a free-form trailing label, used today as an ad hoc progress/count readout (see §1.3, `cancel_measure`'s `"{done} / {max} planned"` string). Not structured — just a string.
- **`Group { id, label, default_open, active_utility_id, value, min, max, step, ready, loading, waiting, on_change, children }`** (1086-1128) — a folder that can *also* carry an optional header slider (mirrors Slider's ready/loading/waiting so a weight-control group row can show the same preload affordance).

No `Progress`, `Log`, `Meter`, `Timeline`, or `Steps` variant exists anywhere in the enum.

### 1.2 Everything else in the schema that is progress-*adjacent*

- **`WindowEngagementControl`** (TS mirror at `🧰️framework/🔨️modules/🧬️schema/📽️projection/🦀️.rs:1739`, a *different*, footer/toolbar-strip-scoped enum, not `WindowMeasure`) already has a `"kind":"stepper"` variant with **optional** `min?`/`max?`/`step?`/`unit?` (`{ kind: "stepper", id?, label?, value, min?, max?, step?, unit?, disabled?, onChange?, onCommit? }`), unlike `WindowMeasure::Slider`'s mandatory `min`/`max`. TS resolution in
  `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx:1663-1706` — `windowEngagementControlToSpec`: the `slider` arm still throws if `min`/`max` are missing (line 1703: `"engagement slider requires explicit minimum and maximum"`), but the `stepper` arm (line 1706, fallthrough) has no such guard — **this is the one place in the whole schema where a numeric control is already allowed to have no bound.**
- **`UiNumberStepperNode`** (form/document-body control, TS mirror `🧰️framework/🔨️modules/🧬️schema/📽️projection/🦀️.rs:1515-1517`: `{ id, value, step, uniform, onAbsolute, onDelta, presence?, menu? }`) — genuinely unbounded: no `min`/`max` field at all. Backed by the native wgpu widget `WidgetNode::NumberStepper`/`ControlNode::NumberStepper { id, value, step, uniform, on_absolute, on_delta }` at `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🪀️widgets/🦀️.rs:167,183,332,408`, rendered in React at `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx:943,990,1192` (`NumberStepperView`) and natively at `…/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1804,1830,1922`. **This widget is never exposed as a `WindowMeasure` variant** — only as a form-body/document-content control.
- **`ActionArgControl::Number { min?, max?, step? }`** (📽️projection/🦀️.rs:29) and **`ArgSchema::Number { min?, max?, step?, integer, unit? }`** (line 314) — action-argument schema, used to build *staged forms* (dialog-style), also with optional bounds. Not a `WindowMeasure` either.
- **`Table`'s inline `"stepper"` cell** (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📊️Table/🟦️.tsx:25`) — `{ value, min, max, step, action }`, bounds mandatory, unrelated context (spreadsheet cell).

### 1.3 Reference plugin implementations — how progress is shown *today*

**Puzzle 3D fill tool** (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs`) — the exact "fill count slider" the task references:
- `count_measure()` (line 24-42): `WindowMeasure::Slider` with `min: 0.0`, `max: PUZZLE3D_FILL_COUNT_MAX as f64` (constant = **1000**, defined `…/✏️editor/🦀️.rs:73`), `ready: Some(available_count)`, `loading: if done {None} else {Some(true)}`, `reveal: Some("puzzle3d-fill")`. Tests pin the max as fixed: `find_measure_slider_max(...) == PUZZLE3D_FILL_COUNT_MAX` (`…/🧪️tests/🔬️unit/🦀️.rs:4481,4649`) — i.e. **today the slider is explicitly NOT unbounded**, and default `fill_count` is **0**, not 100 (`🎚️config/🦀️.rs:220-227,282`).
- `cancel_measure()` (line 45-63): a `WindowMeasure::Toggle` used purely as a progress+cancel row — `text: Some(format!("{count} / {max_count} {label}"))` (string-formatted progress, not a structured field), `pressed: false` always (it's really a button, not a toggle), `on_change` carries the `(job, operation, generation)` triple as action args so a stale cancel is a no-op.
- Terminology (i18n) for these strings: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs:25-28` — `fill_progress`/`fill_cancel`/`fill_planned`/`fill_failed`, each with `native_en`/`native_de` (+ `reuse_en`/`reuse_de` for a terminology-reuse axis).
- Tick: `fillBuildTick` (`…/✏️editor/🎮️commands/🪣️fill-build-tick/🦀️.rs`) polls the retained job (`precompute.poll_fill_job()`), spawns one more `Effect::SpawnJob{ kind: FILL_JOB_KIND, placement: JobPlacement::Isolated }` if needed, surfaces a fault via `ctx.notice(...)`, cadence documented as **120 ms** at `…/✏️editor/🦀️.rs:1635,2680` ("redrives this via 120ms `suggestionsTick`/`fillBuildTick` ticks").

**Energy plugin** (`✏️s/🔌️plugins/🔋️energy/…/🎭️modes/✏️edit/🪟️windows/⚡️simulation/🦀️.rs`, 278 lines) — the richest reference implementation, but it uses **none of `WindowMeasure`**:
- Entirely a `TreeView`/`TreeNodeView` body (accessibility-tree window content), not measures.
- `status_text()` (line 100-111): 9-state textual status (`Idle/Admitting/Queued/Running/Cancelled/Faulted/FinalReady/Adopted/Closing`), each with an English+German pair via a local `say(german, en, de)` helper (not `LocalizedLabel::native`, but the same EN/DE-no-default convention).
- `stage_text()` (113-136): 22-variant `EnergyJobStage` enum, each stage translated — this is a **much richer stage vocabulary than anything in the generic job/UI schema layer**, entirely bespoke to this plugin.
- `tier_nodes()` (185-207): "four-tier progress" — for each of 4 `EnergyQualityTier`s, computes `percent = timestep*100/total_timesteps` **by hand** (line 194) and formats a label string `"{name}: {timestep} / {total} ({percent:.1} %)"` (line 197) — i.e. percent-complete is entirely ad hoc string math, not a schema field.
- `render()` (238-278): wraps the live region in `aria-live=polite · role=status · busy={busy} · {status}` **as a literal label string** (line 251, 259) — accessibility semantics are hand-encoded into tree-node label text, not a first-class accessible-progress element.
- Actions: `start/cancel/retry/discard/adopt-energy-simulation` — `cancel-energy-simulation` (`CANCEL_ACTION_ID`, line 15) takes the 4-field request-identity args (`request/operation/generation/configDigest`, `request_identity_args()` line 48-56) and is presumably answered with `Effect::CancelJob` (session-side, not shown in this file).

**Procedural generation3d — `flowEvalTick`** (`…/🎮️commands/⏱️flow-eval-tick/🦀️.rs`, thin wrapper over the surface-neutral `🧵️preview-eval` module shared with `👁️viewer`):
- Self-redispatch cadence is **not** a fixed-interval `SetTimer`; it's `Effect::DispatchAction{ action:"flowEvalTick", delay_ms: 0 }` (`preview-eval/🦀️.rs:127-129`) — i.e. re-arms itself every reactor turn until the chain settles or a `may_rearm` guard (line 132+) says no more contribution exists.
- Per-round-trip budget: `EVALUATE_STEP_BUDGET: u64 = 8` (units, line 115) and `EVALUATE_STEP_WALL_MICROS: u64 = 2_000_000` (2s wall, line 121) — explicitly *below* the host shard watchdog's 16s silence threshold and 5s heartbeat, so a slow eval round-trip is never mistaken for a dead actor.
- No `WindowMeasure`/progress UI in this file at all — the 👁️preview window (`…/🪟️windows/👁️preview/🦀️.rs`) is where any surfaced status would live; it was not read in full for this audit (flagged as a gap in coverage, not confirmed empty).

**FEM, demonstrator, remodel, lowpoly**: none of these show `loading: Some(true)` / `waiting: Some(true)` on any `WindowMeasure` (grep across each plugin's `🗿️artifacts` tree returned zero non-schema/non-story hits) — i.e. **puzzle3d's fill tool is the ONLY plugin actually using the Slider progress fields (`ready`/`loading`/`waiting`) today.** FEM/lowpoly do reference "session"/"progress" strings but only in prose/comments or unrelated schema noise; demonstrator has none.

### 1.4 Notices, footer, deadline chrome

- **`Effect::Notify { message: String }`** (`🧰️framework/🔨️modules/🎠️kernel/🦀️.rs:368-370`) — a single fire-and-forget plain-string toast. No id, no severity, no progress payload, no way to update/replace an already-shown notice, no cancel affordance. `ctx.notice(|labels| ...)` (used by `fillBuildTick`, line 30 of that file) is the plugin-side sugar over this.
- **`Footer`** (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔚️Footer/🟦️.tsx:23-53`) — a generic `NavbarItem[]`-hosting bottom bar (mirrors `Navbar`). No built-in status/progress semantics; a plugin author supplies arbitrary `content`. The one real "status" convention living here is the sync-status pill built ad hoc in `ShellHelpers/🟦️.tsx` (`SYNC_STATUS_PERSISTED_LABEL`/`SYNC_STATUS_PENDING_LABEL`, ~line 2390) — plain text, not reusable outside the shell's own sync indicator.
- **`WindowContentDeadLine`** (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🚧️WindowContentDeadLine/🟦️.tsx`) — a false lead for this audit: purely a scroll-clearance/layout concept (how much of a window body is obscured by floating measures/engagement/search overlay chrome), unrelated to job deadlines or watchdogs. No progress semantics.
- There is **no log/console panel element** anywhere in `🧰️framework/🔨️modules/🖱️ui` that shows a stream of algorithm steps — confirmed by grep for Log/Console/Timeline/Steps element names in `🧱️elements/`; none exist.

---

## 2. The gap and the smallest schema-first addition

### 2.1 The gap, precisely

1. **`WindowMeasure` has no dedicated progress/log/steps variant.** The only progress-capable field (`Slider.ready`) is borrowed from a control whose primary purpose is a *settable* value, forcing every plugin that wants a progress bar to also expose an (often meaningless) draggable slider, and forcing every plugin that wants a "steps so far" readout to hand-format it into `Toggle.text` (puzzle3d) or a `TreeNodeView` label string (energy) — there is no shared, testable, localizable shape.
2. **A rich progress vocabulary already exists at the job layer and is thrown away at the UI boundary.** `ProgressEvent` (`🧰️framework/🔨️modules/🧵️job/🦀️.rs:1688-1758`) is explicitly documented as "the ten-event progress vocabulary … a caller-side UI/log projection" with `Started/StageChanged/CandidateTested/PreviewPatch/Diagnostic/Checkpoint/CommitCandidate/Completed/Cancelled/Failed`, carrying `stage: &'static str`, `completed_units: u64`, `total_units: Option<u64>` (already `Option`!), `quality: f32`, `tolerance: f32`, and `DiagnosticKind::{Info,Warning,Stalled,Error}` (line 1676-1681) for severity. **None of this ever reaches `WindowMeasure` or any TS type** — hosts that want to show it (energy, puzzle3d) reimplement an ad hoc subset by hand, per plugin, with zero shared schema, zero shared renderer, and zero test-once-use-everywhere guarantee.
3. **No JSON Schema exists for `WindowMeasure` at all.** Repo-wide grep for `WindowMeasure`/`windowMeasure` inside any `🔣️.json` (excluding `storybook-static`/`generated`/`dist`) returns only ticket-title JSON coincidences — never a schema `$defs` entry. The TS mirror is instead a **hand-written raw string literal** kept in a giant `SchemaMetadata::TYPES` const array (see §2.2) — i.e. `WindowMeasure` is outside the `semio_framework_schema_registry` scope-export system (`register_scope_schema_exports`/`ScopeSchemaExports`/`SchemaFormat::JsonSchema` — see `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧬️schema/🦀️.rs:17-35`) entirely; only 3 UI-contract types (`ConformanceCatalogFixture`/`ContractFixture`/`PresenceUpdate`) are registered that way. Adding a JSON-Schema-registered `WindowMeasure` would be a bigger, separate refactor than what's proposed below — the smallest addition stays inside the existing hand-mirrored-TS convention.
4. **No structured way to show "the algorithm's last N steps."** `ProgressEvent::StageChanged`/`Diagnostic` already carry exactly the right per-event shape for this, but no UI type exists to hold `Vec<of-them>`.
5. **The fill-count slider genuinely cannot become "unbounded, arbitrarily settable, default 100"** without a schema change: `WindowMeasure::Slider.max` is mandatory `f64`, and both the Rust builder (`fill/🦀️.rs:34`) and the tests (`🧪️tests/🔬️unit/🦀️.rs:4481,4649`) currently hard-pin it to `PUZZLE3D_FILL_COUNT_MAX = 1000`. The nearest already-unbounded shape in the whole schema is `UiNumberStepperNode`/`WidgetNode::NumberStepper` (`value, step, uniform`, no min/max at all) or `WindowEngagementControl::Stepper` (optional min/max) — neither is reachable from `WindowMeasure` today.

### 2.2 Proposed smallest addition (schema-first, one wave)

Add exactly **one** new `WindowMeasure` variant, modeled directly on the existing `ProgressEvent`/`DiagnosticKind` vocabulary so the job layer's already-designed shape is reused rather than reinvented:

```rust
WindowMeasure::Progress {
    id: String,
    label: Option<String>,
    stage: Option<String>,            // mirrors ProgressEvent::StageChanged.stage
    completed_units: f64,             // mirrors ProgressEvent::PreviewPatch.completed_units
    total_units: Option<f64>,         // Option, same as ProgressEvent — None = indeterminate/unbounded
    steps: Vec<MeasureProgressStep>,  // last-N log: { at_ms: u64, kind: DiagnosticSeverity, text: String }
    cancel_action: Option<ActionDescriptor>,
    loading: Option<bool>,            // reuse Slider's existing loading-ring convention
}
```

And, separately (Q4's own answer), give the numeric-entry side of the problem its own small variant
instead of overloading `Slider`, mirroring `WindowEngagementControl::Stepper`'s already-optional bounds:

```rust
WindowMeasure::Number {
    id: String,
    label: Option<String>,
    value: f64,
    min: Option<f64>,                 // None = no floor
    max: Option<f64>,                 // None = no ceiling — this is what "unbounded" means here
    step: Option<f64>,
    ready: Option<f64>, loading: Option<bool>, waiting: Option<bool>, disabled: Option<bool>,
    on_change: ActionDescriptor,
}
```

`count_measure()` in `🪣️fill/🦀️.rs` would then construct `WindowMeasure::Number` with `max: None`
and drop `PUZZLE3D_FILL_COUNT_MAX` from the clamp entirely (or keep it as a *soft* display hint via
`ready`, not a hard schema ceiling) — and its default would change from `0` to `100` in
`Puzzle3dRuntime::default()` (`🎚️config/🦀️.rs:220-227`).

**Exact file list this addition touches:**

1. **Rust definition** — `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs:1043-1131` (add the two variants + a `MeasureProgressStep`/`DiagnosticSeverity` struct/enum near `MeasureSelectItem` at line 1031).
2. **Widget rendering (native/wgpu)** — new `render_window_measure_progress`/`render_window_measure_number` beside `render_window_measure_slider`/`_toggle`/`_select` in `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🪀️widgets/🦀️.rs:445-458` (the `NumberStepper`/`Slider` `WidgetNode`/`ControlNode` variants already exist at lines 166-183/329-408 and can be reused for `Number`; `Progress` needs a genuinely new widget primitive — there is no progress-bar widget anywhere in this file today).
3. **Hand-mirrored TS projection (the actual "JSON schema" of record for this type)** — `🧰️framework/🔨️modules/🧬️schema/📽️projection/🦀️.rs`, edit the `WindowMeasure` entry inside `pub const TYPES` (currently lines ~1801-1841) to add the two new arms to the discriminated union string, byte-for-byte matching the new Rust shape (per the existing convention, doc comments become the TS JSDoc, prefixed with the docstring's emoji as CLAUDE.md requires).
4. **Regenerate the manifest mirror** — run `nx run @semio-tech/framework-rs:generate` (== `bun 🧰️framework/📦️packages/🦀️rust/📜️script.ts generate`, target declared in `🧰️framework/📦️packages/🦀️rust/📋️project.json`). This runs `cargo test --features typegen exports_typescript_bindings` against the `semio-framework` crate, which resolves to the test at `🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🔬️app-label/🦀️.rs:1471-1480` (`crate::schema_metadata::{validate,render_typescript}`, `crate::schema_metadata` = `🧰️framework/📦️packages/🦀️rust/🦀️.rs:14` → `#[path]`-mounted at `🧰️framework/🔨️modules/🧬️schema/📽️projection/🦀️.rs`), and rewrites `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts` (the `TYPES.len()` assertion at that test's line 1476, currently `184`, must bump by 2). `bun …/📜️script.ts check` (target `check` in the same `📋️project.json`) is the CI-shaped byte-diff gate; `preview-generated` renders the diff without touching the tree.
5. **TS renderer** — no existing renderer path handles `WindowMeasure` variants generically in React (measures are rendered natively via the wgpu widget tree, `select`-only measure-control React glue exists at `🛠️ShellHelpers/🎚️measure-controls/🟦️.tsx`, 92 lines) — so item 2 (wgpu widgets) is the actual renderer that matters; a React-target renderer would need the equivalent addition wherever the `⚛️react` target's own measure switch lives (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx`) — not inspected line-by-line in this pass, flagged as follow-up scope.
6. **Tests** — new `WindowMeasure::Progress`/`Number` round-trip/layout-wire-format tests beside the existing ones at `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-component-layout-layout-wire-format/🦀️.rs` and `…-value-round-trip/🦀️.rs`; a puzzle3d fixture test replacing the fixed-max assertions at `…/🧪️tests/🔬️unit/🦀️.rs:4481,4649`.
7. **i18n** — any new label text (e.g. a generic "Cancel"/progress-stage fallback string baked into the framework itself, not app-owned) must be `LocalizedLabel::native(en, de)` (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🏷️label/🦀️.rs:123-131`, exhaustive match on `Locale` — adding a locale breaks every call site until translated, by design); a plugin-owned label (e.g. puzzle3d's own progress copy) goes through its `🗣️terminology/🦀️.rs` macro table exactly as `fill_progress`/`fill_cancel`/`fill_planned`/`fill_failed` already do (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs:25-28`).

---

## 3. The interactive-job pattern

### 3.1 Step protocol (`semio_framework_job`, `🧰️framework/🔨️modules/🧵️job/🦀️.rs`)

- **`InteractiveJob` trait** (line 1163-1174): `step(&mut self, cx: &mut StepContext) -> StepOutcome`, `begin_close`, `close_step`, `register_close_wake`, `terminal_is_empty`. One call per slice; job-owned state carries everything between calls (no run-to-completion).
- **`StepOutcome`** (1097-1103): `Yield | PreviewReady(payload) | CheckpointReady(Checkpoint{state,applied_progress}) | Complete(CommitCandidate{state,output}) | Cancelled | Fault(JobFault{detail})`. `is_terminal()` (1108) = `Complete|Cancelled|Fault`.
- **`Checkpoint.applied_progress: u64`** (1067) — "the Puzzle 3D `FillBuilder.applied_count` pattern generalized: how much of `state` is COMMITTED versus merely planned, so a caller can show 'these N are done' without decoding `state`."
- **`StepBudget { fuel: u64, deadline_us: u64 }`** (line 369-380, `deadline_us` is ABSOLUTE wall-clock, "so a job never has to re-derive 'how much time is left'"). Per-lane wall budgets: `INTERACTIVE_LANE_WALL_US=1_000`, `USER_VISIBLE_LANE_WALL_US=2_000`, `BACKGROUND_LANE_WALL_US=4_000`, `MAINTENANCE_LANE_WALL_US=4_000` (lines 386-390).
- **`drive_step()`** (1187-1250) is "the ONE place a returned `StepOutcome` becomes a `semio_framework_trace::record_*` call" — runs under a `Watchdog`, pre-checks cancellation, checks `budget.fuel==0 || now>=deadline` → `Yield` before ever calling `job.step`.
- **Cancellation**: `CancelToken` (`🧰️framework/🔨️modules/⏳️async/🦀️.rs:252`, `Arc<CancelNode>`), checked via `StepContext::is_cancelled()`/`cancel_token()` (job/🦀️.rs ~996-1008).
- **The interactive ceiling is 8 ms, hard**: `INTERACTIVE_STEP_CEILING_US: u64 = 8_000` (`🧰️framework/🔨️modules/⏱️trace/🦀️.rs:99`), enforced by the `Watchdog` inside `drive_step` — "ALWAYS caught — never eyeballed" (job/🦀️.rs:1189). Per-stage **soft** targets sit well under that: `UI_EVENT_SOFT_TARGET_US=1_000`, `UI_PRESENT_SOFT_TARGET_US=2_000`, `INTERACTIVE_STEP_SOFT_TARGET_US=1_000`, `USER_VISIBLE_SIM_STEP_SOFT_TARGET_US=2_000`, `BACKGROUND_STEP_SOFT_TARGET_US=4_000` (trace/🦀️.rs:120-129), selected by `InteractiveStage::{UiEvent,UiPresent,InteractiveStep,UserVisibleSimStep,BackgroundStep}` (line 129-136). A separate, much larger ceiling exists for one-shot app-instance mount/teardown: `GUEST_LIFECYCLE_TURN_CEILING_US = 5_000_000` (5s, line 108-113) — explicitly NOT the same budget as a per-frame interactive step.

### 3.2 Publishing progress out of a step

- **`ProgressEvent`** (job/🦀️.rs:1688-1758) — the ten-event vocabulary described in §2.1(2). "A host assembles these from `StepOutcome`s plus its own domain data … to hand to a UI over a channel governed by `channel_policy_for`/`default_channel_kind_for`" (doc comment, line 1682-1685) — `ProgressChannelKind::{PreviewGeometry,LargeGeometry}` routed by patch byte size vs `LARGE_PREVIEW_PATCH_BYTES` (lines 1805-1849).
- **`StepContext::next_preview_sequence()`** (1022-1027) — a monotonic per-operation sequence number threaded across an entire run, one call per `PreviewReady`/`PreviewPatch` emitted.

### 3.3 Reactor-level bounded jobs (`semio_framework_plugin::reactor::jobs`, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🦀️.rs`)

- **`JobBudget { fuel: u64, deadline_ms: u32 }`** (line 98-101, plain-Rust mirror of `jobs.wit`'s `record job-budget`).
- **`JobStep::{Running(Option<Vec<u8>>), Done(Vec<u8>), Failed(Vec<u8>)}`** (106-110) — the `Option<Vec<u8>>` on `Running` IS the opaque progress-bytes channel: `JobCtx::progress(bytes)` (line 236-237: `async fn progress(&self, bytes) { self.state.borrow_mut().progress = Some(bytes) }`) sets it, `step_job` reads+clears it each slice (~line 489,506-533) and returns it as `JobStep::Running(progress)`. **These bytes are 100% job-defined — no standard shape, no percent/step-count schema at this layer either** — every job invents its own encoding (the doc examples at line 596-621 use ad hoc `b"phase.executed"` tags).
- **`BoundedJob` trait** (122-127): `step(&mut self, budget) -> JobStep`, `cancel(&mut self)`, `checkpoint(&self) -> Option<Vec<u8>>`, `terminal_drop_is_shallow(&self)`.
- **`register_bounded_job_kind(kind, factory: BoundedJobFactory)`** (169-173) — registers into `BOUNDED_KIND_REGISTRY` (thread-local, `component_persistent_local!`), resolved by `start_job`.
- **Stall guard**: if `Running` with no progress bytes AND an unchanged `JobBudget` for `STALL_LIMIT` consecutive `step_job` calls → fails with `job.stalled` (module doc lines 34-38, constant near line 322).
- **`JobPlacement::{Inline, Isolated, Exclusive}`** (`🧰️framework/🔨️modules/🎠️kernel/🦀️.rs:712-719`) — `Inline` shares the instance's own turn budget, `Isolated` gets its own pooled actor, `Exclusive` a dedicated one (flow/brep tessellation per `design-abi.md §5`).
- **`Effect::SpawnJob { job: u64, kind: String, input: Vec<u8>, placement }` / `Effect::CancelJob { job: u64 }`** (kernel/🦀️.rs:632-639).
- **`Effect::DispatchAction { req, action, args, delay_ms: u64 }`** (kernel/🦀️.rs:503-514) — "lets a plugin's `handle_action` advance staged/progressive work … over several ticks without blocking the host," self-chaining (the host feeds a `DispatchAction`'s own follow-up `requestedEffects` back through the same pass). `flowEvalTick` uses `delay_ms: 0` (immediate re-arm, §1.3); nothing in the audited surface uses a nonzero `delay_ms`.
- **`Effect::SetTimer { id: u64, after_ms: u64, repeat: bool }`** (kernel/🦀️.rs, adjacent to `SpawnJob`) — "replaces self-tick loops and `pending_effects()` polling." Puzzle3d's `fillBuildTick`/`suggestionsTick` cadence is documented as **120 ms** in prose (`…/✏️editor/🦀️.rs:1635,2680`) but the exact `SetTimer{after_ms:120,...}` call site was not located inside the audited files in this pass (likely client/host-side scheduling, not the command body itself) — flagged as a follow-up lookup, not confirmed.

---

## 4. Unbounded numeric measures — direct answer

**No, `WindowMeasure` has no unbounded numeric-entry variant today.** Summary of every numeric-ish
schema shape and its bound semantics, cross-checked against the TS side that actually consumes `max`:

| Type | Where | min/max | TS behavior on missing bound |
|---|---|---|---|
| `WindowMeasure::Slider` | wgpu `🧩️component/🦀️.rs:1050` | `min: f64, max: f64` **mandatory** | N/A — cannot be omitted; Rust won't compile without them |
| `WindowEngagementControl::Slider` | 📽️projection/🦀️.rs:1739 | `min: number, max: number` in the type, but wire-optional | `ShellHelpers/🟦️.tsx:1703` **throws** `"engagement slider requires explicit minimum and maximum"` if either is `undefined` |
| `WindowEngagementControl::Stepper` | same | `min?: number, max?: number` genuinely optional | `ShellHelpers/🟦️.tsx:1706` — no guard, falls through fine with either/both absent |
| `UiNumberStepperNode` (form/document control) | 📽️projection/🦀️.rs:1515-1517 | **no min/max field exists** | `Interpreter/🟦️.tsx` `NumberStepperView` renders freely; wgpu `WidgetNode::NumberStepper` (`🪀️widgets/🦀️.rs:167`) likewise has no bound fields |
| `ActionArgControl::Number` / `ArgSchema::Number` | 📽️projection/🦀️.rs:29,314 | `min?, max?, step?` optional | staged-form-only, not a live measure |
| `Table` cell `"stepper"` | `📊️Table/🟦️.tsx:25` | `min, max` mandatory | unrelated context (spreadsheet cell) |

So the genuinely-already-unbounded building block is the `NumberStepper` widget
(`value, step, uniform, onAbsolute/onDelta` — absolute-set or relative-nudge, no ceiling at all),
and the nearest *soft-bound-capable* schema sibling is `WindowEngagementControl::Stepper`. Neither
is reachable as a `WindowMeasure`. The recommendation in §2.2 (`WindowMeasure::Number` with
`min: Option<f64>, max: Option<f64>`) directly closes this gap by giving the Measures overlay the
same optional-bound shape the engagement bar already has, and the puzzle3d fill count becomes
"unbounded and arbitrarily settable, default 100" by: (a) switching `count_measure()` from `Slider`
to this new `Number` variant with `max: None`, (b) changing `Puzzle3dRuntime::default().fill_count`
from `0` to `100` (`🎚️config/🦀️.rs:227`), and (c) retiring `PUZZLE3D_FILL_COUNT_MAX` as a hard clamp
(it may still inform `ready`'s soft display extent, since `ready` stays `Option<f64>` either way).

---

## 5. Coverage notes / not fully chased (time-boxed)

- The generation3d `👁️preview` window body (`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/…/👁️preview/🦀️.rs`, in the dirty-tree diff at session start) was not read line-by-line — flagged in case it already contains a progress affordance worth reusing instead of inventing `WindowMeasure::Progress` from scratch.
- The exact host/client call site that arms puzzle3d's 120 ms `fillBuildTick`/`suggestionsTick` cadence (a `SetTimer` or equivalent) was not located; only the prose documenting the 120 ms figure was confirmed.
- The `⚛️react` target's own generic measure-rendering switch (if one exists beyond the `select`-only `measure-controls/🟦️.tsx`) was not traced — native wgpu rendering is confirmed as the primary path (§2.2 item 2/5).
- FEM/lowpoly grep hits (`fem/…/🧵️session/🦀️.rs`, `lowpoly/…/🖌️session/🦀️.rs`) were listed via filename search only, not opened — flagged as unconfirmed absence rather than proven absence of any progress affordance in those two plugins.
