# Concrete Forest Kit Fixture

## Summary

Authored the abbau-aufbau semio dev kit with typology **Concrete Forest** (2 types, 4 mutually-compatible ports, 7 connectors each, glb+3dm representations).

Added puzzle 3d fixture `concrete-forest.3d.json`, wired it as the default play fixture, and generalized `/meshes/` serving to include `semio/fixtures/kit/folder/abbau-aufbau`.

## Fix (puzzle 3d infinite render loop)

`getFixtureCatalog()` returned a new object on every call. The navbar fixture `useSyncExternalStore` hook treated each call as a state change, causing "getSnapshot should be cached" and maximum update depth exceeded. Fixed by caching the catalog on the controller and stabilizing the hook snapshot via `WeakMap`.

## Files

- `semio/fixtures/kit/dev/abbau-aufbau/wip/initialKit/kit.semio.json`
- `semio/fixtures/kit/dev/abbau-aufbau/wip/initialKit/index.semio.json`
- `semio/fixtures/kit/dev/abbau-aufbau/wip/initialKit/types/hexagonal-cut-concrete-forest-left.type.semio.json`
- `semio/fixtures/kit/dev/abbau-aufbau/wip/initialKit/types/hexagonal-cut-concrete-forest-right.type.semio.json`
- `puzzle/3d/fixture/concrete-forest.3d.json`
- `puzzle/3d/play/index.ts`
- `ui/styling/vite-elements-assets.ts`
- `framework/product/playground/renderer/react/index.tsx`

## Removed

- `semio/fixtures/kit/dev/abbau-aufbau/wip/initialKit/types/hexagonal-cut-concrete-forest-left.json`
- `semio/fixtures/kit/dev/abbau-aufbau/wip/initialKit/types/hexagonal-cut-concrete-forest-right.json`
