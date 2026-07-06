---
name: Layout App System Integration
overview: "Wire Layout into the same workbench/catalogue/details/hover conventions used by Flow, Draw, and S: add a draggable Catalogue tab for spawning pages/frames, make every frame kind selectable through the Document tree, make the Details/Inspection panel always show editable fields for the current selection, and add real bidirectional hover between canvas and workbench panels."
todos: []
isProject: false
---


# Layout App System Integration

## Current state (confirmed by reading the code)

- Layout's playground panels are wired in [`framework/product/playground/renderer/react/index.tsx`](framework/product/playground/renderer/react/index.tsx) `🔖LayoutPlayHost` region (lines 7688-7854): `workbench: [Document, Preflight]`, `details: [Inspection]` — **no Catalogue tab**, unlike Draw/Flow/S which all have `[Document, Catalogue]`.
- [`buildLayoutPlayDocumentTree`](layout/core/js/index.ts) (lines 97-176) lists spreads/pages/parentPages/layers/stories/links/styles but **never lists individual frames** — so a rect/text/image object has no selectable row, even though selection already tries to map `selectedIds` to `layout-document.frame.${id}` rows that don't exist.
- [`buildLayoutPlayInspectorTree`](layout/core/js/index.ts) (lines 203-236) is **fully readonly** (`uiInspectorReadonlyField` only) — selecting a page or frame never yields editable controls, unlike Flow's `flowPlayInspectorPatch`-driven editable fields.
- There is no catalogue of spawnable kinds and no drag-and-drop anywhere in layout. `LayoutCommand` (internal.ts lines 189-195) only supports `set_object_bounds`, `set_selection`, `set_story_content`, `apply_parent_page`, `reorder_pages` — nothing to add a page or frame.
- Hover is entirely unimplemented: `LayoutEngineSession.pointerMove` is a no-op ([`layout/react/index.tsx`](layout/react/index.tsx) line 82), the Rust `DisplayRect`/`DisplayList` only carry a `selected` flag ([`layout/rs/display.rs`](layout/rs/display.rs) lines 16-26, 91-103), and no controller command exists for hover.

Draw ([`draw/core/js/index.ts`](draw/core/js/index.ts)) already implements the exact pattern to copy: a `DRAW_LAYER_KIND_DRAG_MIME` catalogue with `draggable`/`dragData` items (lines 289-358), a document tree drag controller that turns a catalogue drop into an `addLayer`-style command (lines 790-815), hover sinks wired to `onPointerEnter`/`onPointerLeave` on tree rows (lines 219, 248), and an editable inspector pattern mirrored in Flow (`flowPlayInspectorPatch`, `flow/core/js/index.ts` lines 457-508).

## 1. Document commands — [`layout/core/js/internal.ts`](layout/core/js/internal.ts)

Extend `LayoutCommand` (line 189) with:
- `add_page` / inverse `remove_page` (spreadId, page snapshot)
- `add_frame` / inverse `remove_frame` (pageId, frame snapshot)
- `patch_frame_props` (objectId, before/after partial frame fields: fill, stroke, storyId content passthrough, linkId, wrapMode, columns)
- `patch_page_props` (pageId, before/after partial page fields: name, width, height, margins, columns)

Add factory helpers (mirroring Draw's `catalogueLayerFromKind`): `createDefaultFrame(kind: FrameKind, layerId: string): Frame` and `createDefaultPage(spreadId: string, index: number): Page`, plus apply/invert branches in `applyLayoutCommand` / `invertLayoutCommand`.

## 2. Catalogue + Document — [`layout/core/js/index.ts`](layout/core/js/index.ts)

- Add `LAYOUT_CATALOGUE_KIND_DRAG_MIME` and `buildLayoutPlayCatalogueTree()`: draggable items **Page**, **Rectangle**, **Text**, **Image** (kind payload `{ kind: "page" | "rect" | "text" | "image" }`). Grid/columns stay page-level settings surfaced in the editable inspector (§3), not a draggable kind — a grid isn't placeable content in this document model.
- Extend `buildLayoutPlayDocumentTree` to list a **Frames** row per frame under each page (or nested under its layer), each with `id: layout-document.frame.${frame.id}`, an icon per `FrameKind`, a `setSelection` command, and `onPointerEnter`/`onPointerLeave` hover sinks — fixing the existing selection-highlight mismatch and enabling "selection works for element kinds."
- Add `createLayoutPlayDocumentTreeDragController()` exported for the framework host: on drop, reads `LAYOUT_CATALOGUE_KIND_DRAG_MIME` payload; if `kind === "page"` dispatches `addPage`; otherwise dispatches `addFrame` targeting the page/layer row that was dropped on (falls back to the active page).
- Rewrite `buildLayoutPlayInspectorTree` (lines 203-236) using editable `input`/`select` fields (same shape as `flowPlayInspectorNumberField`/`TextField`) instead of `uiInspectorReadonlyField`:
  - Page group: name (text), width/height (number), margins (4 numbers), columns count/gutter — `onChange` → `patchPage`.
  - Frame group, branching on `frame.kind`: rect → fill/stroke; text → story content passthrough + wrap mode select; image → link path — `onChange` → `patchFrame`.
  - Keep the empty-state message when nothing is selected.
- `LayoutPlayController.run()`: add `addPage`, `addFrame`, `patchFrame`, `patchPage` handlers pushing through `LayoutHistory`, then `setSelection` to the newly created object; add `setHover` handler delegating to `this.pointerFocus.setHoverFromSource` / `clearHoverFromSource` (same as Draw); add `getHoveredId()` accessor reading `this.pointerFocus.getSnapshot().hover`.

## 3. Canvas hover plumbing — [`layout/react/index.tsx`](layout/react/index.tsx)

- Add `onHover?: (objectId: string | null) => void` to `LayoutCanvasProps` and `LayoutEngineSession` constructor; implement `pointerMove(x, y)` (currently a no-op, line 82) to `hitTest` and invoke the callback only on change.
- Add `hoveredId` prop threading into the session (`setHoveredId(id: string | null)`) calling a new WASM binding, mirroring the existing `setSelectedIds` pattern (lines 70-73).

## 4. Rust hover rendering — [`layout/rs/wasm_session.rs`](layout/rs/wasm_session.rs), [`display.rs`](layout/rs/display.rs), [`engine.rs`](layout/rs/engine.rs)

- `wasm_session.rs`: add `hovered_id: Option<String>` to session state + `#[wasm_bindgen(js_name = setHoveredId)]` setter; pass it into `build_scene_from_document_json` / `hit_test_document_json` calls (lines 109, 118).
- `display.rs`: add `hovered: bool` to `DisplayRect` (line 16-26) and thread through `bounds_to_display_rect` (line 91).
- `engine.rs`: extend `build_display_list_for_page` (line 103) to accept `hovered_id: Option<&str>`, compute `hovered` alongside the existing `selected` (line 156), and in the paint loop (lines 299-328) render a distinct hover stroke (thinner/different hue than the 2.0-2.5px selection stroke) that's visible even when not selected.

## 5. Framework chrome wiring — [`framework/product/playground/renderer/react/index.tsx`](framework/product/playground/renderer/react/index.tsx) `🔖LayoutPlayHost`

- Add `LayoutPlayCataloguePanelDefinition` (workbench tab, order after Document) building from `buildLayoutPlayCatalogueTree()`.
- Attach `dragAndDropController: createLayoutPlayDocumentTreeDragController(...)` to `LayoutPlayDocumentPanelDefinition`'s tree config (mirrors Draw's `DrawPlayLayersPanelDefinition`, lines 10917-10936).
- Update `LayoutPlayInner`'s `augmentPanelTabs.workbench` (line 7829) to `[layoutPlayDocumentPanel, layoutPlayCataloguePanel, layoutPlayPreflightPanel]`.
- `LayoutPlayPaneSurfaceHost` (lines 7741-7760): add an `onHover` callback dispatching `ctrl?.run("setHover", { id, sourceId: CANVAS_HOVER_SOURCE_CANVAS })`, pass `hoveredId={ctrl?.getHoveredId()}` into `LayoutCanvas`, and pass hover sinks from document/catalogue rows using `CANVAS_HOVER_SOURCE_DOCUMENT`/`CANVAS_HOVER_SOURCE_CATALOG` (same constants Draw uses).

## 6. Tests (extend existing in-source suites — no new test files per repo rules)

- `layout/core/js/internal.ts`: apply/invert coverage for `add_page`/`add_frame`/`patch_frame_props`/`patch_page_props`, including undo/redo round-trips.
- `layout/core/js/index.ts`: `buildLayoutPlayCatalogueTree` structure + dragData; `buildLayoutPlayDocumentTree` now includes frame rows; `buildLayoutPlayInspectorTree` returns editable `input` fields with correct `onChange` command; controller `addPage`/`addFrame`/`patchFrame`/`patchPage`/`setHover` behavior.
- `layout/react/index.tsx`: `LayoutEngineSession.pointerMove` invokes `onHover` after a hit-test change.
- `layout/rs/engine.rs` `mod tests`: `build_display_list_for_page` marks the hovered frame's rect `hovered: true` and leaves others `false`.

## Execution notes

- Rebuild the `layout-rs` WASM package (`layout/rs/script.ts` build) after the Rust changes so `layout/react` and the framework host pick up the new `setHoveredId` binding.
- Work happens inside a repo ticket per workspace rules (`ticket_open`/`ticket_close`), associated with the most fitting existing goal.
</plan>
<parameter name="todos">[{"id":"commands","content":"Add add_page/add_frame/patch_frame_props/patch_page_props commands + factories + apply/invert in layout/core/js/internal.ts"},{"id":"catalogue-document","content":"Add buildLayoutPlayCatalogueTree, frame rows in document tree, document drag controller in layout/core/js/index.ts"},{"id":"editable-inspector","content":"Rewrite buildLayoutPlayInspectorTree with editable input/select fields wired to patchPage/patchFrame"},{"id":"controller-commands","content":"Add addPage/addFrame/patchFrame/patchPage/setHover handlers + getHoveredId to LayoutPlayController"},{"id":"react-hover","content":"Implement pointerMove hit-testing and onHover/hoveredId plumbing in layout/react/index.tsx"},{"id":"rust-hover","content":"Thread hovered_id through wasm_session.rs/display.rs/engine.rs and render distinct hover stroke"},{"id":"framework-wiring","content":"Add Catalogue workbench tab, attach drag controller to Document panel, wire bidirectional hover in framework playground LayoutPlayHost"},{"id":"tests","content":"Extend existing in-source vitest/Rust test suites for all new behavior and run them"}]