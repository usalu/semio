---
name: Procedural Preview All Types
overview: Extend the procedural preview pipeline so point and vector outputs are rendered (currently dropped), unifying geometry/point/vector into one preview-item model while keeping existing curve/surface/solid previews intact.
todos: []
isProject: false
---

## Procedural Preview: render every output type

### Problem

`extractGeometryHandles` in [procedural/react/index.tsx](procedural/react/index.tsx) only collects `geometry`/`brep` string outputs. Brep nodes emit `point`/`vector` as raw arrays (e.g. `brep.point`, `brep.vector`, `eval.pointOnCurve`, `eval.tangentOnCurve`, `eval.normalAt`, `eval.faceCenter`), so points and vectors are never previewed. Curves, surfaces, spheres and solids already preview via kernel tessellation and stay unchanged.

### Ticket

Open a procedural ticket via repo MCP (read `repo://goals` first; associate with the procedural/flow goal). Keep all temp files inside the ticket folder.

### 1. Unified preview-item model — [procedural/react/index.tsx](procedural/react/index.tsx)

- Replace `ProceduralGeometryHandle` with a discriminated union `ProceduralPreviewItem`:
  - `{ widgetId, kind: "geometry", handle: GeometryRef }`
  - `{ widgetId, kind: "point", position: Vec3 }`
  - `{ widgetId, kind: "vector", direction: Vec3 }` (drawn from world origin)
- Replace `extractGeometryHandles` with `extractPreviewItems(outputsJson)` mapping per widget dict: `geometry`/`brep` string -> geometry item; `point` array -> point item; `vector` array -> vector item.
- Rename `ProceduralPreviewProps.handles` -> `items` (and update `BrepViewport` deprecated wrapper to build geometry items).

### 2. Render points and vectors — same file `BrepViewport` region

- Add `BrepPointLayer` (small sphere mesh at `position`) and `BrepVectorLayer` (shaft line + cone head from origin to `direction`) built with `sceneHostPort.three` primitives, reusing `worldEntityRenderMode` chrome (color/emissive/opacity), `onHover`/`onPick`, and `previewOff`, consistent with `BrepGeometryLayer`.
- In `ProceduralPreview`, switch on `item.kind` to render the right layer inside the existing `WorldLayer`.
- Extend selection bounds: replace `screenBoundsForHandle` (which calls `kernel.getBoundsSync`) with a per-item world-AABB resolver — geometry via kernel, point as a zero-size box at the position, vector as origin..tip — then project with the existing `projectWorldBoundsToScreen`. Keeps marquee/lasso selection working for all item kinds.

### 3. Update consumers

- [procedural/play/index.ts](procedural/play/index.ts): rename field `geometryHandles` -> `previewItems`, `getGeometryHandles` -> `getPreviewItems`, import/use `extractPreviewItems`; `setEvalOutputs`, `selectAll`, and the preview status count use `previewItems`/`item.widgetId`.
- [framework/product/playground/renderer/react/index.tsx](framework/product/playground/renderer/react/index.tsx) line ~6515: `items={ctrl?.getPreviewItems() ?? []}`.

### 4. Tests (extend existing in-file suites, no new files)

- `procedural/react`: `extractPreviewItems` collects geometry, point, and vector items; `ProceduralPreview` mounts with point and vector items present; verify representative kinds (`brep.point`, `brep.vector`, line/curve/surface/solid) produce the expected item kind/handle prefix.
- `procedural/play`: `setEvalOutputs` stores point/vector items; `selectAll` includes them.

### 5. Verify at runtime

- Run the procedural react + play vitest suites (via `launch.json`/nx, not ad-hoc scripts) and confirm green.
- Add temporary `[DEBUG]` logs if needed to confirm point/vector items flow to the preview, then remove before closing the ticket.

### Close ticket

Close via repo MCP with summary and the file list above.
</plan>
<todos>[{"id":"ticket","content":"Read repo://goals and open/reopen procedural preview ticket"},{"id":"model","content":"Replace ProceduralGeometryHandle with ProceduralPreviewItem union and extractPreviewItems in procedural/react"},{"id":"layers","content":"Add BrepPointLayer and BrepVectorLayer; render by item.kind; extend selection bounds per item kind"},{"id":"consumers","content":"Update procedural/play controller and framework renderer to previewItems/getPreviewItems"},{"id":"tests","content":"Extend procedural/react and procedural/play in-file test suites for points/vectors"},{"id":"verify","content":"Run procedural vitest suites and confirm previews; close ticket via repo MCP"}]</todos>
</invoke>
