---
name: scene 3d board pendant
overview: "Build `@elements/scene` as the 3D counterpart of `@elements/board`: R3F-based React component with the same kinds, compatibility and connect mechanisms (Indirect / Connect / Proximity), swappable glb meshes, central object pool, chunking for infinite worlds, and relocate (Translate/Rotate/Scale) instead of drag. Ship a Nakagin play site at JSON parity with the board fixture, fed by a baked `nakagin-capsule-tower.scene.json` produced from the flattened semio design with planes converted into three.js (Y-up, right-handed) origins + orientation quaternions."
todos:
  - id: bootstrap
    content: Create scene package.json, project.json, script.ts (dev/build/test/bake-nakagin), vite.config.ts, index.html
    status: pending
  - id: runtime
    content: "Implement react/index.tsx: Scene, Object, Vortex, Magnet, Tie, Attraction with regions"
    status: pending
  - id: pool
    content: Implement MeshPool (useGLTF + InstancedMesh, refcounted) and Chunking (per-chunk groups, frustum + radius cull)
    status: pending
  - id: interact
    content: Implement Selection + Relocate (Translate/Rotate/Scale) + Connect/Indirect/Proximity compat checks against kindCatalogs
    status: pending
  - id: coords
    content: Implement semio-plane → three (Y-up) origin + quaternion conversion helper with golden test
    status: pending
  - id: bake
    content: Write bake-nakagin script that flattens shallow design + light kit, maps glbs, writes scene fixture with board-parity ids
    status: pending
  - id: play
    content: Build play site with @elements/ui shell, fixture shelf, selection inspector, relocate-mode toolbar
    status: pending
  - id: tests
    content: Add vitest specs (fixture roundtrip, pool, chunk, coord, connect) + playwright e2e for nakagin scene
    status: pending
isProject: false
---

# Scene 3D Board Pendant

## Terminology mapping

- Board → Scene
- Node → Object (+ `meshUrl` instead of `shape`/`iconKind`, `origin`+`orientation` instead of `x`/`y`)
- Edge → Tie (`source: "<objectId>:<vortexId>"` / `target`)
- Handle → Vortex (3D `position` + `direction` vec; keep `vortexKind` ≈ board `handleKind`)
- Wire → Attraction (compatibility + transient connect preview)
- Magnet (per-object snap geometry, same role as board)

Compatibility/connect rules reuse the board catalog format verbatim (`source`/`target`/`bidirectional`/`specificity`) so authors can copy them between board and scene with zero translation.

## Architecture

`./elements/client/lib/scene` (independent from board, no react-reconciler / Rust / WASM):

- [elements/client/lib/scene/react/index.tsx](elements/client/lib/scene/react/index.tsx) — declarative R3F components.
  - `//#region 🔖Kinds` shared prop types (mirrors `BoardCanvasProps` shape: `kindCatalogs`, `kindCompatibility`, `selectionMode`, `selectionTargets`, `lodZoomThresholds`, `onSelect`, `onConnect`, `onProximityConnect`, `onIndirectConnect`, `onRelocate`, …).
  - `//#region 🎬Scene` `<Scene>` = `<Canvas>` (R3F) + `<SceneRoot>` providing context, camera, lighting, chunking grid, fog.
  - `//#region 🧊Object` `<Object>` renders a pooled mesh via `meshUrl` + origin/quaternion. Children = `<Vortex>` (visual ring on a 3D position) and `<Magnet>` (snap surface).
  - `//#region 🪢Tie` `<Tie>` draws a 3D line between two vortex world positions (resolved via context registry).
  - `//#region 🧲Attraction` `<Attraction>` = transient connect wire shown during relocate-connect.
  - `//#region 🎯Selection` selection store + picking (R3F raycaster) honoring `selectionTargets`.
  - `//#region ✋Relocate` replaces board "drag". Modes: `Translate`, `Rotate`, `Scale` (DREI `<TransformControls>` per mode). Pointer-down on a selected object enters relocate; modifier or toolbar switches mode. Commit emits `onRelocate({mode, before, after})`. While translating, runs proximity/connect compat checks against neighboring vortices and emits `onProximityConnect` / `onConnect` / `onIndirectConnect` mirroring board semantics.
  - `//#region 🏊Pool` central `MeshPool` (Map<url, {gltf, geometries, materials, refCount}>) using `useGLTF.preload` + `InstancedMesh` per unique mesh — objects acquire/release on mount/unmount; identical `meshUrl` share one instanced draw call.
  - `//#region 🧱Chunking` world partitioned into fixed-size cubes (default 256 m). `useChunkVisibility` culls objects/ties outside camera frustum + LOD radius from camera. Each chunk owns its own `<group>` so React can mount/unmount per chunk; `onCamera` triggers reactive chunk visibility.
  - `//#region 🧾Fixture` `parseSceneFixtureV1` / `encodeSceneFixtureForDragV1` mirroring `parseBoardFixtureV1`.

- [elements/client/lib/scene/play/index.tsx](elements/client/lib/scene/play/index.tsx) + `index.html` + `vite.config.ts` — copy of board play harness shape (UI shell, fixture drag shelf, selection inspector, toolbar with Select/Translate/Rotate/Scale + Connect/Indirect/Proximity toggles, level provider) but mounting `<Scene>` and reading `nakagin-capsule-tower.scene.json`.

- [elements/client/lib/scene/package.json](elements/client/lib/scene/package.json) deps: `react`, `react-dom`, `three`, `@react-three/fiber`, `@react-three/drei`, `@elements/ui`, `lucide-react`. No Rust/wasm.

- [elements/client/lib/scene/project.json](elements/client/lib/scene/project.json) targets: `dev`, `build`, `test` calling `bun ./script.ts`. `SCENE_PLAY_PORT=6013` (dev), `6028` (test).

- [elements/client/lib/scene/script.ts](elements/client/lib/scene/script.ts) modeled on board's `script.ts` (no cargo/wasm step): `dev` → vite, `build` → vite build, `test` → vitest + playwright.

- [elements/client/lib/scene/fixtures/meshes/](elements/client/lib/scene/fixtures/meshes/) — `package.json`-free static folder; play `vite.config.ts` aliases `/meshes/*` to `semio/assets/fixtures/metabolism/representations/*` via `server.fs.allow` (same pattern board uses for fixture json).

## Coordinate system

Semio plane `(origin, xAxis, yAxis)` is right-handed with `zAxis = xAxis × yAxis`. Three.js uses Y-up, right-handed. Apply the mapping `(x, y, z)_semio → (x, z, -y)_three` (rotate -90° about world X) consistently for origin, axes and derived quaternion. Quaternion is built from the rotated basis (`new Matrix4().makeBasis(x', y', z')` → `Quaternion.setFromRotationMatrix`).

## Scene fixture schema (parity with board)

```json
{
  "schema": "elements.scene.fixture/v1",
  "camera": { "position": [x,y,z], "target": [x,y,z], "zoom": 1 },
  "kindCatalogs": { /* same shape as board: vortices/attractions/objects/ties + compat */ },
  "ties": [{ "id", "source": "<objectId>:<vortexId>", "target": "<objectId>:<vortexId>" }],
  "objects": [{
    "id", "label", "meshUrl", "objectKind", "scale",
    "origin": [x,y,z], "orientation": [qx,qy,qz,qw],
    "vortices": [{ "id", "vortexKind", "position": [x,y,z], "direction": [x,y,z], "radius" }]
  }]
}
```

Keys, ids, kindCatalogs and tie wiring are 1:1 with `nakagin-capsule-tower.board.json` so the same Nakagin compat catalog drives both surfaces.

## Baking the Nakagin scene fixture

One-off offline bake (no runtime semio dependency in the play site):

- Add a `bake-nakagin` subcommand to [elements/client/lib/scene/script.ts](elements/client/lib/scene/script.ts) that:
  1. Loads `semio/assets/fixtures/nakagin-capsule-tower.shallow.design.semio.json` + `semio/assets/fixtures/metabolism.kit.light.semio.json`.
  2. Calls the existing JS `flattenDesign` (see `semio/client/lib/react/rendering/index.tsx:4839` `// JS-flattened design (correct BFS placement centers)`) to materialize each piece's plane and connector world points.
  3. For every piece → emits an `object` with same `id` used in the board fixture (preserves tie ids by reusing connector `port` → vortex id mapping `<pieceId>:<connectorId>`).
  4. Resolves `meshUrl` from the piece's type representation file (`*.glb`) under `semio/assets/fixtures/metabolism/representations/` — aliased to `/meshes/<filename>.glb` at play-site serve time.
  5. Converts plane → three quaternion + origin via the mapping above.
  6. Copies `kindCatalogs` and tie list straight from the board fixture (ids match because both flow from the same kit), then writes `elements/client/lib/scene/fixtures/nakagin-capsule-tower.scene.json`.

Output is checked in; play site only imports the JSON. Re-bake whenever the board fixture is regenerated.

## Pool, chunking, relocate details

- Pool: `MeshPool` uses `useGLTF` to load each unique `meshUrl` once; objects of the same kind share an `InstancedMesh`. Acquire/release driven by mount/unmount; LRU eviction on chunk unload.
- Chunking: world `chunkSize` is a scene prop (default 256). `<SceneChunks>` groups objects by `floor(origin / chunkSize)`. Per-frame visibility test via `camera.frustum` + max-distance ring; offscreen chunks unmount their `<Object>`s (releasing pool refs).
- Relocate: `useRelocate(objectId)` returns `{mode, setMode, start, update, commit, cancel}`. Translate uses DREI `TransformControls mode="translate"`; rotate/scale analogous. During translate, every frame compares each magnet's world AABB against neighboring magnets via the chunk's spatial index (proximity radius from `kindCatalogs`) and fires `onProximityConnect` on snap, `onConnect` on release-over-vortex, `onIndirectConnect` on follow-up click on indirect ring.

## Tests

Extend (do not create new) [elements/client/lib/board/vitest.config.ts](elements/client/lib/board/vitest.config.ts) sibling — i.e. add a single `vitest.config.ts` in `scene/` mirroring board's and add specs alongside `react/index.tsx` and `play/index.tsx` for:
- fixture round-trip (`parseSceneFixtureV1` ↔ JSON parity with board ids),
- pool acquire/release reference counting,
- chunk visibility cull math,
- relocate translate → proximity-connect emits `onProximityConnect` with the right tie payload,
- coordinate conversion (plane → three quaternion) golden values.

Playwright play-site e2e: load Nakagin scene, assert glb meshes render, click an object, translate it, drop on a compatible vortex → assert `onConnect` callback observed via instrumentation div.

## Ticket flow

1. `ticket_open` "Extend Elements With Scene 3D" under the most appropriate goal (read `repo://goals` first).
2. Wire `package.json` workspace entry + nx project graph; run `bun install`.
3. Implement scene runtime in `react/index.tsx` (regions: Kinds, Scene, Object, Vortex, Magnet, Tie, Attraction, Pool, Chunking, Selection, Relocate, Fixture).
4. Implement bake script + run it to produce `nakagin-capsule-tower.scene.json`.
5. Implement play site reusing the `@elements/ui` `UI` shell, toolbar, fixture shelf.
6. Cargo/wasm not needed; ensure `script.ts test` runs vitest + playwright green.
7. `ticket_close` with file list.