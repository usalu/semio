# Spatial Brepjs Refactor

**Repo MCP:** unavailable; ticket recorded manually.

## Summary

Refactored spatial brepjs integration to match official brepjs docs and playground patterns:

- Replaced `MeshPreview` with `MeshTransfer` (grouped buffers, B-Rep edge polylines, face/edge metadata).
- `BrepjsWasmEngine` runs WASM; `BrepjsKernel` facade delegates via `BrepjsWorkerClient` (worker in browser, local engine in vitest).
- Tessellation uses `mesh` + `meshEdges` + `toGroupedBufferGeometryData` + `toLineGeometryData` with LRU cache and `disposeCell`.
- Renderer: `TessellatedCommitMesh`, `CommittedEdgeOverlay`, `useTessellation`, `findFaceGroupAt`, `SpatialInvalidator`.
- Fixed WASM path for vitest (`fileURLToPath`), selfMerge uses `getFaces`, standalone recording stub in core tests.

## Files

- `spatial/js/core/index.ts`
- `spatial/js/kernel-brepjs/index.ts`
- `spatial/js/renderer-r3f/index.tsx`

## Tests

- `@spatial/js-kernel-brepjs`: 26 passed
- `@spatial/js-renderer-r3f`: 64 passed
- `@spatial/js-core`: 110 passed
