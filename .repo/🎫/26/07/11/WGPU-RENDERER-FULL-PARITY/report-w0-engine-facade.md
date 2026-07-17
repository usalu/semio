# W0 — Engine Facade (agent: w0-engine-facade)

Status as of this write: implementation complete, inserted, pure-diff verified (no other regions
touched). `cargo check -p ui_wgpu --features engine` (pid 44977) has been running for ~8 minutes at
0% CPU (sleeping/blocked) — the repo has heavy concurrent cargo activity right now (`ps` shows a
workspace-wide `RUST-WIDE-CLEAN-REFACTOR-CAMPAIGN` gate check plus many other per-crate `cargo check`
processes from other sessions, all sharing the same 198GB `target/` dir), so this is very likely
lock/IO contention rather than a hang in my own build. Per this repo's documented pattern for
concurrent cargo-workspace churn, I am not killing/restarting it and am polling via a background
Monitor rather than busy-waiting. This report will be amended in place once check/test results land —
treat the "Build status" section below as the single source of truth for whether this is actually
green; do not treat the "Golden harness" table as verified until that line is updated with real
`cargo test` output.

## Region / line range added

`ui/wgpu/rs/lib.rs`, new `pub mod engine { // #region engine ... // #endregion engine }`, inserted
between `// #endregion shell` (previously line 12078) and `pub mod widgets {` (previously line
12081). After insertion: **lines 12082–12777** (`git diff --stat` confirms a pure 699-line insertion,
zero lines changed/removed anywhere else in the file).

Sub-regions inside it (matching this file's `#region`/`#endregion` convention):
- `🔖UiWindow` — per-window retained pipeline state (`UiTree` + `LayoutEngine` + `EventRouter` +
  `DrawList` + last-set viewport).
- `🔖Ui` — the façade type itself.
- `#[cfg(test)] mod tests` with `🔖FacadeTests` (unit tests for the façade's own contract) and
  `🔖GoldenHarness` (the DrawList-parity acceptance gate, see below).

No other region in `ui/wgpu/rs/lib.rs` was edited. `arena`, `tree`, `reconcile`, `flex`, `paint`,
`events`, `scene_slots`, `shell`, `draw`, `gpu` were read-only inputs. `widgets`, `input`, `layout`,
`component`, `theme`, `re-exports`, `host` were never touched. This matches this agent's claim in
`region-claims.json` exactly.

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
  `drain_commands` empties that queue. Satisfies the ticket's "or via a separate `drain_commands`"
  alternative without forcing callers to pick one style.
- `needs_frame` is purely dirty-flag-driven — no animation-clock scaffolding exists anywhere in
  `arena`/`tree`/`reconcile`/`flex`/`paint`/`events`/`scene_slots`/`shell` yet, so there was nothing to
  wire an animation deadline to. Building a real animation scheduler was explicitly out of scope.
- Shell/scene_slots wiring is intentionally minimal: `Ui` owns one `Shell` and forwards
  `set_window_layout`/`set_navbar`/`dispatch_shell_event`/`shell()`; `scene_slots::SceneHost` is an
  optional `Box<dyn SceneHost>` the host can register, invoked from `frame` after paint. Known gaps in
  `shell` (drag/drop stubs) and `paint`'s scene/select/tree placeholders are untouched, as instructed.

## Build status

`cargo check -p ui_wgpu --features engine` — **launched, still pending** as of this write (pid 44977,
running under heavy concurrent cargo/target-dir contention from other sessions in this repo; not
killed or restarted, per the guidance to be patient rather than chase transient contention). A
`Monitor` is armed to report the moment it exits, at which point this section will be replaced with
either a clean-compile confirmation or the concrete errors + the fix applied (within the `engine`
region only) + re-check result. `cargo test -p ui_wgpu --features engine` has not been run yet —
it is queued to run immediately after `cargo check` comes back clean.

## Golden DrawList-parity harness — design and expected coverage

Location: `#[cfg(test)] mod tests` inside `engine`, sub-region `🔖GoldenHarness`. (Test *results* below
are the harness's self-described intent, not yet confirmed by a passing `cargo test` run — see Build
status above.)

Approach: each of the 19 `UiNode` variants gets a fixture. `retained_stats` runs it through
`Ui::apply_tree` + `Ui::frame` at a 400×400 viewport; `immediate_stats` hand-converts the same
`UiNode` to a `widgets::WidgetNode<ActionDescriptor>` (via a test-local `to_widget_node` mirroring
`framework/renderer/wgpu/rs/lib.rs`'s private `ui_node_to_widget` — duplicated here since that crate
depends on `ui_wgpu`, never the reverse) and runs it through `widgets::render_widget` at the same
bounds. Both sides reduce to `(ui_instances+overlay, vector_vertices+overlay, raster_instances)`
totals across all `DrawList` layers, compared with `assert_eq!` (not full geometry — an intentional,
documented coarsening per the ticket's tolerance allowance).

For the 12 simple leaf kinds plus `Stack`, each leaf fixture is wrapped as the sole child of a
gap-less/padding-less vertical `Stack` — this is what makes bounds line up exactly between the two
pipelines (`flex::LayoutEngine::compute` forces the *root* to the full viewport and gives a lone
`Stack` child `flex_grow: 1.0`; `layout::layout_vertical`'s `extra_per_child` gives a lone child the
same full bounds on the immediate side), so any instance-count mismatch reflects a real paint-logic
difference, not an artifact of the two layout engines disagreeing about size.

| # | UiNode kind | Test | Intended result |
|---|---|---|---|
| 1 | Stack | `golden_stack` | real retained-vs-immediate equivalence assertion |
| 2 | Text | `golden_text` | real equivalence assertion |
| 3 | Button | `golden_button` | real equivalence assertion |
| 4 | Separator | `golden_separator` | real equivalence assertion |
| 5 | Input | `golden_input` | real equivalence assertion |
| 6 | Select | `golden_select` | real equivalence assertion (closed-state only, both sides — open-popup state is a documented gap in `paint::paint_select` itself, not tested either side) |
| 7 | Toggle | `golden_toggle` | real equivalence assertion |
| 8 | Vec3 | `golden_vec3` | real equivalence assertion |
| 9 | KeyValue | `golden_key_value` | real equivalence assertion |
| 10 | Slider | `golden_slider` | real equivalence assertion |
| 11 | NumberStepper | `golden_number_stepper` | real equivalence assertion |
| 12 | Ring | `golden_ring` | real equivalence assertion |
| 13 | IconSelect | `golden_icon_select` | real equivalence assertion |
| 14 | Tree | `golden_tree` | real equivalence assertion (flat, unselected, no-icon items chosen specifically to sidestep the immediate path's scroll-region/guide-line/collapsed-state extras that the retained `paint_tree_widget` doesn't implement) |
| 15 | Field | `golden_field_known_gap` | **KNOWN GAP** — retained-only sanity check (non-empty output), no cross-pipeline assertion |
| 16 | Section | `golden_section_known_gap` | **KNOWN GAP** — retained-only sanity check |
| 17 | Image | `golden_image_known_gap` | **KNOWN GAP** — retained-only sanity check |
| 18 | ComponentScene | `golden_component_scene_known_gap` | **KNOWN GAP** — retained-only sanity check |
| 19 | ExternalSlot | `golden_external_slot_known_gap` | **KNOWN GAP** — retained-only sanity check |

**KNOWN GAP — Field/Section (#15, #16):** `reconcile` only expands `Field`/`Section` into a real
retained child for their `child`/`children` payload (per `reconcile`'s own module doc comment), but
`flex::LayoutEngine::style_with_grow` only grants `flex_grow: 1.0` to a `Stack`'s children (gated on
`matches!(node.spec.0, UiNode::Stack(_))`). A `Field`/`Section`'s synthetic retained child is therefore
laid out at its own intrinsic content size instead of filling the label-adjusted remainder that
`widgets::render_widget`'s hand-rolled `Field`/`Section` branches explicitly carve out
(`Rect::new(bounds.x, bounds.y + label_h + gap, bounds.w, bounds.h - label_h - gap)` for `Field`;
accumulated per-child `y` for `Section`). The two pipelines' geometry can genuinely diverge here —
this is real follow-up work for `flex` (teaching it about non-`Stack` containers that also want
grow/fill semantics), not something this façade should paper over by fudging the harness.

**KNOWN GAP — Image/ComponentScene/ExternalSlot (#17–19):** `widgets::WidgetNode<E>` has **no
variant at all** for these three kinds. `framework/renderer/wgpu/rs/lib.rs`'s own `ui_node_to_widget`
collapses all three to an empty placeholder `WidgetNode::Text`, which is not a like-for-like
rendering of the same node — comparing against that placeholder would just be asserting "an empty
Text box roughly matches an empty Text box," not real parity. There is no immediate-mode output to
honestly compare `paint::paint_image`/`paint_component_scene`/`paint_external_slot` against yet;
those three retained functions are themselves documented placeholders (no host texture-upload queue /
`SceneHost` wiring / plugin-body host exists in `ui_wgpu` today either).

## Wiring requests for a later Integrator

None required to make this façade compile and be internally consistent — it only reads existing
`pub`/`pub(crate)` items from `arena`/`tree`/`reconcile`/`flex`/`paint`/`events`/`scene_slots`/`shell`/
`draw`, and does not touch `Cargo.toml`, `ShellState`, or `render_ui_node`. No new Cargo dependency
was needed; the `engine` feature's existing deps (`taffy`, `fontdue`, `wgpu`, `winit`, `bytemuck`,
`pollster`) were sufficient. When a later workstream wires this façade into the actual renderer, it
will need (not done here — a choke-point file per `region-claims.json`):
- A `render_ui_node`/host-loop dispatch path in `framework/renderer/wgpu/rs/lib.rs` that constructs
  one `ui_wgpu::engine::Ui`, calls `apply_tree`/`frame`/`dispatch_event`/`needs_frame`, and feeds the
  returned `&DrawList` into the existing `GpuContext::render_frame` call site.

## Files touched

- `ui/wgpu/rs/lib.rs` — added the `engine` region (lines 12082–12777 after insertion). No other file
  created, moved, or edited.
