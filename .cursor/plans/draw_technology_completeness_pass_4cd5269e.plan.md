---
name: Draw Technology Completeness Pass
overview: Close the remaining gaps in the Draw vector technology so all 7 layer kinds are fully modeled, renderable, hoverable and selectable, the document shows true nesting (including boolean references), the inspector exposes every editable property, and the canvas supports real authoring gestures for pen/shape/trace tools with selection mechanics matching the shared ui-react primitives used by raster (default/additive/subtractive/invertive merge, partial/full coverage).
todos:
 - id: selection-fix
   content: Fix marqueeModeFromModifiers/marqueeCoverageFromGesture/selectionMergeIds/screenRectFromPoints signatures in draw/react; add selectLasso tool; wire kindHover; add shift/ctrl modified direct-click select
   status: completed
 - id: core-model
   content: Add createDrawShapeLayer/createDrawTextLayer/createDrawImageLayer factories, ellipse/circle path segments, real text/image bounds, DrawSceneNode text/image fields, flatten image layers, boolean-child row id helpers, rgbaToHex
   status: completed
 - id: document-catalogue
   content: Nest boolean children as read-only rows, distinct icons per kind, extend catalogue/add-layer to shape/text/image kinds with DnD
   status: completed
 - id: canvas-render
   content: Add DrawTextShape/DrawImageShape renderers with hover/selection overlays in draw/react
   status: completed
 - id: inspector
   content: Add fill/stroke/transform fields for all kinds plus shape geometry, text content/size, image readonly fields; wire new patchLayer cases
   status: completed
 - id: authoring-tools
   content: Add commitDocument controller command + onCommit canvas prop; implement pen/shapeRect/shapeEllipse/shapeLine/shapePolygon/trace pointer gesture state machines in DrawCanvas
   status: completed
 - id: verify
   content: Re-run all draw+kernel tests, extend runtime-check.mjs and preview-check.mjs in the ticket folder, reopen and close ticket 26/06/30/DRAW-VECTOR-TECHNOLOGY
   status: completed
isProject: false
---

# Draw Technology Completeness Pass

Continues ticket `26/06/30/DRAW-VECTOR-TECHNOLOGY` (reopen via `ticket_reopen`, do not open a new ticket). All temp scripts go in that ticket's folder; reuse `.repo/🎫/26/06/30/DRAW-VECTOR-TECHNOLOGY/runtime-check.mjs` and `preview-check.mjs`, extending them rather than creating new files.

## 1. Fix selection/hover mechanics to match the shared primitives (`draw/react/index.tsx`)

This is the literal "partial / default / invertive" the user referenced — it comes from `@semio-tech/ui-react`'s `SelectionMarqueeCoverage` (`"partial" | "full"`) and `SelectionMergeMode` (`"default" | "additive" | "subtractive" | "invertive"`), the exact primitives [raster/react/index.tsx](raster/react/index.tsx) already uses correctly. Draw currently calls these APIs with the wrong signatures, so selection is effectively broken at runtime:

- Replace the ad hoc `event.shiftKey ? "add" : event.altKey ? "subtract" : "default"` with `marqueeModeFromModifiers(event)` from `@semio-tech/ui-react` (gives `default/additive/subtractive/invertive`, matching shift+ctrl => invertive).
- Fix `marqueeCoverageFromGesture(...)` call to the real signature `{ method, startX, endX, path }` returning `"partial" | "full"`; derive `crossing = coverage === "partial"`.
- Fix `selectionMergeIds(...)` argument order to `(mode, current, incoming)` (draw currently passes `(current, incoming, mode)`).
- Fix `screenRectFromPoints(...)` to take a single array of points, not two positional points.
- Add a `selectLasso` tool (mirrors raster's `selectLasso`): track a point path, render via `SelectionMarquee` polygon, resolve hits via path-based coverage. Add `"selectLasso"` to `DRAW_TOOL_IDS` in [draw/core/index.ts](draw/core/index.ts) and a toolbar button in [draw/play/index.ts](draw/play/index.ts) next to `selectMarquee`.
- Add shift/ctrl-modified single click on `selectDirect` (not just marquee) so direct-select also supports additive/subtractive/invertive via `selectionMergeIds`.
- Destructure and actually use the already-declared `kindHover` prop (currently dead) to drive cross-highlighting between canvas and the new boolean-reference document rows (section 3).

## 2. Layer model completeness (`draw/core/index.ts`)

- Add `createDrawShapeLayer(shapeKind, geometry, name?)`, `createDrawTextLayer(name?, content?, size?)`, `createDrawImageLayer(name?, imageKey, width, height)` factories alongside the existing `createDrawPathLayer` / `createDrawGroupLayer` / `createDrawBooleanLayer` / `createDrawTraceLayer`.
- Extend `layerToPathSegments` to handle `shapeKind === "ellipse"` and `"circle"` via the standard 4-cubic-bezier circle/ellipse approximation (kappa ≈ 0.5523), so booleans/hit-testing/rendering work for all 5 shape kinds, not just rect/line/polygon.
- Fix `drawLayerWorldBounds` (currently returns a blind 128x128 placeholder whenever `layerToPathSegments` is empty) to compute real bounds:
  - `text`: box at `(x, y)` sized from `content.length * size * 0.6` x `size * 1.2` (monospace heuristic, no font metrics dependency).
  - `image`: box at the layer's local origin sized `width x height`.
  - Keep the placeholder only as a last-resort fallback for genuinely degenerate shapes (e.g. empty polygon).
- Extend `DrawSceneNode` with optional `text?: { content: string; size: number }` and `image?: { src: string; width: number; height: number }` fields; populate `image.src` by resolving `doc.assets[layer.imageKey]` into a `data:` URL.
- Update `flattenDrawDocumentToSceneNodes` to stop skipping `image` layers and to carry the new `text`/`image` fields through.
- Add a helper `drawPlayBooleanChildRowId(booleanId, childId)` / `drawPlayLayerIdFromBooleanChildRowId(rowId)` pair (format `draw-play-layers.ref.{booleanId}::{childId}`) so the document can show boolean children as distinct, non-colliding nested rows that still resolve back to the real layer id for selection/hover.
- Add `rgbaToHex` helper (inverse of existing `hexToRgba`) for the new inspector color fields (section 5).

## 3. Document & catalogue completeness (`draw/play/index.ts`)

- `buildDrawPlayLayersTree`: for `kind === "boolean"`, resolve each id in `layer.children` via `findDrawLayer` and render them as nested, read-only rows (icon + name, no drag handle) using `drawPlayBooleanChildRowId`; clicking still issues `setSelection` for the real layer id; missing/unresolved ids render as a disabled placeholder row instead of being silently dropped.
- Give `shape`, `text`, `image` their own icons (e.g. `"square"`, `"type"`, `"image"`) instead of falling back to the shared `"shapes"` icon for all three.
- Extend `DrawCatalogueLayerKind` from `"path" | "group" | "boolean" | "trace"` to also include `"shape:rect" | "shape:ellipse" | "shape:line" | "shape:polygon" | "text" | "image"`, update `drawPlayCreateLayerByKind` to use the new core factories, and extend the add-layer toolbar + catalogue tree + drag-and-drop MIME handling accordingly.
- Verify `drawPlayHoverPayloadFromTreeRowId` / `drawPlayLayersTreeHighlightedIds` correctly highlight both the canonical row and any boolean-reference rows for the same layer id.

## 4. Canvas rendering for all kinds (`draw/react/index.tsx`)

- Add a `DrawTextShape` renderer (`<text x y fontSize fill>`) and `DrawImageShape` renderer (`<image href width height>`), used when a `DrawSceneNode` carries `text`/`image` payloads, alongside the existing `DrawPathShape`.
- Ensure hover/selection outlines render consistently for text/image nodes (e.g. a dashed bounding rect overlay since `<text>`/`<image>` don't get a meaningful stroke outline the way paths do).

## 5. Inspector completeness (`draw/play/index.ts`)

Currently the inspector only exposes name/opacity/blend/visible plus boolean-operation/trace-params — there is no fill, stroke, or transform editing at all even though the data model fully supports it. Add, gated by layer kind:

- **All kinds**: fill color (hex text field via `rgbaToHex`/`hexToRgba`, wired to `setFill`), stroke color + width (wired to `setStroke`), transform `x`/`y`/`scaleX`/`scaleY`/`rotation` number fields (wired to `setLayerTransform`).
- **shape**: numeric fields for the active `shapeKind`'s geometry (rect: x/y/width/height; ellipse: cx/cy/rx/ry; circle: cx/cy/r; line: x1/y1/x2/y2; polygon: read-only point count + "Edit on canvas" hint).
- **text**: content (text input), size (number/slider).
- **image**: read-only `imageKey`, width, height.
- Add corresponding `patchLayer` field cases in the controller (`fillColor`, `fillAlpha`, `strokeColor`, `strokeWidth`, `transformX/Y/ScaleX/ScaleY/Rotation`, shape geometry fields, `textContent`, `textSize`).

## 6. Canvas authoring tools (pen / shapes / trace)

Add a single new controller command `commitDocument` (`{ document: DrawDocument; selectLayerId?: string }`) that replaces `this.document` directly and updates selection/bumps — this is the one new wiring point all authoring gestures funnel through. Wire a matching `onCommit?: (document: DrawDocument, selectLayerId?: string) => void` prop on `DrawCanvas`, connected in [framework/product/playground/renderer/react/index.tsx](framework/product/playground/renderer/react/index.tsx) to `ctrl.run("commitDocument", { document, selectLayerId })`.

In `DrawCanvas`, add pointer-gesture state machines per tool, each ending in `onCommit?.(applyDrawEditOp(doc, operator), newLayer.id)`:

- **`shapeRect` / `shapeLine`**: pointerdown anchors world point, pointermove shows a live preview overlay, pointerup computes bounds/endpoints and commits `addShapeLayer`.
- **`shapeEllipse`**: same drag gesture, computing `cx/cy/rx/ry` from the drag bounding box.
- **`shapePolygon`**: click-to-add-point state machine (append world point per click, live polyline preview), double-click or Enter commits `addShapeLayer` with the accumulated points, Escape cancels.
- **`pen`**: click-to-add-point state machine producing `PathSegment[]` (move + line per click), double-click/Enter commits `addPathLayer`, Escape cancels.
- **`trace`**: click resolves the layer under the cursor; if it's an `image` layer, commit `addTraceLayer` referencing that layer's `imageKey`; if there is no image under the cursor, fall back to the first key in `doc.assets` (if any); no-operation if no assets exist.
- After each commit, the tool auto-resets to `selectDirect` (issue `setActiveTool` alongside `commitDocument`) and the new layer is auto-selected, mirroring the existing `patchDocument(..., selectLayerId)` pattern used by `addLayer`.

## 7. Verification

- Re-run `geometry_drawing_rs`, `@semio-tech/draw-core:test`, `@semio-tech/draw-react:test`, `@semio-tech/draw-play:test`.
- Extend `.repo/🎫/26/06/30/DRAW-VECTOR-TECHNOLOGY/runtime-check.mjs` to also exercise: shape/text/image factories + flatten, boolean child row id round-trip, `selectionMergeIds`/`marqueeModeFromModifiers` behavior for all four modes, and `commitDocument`-style edit application for a drawn shape/path/trace.
- Extend `.repo/🎫/26/06/30/DRAW-VECTOR-TECHNOLOGY/preview-check.mjs` (Playwright) to: hover/select a few different layer kinds and assert document highlight sync, perform a marquee with shift (additive) and shift+ctrl (invertive) and assert resulting selection set, expand a boolean row and assert its children rows are present, and exercise one shape-drawing gesture end-to-end (drag a rect, assert a new path/shape SVG element appears and document gains a row).
- `ticket_reopen` for `26/06/30/DRAW-VECTOR-TECHNOLOGY`, then `ticket_close` with the summary and full file list once verified.
