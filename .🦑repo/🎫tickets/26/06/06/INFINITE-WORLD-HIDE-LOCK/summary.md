# Infinite World Hide Lock

## Summary

Implemented a shared persisted hide/lock mechanism for CAD and puzzle 3d, wired through the shared Tree UI.

### Shared engine (`@semio-tech/infinite-world-r3f`)

- `WorldEntityFlags`, `WORLD_LOCKED_OPACITY_SCALE`, `WORLD_LOCKED_DESATURATION`
- Pure helpers: `worldEntitySelectable`, `worldEntityRendered`, `worldEntityRenderMode`
- In-file tests for flag helpers

### Puzzle 3d

- `hidden`/`locked` on `ObjectProps`, `VortexProps`, `AttractionProps` (replaces object `visible?`)
- `parseFixtureV1` round-trip; fingerprint + `ObjectRecord` threading
- Rendering: hide unless tree-hover reveal; dim + non-interactive when locked
- Play: document row toggles, context menu, selection pruning/filtering, `[DEBUG]` toggle logs

### CAD

- Per-entity flags in `Model.metadata` via `AttributeStore`/`Model` helpers
- Pick-target filtering, render skip for hidden (pinned/hover reveal), locked dim in `targetStyle`
- Play document chrome + toggle callbacks with selection pruning

### UI

- Tree row hide/lock actions (`revealOnHover`), muted hidden rows, right-click context menu
- `lock-open` icon vendored
- Platform/playground `UiTreeItemNode` plumbing for `actions`, `contextMenu`, `isHidden`

## Tests run

- `@semio-tech/infinite-world-r3f`: 34 passed (includes `worldEntityFlags`)
- `@semio-tech/puzzle-3d-react`: 318 passed (includes fixture flag parse)
- `@semio-tech/cad-js-renderer` `index.tsx`: 56 passed (includes entity-flag pick filtering)
- `@semio-tech/cad-js-core`: `getEntityFlags` test added (suite has unrelated pre-existing e2e failures)
- `@semio-tech/ui-react`: 154 passed; 6 pre-existing tree layout snapshot failures unrelated to hide/lock

## Files touched

- `infinite/world/r3f/index.tsx`
- `puzzle/3d/react/index.tsx`
- `puzzle/3d/play/index.ts`
- `cad/js/core/index.ts`
- `cad/js/renderer/index.tsx`
- `cad/js/renderer/play/index.tsx`
- `ui/react/index.tsx`
- `ui/assets/script.ts` (+ generated icons)
- `framework/product/platform/core/index.ts`
- `framework/product/playground/core/index.ts`
- `framework/product/platform/renderer/react/index.tsx`
- `framework/product/playground/renderer/react/index.tsx`
