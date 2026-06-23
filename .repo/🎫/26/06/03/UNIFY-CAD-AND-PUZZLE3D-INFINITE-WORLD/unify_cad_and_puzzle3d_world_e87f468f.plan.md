---
name: Unify CAD And Puzzle3D World
overview: Migrate @semio-tech/infinite-world-r3f and puzzle/3d to a native z-up scene (dropping the CAD->Three remap), then rebuild cad/js/renderer's canvas/orbit/grid/scene on the same engine so both consume one infinite-world mechanism including chunking, view-radius, pooling, and LOD grid.
todos:
  - id: ticket
    content: Open/reopen the infinite-world ticket via repo MCP and associate with the appropriate goal.
    status: completed
  - id: engine-zup
    content: "Make @semio-tech/infinite-world-r3f native z-up: identity remap in Precision, rotate WorldLodGridHelper grids into XY plane, keep GLB mesh rotation; update engine tests."
    status: completed
  - id: engine-shell
    content: "Generalize WorldCanvas into the shared shell: cameraUp/position/fov/near/far, dpr, shadows, gl, background, frameloop, onCanvasReady, host pointer callbacks, optional owned PerspectiveCamera."
    status: completed
  - id: puzzle-zup
    content: "Adapt puzzle/3d to z-up: set camera up=[0,0,1], verify lights/grid/mesh standing, fix coordinate-dependent tests."
    status: completed
  - id: cad-canvas
    content: Rebuild cad InteractionCanvas + SpatialOrbitControls on WorldCanvas + WorldOrbitGated; add @semio-tech/infinite-world-r3f dep and vite/vitest aliases.
    status: completed
  - id: cad-grid-layers
    content: Replace cad fixed GridHelper with WorldLodGridHelper + LOD provider; compose InteractionSpatialView contents as ordered WorldLayers.
    status: completed
  - id: cad-chunk-pool
    content: Adopt chunking + view-radius for committed meshes (keyed by origin) and asset pooling (createTemplatePool/createRefCountPool) in cad.
    status: completed
  - id: validate
    content: Run world/puzzle/cad tests and smoke both plays (z-up grid, orbit, LOD, pick/gumball, chunk unload); confirm via console logs.
    status: completed
isProject: false
---

## Goal

Both `puzzle/3d` and `cad/js/renderer` render through one shared `@semio-tech/infinite-world-r3f` engine. Per decisions: the engine + puzzle move to native z-up (drop the `cadVec3ToThree` remap), and cad adopts the full layer set (chunking, view-radius, pooling, LOD grid).

## Current state

- `@semio-tech/infinite-world-r3f` ([infinite/world/r3f/index.tsx](infinite/world/r3f/index.tsx)) bakes in a CAD-z-up -> Three-y-up remap: `_eulerCadToThree = new Euler(-PI/2,0,0)` (line 71), used by `cadVec3ToThree`/`threeVec3ToCad`/`cadQuatToThree`/`threeQuatToCad` and `WorldLodGridHelper` (line 545). `WorldCanvas` (line 779) hardcodes `frameloop="demand"`, owns no camera, forwards no host pointer callbacks.
- `puzzle/3d` ([puzzle/3d/react/index.tsx](puzzle/3d/react/index.tsx)) consumes the engine with a default y-up `<PerspectiveCamera makeDefault>` (line 8306) and 58 remap call sites.
- `cad/js/renderer` ([cad/js/renderer/index.tsx](cad/js/renderer/index.tsx)) is a separate stack: native z-up `InteractionCanvas` (`up:[0,0,1]`, line 2922), single fixed `GridHelper` rotated to XY at z=0 (line 3058), `SpatialOrbitControls` (line 2872), `SpatialAutoFit`/`SpatialInvalidator`. No chunking/LOD/pooling. Consumed by `cad/js/renderer/play` and aliased by `framework/product/platform/renderer/react` + sketchpad (public API is CAD coords, so scene-convention change is internal).

## Stage 1 - Engine: native z-up + generalized shell

In [infinite/world/r3f/index.tsx](infinite/world/r3f/index.tsx):

- Precision region: change `_eulerCadToThree` to identity (no rotation) so `cadToThreeMatrix()` is identity and `cadVec3ToThree`/`threeVec3ToCad`/`cadQuatToThree`/`threeQuatToCad`/`cadObjectLocalToThreeGroupLocal`/`...Direction` become passthroughs (model frame == scene frame, both z-up). Keep the function names so puzzle's 58 call sites and cad keep compiling. Update their docstrings to reflect z-up identity.
- Keep `GLB_MESH_FRAME_ROTATION_X = PI/2` (glTF y-up mesh -> z-up standing); verify sign at runtime.
- `WorldLodGridHelper` (line 523): rotate each `GridHelper` into the XY ground plane (`grid.rotation.x = Math.PI/2`, matching cad line 3061) and keep a tiny z offset to avoid z-fighting; placement stays via `gridPlacementAnchorCad` (already pans XY, fixed z datum).
- Generalize `WorldCanvas` (line 779) into the shared shell for both apps: add props for `cameraUp` (default `[0,0,1]`), `cameraPosition`, `cameraFov`, `cameraNear`, `cameraFar`, `dpr`, `shadows`, `gl`, `background`, `frameloop` (default `demand`), `onCanvasReady`, and host pointer/`on*` callbacks (mirroring `InteractionCanvasProps` lines 2748-2772). Internally mount an optional `<PerspectiveCamera makeDefault up={cameraUp} ...>` when `cameraPosition` is provided, and forward callbacks to `<Canvas>`. Specializations may still pass their own camera child.
- Tests region (line 803): update `cadVec3ToThree` assertions to identity expectations.

## Stage 2 - puzzle/3d adapts to z-up

In [puzzle/3d/react/index.tsx](puzzle/3d/react/index.tsx):

- Set camera up: `<PerspectiveCamera ... up={[0,0,1]}>` (line 8306) so orbit/controls use z-up.
- Verify the directional light direction (line 8336) and grid still read correctly in z-up; adjust light position if needed.
- The 58 remap call sites now no-op; confirm bounds/auto-fit/selection-zoom/marquee/gumball still behave (they operate in CAD coords == scene coords).
- Fix coordinate-dependent in-file tests: `cadVec3ToThree`/`threeVec3ToCad` round-trip (lines 8777-8807), camera target tests (lines 9701-9710), pose tests (lines 10008-10017); keep `frame.rotation.x === PI/2` (line 10104).

## Stage 3 - cad/js/renderer onto the engine (full layer set)

In [cad/js/renderer/index.tsx](cad/js/renderer/index.tsx):

- Add `@semio-tech/infinite-world-r3f` dependency in [cad/js/renderer/package.json](cad/js/renderer/package.json) and a vite alias where cad is consumed ([cad/js/renderer/play/vite.config.ts](cad/js/renderer/play/vite.config.ts), and the `@semio-tech/infinite-world-r3f` alias in [framework/product/platform/renderer/react/vitest.config.ts](framework/product/platform/renderer/react/vitest.config.ts) and sketchpad vite config if it imports cad).
- `InteractionCanvas` (line 2891): re-implement on top of the generalized `WorldCanvas`, passing `cameraUp=[0,0,1]`, existing camera defaults (`position [10,10,8]`, `fov 45`, near/far), `background`, `frameloop`, `gl`, and all host pointer callbacks + `onCanvasReady`. Wrap children in `WorldLayerStack` (handled by WorldCanvas).
- `SpatialOrbitControls` (line 2872): replace with `WorldOrbitGated` (same LEFT-disabled / MIDDLE-dolly / RIGHT-rotate mapping); wire `onCameraNavigate` through its gate. Drop `SpatialInvalidator` in favor of engine demand-frame kicks (or keep as a thin layer if camera-move invalidation differs).
- Grid: replace the fixed `GridHelper` (line 3058) with `WorldLodGridHelper` + a `LodBridge`/`useLod` provider so cad gets progressive LOD grid bands like puzzle.
- Compose `InteractionSpatialView` (line 3104) scene contents as ordered `WorldLayer`s: grid, ground-pick plane, committed-mesh layer, display layer, gumball/overlays.
- Adopt chunking + view-radius: route `CommittedMeshLayer` meshes (which carry `origin`) through `WorldChunkedSceneChildren` keyed by origin so far meshes unload; tune `chunkSize`/`maxDistance` for CAD scale.
- Adopt pooling: back committed/preview mesh materials/geometries with `createTemplatePool`/`createRefCountPool` to reuse styled meshes across revisions.

## Stage 4 - Wire + validate

- Update [.vscode/launch.json](.vscode/launch.json) only if new run/test entries are needed (existing cad `dev`/`test` and world `r3f` test entries already exist).
- Run `@semio-tech/infinite-world-r3f` tests, `@semio-tech/puzzle-3d-react` tests (262), and `@semio-tech/cad-js-renderer` tests; fix fallout.
- Smoke both plays in the browser: puzzle/3d play (Nakagin tower renders, grid is horizontal XY, orbit z-up, LOD bands), and cad play port 6020 (model renders z-up, LOD grid, orbit, pick/gumball intact, meshes chunk/unload at distance). Confirm runtime via console logs per repo rules.

## Notes / risks

- Lowest-churn coordinate migration: make the remap identity rather than deleting 58 call sites; optional follow-up to inline the now-identity helpers.
- Main runtime risks: grid plane orientation (must rotate to XY in z-up), glTF mesh standing rotation sign, orbit `camera.up` set before controls mount, and cad's pick-plane/gumball math which already assumes z-up (low risk).
- Repo workflow: do this inside a ticket via the repo MCP (reopen the existing infinite-world ticket if present, else open a new one); keep temp files under the ticket folder.
