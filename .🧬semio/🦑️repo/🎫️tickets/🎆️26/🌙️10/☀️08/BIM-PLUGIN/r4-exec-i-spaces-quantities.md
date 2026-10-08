# r4 execution report: i-spaces-quantities (Wave I, BIM model artifact)

`T` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN`, `S` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`, `I` = `S/🧬️schema/💡️inferences`.
API docs: `T/r4-api-stair-runs.md` (published early, consumed by `i-solids-rest`), `T/r4-api-spaces-quantities.md`.

## 1. Result

Three new fields of `ModelInference`, each with unit tests, a committed fixture, a third-party oracle (shapely, run green through the test platform) and facets in json/ts/graphql/proto:

| Field | Kind | Value | Tests (all green) |
|---|---|---|---|
| `🪜️stair-runs` | `InferredField`, key `Rooted` (storey roots, stair children with the own and the constrained storey as parents) | `StairRun` (rise, riser count/height, tread, stride, flights with placement, landings, winder, run length, code flags) | 15 |
| `🏠️spaces` | `InferredField`, one child per storey that has spaces, parent = that storey's level | `SpaceRoom` (status, outline, islands, area, perimeter, net floor area, clear height, volume, ceiling slab, bounding walls) | 16 |
| `🧮️quantities` | derived from the other inferred fields, `compute_quantities(snapshot, &ModelInference)`, last in `infer` | `ModelQuantities` (element rows, totals per kind/type/material for storeys, buildings, project) | 14 |

Build/test evidence (all through `T/🚦️gate.sh`, slot label `i-spaces-quantities`):
- `cargo test -p semio-s-artifact-bim-model --lib` (last full run 10:50): 3105 passed, 15 failed. All 15 are other agents' work in progress (12 `editor::bim` unit tests, 1 `viewer::bim` test, 2 `io::…::export::ifc::projection` tests: "the committed export drifted: rewrite it with BIM_BLESS=1" and its oracle twin). Every test of `inferences::{spaces, quantities, stair_runs, storey_levels}` passes (16, 14, 15, 4).
- `cargo check -p semio-s-artifact-bim-model --target wasm32-wasip2`: exit 0 (re-run after the last source edit).
- Oracle through the platform (`cd 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test && SEMIO_TEST_LEVEL=quick bun ./📜️script.ts oracle quick --case <case>`): `🏠️infer-bim-1-spaces`, `🪜️infer-bim-1-stair-runs`, `🧮️infer-bim-1-quantities` each `executed=1 passed=1 failed=0`. Direct runs of `🐍️.py write` then `check`: "oracle agrees" for all three.
- Subject vs oracle: `the_subject_reproduces_the_third_party_oracle_table` passes in all three leaves (tolerance 1e-9).
- Not run: the Rust subject role of the new cases through the platform (`subject`/`parity`), because the generated host package is built by the repo test platform (same open item as f1 section 8.2); `bun ./📜️script.ts contract` (about 8 minutes, repo wide).

## 2. Files

Created (all under `I` unless noted):
- `🪜️stair-runs/{🦀️.rs,🟦️.ts,🧪️tests/🔬️unit/🦀️.rs}`, `🏠️spaces/{…}`, `🧮️quantities/{…}` (module, per-leaf TS facet, unit tests).
- Fixtures `S/🧫️fixtures/💡️inferences/{🪜️stair-runs/🪜️flights,🏠️spaces/🏡️rooms,🧮️quantities/🏗️building}/{📸️snapshot,💡️inference/<slug>}/🔣️.json` (snapshots written by `T/r4-i-spaces-quantities-fixtures.py`, tables written by the oracles).
- Cases `S/🧪️tests/{🪜️infer-bim-1-stair-runs,🏠️infer-bim-1-spaces,🧮️infer-bim-1-quantities}/{🥒️.feature,🐍️.py,🦀️.rs}`; each feature is `@capability-bim-1-infer @oracle-bim-1-shapely-geometry @comparison-floating-point-v1` with one scenario (`stair-runs-flights`, `spaces-rooms`, `quantities-building`). No new oracle manifest rows were needed (the existing `bim-1-shapely-geometry` row is reused).
- Kept inputs in `T`: `r4-i-spaces-quantities-facets.ts` (idempotent facet splice), `r4-i-spaces-quantities-fixtures.py`, `r4-api-stair-runs.md`, `r4-api-spaces-quantities.md`, this report.

Updated (surgical edits to shared files):
- `A/🦀️.rs` (artifact root): three mounts (`stair_runs`, `spaces`, `quantities`) in `pub mod inferences`.
- `I/🦀️.rs` (aggregate): `use`, fields `stair_runs`, `spaces`, `quantities`, `infer` now builds the struct and then `inferred.quantities = compute_quantities(snapshot, &inferred)`, three `InferenceFieldSpec` rows (`quantities` uses `quantities::READS`).
- `I/🔣️.json`, `🟦️.ts`, `🔗️.graphql`, `🛰️.proto`: fields and all defs (spliced, not regenerated; proto tags 5 to 7 were free at that time).
- `I/🪜️storey-levels/🦀️.rs`: new region `🔖️Rooted` (`Rooted`, `RootedValue`, `rooted_plan`, `rooted_dependency`, `rooted_value`, `rooted_elements`) shared by my three fields, plus the `#[cfg(test)]` helper `table_problems` that compares a committed oracle table with a produced one.
- `S/🚪️io/📝️text/📸️snapshot/🦀️.rs`: `encode_inference_projection_json` slugs `stair-runs`, `spaces`, `quantities` (and a refreshed docstring).

## 3. Design decisions

1. Fixture layout follows the peers and the brief: `💡️inferences/<slug>/<case>/…` (not the older `<case>/💡️inference/<slug>` of the house case).
2. `stair-runs` is a storey-rooted DAG. Placement is a contract (see the API doc). The flight kinds: straight, L-turn (`split` = fraction of risers in the first flight), U-turn (`gap`), spiral (`radius` = outer radius, signed sweep). Blondel target 0.62, limits 0.59 to 0.65, cap 512 risers.
3. `spaces` reuses the very join functions of `wall-layout` (`storey_bands`, `joins::{join, footprint}`), so the rooms and the wall footprints cannot disagree. One DAG child per storey (not per space): the arrangement is computed once per storey. A face that reaches the extent is open (diagnostic `NotEnclosed`).
4. `quantities` is derived from `&ModelInference` (layouts, frames, rooms, runs, solids): wall/slab/column/beam/space measures are closed form; curtain walls, roofs, openings, stairs and railings take volumes and material rows from `element_solids` (group volumes by signed tetrahedra). Per-layer wall areas are exact: footprint area split by the linear face-length model, normalised to the exact loop area; opening cuts are scaled by the arc radius for curved walls.
5. The committed `quantities` table lists only the closed-form kinds, re-summed without the solid-based kinds (`table_json`); the rest is cross-checked against the solid meshes in the unit tests (walls within 1e-4, slabs and columns within 1e-3, circle chord error).
6. Third-party strategy: shapely `polygonize` of the unioned footprints (rooms), GEOS `intersection`s for layer and opening areas, GEOS polygons for slabs, sibling oracle `../🧱️infer-bim-1-wall-joins` for the footprints; the stair oracle is a second independent implementation of the rules audited by GEOS strips/boxes and a parametric metamorphic law. Footprints are snapped with `shapely.set_precision(1e-9)` before the union, because mitered corners agree only to about 1e-15.

## 4. Open issues and notes

1. Round-off: arcs in inferred rooms are flattened at 1e-6 (area error about 1e-6 relative); the committed differential table therefore uses straight-walled rooms (1e-9) and explicit arcs (exact). The curved room is covered by a unit test against the circular-segment closed form (1e-3).
2. `spaces` ignores the slope of a ceiling slab (uses its reference height) and treats only slabs as ceilings (no roof underside).
3. The 15 failing lib tests listed in section 1 belong to `u-editor`, `u-viewer` and `x-ifc` (the IFC drift is theirs: the exporter does not read my fields). Please re-run the full gate once those waves finish.
4. While waiting for the build I did NOT edit foreign files; the stdio crates that broke the dependency chain (`semio-s-artifact-stdio-contract`, `-binary`: `apply` now takes an `ApplyCapability`) were repaired by their owners during the session.
5. `T/📓️coordination.md` was not touched. Repo MCP was down, so no ticket tool was used.
