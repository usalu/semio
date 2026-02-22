---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Fixed Type scene filter propagation across GoldenLayout window roots by replacing React-context filter propagation with a shared module-level filter store, while keeping URL persistence via `filter` query params.

## Changes

- Updated `semio/js/sketchpad/Type.tsx` filter architecture:
- Removed Type filter context/provider usage for scene rendering.
- Added shared external filter store (`useSyncExternalStore`) to bridge main-toolbar root and GoldenLayout window root.
- Added strict parser for `filter` values (`connectors`, `models`).
- `TypeKindToggles` now updates both URL params and shared filter store.
- Scene rendering continues to use filter state:
- connectors off => connector visuals not rendered.
- models off => model mesh not rendered.
- Existing Type e2e URL assertions remain aligned to `filter=` in `semio/js/sketchpad.test.ts`.

## Log

- Investigated user report that scene still did not react to Type filters.
- Found root cause in `LayoutCanvas` implementation: GoldenLayout windows are mounted in separate React roots (`createRoot`) with independent `MemoryRouter` state, so context/router-based filter propagation from main toolbar does not reliably reach scene windows.
- Refactored Type filter state to shared module store and wired both toolbar and scene to that store.
- Ran targeted Type e2e twice:
- Command: `npm run test:e2e -- sketchpad.test.ts --grep "Type"`
- Result: both runs failed before Type assertions due existing setup issue in `initHome`: missing `[id="semio.sketchpad.app.home.importKit"]` attachment.

## Todos

- [x] Reopen Type filter ticket for follow-up.
- [x] Remove dependence on cross-root React context for Type scene filters.
- [x] Keep URL persistence using dedicated `filter` params.
- [x] Wire scene rendering to shared filter state.
- [x] Attempt targeted Type e2e verification.
- [ ] Stabilize upstream `initHome` setup flake in this environment (missing import input) and rerun end-to-end verification.

## Plan

1. Validate cross-root rendering constraints in `LayoutCanvas`.
2. Replace filter propagation mechanism with root-agnostic shared store.
3. Keep URL <-> filter synchronization from toolbar interactions.
4. Re-run Type e2e and record results.
5. Close ticket with current status and residual validation blocker.
