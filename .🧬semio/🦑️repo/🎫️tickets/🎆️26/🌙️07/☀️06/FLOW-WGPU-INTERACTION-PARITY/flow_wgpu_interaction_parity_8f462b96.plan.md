---
name: Flow Wgpu Interaction Parity
overview: "Close the remaining premigration parity gaps in the wgpu Flow/DAG node-graph renderer: invisible labels/selection overlay, wrong zoom/pan/drag semantics, missing double-click note editing with live text + caret, a dead evaluate/preview pipeline, and missing drag-and-drop ghost preview from the catalogue."
todos:
 - id: ticket
   content: Reopen/open ticket for interaction parity work and record scope in important.md
   status: completed
 - id: zorder
   content: Add overlay draw buffers + post-raster ui_overlay_pass in ui/wgpu/rs/lib.rs; repoint label/selection overlay painting to use it
   status: completed
 - id: zoom-pan
   content: Fix wheel-always-zooms, add ctrl_or_meta helper, add Dag pan gesture (middle/alt/space+drag), wire wheel-active LOD pin
   status: completed
 - id: note-edit
   content: Wire double-click note editing, keyboard forwarding into note_insert_text/backspace/caret, and caret blink
   status: completed
 - id: preview-pipeline
   content: Enable evaluate on wasm, stop camera-only fixture resync from wiping OutputPreview state, evaluate on mount
   status: completed
 - id: dnd-ghost
   content: Make Flow catalogue rows draggable, wire ghost widget preview during drag over node-graph, handle drop to addWidget at cursor
   status: completed
 - id: tests
   content: Extend existing test modules (flow/core, dag, ui-wgpu, framework-renderer-wgpu, flow-plugin) with regression coverage for each fix
   status: completed
 - id: verify
   content: Rebuild wasm bundles, manually verify all premigration parity behaviors, run cargo test across affected crates
   status: completed
 - id: close-ticket
   content: Update important.md and close ticket with full file list
   status: completed
isProject: false
---

# Flow Wgpu Interaction Parity

## Root causes (confirmed by code inspection)

1. **Labels/selection overlay are invisible** because of compositor z-order, not missing data. `render_node_graph` calls, in order:

```5628:5630:framework/renderer/wgpu/rs/lib.rs
    engine_canvas::paint_node_graph(gpu, ctx, scene, inner);
    engine_canvas::paint_node_graph_labels(ctx, scene, inner);
    engine_canvas::paint_node_graph_overlays(ctx, scene, inner);
```

but `ui/wgpu/rs/lib.rs::render_scene_content` always runs a `ui_pass` (draws all `ui_instances`/`vector_vertices`, i.e. glyphs + selection lines) **before** a separate `ui_raster_pass` with `LoadOp::Load` that paints `raster_instances` (the Vello node-graph texture) on top — regardless of push order within the frame. So node/port text and the selection marquee/bounds are drawn, then fully painted over by the opaque graph raster quad every frame. This affects the standalone `flow` surface and the `procedural2d`/`procedural3d` surfaces identically (no surface-specific gate exists in `paint_node_graph_labels`).

2. **Zoom/pan/drag differ from premigration** (`git show premigration:flow/react/index.tsx`, `premigration:mathematical/graph/port/directed/dag/react/index.tsx`):
   - Premigration: wheel **always zooms** (Flow and DAG). Current wgpu Flow only zooms when Ctrl is held, otherwise pans (`node_graph_wheel` passes `ctrl` as `zoom_gesture` into `FlowHost::wheel_screen`).
   - Premigration DAG-react supported pan via middle-mouse, Alt+drag, or Space+drag. Current wgpu only wires Flow's middle-mouse pan (`pointer_down_screen(..., pan: button == 1)`); the generic `DagHost::pointer_down_screen` (`mathematical/graph/port/directed/dag/rs/lib.rs:3404`) has no `pan` parameter at all (unlike the vendored copy in `flow/core/rs/lib.rs:3848` which already has `pan: bool`).
   - `modifiers.ctrl` is used everywhere `node_graph_pointer_{down,move,up}`/`node_graph_wheel` are called (`framework/renderer/wgpu/rs/lib.rs:12212,12280,12291,12313,12399,11957`); Cmd/Meta is never folded in, breaking macOS additive/subtractive selection.
   - `setWheelZoomActive`-equivalent (LOD pin during a zoom gesture) is never called from wgpu, unlike premigration.

3. **Double-click note editing and live typing do not exist in the wgpu event path.** `FlowHost::begin_note_edit`/`note_insert_text`/`note_backspace`/`note_move_caret`/`note_commit_edit` (`flow/core/rs/lib.rs`) and `DagHost` caret painting (`mathematical/graph/port/directed/dag/rs/lib.rs`, `paint_note_caret_bar`) are fully implemented but **nothing calls them**: there's no double-click detection in the live pointer path (`node_graph_pointer_down/up`), and no keyboard route forwards typed characters into note editing (the keyboard entry point `ShellState::handle_keyboard` only handles shell shortcuts/palette/generic text-editor focus).

4. **Live evaluate → `OutputPreview` pipeline is broken on wasm.** Preview painting itself works (`DagNodeKind::Preview` → `paint_preview_content` in `mathematical/graph/port/directed/dag/rs/lib.rs`), but nothing feeds it:

```2445:2448:flow/core/rs/lib.rs
    fn touch_channel_eval(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        self.evaluate_internal();
    }
```

Evaluation is a no-op on `wasm32` (the target the wgpu renderer actually ships as), and `sync_flow_host` calls `host.replace_fixture(...)` (which clears `last_eval_json`) whenever `fixture_json` changes — including on every `nodeGraphViewport` camera update — so even manually-triggered evaluations would get wiped on the next pan/zoom.

5. **Catalogue drag-and-drop has no ghost preview.** wgpu has a generic tree-drag mechanism (`pending_tree_drag`/`TreeDragState`, `finish_tree_drag`, ~`framework/renderer/wgpu/rs/lib.rs:7795-8650`) but Flow catalogue rows are built as click-only (`draggable: None, drag_data: None` in `flow/plugin/rs/lib.rs`), and `finish_tree_drag` only recognizes `HitKind::World3d`/`HitKind::Window` drop targets (used by the S-play spawn-app feature), never the node-graph surface. Meanwhile `FlowHost::set_ghost_widget`/`clear_ghost_widget` and `DagNodePaintChrome::ghost_preview()` painting already exist and are covered by existing `flow/core` tests — only the drag-gesture-to-ghost wiring is missing.

## Fix plan

### 1. Compositor z-order (`ui/wgpu/rs/lib.rs`) — foundational, fixes labels AND selection overlay

- Add `overlay_ui_instances: Vec<UiInstance>` and `overlay_vector_vertices: Vec<VectorVertex>` to `DrawLayer` (update `Default for DrawLayer` and the four literal construction sites in `push_scissor`/`pop_scissor`/`begin_glass_content`/`end_glass_content`).
- Add `DrawList::push_glyph_overlay`, `push_line_overlay`, `push_solid_overlay` (mirroring existing `push_glyph`/`push_line`/`push_solid` but targeting the overlay vectors via `active_layer()`), so scissor/`foreground_of` context is inherited exactly like today's calls.
- In `render_scene_content`, after the existing `ui_raster_pass` block, add a new `ui_overlay_pass` (color attachment `LoadOp::Load`) that batches and draws the `overlay_ui_instances`/`overlay_vector_vertices` for `LayerBatchFilter::Backdrop` layers. Mirror the same addition in `render_glass_foreground` for `LayerBatchFilter::Foreground` layers, so glass-panel content composites correctly too.
- Repoint `engine_canvas::paint_node_graph_labels` and `engine_canvas::paint_node_graph_overlays` (`framework/renderer/wgpu/rs/lib.rs`) to call the new `_overlay` push methods instead of the regular ones.
- Extend the existing `ui/wgpu/rs/lib.rs` compositor test module with a regression test asserting overlay instances end up in a pass that composites after raster instances (mirror the existing `glass_content_layers_tagged_with_foreground_of` test style).

### 2. Zoom / pan / drag parity (`framework/renderer/wgpu/rs/lib.rs`, `mathematical/graph/port/directed/dag/rs/lib.rs`)

- `node_graph_wheel`: always pass `zoom_gesture: true` for the Flow branch (`host.wheel_screen(sx, sy, 0.0, delta as f64, true)`), matching premigration's "scroll always zooms" behavior for both Flow and DAG. Drop the now-unused `ctrl` parameter from this function's call sites, or repurpose it if a future "hold key to pan" gesture is desired — default to always-zoom to match premigration exactly.
- Add a `pan: bool` parameter to `DagHost::pointer_down_screen` (`mathematical/graph/port/directed/dag/rs/lib.rs:3404`), implementing the same anchor-based pan drag as the vendored copy in `flow/core/rs/lib.rs:3848`. Compute `pan` at the call site in `node_graph_pointer_down` as `button == 1 || (button == 0 && alt)` (mirror premigration's `dagPanGestureActive`), applied identically to the Flow branch's `pan` argument.
- Track a `space_pressed` flag on the wgpu app state (set/cleared from the existing keyboard modifier/key event path) and OR it into the `pan` computation for both Flow and Dag branches of `node_graph_pointer_down`.
- Add `PointerModifiers::ctrl_or_meta(&self) -> bool` in `ui/wgpu/rs/lib.rs` and use it at every `node_graph_wheel`/`node_graph_pointer_{down,move,up}` call site in place of the raw `modifiers.ctrl` (lines ~11957, 12212, 12280, 12291, 12313, 12399), matching premigration's `metaKey || ctrlKey`.
- Wire a wheel-active LOD pin: call the existing `set_wheel_zoom_active`-equivalent host method (or add one alongside `wheel_screen` if absent) when a wheel event fires on a node-graph surface, and clear it after a short idle debounce, so `wheel_zoom_pins_draw_lod_until_gesture_ends` behavior (already tested in `mathematical/graph/port/directed/dag/rs/lib.rs`) actually engages from live input.
- Extend existing tests in `flow/core/rs/lib.rs` and `mathematical/graph/port/directed/dag/rs/lib.rs` to cover: wheel always zooms regardless of ctrl, and Dag pan-gesture drag moves the camera via anchor math (mirroring `flow`'s existing pan test).

### 3. Double-click note editing with live text + caret

- In `framework/renderer/wgpu/rs/lib.rs`, extend `node_graph_pointer_down` (or `_up`, matching the existing `DagHost` double-click pattern used for `DagNodeKind::AppInstance` at `mathematical/graph/port/directed/dag/rs/lib.rs:~3414-3428`) with double-click detection (reuse the existing "last click within ~400ms on same target" pattern) that, when the hit target is an `InputNote` widget, calls `host.begin_note_edit(widget_id, world_x, world_y)` on the surface's `FlowHost`.
- Add a keyboard route: when `ENGINE_SURFACES` has a Flow host with an active `editing_note_id`, forward `KeyAction::Char`/`Backspace`/`Delete`/arrow keys into `note_insert_text`/`note_backspace`/`note_delete_forward`/`note_move_caret` before falling through to shell shortcuts/palette handling — mirror the existing focused-text-editor pattern (`text_editor_apply_key`, ~`framework/renderer/wgpu/rs/lib.rs:3244-3269`). Commit on Enter/Escape via `note_commit_edit`, then persist the updated fixture and trigger a repaint.
- Ensure caret visibility is toggled each frame (or on a blink timer) via the existing `set_note_caret_visible`-style host call while a note is being edited, so `paint_note_caret_bar` actually renders (this becomes visible once fix #1 lands, since caret rendering also goes through the raster/paint_scene path — confirm whether it's drawn in the raster texture already, which would make it unaffected by the z-order bug).
- Extend the existing `flow/core/rs/lib.rs` test module with a regression test simulating double-click → `begin_note_edit` → `note_insert_text` → `note_commit_edit` end to end (state assertions only, no new test files).

### 4. Live evaluate / `OutputPreview` pipeline

- Remove the `#[cfg(not(target_arch = "wasm32"))]` gate on `touch_channel_eval` in `flow/core/rs/lib.rs` (or replace with an explicit, always-available synchronous evaluate call) so wasm builds evaluate the graph after interactions, matching premigration's `session.evaluate()` calls after every interaction.
- Audit `sync_flow_host` (`framework/renderer/wgpu/rs/lib.rs:~2001-2069`): avoid calling `host.replace_fixture(...)` (which clears `last_eval_json`) when only the camera/viewport portion of `fixture_json` changed (e.g. from `nodeGraphViewport` commands) — compare fixture content excluding camera fields, or apply the camera update in-place on the existing host instead of a full fixture replace, so evaluated preview state survives pan/zoom.
- Trigger an initial evaluate on graph mount (first successful `ensure_surface`/`sync_flow_host` for a Flow surface), matching premigration's evaluate-on-session-init behavior.
- Extend existing `flow/core/rs/lib.rs` tests to assert `OutputPreview` content survives a camera-only fixture resync and that evaluate runs on wasm-targeted code paths (no `cfg` skip).

### 5. Catalogue drag-and-drop with ghost preview

- `flow/plugin/rs/lib.rs::build_catalogue_tree`: make each catalogue row `draggable: Some(true)` with `drag_data` carrying a widget-descriptor payload (mirror the existing MIME-tagged pattern already used by `s/plugin/rs/lib.rs` for `application/x-semio-catalogue-item`, e.g. `application/x-flow-widget`).
- In `framework/renderer/wgpu/rs/lib.rs`'s tree-drag update path (near `TreeDragState` handling, ~7835-8650): when an active tree-drag's payload MIME matches the flow widget type and the pointer is over a `node_graph_states` surface bounds, convert screen→world via that surface's `FlowHost` camera and call `set_ghost_widget(descriptor_json, world_x, world_y)` each move; call `clear_ghost_widget()` when the pointer leaves the bounds or the drag is cancelled.
- Extend `finish_tree_drag` to recognize a drop over `node_graph_states` bounds for this MIME type: clear the ghost and dispatch `addWidget` (or an equivalent command) at the cursor's world position, instead of only handling `HitKind::World3d`/`HitKind::Window`.
- Extend existing `flow/core/rs/lib.rs` tests for `set_ghost_widget`/`clear_ghost_widget` to cover the descriptor-driven drop path if not already covered, and add/extend a `flow-plugin` test asserting catalogue rows are draggable with the expected `drag_data` MIME.

## Verification

- Rebuild the wgpu wasm bundle and flow plugin wasm (`trunk build`, `nx build` as used in the prior parity fix).
- Manually verify in-browser (Playwright's WebGPU adapter is unavailable in this environment, per earlier findings) or via targeted screenshots: node/port labels visible, selection marquee/bounds visible, scroll-wheel zooms (no Ctrl needed), pan works via middle-mouse/Alt+drag/Space+drag on both Flow and generic DAG playgrounds, double-click a note enters edit mode with visible caret and typed text, `OutputPreview` nodes show live evaluated content after interaction and after pan/zoom, dragging a catalogue item shows a ghost node following the cursor and drops it at the release position.
- Run `cargo test` for `flow-core`, `dag` (mathematical/graph/port/directed/dag), `framework-graph`, and the `ui-wgpu`/`framework-renderer-wgpu` crates to confirm all extended test modules pass.

## Ticket

Per repo workflow, reopen `.repo/🎫️/26/07/06/FLOW-WGPU-RICH-RENDERING-PARITY` (closely related follow-on) or open a new ticket if the scope is judged distinct enough during execution; update `important.md` with root causes/fixes per item above, and close with a full file list when done.
