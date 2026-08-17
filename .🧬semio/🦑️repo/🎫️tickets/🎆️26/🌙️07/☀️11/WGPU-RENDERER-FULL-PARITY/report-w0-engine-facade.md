# W0 — Engine Facade (agent: w0-engine-facade)

**Status: DONE.** `cargo check -p ui_wgpu --features engine` is clean (0 errors, 11 pre-existing
warnings, none from my code). `cargo test -p ui_wgpu --features engine`: **99 passed, 1 failed** —
the 1 failure (`component::ui::ui_node_wire_format_tests::scene_records_serialize_to_golden_json`) is
in a region I never touched and don't own (see "Foreign test failure" below); every test in my own
`engine` region passes (24/24: 4 facade unit tests + 20 golden-harness tests covering all 19 `UiNode`
kinds).

## Region / line range added

`ui/wgpu/rs/lib.rs`, new `pub mod engine { // #region engine ... // #endregion engine }`, inserted
between the `shell` region and the `widgets` region. Current location (line numbers shift as other
concurrent sessions edit the shared file — this is expected in this repo; re-grep before trusting
these): **lines 12021–12750**.

Sub-regions inside it (this file's `#region`/`#endregion` convention):
- `🔖️UiWindow` — per-window retained pipeline state (`UiTree` + `LayoutEngine` + `EventRouter` +
  `DrawList` + last-set viewport).
- `🔖️Ui` — the façade type itself.
- `#[cfg(test)] mod tests` with `🔖️FacadeTests` (unit tests for the façade's own contract) and
  `🔖️GoldenHarness` (the DrawList-parity acceptance gate).

No other region in `ui/wgpu/rs/lib.rs` was edited by me. `arena`, `tree`, `reconcile`, `flex`,
`paint`, `events`, `scene_slots`, `shell`, `draw`, `gpu`, `input` were read-only inputs (the `input`
import was added because `InputState<E>` actually lives in `input`, not `widgets`, which only
privately `use`s it — see "Issues found and fixed" below). `widgets`, `layout`, `component`, `theme`,
`re-exports`, `host` were never touched. Matches this agent's claim in `region-claims.json` exactly.

## `Ui` facade public API

```rust
pub struct Ui { /* private: windows: HashMap<String, UiWindow>, shell: Shell, theme: Theme,
                   atlas: FontAtlas, icons: Option<IconAtlas>,
                   scene_host: Option<Box<dyn SceneHost>>, pending_commands: Vec<UiCommand> */ }

impl Ui {
    pub fn new() -> Self;
    pub fn set_theme(&mut self, theme: Theme);
    pub fn set_icons(&mut self, icons: Option<IconAtlas>);
    pub fn set_scene_host(&mut self, host: Box<dyn SceneHost>);
    pub fn set_viewport(&mut self, window_id: &str, width: f32, height: f32);
    pub fn apply_tree(&mut self, window_id: &str, ui_node: &UiNode);
    pub fn set_window_layout(&mut self, layout: WindowLayout);
    pub fn set_navbar(&mut self, items: Vec<String>);
    pub fn shell(&self) -> &Shell;
    pub fn needs_frame(&self) -> bool;
    pub fn frame(&mut self, window_id: &str, viewport_width: f32, viewport_height: f32) -> Option<&DrawList>;
    pub fn draw_list(&self, window_id: &str) -> Option<&DrawList>;
    pub fn dispatch_event(&mut self, window_id: &str, event: UiEvent) -> Vec<UiCommand>;
    pub fn dispatch_shell_event(&mut self, event: &UiEvent) -> Vec<ShellEvent>;
    pub fn drain_commands(&mut self) -> Vec<UiCommand>;
}
impl Default for Ui { fn default() -> Self { Self::new() } }
```

Design notes:
- One `UiWindow` (tree + its own taffy `LayoutEngine` + its own `EventRouter` + its own `DrawList`)
  per `window_id`, per `tree` module's own doc comment ("the engine facade... holds
  `HashMap<window_id, UiTree>`"). Window-chrome (dock/split/tab) is the separate shared `Shell`, per
  `shell`'s own doc comment that models it as independent of any window's content tree.
- `frame` is dirty-gated on the root's `DIRTY_LAYOUT`/`DIRTY_PAINT`/`SUBTREE_DIRTY` flags (the same
  flags `needs_frame` reads), runs `flex::LayoutEngine::compute` then `paint::paint_tree` then hands
  every `scene_slots::collect_scene_slots` leaf to an optional registered `SceneHost`. It returns
  `&DrawList` and never touches the GPU itself — submitting that `DrawList` via
  `gpu::GpuContext::render_frame` is left to the caller, matching the existing immediate-mode path's
  contract and the instruction not to invent a new GPU submission path.
- `dispatch_event` both returns the `UiCommand`s produced immediately *and* queues them internally;
  `drain_commands` empties that queue — satisfies the ticket's "or via a separate `drain_commands`"
  alternative without forcing callers to pick one style.
- `needs_frame` is purely dirty-flag-driven — no animation-clock scaffolding exists anywhere in
  `arena`/`tree`/`reconcile`/`flex`/`paint`/`events`/`scene_slots`/`shell` yet, so there was nothing to
  wire an animation deadline to. Building a real animation scheduler was explicitly out of scope.
- Shell/scene_slots wiring is intentionally minimal: `Ui` owns one `Shell` and forwards
  `set_window_layout`/`set_navbar`/`dispatch_shell_event`/`shell()`; `scene_slots::SceneHost` is an
  optional `Box<dyn SceneHost>` the host can register, invoked from `frame` after paint. Known gaps in
  `shell` (drag/drop stubs) and `paint`'s scene/select/tree placeholders are untouched, as instructed.

## Build/test output (real, from the final green run)

`cargo check -p ui_wgpu --features engine` (isolated `CARGO_TARGET_DIR`, no lock contention):
```
    Checking ui_wgpu v0.1.0 (/Users/ueli/Documents/semio/ui/wgpu/rs)
warning: `ui_wgpu` (lib) generated 11 warnings (run `cargo fix --lib -p ui_wgpu` to apply 8 suggestions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.99s
```
(All 11 warnings are pre-existing, in `flex`/`chrome`/`gpu`/`events` — none touch `engine`.)

`cargo test -p ui_wgpu --features engine` (full crate):
```
test result: FAILED. 99 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```
The 1 failure is `component::ui::ui_node_wire_format_tests::scene_records_serialize_to_golden_json`
— see "Foreign test failure" below.

`cargo test -p ui_wgpu --features engine -- engine::` (my region only):
```
running 24 tests
test engine::tests::frame_before_any_apply_tree_returns_none ... ok
test engine::tests::golden_component_scene_known_gap ... ok
test engine::tests::golden_image_known_gap ... ok
test engine::tests::dispatch_event_emits_a_button_click_command_and_it_is_also_drainable ... ok
test engine::tests::golden_external_slot_known_gap ... ok
test engine::tests::apply_tree_then_frame_produces_a_non_empty_draw_list ... ok
test engine::tests::golden_section_known_gap ... ok
test engine::tests::golden_ring ... ok
test engine::tests::golden_icon_select ... ok
test engine::tests::golden_button ... ok
test engine::tests::golden_select ... ok
test engine::tests::golden_field_known_gap ... ok
test engine::tests::golden_separator ... ok
test engine::tests::golden_input ... ok
test engine::tests::golden_key_value ... ok
test engine::tests::golden_number_stepper_known_gap ... ok
test engine::tests::golden_slider ... ok
test engine::tests::golden_stack ... ok
test engine::tests::golden_toggle ... ok
test engine::tests::set_window_layout_wires_into_the_facades_shell ... ok
test engine::tests::needs_frame_is_false_once_a_stable_tree_has_been_framed ... ok
test engine::tests::golden_vec3 ... ok
test engine::tests::golden_tree ... ok
test engine::tests::golden_text ... ok

test result: ok. 24 passed; 0 failed; 0 ignored; 0 measured; 76 filtered out; finished in 0.00s
```

## Issues found while getting to green (and fixes applied, all within `engine`)

1. **Duplicate `fn action()`** — I had defined a test helper `fn action() -> ActionDescriptor` twice
   in `mod tests` (once under `🔖️FacadeTests`, once under `🔖️GoldenHarness` — both are just comment
   markers, not real Rust modules, so they share one scope). Fixed by deleting the second definition
   and leaving a one-line comment noting it's shared.
2. **Match-guard shadowing** — `matches!(cmd, UiCommand::App { action, .. } if *action == action())`
   bound a local `action` from the pattern that shadowed the `action()` helper fn inside its own guard
   expression. Fixed by renaming the binding to `action: fired_action` and comparing
   `*fired_action == action()`.
3. **Wrong conversion function for `TreeItem::control`** — `TreeItem<E>::control` is
   `Option<Box<WidgetNode<E>>>` (a full nested widget), not `Option<Box<ControlNode<E>>>`. I had
   reused `control_to_widget` (which produces `ControlNode<E>`, correct for `Field`'s `child`) for
   this field too. Added a second helper, `control_to_widget_node`, mirroring
   `framework/renderer/wgpu/rs/lib.rs`'s private `control_to_widget_node` (same per-variant mapping,
   just into `WidgetNode` variants instead of `ControlNode` variants), and pointed
   `tree_item_to_widget`'s `control:` field at it.
4. **`InputState` imported from the wrong module** — I wrote
   `use crate::widgets::{..., InputState, ...}`, but `InputState<E>` is actually defined in `input`
   (`crate::input::InputState`) and only privately `use`d (not `pub use`d) into `widgets` for that
   region's own internal use — so `crate::widgets::InputState` isn't a valid public path. Fixed by
   importing it from `crate::input::InputState` directly.
5. **Real golden-harness gap: NumberStepper** — `golden_number_stepper` failed with retained=14 vs
   immediate=19 instances. Root cause (confirmed by reading both paint paths, not guessed):
   `widgets::render_number_stepper` renders its center value via a full `render_input` call, which
   itself calls `push_control_border` (a 5-instance background+4-border-edge box) around the value —
   giving the center segment its own nested input-style border. `paint::paint_number_stepper` instead
   just `draw_text_on`s the formatted value with no surrounding border (14 = 1 `push_control_border`
   for the whole control + 2 divider lines + 3 text runs; 19 = the same 14 plus the center value's
   nested 5-instance border box). This is a real, reproducible paint-logic difference between the two
   pipelines, not a fixture artifact — converted `golden_number_stepper` to
   `golden_number_stepper_known_gap` (retained-only sanity check) with a doc comment naming the exact
   root cause, matching this ticket's "document, don't paper over" instruction.

Two more compile errors in the crate (a stale `raster: None` field that should've been `paint_2d:
None` in two pre-existing test fixtures around `component::ui`, unrelated to my work and outside my
region) were already fixed upstream before I started this pass — confirmed not present in my region
and not touched by me.

## Foreign test failure (not mine, not fixed, not touched)

`component::ui::ui_node_wire_format_tests::scene_records_serialize_to_golden_json` fails because the
golden JSON literal it asserts against doesn't include `GisMapScene`'s `layerVisibilityJson`/
`layerStrokeScaleJson`/`selectionJson` default values that the actual `base()` constructor now
produces — a live mismatch between a `component::ui` fixture/golden-string pair, entirely inside the
`component` region I am explicitly barred from touching (`region-claims.json`:
`must_not_touch: [..., "component", ...]`). This crate-wide diff never touched `component` (verified:
my only edits are inside `engine`, all insertions/replacements between the `shell` and `widgets`
region markers). This is very likely fallout from concurrent work on `GisMapScene`/raster surfaces
happening elsewhere in the repo right now (there's an open
`.repo/🎫️/26/07/17/GENERALIZE-RASTER-SURFACE-KIND-TO-PAINT2D` ticket touching related types). Flagging
for whichever agent owns `component::ui` — not actionable by this workstream.

## Golden DrawList-parity harness — final pass/fail table

Location: `#[cfg(test)] mod tests` inside `engine`, sub-region `🔖️GoldenHarness`. Approach: each of
the 19 `UiNode` variants gets a fixture; `retained_stats` runs it through `Ui::apply_tree` +
`Ui::frame` at a 400×400 viewport; `immediate_stats` hand-converts the same `UiNode` to a
`widgets::WidgetNode<ActionDescriptor>` (test-local `to_widget_node`, mirroring
`framework/renderer/wgpu/rs/lib.rs`'s private `ui_node_to_widget`/`control_to_widget_node` — duplicated
here since that crate depends on `ui_wgpu`, never the reverse) and runs it through
`widgets::render_widget` at the same bounds. Both sides reduce to `(ui_instances+overlay,
vector_vertices+overlay, raster_instances)` totals across all `DrawList` layers, compared with
`assert_eq!`. Simple leaf kinds are wrapped as the sole child of a gap-less/padding-less vertical
`Stack` so both pipelines resolve to identical bounds (root always forced to full viewport by
`flex::LayoutEngine::compute`; a lone `Stack` child gets `flex_grow: 1.0` on the retained side and the
same full-bounds result from `layout::layout_vertical`'s `extra_per_child` on the immediate side).

| # | UiNode kind | Test | Result |
|---|---|---|---|
| 1 | Stack | `golden_stack` | **PASS** — real equivalence |
| 2 | Text | `golden_text` | **PASS** — real equivalence |
| 3 | Button | `golden_button` | **PASS** — real equivalence |
| 4 | Separator | `golden_separator` | **PASS** — real equivalence |
| 5 | Input | `golden_input` | **PASS** — real equivalence |
| 6 | Select | `golden_select` | **PASS** — real equivalence (closed-state only; open-popup is a documented gap in `paint::paint_select` itself) |
| 7 | Toggle | `golden_toggle` | **PASS** — real equivalence |
| 8 | Vec3 | `golden_vec3` | **PASS** — real equivalence |
| 9 | KeyValue | `golden_key_value` | **PASS** — real equivalence |
| 10 | Slider | `golden_slider` | **PASS** — real equivalence |
| 11 | NumberStepper | `golden_number_stepper_known_gap` | **KNOWN GAP** (confirmed, root cause identified — see above) |
| 12 | Ring | `golden_ring` | **PASS** — real equivalence |
| 13 | IconSelect | `golden_icon_select` | **PASS** — real equivalence |
| 14 | Field | `golden_field_known_gap` | **KNOWN GAP** — retained-only sanity check |
| 15 | Section | `golden_section_known_gap` | **KNOWN GAP** — retained-only sanity check |
| 16 | Tree | `golden_tree` | **PASS** — real equivalence (flat, unselected, no-icon items chosen to sidestep the immediate path's scroll-region/guide-line/collapsed-state extras) |
| 17 | Image | `golden_image_known_gap` | **KNOWN GAP** — retained-only sanity check |
| 18 | ComponentScene | `golden_component_scene_known_gap` | **KNOWN GAP** — retained-only sanity check |
| 19 | ExternalSlot | `golden_external_slot_known_gap` | **KNOWN GAP** — retained-only sanity check |

13/19 kinds have real, passing cross-pipeline equivalence assertions. 6/19 are documented KNOWN GAPs,
each with a specific, code-cited root cause (not a vague placeholder):

- **Field, Section:** `reconcile` expands their `child`/`children` into real retained children, but
  `flex::LayoutEngine::style_with_grow` only grants `flex_grow: 1.0` to a `Stack`'s children (gated on
  `matches!(node.spec.0, UiNode::Stack(_))`), so a `Field`/`Section`'s synthetic child sizes to its own
  intrinsic content instead of filling the label-adjusted remainder
  `widgets::render_widget`'s hand-rolled `Field`/`Section` branches explicitly carve out. Real
  follow-up work for `flex`.
- **NumberStepper:** `widgets::render_number_stepper` nests a full bordered `render_input` around its
  center value; `paint::paint_number_stepper` paints that value as bare text. Confirmed exact
  instance-count delta (19 vs 14 = the nested border's 5 instances). Real follow-up work for `paint`
  (or a product decision to drop the nested border immediate-side).
- **Image, ComponentScene, ExternalSlot:** `widgets::WidgetNode<E>` has no variant for any of these
  three at all — there is no immediate-mode output to honestly compare against (the renderer's own
  `ui_node_to_widget` collapses all three to an empty placeholder `Text`, which would just be
  asserting a fake match). `paint::paint_image`/`paint_component_scene`/`paint_external_slot` are
  themselves documented placeholders pending host texture-upload/`SceneHost`/plugin-body wiring that
  doesn't exist in `ui_wgpu` yet.

## Wiring requests for a later Integrator

None required to make this façade compile and be internally consistent — it only reads existing
`pub`/`pub(crate)` items from `arena`/`tree`/`reconcile`/`flex`/`paint`/`events`/`scene_slots`/`shell`/
`draw`/`input`, and does not touch `Cargo.toml`, `ShellState`, or `render_ui_node`. No new Cargo
dependency was needed; the `engine` feature's existing deps (`taffy`, `fontdue`, `wgpu`, `winit`,
`bytemuck`, `pollster`) were sufficient. When a later workstream wires this façade into the actual
renderer, it will need (not done here — a choke-point file per `region-claims.json`):
- A `render_ui_node`/host-loop dispatch path in `framework/renderer/wgpu/rs/lib.rs` that constructs
  one `ui_wgpu::engine::Ui`, calls `apply_tree`/`frame`/`dispatch_event`/`needs_frame`, and feeds the
  returned `&DrawList` into the existing `GpuContext::render_frame` call site.

## Files touched

- `ui/wgpu/rs/lib.rs` — added the `engine` region (currently lines 12021–12750; re-grep before
  trusting exact numbers, the file is edited concurrently by other sessions). No other file created,
  moved, or edited.
