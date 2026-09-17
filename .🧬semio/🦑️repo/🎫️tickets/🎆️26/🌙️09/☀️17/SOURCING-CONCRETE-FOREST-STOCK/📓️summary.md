# Sourcing demo stock — concrete forest GLB kinds

## Change

- Added `GeometryRecipe::Glb { url, extent }` so stock kinds can reference delivered meshes (`/mesh/🧊️hexagonal-cut-concrete-forest-{left,right}.gb`) like puzzle 3d and cad.
- New authored `reuse` module with two demo kinds in `demo_stock()` and the `demo` example DSL.
- Grid/preview world3d scenes emit `{ id, url }` mesh atoms; `sourcing_catalog_fragment` sets `meshUrl` for GLB rows.

## Proof

- `cargo test -p semio-s-artifact-sourcing-curation demo_stock_example sourcing_catalog_fragment` (fresh test binary).
- Grid/preview unit tests pass on the same binary.
