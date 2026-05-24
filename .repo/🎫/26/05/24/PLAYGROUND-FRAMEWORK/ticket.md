# Playground Framework and Spatial Play Wiring

**Goal:** elements architecture — framework core (TS classes), framework react renderer, playground framework, spatial play consumes playground via framework only.

**Status:** open

## Plan

- Finish `@elements/playground` bootstrap API (`bootstrapPlaygroundWorkbench`).
- Spatial play core (`index.ts`) framework + playground only; React host in `spatial-play-host.tsx`.
- Spatial play uses `bootstrapSpatialPlayWorkbench` + playground declarative bodies.

## Summary

- Added `bootstrapPlaygroundWorkbench` with optional declarative registration and existing workbench reuse.
- Spatial play extends `PlaygroundController`; removed duplicate declarative builders.
- Moved React mount to `elements/lib/react/spatial/spatial-play-host.tsx` (framework-react + geometry-spatial-react); play bundle imports framework/playground only from `index.ts`.

## Files

- `elements/lib/playground/index.ts`
- `elements/lib/react/spatial/play/index.ts`, `main.ts`, `package.json`
- `elements/lib/react/spatial/spatial-play-host.tsx` (new)
- `elements/lib/react/spatial/play/host.tsx` (removed)
