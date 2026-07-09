# Concrete Forest Kit Fixture

## Summary

Authored the abbau-aufbau compose dev kit with typology **Concrete Forest** (2 types, 4 mutually-compatible ports, 7 connectors each, glb+3dm representations).

Added puzzle 3d fixture `concrete-forest.3d.json`, wired it as the default play fixture, and generalized `/meshes/` serving to include `compose/fixtures/kit/folder/abbau-aufbau`.

## Fix (puzzle 3d infinite render loop)

`getFixtureCatalog()` returned a new object on every call. The navbar fixture `useSyncExternalStore` hook treated each call as a state change, causing "getSnapshot should be cached" and maximum update depth exceeded. Fixed by caching the catalog on the controller and stabilizing the hook snapshot via `WeakMap`.

## Fix (concrete forest fill missing b-\* pairings)

Fill never attached `b-p1-t-t2-c1-l` (b-s, index 6) to `b-p1-t-t1-c3-l` (b-l, index 0) because seed vortices were starved: other targets always won first, and by the time v0 was tried only Right-side candidates were collision-free.

Changes in `puzzle/3d/react/index.tsx`:

- Seed/base fixture vortices are filled before frontier vortices on placed pieces
- Target order rotates each fill step so every free vortex gets a turn
- Cross-port candidates (e.g. b-s → b-l) rank before same-port retries
- Same-kind fill prefers distant connector indices (Left@6 on seed v0)
- `brushPlacementUsesHostOrientation` only when source and target vortex kinds match
- `brushCandidateRank` deprioritizes same vortex kind for non-stack ports

## Files

- `compose/fixtures/kit/dev/abbau-aufbau/wip/initialKit/kit.compose.json`
- `compose/fixtures/kit/dev/abbau-aufbau/wip/initialKit/index.compose.json`
- `compose/fixtures/kit/dev/abbau-aufbau/wip/initialKit/types/hexagonal-cut-concrete-forest-left.type.compose.json`
- `compose/fixtures/kit/dev/abbau-aufbau/wip/initialKit/types/hexagonal-cut-concrete-forest-right.type.compose.json`
- `puzzle/3d/fixture/concrete-forest.3d.json`
- `puzzle/3d/play/index.ts`
- `ui/styling/vite-elements-assets.ts`
- `framework/product/playground/renderer/react/index.tsx`

## Removed

- `compose/fixtures/kit/dev/abbau-aufbau/wip/initialKit/types/hexagonal-cut-concrete-forest-left.json`
- `compose/fixtures/kit/dev/abbau-aufbau/wip/initialKit/types/hexagonal-cut-concrete-forest-right.json`
