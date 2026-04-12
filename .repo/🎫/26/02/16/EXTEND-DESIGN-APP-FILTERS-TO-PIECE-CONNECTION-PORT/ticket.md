---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Extended Design app toolbar filters to include piece, connection, and port toggles with URL state persistence. Filters actually control rendering visibility in both diagram and 3D scene views.

## Changes

- `semio/assets/icons.ts`: Added `Puzzle as PieceIcon` export
- `semio/js/sketchpad/locales/en.json`: Added `showPorts` i18n entries under `design.toolbar`
- `semio/js/sketchpad/locales/de.json`: Added `showPorts` i18n entries under `design.toolbar`
- `semio/js/sketchpad/Design.tsx`:
  - Added `DesignFilterKind`, `DesignFilterState`, `DesignFilterContext`, `DesignFilterProvider`, `useDesignFilters` in Filters region
  - Replaced static `DesignToolbarFilters` with working URL-state toggles (pieces, connections, ports)
  - Wired `useDesignFilters()` into `PieceNodeInner` and `DesignNodeInner` to conditionally render port handles
  - Wired `useDesignFilters()` into `DesignDiagram` to set `hidden: true` on nodes/edges
  - Wired `useDesignFilters()` into `ModelDesign` to conditionally render 3D piece models
  - Wrapped `DesignApp` with `<DesignFilterProvider>`
- `semio/js/sketchpad/Sketchpad.tsx`: Fixed 11 merge conflicts and removed stale HUD panel references

## Log

- Explored repo structure via `tree` CLI
- Analyzed existing Type app filter pattern (URL state via `useSearchParams`)
- Added PieceIcon to assets
- Added i18n entries for port filter
- Implemented filter context, provider, hook
- Implemented toolbar filters with URL state persistence
- Wired filters to diagram rendering (nodes, edges)
- Wired filters to port/connector handle rendering
- Wired filters to 3D scene rendering
- Fixed pre-existing merge conflicts in Sketchpad.tsx
- Fixed pre-existing HUD panel references
- TypeScript compilation: 0 errors
- Vitest unit tests: 12/12 passing

## Todos

- [x] Add PieceIcon to assets/icons.ts
- [x] Add showPorts i18n entries (en + de)
- [x] Implement DesignToolbarFilters with URL state
- [x] Create DesignFilterContext/Provider/hook
- [x] Wire filter state to diagram rendering
- [x] Wire filter state to 3D scene rendering
- [x] Wrap DesignApp with DesignFilterProvider
- [x] Fix Sketchpad.tsx merge conflicts
- [x] Validate TypeScript compilation
- [x] Validate unit tests

## Plan

Follow the Type app filter pattern: URL-based state persistence using `useSearchParams`, React Context for state propagation, and conditional rendering based on filter state.
