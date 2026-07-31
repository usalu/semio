# Agent session — R3F renderer + play

## Summary

- Implemented `spatial/js/renderer-r3f/index.tsx`: `computeBoxPreviewLayout`, `FactoryDisplay`, `GroundPickPlane`, `createR3FInteractionAdapter`, `TessellatedCommitMesh`, `useFactoryRuntime`, `useFactorySnapshot` (`useSyncExternalStore`), `FactoryCanvas`, `FactorySpatialView`.
- Removed accidental duplicate second half of `index.tsx` (old `InMemoryKernel` + conflicting exports).
- Added Vite play app: `play/index.html`, `play/main.tsx`, `play/vite.config.ts` with correct aliases (`../../core`, `../../kernel-brepjs`).
- Renderer `tsconfig`: dropped `rootDir` (path-mapped core); `Uint32BufferAttribute` for indexed commit mesh.
- Vitest: layout + interaction adapter + stub-kernel snapshot tests (6 tests).

## Commands

- `bunx vitest run --config spatial/js/renderer-r3f/vitest.config.ts` — pass
- `bun nx run @spatial/js-renderer-r3f:build` — pass

## Files touched

- `spatial/js/renderer-r3f/index.tsx`
- `spatial/js/renderer-r3f/play/index.html`
- `spatial/js/renderer-r3f/play/main.tsx`
- `spatial/js/renderer-r3f/play/vite.config.ts`
- `spatial/js/renderer-r3f/vitest.config.ts`
- `spatial/js/renderer-r3f/tsconfig.json`
