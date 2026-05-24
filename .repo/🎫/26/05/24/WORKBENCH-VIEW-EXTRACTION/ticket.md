# Workbench View Extraction

**Goal:** elements architecture — `@elements/framework-react` owns shell bridge + workbench chrome; break `@elements/ui` ↔ `@elements/framework-react` cycle.

**Status:** closed

## Update (WorkbenchView moved)

- `workbench-view.tsx` — full `WorkbenchView` + default panel tabs
- `workbench-mount.tsx` — `ReactUI`, `mountReactApp`, `mountAsyncReactApp`
- `@elements/ui` re-exports; exports `UICanvas`, `UISearch`, `UIFind`, `UIToolbar` for framework-react

## Plan

- Export `resolveElementIcon`; shell-bridge uses plain `SidePanelTabConfig` + `import type` only from `@elements/ui`.
- Subpath exports for shell-bridge, workbench-app-context, ui-declarative-renderer.
- `WorkbenchView` imports shell via subpaths (no barrel cycle).
- Later: move `WorkbenchView` body into `workbench-view.tsx`.

## Summary

- Broke `@elements/ui` ↔ `@elements/framework-react` barrel cycle via subpath imports.
- `shell-bridge` no longer runtime-imports `@elements/ui` (plain `SidePanelTabConfig`, `import type` only).
- Added `resolveElementIcon`; fixed `WorkbenchView` navbar/search icon lookups.
- `ui-declarative-renderer` re-export uses `@elements/framework-react/ui-declarative-renderer` subpath.
- Vitest aliases updated; declarative renderer test uses detached `UiRenderer` node (canvas-only window rule).

## Tests

- `@elements/ui`: 47 passed
- `@elements/framework-react`: 1 passed
- playground: 3, spatial play: 8, board play: 2
