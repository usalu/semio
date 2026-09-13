# Puzzle 3D Catalogue Drop Live Preview

## Symptom

Dragging an object kind from the catalogue onto the 3D viewport places the object correctly on drop, but no mesh ghost follows the cursor during the drag.

## Root causes

1. **HTML5 drag never fires `pointermove`** — the React host only listened on `pointermove` for palette-style drags; native `draggable` catalogue rows need `dragover` (window capture) to update preview each frame.

2. **`frameloop="demand"`** — catalogue ghosts must call `invalidate()` when preview state changes (`DemandInvalidateOnToken` / `CatalogueDropPreviewInvalidate`).

3. **wgpu shell had no catalogue-drop ghost** — Puzzle 3D dev defaults to `SEMIO_RENDERER=wgpu`; native `World3dState` rendered brush previews but never tracked catalogue-drop preview or `addObjectKind` drop from the catalogue tree drag.

4. **Pointer-palette `begin` did not publish `activeCatalogueDragPayload`** — fixed in `catalogueTreeDragController`.

5. **Interpreter `TreeView` omitted `catalogueTreeDragController`** — declarative catalogue rows had `dragData` but no `onDragStart` hook, so `getActiveCatalogueDragPayload()` stayed null during `dragover` and `World3dHost` could not parse a preview (drop still worked via `dataTransfer` on release).

6. **Retained wgpu panel drags never hit shell `tree_drag`** — catalogue drags from retained document trees use `EventRouter::DragSession`, not `ShellState::tree_drag`, so `sync_world3d_catalogue_drop_preview` never ran during move (wave 2 only wired legacy chrome tree hits).

## Fix

- **React `World3dHost`**: window `dragover` (capture), drag-enter preview kick, `CatalogueDropPreviewInvalidate`, catalogue ghost priority over brush.
- **wgpu**: `World3dState.catalogue_drop_preview`, ground-plane pick + grid snap, translucent draw pass, shell tree-drag sync + `addObjectKind` commit.
- **`engine_canvas`**: `puzzle3d_catalogue_drag_payload_json` parser + unit test.
- **Interpreter `TreeView`**: merge `catalogueTreeDragController` when catalogue MIME rows are present.
- **wgpu shell**: `sync_world3d_catalogue_drop_preview_from_retained_ui` driven by `Ui::active_drag_sessions()` on pointer move.

## Files

- `🧰️framework/…/🌐️World3dHost/🟦️.tsx`
- `🧰️framework/…/♾️infinite/🌍️world/🦀️.rs`
- `🧰️framework/…/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/…/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx`
