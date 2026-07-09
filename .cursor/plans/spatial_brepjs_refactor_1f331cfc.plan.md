---
name: spatial brepjs refactor
overview: "Refactor `spatial/js/kernel-brepjs` and `spatial/js/renderer-r3f` to match the official brepjs playground architecture: kernel runs in a Web Worker with `using`/`DisposalScope` memory management, ships zero-copy grouped buffer geometry + face/edge groups to the main thread, and the R3F renderer consumes those outputs through a thin geometry layer with demand-frameloop and content-keyed caches. Single-file per package, organized with regions."
todos:
 - id: ticket
   content: Open/reopen ticket 2026/05/25/SPATIAL-BREPJS-REFACTOR via repo MCP.
   status: completed
 - id: core-types
   content: Replace MeshPreview with MeshTransfer (+ FaceGroup, EdgeGroup, FaceInfo, EdgeInfo) in spatial/js/core/index.ts; update SpatialKernel.tessellate to return Promise<MeshTransfer>.
   status: completed
 - id: kernel-worker
   content: "In spatial/js/kernel-brepjs/index.ts: add Worker entry + protocol regions; move WASM init, brepjs builders, and tessellation into worker; wrap every brepjs handle in using/DisposalScope; remove brepjsScratch shared mutable buffers."
   status: completed
 - id: kernel-client
   content: Add main-thread BrepjsKernel client (same SpatialKernel API) that posts to the worker, awaits MeshTransfer, owns the content-keyed LRU cache.
   status: completed
 - id: renderer-mesh
   content: Rewrite TessellatedCommitMesh in renderer-r3f/index.tsx to build BufferGeometry from MeshTransfer (position/normal/index + addGroup per faceGroup, polygonOffset) and dispose on unmount.
   status: completed
 - id: renderer-edges
   content: Add EdgeOverlay LineSegments from MeshTransfer.edges (replace ad-hoc wireframe).
   status: completed
 - id: renderer-pick
   content: Switch topology picking to event.faceIndex + faceGroups binary search; keep raycast=none on visuals; preserve existing camera-perf fixes (demand frameloop, DOM hover, deferred derived refresh).
   status: completed
 - id: renderer-worker-hook
   content: Add useTessellation hook that requests through the worker client, debounces via rAF, swaps geometries with proper dispose.
   status: completed
 - id: tests
   content: Extend existing vitest blocks (kernel-brepjs, renderer-r3f, core) to cover MeshTransfer, faceGroups mapping, cache invalidation, geometry disposal accounting.
   status: completed
 - id: validate
   content: Run nx tests, manual REPL freeze + heap-growth smoke (50x box commit loop, log usedJSHeapSize), strip [DEBUG] logs.
   status: completed
 - id: ticket-close
   content: Close ticket via repo MCP with summary + touched files.
   status: completed
isProject: false
---

## Goals

- Stop main-thread freezes by hosting brepjs/OpenCascade in a Web Worker (matches `temp/brepjs/apps/playground/src/workers/cad.worker.ts`).
- Stop kernel-handle leaks by wrapping every brepjs call in `using` / `DisposalScope` per `temp/brepjs/doc/memory-management.md`.
- Replace the home-grown `MeshPreview = { positions, indices, normals? }` with brepjs's official `toGroupedBufferGeometryData` + `toLineGeometryData` outputs (Transferable typed arrays, face/edge groups for picking).
- Keep `pick/camera-performance` fixes from the SPATIAL-PICK-CAMERA-PERFORMANCE ticket (raycast=none for visuals, demand frameloop, DOM-pointer hover, deferred derived refresh).

## Reference implementation to mirror

- Worker shape: [temp/brepjs/apps/playground/src/workers/cad.worker.ts](temp/brepjs/apps/playground/src/workers/cad.worker.ts) — `loadWasmBuild` (cached fetch), `brepjs.initFromOC(oc)`, `mesh()` + `meshEdges()` + `toGroupedBufferGeometryData` + `toLineGeometryData`, transferables list, content cache.
- Worker protocol: [temp/brepjs/apps/playground/src/workers/workerProtocol.ts](temp/brepjs/apps/playground/src/workers/workerProtocol.ts) — `MeshTransfer { position, normal, index, edges, faceGroups, edgeGroups, faceInfos, edgeInfos }`.
- R3F consumer: [temp/brepjs/apps/playground/src/components/playground/ShapeRenderer.tsx](temp/brepjs/apps/playground/src/components/playground/ShapeRenderer.tsx) (BufferGeometry from typed arrays, `findFaceGroupAt` binary search on `event.faceIndex`), [EdgeRenderer.tsx](temp/brepjs/apps/playground/src/components/playground/EdgeRenderer.tsx), [ViewerPanel.tsx](temp/brepjs/apps/playground/src/components/playground/ViewerPanel.tsx) (`Invalidator`, demand frameloop, `AutoFit`).

## Ticket

- Reopen / create ticket `2026/05/25/SPATIAL-BREPJS-REFACTOR` via repo MCP. All temp logs/screens land in that ticket folder.

## Phase 1 — Kernel Worker ([spatial/js/kernel-brepjs/index.ts](spatial/js/kernel-brepjs/index.ts))

Single-file, regions:

- `🌐BrepjsWorker` — Worker entry. Imported via `new Worker(new URL("./index.ts?worker_entry", import.meta.url), { type: "module" })`. Vite picks this up; vitest stub via `?worker_entry` query like brepjs playground.
- `📨WorkerProtocol` — message kinds: `init`, `init-done/progress/error`, `tessellate { id, cellSpec, tolerance }`, `tessellate-result { id, mesh: MeshTransfer }`, `cancel`, `dispose-cell`. Mirrors `workerProtocol.ts` field shape.
- `🧊MeshTransfer` — replaces `MeshPreview`. Shape:

```ts
type MeshTransfer = {
 position: Float32Array;
 normal: Float32Array;
 index: Uint32Array;
 edges: Float32Array;
 faceGroups: { start: number; count: number; faceId: number }[];
 edgeGroups: { start: number; count: number; edgeId: number }[];
 faceInfos: { faceId: number; surfaceType: string; area: number; normal: [number, number, number] }[];
 edgeInfos: { edgeId: number; curveType: string; length: number }[];
};
```

Re-export this from `@spatial/js-core` and remove `MeshPreview`.

- `🔌BrepjsKernel` (worker-side) — converts received `cellSpec` to `ValidSolid` using the existing builder logic (`topoWireToOrientedFace`, `extrudeTopoWire`, `cellSolidToBrep`, …). Every helper uses `using scope = new DisposalScope()` and registers intermediate handles. Exposed kernel ops `tessellate`, `cellSolid`, `executeCommandDiff`, `offsetFace`, … move into the worker. The main-thread `BrepjsKernel` class becomes a thin client that posts messages and returns Promises (the `SpatialKernel` interface in `@spatial/js-core` is already async).
- `♻️Cache` — content-key cache `(topologyHash + tolerance) → MeshTransfer` like `codeCache`. LRU cap. On `dispose-cell`, drop entries.
- `🧮PreciseSpatialKernelMath` stays main-thread (pure math, no WASM). `R3FPreviewKernel` move to renderer file (it belongs to the R3F host).

Memory rules applied throughout:

- All `mesh()`, `meshEdges()`, `getFaces()`, `getEdges()`, `castShape()`, boolean ops wrapped in `using` or `scope.register`.
- Drop `brepjsScratch` aliasing — it currently mutates shared `Vec3[]` arrays across calls, which causes the "jankiness" when two operations interleave. Use scope-registered locals instead.
- `meshPreviewFromBrep` → `meshTransferFromBrep(solid, tolerance)` that builds grouped + line data.

## Phase 2 — Renderer ([spatial/js/renderer-r3f/index.tsx](spatial/js/renderer-r3f/index.tsx))

Edit-in-place, regions reorganized to:

- `📥Imports` (drop `MeshPreview`, add `MeshTransfer`).
- `⚡PreviewKernel` — keeps `R3FPreviewKernel`.
- `🎬WorkerClient` — `useTessellation(cellRef, tolerance)` hook: subscribes to the kernel worker, debounces requests via `requestAnimationFrame`, returns `MeshTransfer | null`. Disposes prior `BufferGeometry` on swap.
- `🧊CommittedMesh` — rewritten to use `MeshTransfer`:

```tsx
const geo = new THREE.BufferGeometry();
geo.setAttribute("position", new THREE.Float32BufferAttribute(data.position, 3));
geo.setAttribute("normal", new THREE.Float32BufferAttribute(data.normal, 3));
geo.setIndex(new THREE.BufferAttribute(data.index, 1));
for (const g of data.faceGroups) geo.addGroup(g.start, g.count, 0);
```

With `polygonOffset` (per [threejs-integration.md](temp/brepjs/doc/threejs-integration.md)). `useEffect` cleanup calls `geo.dispose()`.

- `🧲EdgeOverlay` — new `LineSegments` from `data.edges` (replaces ad-hoc wireframe).
- `🧲TopologyTargets` — picking now uses `event.faceIndex` + binary search on `faceGroups` to map → `faceId` → topology, instead of one raycast mesh per target. Keeps `raycast={raycastNone}` on visuals.
- `🪩Canvas` — `frameloop="demand"` + `Invalidator` (camera-moved check) + `AutoFit` modeled on playground.
- `🪩Repl` — adapt to async `useTessellation`. Pending requests show a low-tolerance preview; final mesh swaps in on resolve.
- `🧪Tests` — extend existing tests in this file (no new files). Cover: (1) `MeshTransfer` deserialization, (2) `faceGroups` → `faceId` mapping, (3) cache invalidation on `dispose-cell`, (4) no orphan `BufferGeometry` after unmount (count via `THREE.BufferGeometry` ctor spy).

## Phase 3 — Core types ([spatial/js/core/index.ts](spatial/js/core/index.ts))

- Replace `MeshPreview` exports with `MeshTransfer`, `FaceGroup`, `EdgeGroup`, `FaceInfo`, `EdgeInfo`.
- Update `SpatialKernel.tessellate(cell, tolerance): Promise<MeshTransfer>` signature.
- Update `meshFaceTopologyDiff(mesh, tag)` to read `faceGroups` instead of legacy positions/indices.

## Phase 4 — Validation

- Run vitest in `kernel-brepjs` and `renderer-r3f` via existing `script.ts` commands (no new launch entries needed; existing test entries already wired).
- Manual REPL smoke: commit a box, orbit during finalize, confirm no freeze, confirm pick still resolves on faces/edges. Logs prefixed `[DEBUG] ` and removed before close.
- `performance.memory.usedJSHeapSize` logged before/after a 50× box-commit loop to confirm no growth (was previously leaking on every commit).

## Out of scope

- Kernel swap to `brepkit-wasm` / `occt-wasm` (covered by separate brepjs `kernel-swap.md`).
- Splitting `renderer-r3f/index.tsx` into multiple files (user chose single-file).
- Touching `coda` / `elements` / `reuse`.
