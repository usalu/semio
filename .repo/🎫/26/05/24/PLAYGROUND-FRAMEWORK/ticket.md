# Playground Framework and Spatial Play Wiring

**Goal:** elements architecture — framework core (TS classes), framework react renderer, playground framework, spatial play consumes playground via framework only.

**Status:** in progress

## Summary

- Renamed `@elements/ui-shell` → `@elements/framework` (pure TS workbench core).
- Added `@elements/playground` — one-app shell with selection/filter toolbars and workbench + details side tabs; `PlaygroundController` base class.
- Added `@elements/framework-react` — declarative `UiNode` renderer; `./workbench` subpath bridges `WorkbenchView` / `mountReactApp` (until full extraction from `@elements/ui`).
- Refactored `@elements/geometry-spatial-play` to extend playground; host imports framework + framework-react only (not `@elements/ui`).
- Workspace paths updated under `elements/lib/*`; all three test suites pass locally.

## Files

- `elements/lib/playground/*` (new)
- `elements/lib/framework/core/*` (renamed package)
- `elements/lib/framework/renderer/react/*` (new)
- `elements/lib/react/spatial/play/*` (refactored)
- `elements/lib/react/core/ui-declarative-renderer.tsx`, `package.json`, `index.tsx`
- `elements/lib/react/board/play/index.ts`, `board-play-host.tsx`
- `package.json` (workspaces)
- `semio/client/lib/sketchpad/js/*` (framework import)
