# WP-10 Direct manipulation and live lanes (A10)

Status: implemented, NOT compiled or tested (build queue blocked; coordinator told A10 to stand down). Compile request is in `📌️important/requests.md`.

## Delivered (SURFACE = `.../🖥️app-surface`)
- `🕹️direct-manipulation/🦀️.rs`: mounted as `pub mod direct_manipulation` (mount applied by A10, noted in mounts.md; A7's `🧭️domain` mount untouched).
  - `🧲️snap`: `snap(handle, raw, Modifiers) -> Snapped{s, source}` (Free/Target/Catalogue/Increment/Grid/Limit; Ctrl frees snapping, limits stay), equals `Handle::resolve_s` without Ctrl; `nudge_step`/`nudge` (Shift x10, Alt x0.1).
  - `🛠️handle-drag-tool`: pure statechart `HandleDragTool::step(DragState, DragEvent) -> (DragState, DragEffect)`: Idle, Hover, Armed, Dragging; effects Preview(DragPreview) / Commit(Vec<FieldWrite>) / Cleared. Release at origin commits nothing; Escape returns to Hover, capture loss to Idle with zero trace; typed numeric entry (`.` or `,`) overrides exactly (clamped to limits); `DragPhase::parse` mirrors the host stream/commit/abort+reason vocabulary. No dependency on the fem `GestureTool` crate was added; adapter is `DragPhase::event`.
  - `⌨️keyboard`: `key_intent` (Tab/Shift+Tab cycle, arrows nudge, digits, Backspace, Enter commit, Esc abort), `cycle_focus`, `focus_announcement`.
  - `🎯️pick-targets`: `PickTarget` (`id#face:3` etc., round-trip), `resolve_pick` (grip wins), `handles_for(selection)` stable Tab order, `PickTarget::owns`.
- `🫧️live-lanes/🧵️lane-runtime/🦀️.rs`: `LaneEvent` + `NormTransient::apply` (pure fold), `overlay_document`, `Coalescer`, `PresencePublisher` (34 ms gate = <=30 Hz, changes only, drops while `Link::Lost`, immediate resend on reconnect, `offline_too_long` after 30 s is a notice only), `EvalScheduler` (T1 60 ms, T2 250 ms, commit immediate), `LaneRuntime`.
- Commands: `NORM_LIVE_TOOL_IDS` = setFields, previewRemedy, clearPreview, cancelEvaluation, evaluateUnits; `LiveCommand::parse`, `dispatch_live_command` -> `LiveOutcome::{Mutations(Emit), Lanes(Vec<LaneEvent>)}`; `handle_set_fields`/`resolve_set_fields`/`validate_writes`/`coerce_like`/`preview_remedy` in edit-rules (all-or-nothing; one Emit = one undo step; ghost equals applyRemedy write); `NORM_LIVE_PUBLICATION_CONTRACTS` + `norm_live_extent` in retained-commands.
- Deliberate: `NORM_RETAINED_TOOL_IDS` and `NORM_PUBLICATION_CONTRACTS` unchanged (every artifact's retained-disposition JSON and `app_commands!` rows are pinned to them). Artifact owners append the five live rows and use `NORM_LIVE_*` during rollout.

## Tests
- Unit: `🕹️direct-manipulation/🧪️tests/🔬️unit`, `🫧️live-lanes/🧵️lane-runtime/🧪️tests/🔬️unit`, `⌨️command-args-and-dispatch/🧪️tests/🔬️unit`.
- Language-agnostic: `🥒️.feature` in each of the three tests dirs.
- Third-party oracle: `🕹️direct-manipulation/🔮️oracles/🐍️.py` (numpy) vs Rust snapping; dev-dep with `compliance-testing` added to SURFACE `Cargo.toml`.

## Not done / open
- Nothing compiled or run; expect small compile fixes (see requests.md).
- Insert-tool ghost following the cursor and dimension-line inline entry not implemented (need A7 overlay hooks).
- Wiring `LaneRuntime` into each artifact's `Transient`/`Presence` types is rollout work.
