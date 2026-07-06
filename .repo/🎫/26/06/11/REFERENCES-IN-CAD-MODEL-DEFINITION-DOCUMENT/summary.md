# References in Cad Model Definition Document

## Summary

CAD reference planes now live in shared play state keyed by model definition id and appear in the workbench document under each model definition branch (References group alongside objects).

### Document
- `buildCadPlayDocumentSections` lists references per model definition with selection, hover highlight mapping, and hide/lock row chrome (eye/lock actions + context menu).

### State & interaction
- References lifted from per-pane local state into `CadPlayModelSpaceProvider` (`referencesByModelDefinitionId`, `selectedReference`).
- Document and canvas share selection/hover via `reference:{id}` hover keys.
- Hidden references reveal while their document row is hovered (`revealedReferenceIds` → `WorldReferenceLayer`).
- Geometry selection clears reference selection and vice versa.
- Default sketch references remain on `spatial.shape`.

### Renderer plumbing
- `InteractionRepl` / `InteractionSpatialView` accept `revealedReferenceIds` for document hover reveal.

## Tests
- `@semio-tech/cad-js-renderer` `index.tsx`: 65 passed
- `play/index.tsx` suite: pre-existing load failure (`gis_map_bg.wasm` missing in vitest graph)
- Build: passed

## Files touched
- `cad/js/renderer/play/index.tsx`
- `cad/js/renderer/index.tsx`
