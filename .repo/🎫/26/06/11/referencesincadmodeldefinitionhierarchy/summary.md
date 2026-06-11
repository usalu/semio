# References in Cad Model Definition Hierarchy

## Summary

CAD reference planes now live in shared play state keyed by model definition id and appear in the workbench hierarchy under each model definition branch (References group alongside objects).

### Hierarchy
- `buildCadPlayHierarchySections` lists references per model definition with selection, hover highlight mapping, and hide/lock row chrome (eye/lock actions + context menu).

### State & interaction
- References lifted from per-pane local state into `CadPlayModelSpaceProvider` (`referencesByModelDefinitionId`, `selectedReference`).
- Hierarchy and canvas share selection/hover via `reference:{id}` hover keys.
- Hidden references reveal while their hierarchy row is hovered (`revealedReferenceIds` → `WorldReferenceLayer`).
- Geometry selection clears reference selection and vice versa.
- Default sketch references remain on `spatial.shape`.

### Renderer plumbing
- `InteractionRepl` / `InteractionSpatialView` accept `revealedReferenceIds` for hierarchy hover reveal.

## Tests
- `@cad/js/renderer` `index.tsx`: 65 passed
- `play/index.tsx` suite: pre-existing load failure (`gis_map_bg.wasm` missing in vitest graph)
- Build: passed

## Files touched
- `cad/js/renderer/play/index.tsx`
- `cad/js/renderer/index.tsx`
