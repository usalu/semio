# Playground Framework and Spatial Play Wiring

**Goal:** elements architecture — framework core (TS classes), framework react renderer, playground framework, spatial play consumes playground via framework only.

**Status:** closed

## Summary

- `@elements/playground`: `bootstrapPlaygroundWorkbench`, spatial play on `PlaygroundController`.
- `@elements/framework-react`: `WorkbenchView` + mount in `workbench-view.tsx` / `workbench-mount.tsx`; shell bridge + chrome types; `useUIHistory` in `workbench-history.tsx`.
- `@elements/ui`: re-exports workbench shell from framework-react; chrome primitives stay in ui barrel.
- Spatial play: framework-free `index.ts`; `spatial-play-host.tsx` for React.
- Board play: `bootstrapBoardPlayWorkbench` + `registerPlaygroundSidePanelBodies`.

## Tests

- framework-react: 3 passed (UiRenderer, WorkbenchView, useUIHistory types)
- playground: 3 passed
- spatial play: 8 passed
- board play: 2 passed

## Files

- `elements/lib/framework/renderer/react/workbench-view.tsx`, `workbench-mount.tsx`, `workbench-history.tsx`, `shell-chrome-types.tsx`, `shell-bridge.tsx`, `workbench-app-context.tsx`, `workbench-bridge.tsx`, `index.tsx`, `package.json`, `vitest.config.ts`
- `elements/lib/playground/index.ts`
- `elements/lib/react/core/index.tsx`
- `elements/lib/react/spatial/spatial-play-host.tsx`, `elements/lib/react/spatial/play/*`
- `elements/lib/react/board/play/index.ts`
