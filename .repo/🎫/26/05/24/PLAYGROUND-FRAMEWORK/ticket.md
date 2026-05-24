# Playground Framework and Spatial Play Wiring

**Goal:** elements architecture — framework core (TS classes), framework react renderer, playground framework, spatial play consumes playground via framework only.

**Status:** closed

## Plan

- Finish `@elements/playground` bootstrap API (`bootstrapPlaygroundWorkbench`).
- Spatial play core (`index.ts`) framework + playground only; React host in `spatial-play-host.tsx`.
- Spatial play uses `bootstrapSpatialPlayWorkbench` + playground declarative bodies.
- Extract shell bridge + `AppContext`/`useApp` into `@elements/framework-react`; `@elements/ui` re-exports.
- Shell chrome types live in framework-react (`shell-chrome-types.tsx`), not `@elements/ui`.

## Summary

- Added `bootstrapPlaygroundWorkbench` with optional declarative registration and existing workbench reuse.
- Spatial play extends `PlaygroundController`; removed duplicate declarative builders.
- Moved React mount to `elements/lib/react/spatial/spatial-play-host.tsx` (framework-react + geometry-spatial-react); play bundle imports framework/playground only from `index.ts`.
- Board play wired via `bootstrapBoardPlayWorkbench` + `registerPlaygroundSidePanelBodies`; tests pass.
- Restored corrupted `elements/lib/framework/core/index.ts` from git history.
- Styling generate script paths fixed under `elements/lib/styling`.
- **Shell extraction:** `shell-bridge.tsx` + `workbench-app-context.tsx` + `shell-chrome-types.tsx` in `@elements/framework-react`; `shell-bridge` no longer imports types from `@elements/ui`.
- `elements/lib/react/core/index.tsx` imports shell bridge from `@elements/framework-react` (`WorkbenchView` still implemented in `@elements/ui`).

## Tests

- `@elements/framework-react`: 1 passed
- `@elements/playground`: 3 passed
- `@elements/framework` core: 3 passed
- spatial play: 8 passed
- board play: 2 passed

## Files

- `elements/lib/playground/index.ts`
- `elements/lib/framework/renderer/react/shell-bridge.tsx`, `shell-chrome-types.tsx`, `workbench-app-context.tsx`, `index.tsx`, `workbench-bridge.tsx`, `package.json`
- `elements/lib/react/core/index.tsx`, `ui-declarative-renderer.tsx`
- `elements/lib/react/spatial/play/index.ts`, `main.ts`, `package.json`
- `elements/lib/react/spatial/spatial-play-host.tsx`
- `elements/lib/react/board/play/index.ts`, `board-play-host.tsx`
- `elements/lib/styling/script.ts`, `js/script.ts`, `project.json`
- `elements/lib/framework/core/index.ts`
