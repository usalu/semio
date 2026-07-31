---
name: Flow Node Rendering Cleanup
overview: Refactor flow node rendering so every widget maps to a proper native DAG node kind (removing the duplicate overlay), fix vertical label centering and handle/label overlap, and make node detail reveal progressively by LOD.
todos:
 - id: dag-kinds
   content: Add Note/Preview/Action variants to DagNodeKind and update inputs()/outputs() accessors in mathematical/.../dag/lib.rs
   status: completed
 - id: flow-map
   content: Rewrite widget_to_dag_node/widget_io_ports/widget_node_size in flow/core/lib.rs to map each widget to its native DAG kind
   status: completed
 - id: drop-overlay
   content: Delete paint_flow_widget_chrome and flow's duplicate slider hit/adjust pointer logic
   status: completed
 - id: sync-values
   content: Sync slider/select/note values from dag.fixture.nodes back into fixture.widgets in sync_from_dag
   status: completed
 - id: label-extent
   content: Add label_extent helper in cavas text module and fix paint_node_name_vertical centering
   status: completed
 - id: port-overlap
   content: Inset/right-align port labels in paint_port_labels so channel names clear the handles
   status: completed
 - id: lod-ladder
   content: Add LOD name gate and render all node kinds with progressive disclosure in paint_scene
   status: completed
 - id: verify
   content: Extend Rust tests, run cargo test, rebuild dag+flow wasm, verify slider drag/preview at runtime
   status: completed
isProject: false
---

## Problem

Nodes look inconsistent because rendering happens in two layers that disagree:

- [mathematical/graph/port/directed/dag/lib.rs](mathematical/graph/port/directed/dag/lib.rs) draws every node as a rectangle with handles, dividers and a vertical name, and already has clean native kinds `Slider`/`Select`/`Screen` that go unused.
- [flow/core/lib.rs](flow/core/lib.rs) maps EVERY widget (slider, note, preview, action, neuron) to a generic `Computation` rectangle in `widget_to_dag_node`, then paints a second fake layer in `paint_flow_widget_chrome` (slider track, preview text). So a slider is a box + handles + dividers + vertical "Slider" + an overlaid track.

Concrete bugs found:

- `paint_node_name_vertical` appends text at origin then rotates about the node center, so text is not centered (unlike `paint_node_name_horizontal`, which offsets by half its extent).
- `paint_port_labels` places channel names at `node.x ± hw ∓ 8/zoom`, exactly where the handle circles are drawn, so names sit under handles.

## Approach: one node = one native kind, LOD-driven detail

### 1. Extend native DAG node kinds — `mathematical/graph/port/directed/dag/lib.rs`

In `enum DagNodeKind` add the kinds flow needs so nothing maps to `Computation` unless it truly is one:

- `Note { text: String, output: IoPortSpec }`
- `Preview { text: String, input: IoPortSpec }`
- `Action { label: String, input: IoPortSpec }`
  (`Slider`, `Select`, `Screen`, `Computation` already exist.)

Update `inputs()` / `outputs()` accessors so `Note` exposes its output, `Preview`/`Action` expose their input. These feed `rebuild_engine_with_layout` handle creation automatically.

### 2. Map flow widgets to native kinds — `flow/core/lib.rs`

Rewrite `widget_to_dag_node`, `widget_io_ports`, `widget_node_size`:

- `Neuron` -> `Computation` (unchanged)
- `InputSlider { value }` -> `Slider { min: FLOW_SLIDER_MIN, max: FLOW_SLIDER_MAX, step: FLOW_SLIDER_STEP, value, output }`
- `InputNote { text }` -> `Note { text, output }`
- `OutputPreview { preview }` -> `Preview { text: format_dictionary_preview(preview), input }`
- `OutputAction { action }` -> `Action { label: action, input }`

### 3. Delete the overlay and duplicate slider interaction — `flow/core/lib.rs`

- Remove `paint_flow_widget_chrome` and its call in `paint_scene` (keep `self.dag.paint_scene(...)` only).
- Remove flow's `hit_slider_widget_at`, `adjust_slider_at_world`, `slider_adjust_id` and the slider branch in `pointer_down_screen` / `pointer_move_screen` / `pointer_up_screen`; the native DAG slider drag (`try_widget_pointer_down`, `set_slider_value_from_x`, `widget_drag`) handles it.

### 4. Sync live control values back — `flow/core/lib.rs`

In `sync_from_dag`, read mutated values out of `self.dag.fixture.nodes` into `self.fixture.widgets` so evaluation seeds update: `Slider.value -> Widget::InputSlider.value`, `Select.selected`, `Note.text`. This replaces the value path the overlay used to own.

### 5. Fix vertical label centering — `mathematical/.../dag/lib.rs`

Add a shared text-extent helper `label_extent(text, px) -> (w, h)` in the cavas text module ([infinite/cavas/vello/lib.rs](infinite/cavas/vello/lib.rs)) mirroring the box math already inside `append_label`. Use it in both name painters. For vertical, center the box on the origin before rotating:

```rust
let (w, h) = label_extent(name, px * 1.05);
let rot = Affine::translate((center_screen.x, center_screen.y))
    * Affine::rotate(-std::f64::consts::FRAC_PI_2)
    * Affine::translate((-w * 0.5, -h * 0.5));
```

### 6. Fix channel names hidden behind handles — `mathematical/.../dag/lib.rs`

In `paint_port_labels`, move labels off the edge handles: inset input labels toward center past the handle radius, and right-align output labels by subtracting their measured width (using `label_extent`) so text reads inward instead of sitting on the handle dots.

### 7. LOD-driven progressive disclosure — `mathematical/.../dag/lib.rs`

Per the requested behavior (low detail = just a draggable control; zoom in to reveal name, then value, then ports), render each kind through the existing `DagDrawLod` gates plus a new name gate, e.g.:

- Minimap: fill only.
- Overview / Compact: horizontal node name only.
- Normal: control shape (slider track+thumb, note/preview frame, action button) + handles + dividers; slider draggable; no value text, no vertical name.
- Detail: + centered name + value/text.
- Micro: + channel (port) labels.

Add a `shows_name`-style helper on `DagDrawLod` and apply it uniformly across `Computation`, `Slider`, `Note`, `Preview`, `Action` in `paint_scene` so all kinds reveal detail on the same ladder.

## Build, verify, ticket

- Rust unit tests: `cargo test` for the dag and `@semio-tech/flow-core` crates (extend the existing `#[cfg(test)] mod tests` in both `lib.rs` files for new kinds, mapping, centering helper, and sync-back; do not add new test files).
- Rebuild wasm so the playground reflects changes: `@semio-tech/dag-core:wasm` and `@semio-tech/flow-core:wasm` (both run `bun ./📜️script.ts wasm`), then confirm visually in the procedural play canvas; verify slider drag still updates the preview at runtime via the existing `[DEBUG]` logs.
- Per repo rules, do this inside a ticket: read `repo://goals`, then open a new ticket (e.g. "Flow Node Rendering Cleanup") under the most fitting goal and close it with the touched files when done. The existing `PROCEDURAL-BREP-PLAYGROUND` ticket is a different task.

## Out of scope

Inline text editing for `Note` (no inline editor exists today; current `setNoteText` API is kept). Rendering of `Note` is display-only.
