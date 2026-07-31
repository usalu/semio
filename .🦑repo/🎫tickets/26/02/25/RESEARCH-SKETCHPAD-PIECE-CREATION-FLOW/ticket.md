---
goal: SKETCHPAD-PIECE-CREATION
---

# Ticket: Research Sketchpad Piece Creation Flow

## Summary

Research complete: Documented full piece creation flow, model resolution, scene rendering, test structure, found 5 inconsistencies.

## Findings

### 1. Piece Creation Logic (addPiece / createPiece)

**Three UI entry points for adding pieces:**

1. **Diagram drag-drop** — `Design.tsx:7790-7837` (`handleDragEnd` callback in `DesignDiagram`)
   - Computes diagram `center` (u,v) from viewport coordinates
   - Computes `plane` from `center` via `worldX = (center.u - 6) / 0.3`, `worldZ = (center.v + 7) / 0.3`
   - Creates piece: `{ guid, type: { guid }, center, plane }`
   - Calls `addPiece?.(piece)` (from `useDesignAppAddPiece()` at line 7431)

2. **Scene drag-drop** — `Design.tsx:9490-9546` (`handleSceneDragEnd` in `DesignAppScene`)
   - Raycasts from camera to Y=0 ground plane
   - Computes `plane` from worldX/worldZ
   - Computes `center` from `{ u: worldX * 0.3 + 6, v: worldZ * 0.3 - 7 }`
   - Creates piece: `{ guid, id_: pieceGuid, type: { guid }, plane, center }`
   - Note: Scene drop includes `id_` field, diagram drop does NOT

3. **Workbench + button** — `Design.tsx:10351-10361` (type) and `10425-10435` (design)
   - Uses hardcoded `center = { u: 0, v: 0 }` (places at origin)
   - Derives `plane` from center via same formula: `worldX = (0 - 6) / 0.3 = -20`, `worldZ = (0 + 7) / 0.3 = 23.33`
   - Creates piece: `{ guid: guid(), type: { guid: type.guid }, center, plane }`

**Command chain:**

- `useDesignAppAddPiece()` → `Design.tsx:2580` → `store.execute("compose.designApp.addPiece", ...)`
- `"compose.designApp.addPiece"` handler → `Design.tsx:785` → returns `kitDiff` with `pieces.added: [piece]`
- `kitDiff` applied via `kitStore.change(result.kitDiff)` → `Design.tsx:1371-1372`
- `kitStore.change()` → `Sketchpad.tsx:6664` → `designStore.change(diff)` → `Sketchpad.tsx:5254`
- `DesignStore.change()` at line 5270 calls `this.createPiece(piece)` for each `diff.pieces.added`

**Alternate path: `compose.kit.addPiece`** — `Sketchpad.tsx:7984`

- Lower-level command used by `kitStore.execute()`
- Auto-adds default plane `{ origin: {0,0,0}, xAxis: {1,0,0}, yAxis: {0,1,0} }` if piece has no `plane` AND is not part of existing connections

**Y.js persistence:**

- `DesignStore.createPiece()` at `Sketchpad.tsx:5123-5128`
- Creates `Y.Map` for piece, pushes to `yPieces` array
- `PieceStore` constructor at `Sketchpad.tsx:3764-3827` stores: guid, id\_, type, name, scale, isHidden, isLocked, color, description, plane, center, mirrorPlane, attributes

### 2. Placement Data (Plane/Center) in Piece Creation

**Plane** — 3D coordinate system placed on the piece:

```
{ origin: { x, y, z }, xAxis: { x, y, z }, yAxis: { x, y, z } }
```

**Center** — 2D diagram coordinate:

```
{ u: number, v: number }
```

**All three UI entry points provide BOTH plane and center:**

- Diagram drop: `Design.tsx:7807-7813` — `plane` and `center` computed from viewport coords
- Scene drop: `Design.tsx:9528-9530` — `plane` and `center` computed from camera ray
- Workbench button: `Design.tsx:10353-10357` — `center={u:0,v:0}`, `plane` derived

**Fallback plane for rendering** (`useFlatPiecePlane` at `Sketchpad.tsx:4150`):

- Uses `piecesMetadata()` from `compose.ts:8171` which flattens the design via `flattenDesign(kit, designGuid)`
- Falls back to `{ origin: {0,0,0}, xAxis: {1,0,0}, yAxis: {0,1,0} }`

**Inconsistency found:** Diagram drop does NOT set `id_` on the piece, but scene drop DOES set `id_: pieceGuid`. The `PieceStore` constructor at `Sketchpad.tsx:3770` does `this.localId = piece.guid` which uses `piece.guid` as fallback for `id_`.

### 3. Scene Piece Rendering

**Component document:**

- `ModelDesign` (`Design.tsx:9370-9458`) — Top-level scene design renderer
  - Iterates `flatDesign.pieces` and wraps each in `PieceScopeProvider` + `ModelPiece`
- `ModelPiece` (`Design.tsx:9190-9366`) — Individual piece 3D renderer
  - Gets plane from `useFlatPiecePlane()` (flattened metadata)
  - Computes matrix: `planeToMatrix(plane)` → `toThreeRotation() * planeMatrix`
  - Renders `<group matrix={pieceMatrix}>` with either:
    - `<Geometry>` component (for design-type pieces)
    - `<PieceMesh>` component (for type-based pieces)
- `PieceMesh` (`Design.tsx:9113-9186`) — Loads 3D model file
  - Gets type via `useType()`, finds model via `selectBestModel()`
  - Gets file URL via `kitStore.getFileUrl()` then `kitStore.getFileBlobUrl()`
  - Renders `<LoadedPieceMesh url={blobUrl} fileExtension={ext}>`
- `LoadedPieceMesh` (`Design.tsx:9100-9112`) — Routes to format-specific loader
  - `.glb`/`.gltf` → `GLTFMesh` (uses `useGLTF`)
  - `.fbx` → `FBXMesh` (uses `useFBX`)
  - `.obj` → `OBJMesh` (uses `useLoader(OBJLoader)`)
- `GLTFMesh`/`FBXMesh`/`OBJMesh` (`Design.tsx:8989-9097`) — Load, clone, apply rotation `toComposeRotation()`, apply plaster material, support highlight color

**Scene container:**

- `DesignAppScene` (`Design.tsx:9458+`) — Wraps scene with drop zone, camera sync, fullscreen toggle
- Uses `<Scene>` from `elements.tsx:6516` (wraps `<Canvas>` from react-three-fiber)

### 4. Model Resolution for Types

**Flow: Type → Model → File → URL**

1. **Type has models array**: `type.models: Model[]`
2. **Select best model** via `selectBestModel()` (`compose.ts:4139-4149`):
   - If no tags selected: pick model with no tags, else first model
   - If tags selected: filter by tags via `filterModelsByTagGuids()`, then `findModel()`
3. **Model → File**: `model.file` (string or `{ guid }`) → find in `kit.files`
4. **File → URL**: `kitStore.getFileUrl(file.guid)` → `kitStore.getFileBlobUrl(file.guid)`
5. **Supported formats**: `SUPPORTED_3D_EXTENSIONS` at `compose.ts:4159+`: gltf, glb, fbx, obj, dae, 3ds, stl, ply, usdz, vrm, ifc, 3mf, amf

**In Type.tsx** (`Type.tsx:1617-1660`): More elaborate resolution considering:

- Explicit `selectedModelGuid`
- Manual `selectedModelTags`
- Type concepts (`typeConcepts`)
- Default/first fallback

**In Design.tsx PieceMesh** (`Design.tsx:9121-9152`): Simpler resolution:

- Uses `selectedModelTags` from `useDesignAppSelectedModelTags()`
- Calls `selectBestModel(type.models, tagsForType)`

### 5. Design Component Location

**Design.tsx is NOT empty** — it's **10,697 lines** at `/workspaces/semio/compose/js/sketchpad/Design.tsx` (439KB).

The `read_file` tool returns empty due to file size, but `sed`/`grep` confirm the content.

**Key components in Design.tsx:**

- `DesignDiagram` (line ~7428) — ReactFlow diagram view
- `ModelPiece` (line ~9190) — 3D piece renderer
- `ModelDesign` (line ~9370) — Scene design container
- `DesignAppScene` (line ~9458) — Scene view wrapper
- `DesignApp` plugin registration (bottom of file)
- All `useDesignApp*` hooks (lines 2184-2848)
- Command handlers (lines 674-1114)

**Old files exist but are NOT used:**

- `Desing.tsx.old` (typo in name) — 241 lines, old DesignEditor with dnd-kit
- `Design.Details.tsx.old` — old details panel
- `Design.Diagram.tsx.old` — old diagram
- `Design.Model.tsx.old` — old model view

### 6. Test Structure (sketchpad.test.ts)

**File**: `compose/js/sketchpad.test.ts` — **4,598 lines** (Playwright e2e tests)

**Test sections:**

- `test("Home", ...)` — line 873 (180s timeout)
- `test("Kit", ...)` — line 1038 (180s timeout)
- `test("Type", ...)` — line 1751 (120s timeout)
- `test("Design", ...)` — line 2012 (600s timeout) ← **Main piece creation test**
- `test("Docs", ...)` — line 3755
- `test("Feedback", ...)` — line 3825
- `test("Panels", ...)` — line 4024 (300s timeout)

**Piece creation tests in Design test (line 2012-2400):**

1. **Drag-drop test** (line 2254-2319):
   - Drags type avatar from workbench to diagram drop zone
   - Falls back to dispatching `design-drag-end` custom event if native drag fails
   - Asserts: piece count +1, typeGuid matches, plane not null, center not null, model resolution works

2. **Plus button test** (line 2334-2370):
   - Clicks `[id="compose.sketchpad.app.design.panel.workbench.types.addPiece"]`
   - Asserts: piece count +1, typeGuid matches, plane not null, center = {u:0, v:0}, model resolution works

**Test helpers:**

- `getDesignPieces(page)` — line 608: evaluates store snapshot to get pieces with guid, name, plane, center, typeGuid
- `getSceneModelResolutionForPiece(page, pieceGuid)` — line 635: evaluates store to check model resolution chain (type → model → file)

### 7. Potential Bugs / Inconsistencies

1. **Missing `id_` on diagram drop**: `Design.tsx:7817` creates `{ guid, type, center, plane }` without `id_`. Scene drop at line 9532 includes `id_: pieceGuid`. The `PieceStore` constructor at `Sketchpad.tsx:3770` sets `this.localId = piece.guid` as fallback, so it works but is inconsistent.

2. **Center ↔ Plane coordinate coupling**: The formula `worldX = (center.u - 6) / 0.3` and inverse `center.u = worldX * 0.3 + 6` is hardcoded in three places (diagram drop, scene drop, workbench button). These magic numbers (6, 7, 0.3) should be constants.

3. **PieceMesh returns null when blobUrl is unavailable**: At `Design.tsx:9180`, if `blobUrl` is null (file not loaded yet or missing), the piece renders nothing. No fallback geometry.

4. **`compose.designApp.addPiece` does NOT add default plane**: The Design-level command at `Design.tsx:785` just passes through the piece as-is. The Kit-level command at `Sketchpad.tsx:7984` adds a default plane. So if a piece is added without plane via the Design app (which in practice doesn't happen because all three UI paths provide plane), it would have no plane.

5. **No center computation in `compose.kit.addPiece`**: The Kit-level fallback at `Sketchpad.tsx:7984` adds a default plane but NOT a center. If a piece comes in without center, it stays without center.

## Changes

No code changes — research only.

## Log

- Searched for addPiece/createPiece in Sketchpad.tsx and Design.tsx
- Traced the complete flow: UI event → useDesignAppAddPiece → designApp command → kitDiff → kitStore.change → DesignStore.createPiece → PieceStore constructor → Y.js
- Identified three UI entry points (diagram drop, scene drop, workbench button)
- Found model resolution chain: Type → Model → File → URL
- Confirmed Design.tsx is 10,697 lines (not empty)
- Reviewed test structure and test helpers

## Todos

- [x] Research addPiece/createPiece logic
- [x] Research placement data flow
- [x] Research scene piece rendering
- [x] Research model resolution
- [x] Confirm Design.tsx location/status
- [x] Review test structure

## Plan

Research-only ticket. All findings documented above.
