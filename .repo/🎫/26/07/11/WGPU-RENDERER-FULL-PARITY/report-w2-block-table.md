# w2-block-table — final report

File touched (only): `framework/renderer/wgpu/rs/lib.rs`

## Table interaction parity

Confirmed via the read-only React reference (`framework/renderer/react/components/table-host.tsx`, the underlying `Table` component in `ui/js/react/index.tsx`, and the real consumer `sourcing/plugin/rs/lib.rs`):

- **Sort-on-header-click — was missing, now implemented.** Added `sortable: bool` to the local `TableColumn` mirror struct and a new `TableSortJson { column_id, direction }` mirroring `sourcing::TableSort`'s wire format. `render_table` now draws an asc/desc indicator on sortable headers and registers a hit target per sortable column dispatching `sortTable` with `{surfaceId, columnId, direction}`, cycling asc→desc→asc based on the currently-sorted column read from `table.sort_json`. Matches the real `"sortTable"` handler in `sourcing/plugin/rs/lib.rs:446`.
- **Row selection — already at parity, no change needed.** Existing code already dispatches `selectRow` with `{surfaceId, row}` on row click, identical to `table-host.tsx`. Neither the wire schema, `table-host.tsx`, nor any plugin handler (`sourcing::selected_object_id: Option<String>`) supports shift/ctrl multi-select extension — that only exists in the unrelated `VirtualFileSystemHost`'s bespoke `additiveKey`/`rangeKey` mechanism. Did not invent shift/ctrl-extend for Table since the schema genuinely doesn't support it.
- **Inline cell editing — confirmed unsupported, not implemented.** `TableScene`/`TableCell` has no `editable` flag anywhere (that field exists only on `NodeGraphScene`); `table-host.tsx` never renders an editable text cell — only `stepper`/`buttons` cell kinds are interactive, both already implemented in wgpu.

Added a `//#region TableTests` module (6 tests) covering header sort direction cycling, per-column independence, non-sortable columns registering no hit, and row-select payload shape. All pass.

## BlockList engine (new)

`BlockListScene { steps_json, palette_json, selected_id, dragging_id }` (from `ui/wgpu/rs/lib.rs:2552`) holds `steps_json` as `protocol::ProtocolStep[]` and `palette_json` as `BlockPaletteEntry[]`. The real action verbs come from `protocol::builder_kit` + `protocol-plugin`'s `handle_action`: `addStep {}`, `removeStep {stepId}`, `moveStep {stepId, index}`, `addBlock {kind, stepId?}`, `removeBlock {stepId, blockId}`, `moveBlock {blockId, fromStepId, toStepId, index}`.

Implemented `fn render_block_list(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>)` (3-arg signature, matching `render_table`/`render_graph_timeline`'s convention) in a new `//#region BlockList` (currently lines 8446-8700, `#[allow(dead_code)]` pending wiring). Renders steps stacked vertically (title, description, bordered/rounded card matching Table/VFS's theme conventions) each with their blocks, plus a right-hand palette rail, mirroring `block-list-host.tsx`'s layout. Dispatches `addStep`, `removeStep`, `addBlock` (from palette clicks), `removeBlock`. **Reordering deviation, stated honestly**: the React host uses dnd-kit free-pointer drag; this renderer's click-dispatch model has no established cross-frame drag-position-tracking primitive for list reordering, so move-up/move-down button hit targets dispatch `moveStep`/`moveBlock` with a computed target index instead — functionally equivalent, not visually identical. Selection highlighting (`selected_id`) is rendered even though `block-list-host.tsx` doesn't yet render it either (harmless, data-driven, mirrors Table's own selected-row convention).

Added a `//#region BlockListTests` module (8 tests): missing-scene placeholder, `addStep`, palette `addBlock`, first/last-step move-button gating, `removeStep`, block move/remove action shapes, empty-steps registering no hit targets. All pass.

## Wiring request for `w2-scene-wiring` / coordinator

`RenderEntry`'s match (currently `framework/renderer/wgpu/rs/lib.rs:7443`) needs one arm added, matching the existing `Table` style exactly:
```rust
SurfaceKind::BlockList => render_block_list(scene, bounds, ctx),
```
Function signature: `fn render_block_list(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>)`. Noticed `apply_scene_pointer(scene, bounds, ctx)` has already been added after `apply_scene_wheel` in `render_component_scene` (line 7464) — generic pointer routing (from `w2-scene-wiring`) is landed, which is what this agent's click hit-targets depend on.

## Verification

`cargo check -p semio-framework-renderer-wgpu --lib` — clean (only pre-existing warnings, unrelated to owned regions). `cargo test -p semio-framework-renderer-wgpu --lib`: **80/81 passing**; all 14 new Table/BlockList tests pass. The one failure, `dock::tests::apply_drop_tab_moves_window_across_stacks`, is in `dock` (out of scope, read-only) — panics on cross-stack tab-drop logic never touched here. Also observed transient unrelated compile errors in `infinite_world`/`base64::Engine` usage elsewhere in the shared file that resolved themselves as other concurrent sessions finished their edits.

## NOTE FOR COORDINATOR
This is now a REAL (not blocked-by-contention) test failure signal: `dock::tests::apply_drop_tab_moves_window_across_stacks` actually FAILS, not just "couldn't run." This is one of `w2-dock-dnd`'s own new tests. Needs investigation/fix by whoever owns `dock` next.
