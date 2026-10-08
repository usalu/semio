# r4 execution report: i-solids-rest (Wave I, element solids of columns, beams, slabs, roofs, stairs, railings)

`T` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN`. `S` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`. `E` = `S/🧬️schema/💡️inferences/🧊️element-solids`.

## 1. Result

The six families plug into the shared `🧊️element-solids` field of `i-solids-walls` through its `SolidFamilyBuilder` extension point (one unit struct per family, one line each in `builders()`). Every family
resolves its heights from the storey levels, so the parametric law holds end to end: a storey height edit re-infers exactly the columns, beams, slabs, roofs, stairs and railings that stand on or reach that storey.
`cargo test -p semio-s-artifact-bim-model --lib element_solids::{columns,beams,slabs,roofs,stairs,railings,plan_kit}`: 45 passed, 0 failed. The shapely oracle `check` and the platform `oracle quick` case are green.

## 2. Files created (all new, under `E/`)

| Dir (module `element_solids::…`) | Content |
|---|---|
| `📐️plan-kit` (`plan_kit`) | authored-plan to geometry conversions (`point`, `bulged`, `placed`, `rectangle`, `direction`), `layer_thicknesses`, `stack`; `is_curved`, `planar_projection_json`; `#[cfg(test)] testing` (fixture `case`, `raised`, `group_extent`, `is_closed`, `assert_matches_oracle`) |
| `🏛️columns` (`columns`) | `profile_loop` (Rectangle, Circle as two semicircles, IShape, Custom), `column_geometry`, `ColumnSolids` |
| `➖️beams` (`beams`) | `section_of`, `beam_geometry` (mitred `sweep_profile` along start to end), `BeamSolids` |
| `⬜️slabs` (`slabs`) | `top_plane`, `slab_layers` (one prism per layer through `extrude_loops`, holes and bulged edges), `SlabSolids` |
| `🏠️roofs` (`roofs`) | `roof_geometry` (flat, shed, gable, hip, mansard, overhang, ridge/hip/break lines, eave loop, fallback reason), `RoofFallback`, `RoofLine`, `RoofSolids` |
| `🪜️stairs` (`stairs`) | `steps_of(&StairRun)`, `stair_geometry`, `StairSolids` (built on `stair_runs::run_of`) |
| `🛤️railings` (`railings`) | `post_positions`, `railing_geometry`, constants `POST_SIZE`, `RAIL_WIDTH`, `RAIL_DEPTH`, `RailingSolids` |

Each dir has `🦀️.rs` and `🧪️tests/🔬️unit/🦀️.rs`. Fixtures: `S/🧫️fixtures/💡️inferences/🧊️element-solids/{columns-profiles,beams-profiles,slabs-holes-slope,roofs-shapes,stairs-flights,railings-posts}/🔣️.json`
(`snapshot`, oracle-written `expected`, empty `meshes`; `null` = element without geometry). Oracle case `S/🧪️tests/🧊️infer-bim-1-solids-rest/{🥒️.feature,🐍️.py,🦀️.rs}`.
Input script kept in `T`: `r4-i-solids-rest-fixtures.py` (writes the authored snapshots, keeps `expected`).

## 3. Files updated (surgical)

| File | Change |
|---|---|
| `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🦀️.rs` | + 7 mounts (`plan_kit`, `columns`, `beams`, `slabs`, `roofs`, `stairs`, `railings`) inside `pub mod element_solids` |
| `E/🦀️.rs` | `builders()` + 6 families; `READS` + `columns, column_types, beams, beam_types, slabs, slab_types, roofs, roof_types, stairs, railings`; `parts::{STEP, POST, RAIL}` |
| `S/🚪️io/📝️text/📸️snapshot/🦀️.rs` | + `"frame-solids"` arm of `encode_inference_projection_json` (planar projection for the platform case) |
| `S/🔮️oracles/🔣️.json` | rationale of `bim-1-shapely-geometry` extended (no new oracle row: the case reuses it) |
| `S/🧬️schema/💡️inferences/🔣️.json` and the other facets | unchanged: the field `element_solids` and `ElementSolid` are the walls agent's and do not change |

OUTSIDE my scope, mechanical, needed to compile anything (the DIFF-ONLY `ApplyCapability` migration had left the stdio dependency chain of `semio-s-artifact-stdio-ifc` red, which blocked the whole fleet): replaced
`MutationDiff::apply(..)` / `<X as MutationDiff<Y>>::apply(..)` by `protocol::apply_diff(..)` (and the `apply` signature of `BinaryDiff`) in `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/{💾️binary,🔤️txt,📐️step,🏗️ifc}` (about 30 call sites,
`stdio-contract` had meanwhile been fixed by someone else). Please let the DIFF-ONLY owner review.

## 4. Design and documented limits

- **Columns**: profile centred on `position`, rotated by `rotation`; `base = storey elevation + base_offset`, `top` from `TopConstraint` through the shared `resolve_vertical` (StoreyTop, Storey, Unconnected). Absent: unknown type, invalid profile, top at or below base.
- **Beams**: `x` of the profile is the offset left of travel, `y` the height; the profile is dropped so its highest point touches `storey top + top_offset`; sweep with `sweep_profile`.
- **Slabs**: layers stack downward from `storey elevation + offset`, layer 0 topmost, thickness measured vertically. `Slope.direction` is the fall direction; the top plane keeps the reference height at the uphill edge of the boundary.
- **Roofs**: eave = footprint grown by `overhang` (`loops::offset`); underside at the eave = storey top + `base_offset`; layers stack upward from the underside, layer 0 outermost; thickness is vertical (volume = eave area times layers, independent of the pitch).
  Gable, hip and mansard are the lower envelope of planes over a CONVEX footprint with straight edges (exact for any convex polygon, not only rectangles); shed and flat work on any footprint, arcs included. Anything else (curved or concave footprint, pitch outside `[0, pi/2)`) falls back to a flat roof at the eave height and reports why in `RoofGeometry::fallback`
  (`roof.fallback-flat.{curved-footprint,non-convex-footprint,degenerate-footprint,invalid-pitch}`, `roof.overhang-collapsed`). No straight skeleton: concave hips are a documented limit. `RoofGeometry::{eave, lines (Ridge/Hip/Break), eave_z, ridge_z}` are the plan outputs for `🗺️plan-linework`.
- **Stairs**: built on the published `stair_runs::run_of` (riser count `ceil(rise / max_riser)`, tread by Blondel, flights, landings, winder), so the solid, quantities and code flags share one run. Monolithic steps from the storey floor to each tread level; stringers and nosings are NOT modelled (the stair record has no parameters for them; schema-first). A one-riser stair has no tread and is absent.
- **Railings**: no profile parameters exist, so post and rail sections are the named constants `POST_SIZE` 0.05, `RAIL_WIDTH` 0.06, `RAIL_DEPTH` 0.04; posts on every vertex and equal divisions of at most `post_spacing`; mitred rail sweep.
- Parts: `BODY` (columns, beams), `LAYER` (slabs, roofs; group `layer` = layer index), `STEP`, `POST`, `RAIL`; material = the type's material id (railing: its own, stairs: none).
- Tessellation tolerance `CHORD_TOLERANCE` 1e-4 (the walls agent's constant); arcs are inscribed, so round parts are below the analytic value within `arc length * 1e-4 * extent`.

## 5. Verification

| Command (through `T/🚦️gate.sh i-solids-rest`) | Result |
|---|---|
| `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-bim-model --lib -- element_solids::{columns,beams,slabs,roofs,stairs,railings,plan_kit}` | 45 passed, 0 failed |
| `cargo test … --lib` (whole crate, last run) | 3110 passed, 15 failed; all 15 are the editor and viewer unit tests of other agents' in-flight work; none in element-solids (an earlier run had 18 failures incl. ifc export, spaces) |
| `cargo check … --target wasm32-wasip2` | exit 0 |
| `.venv/Scripts/python.exe S/🧪️tests/🧊️infer-bim-1-solids-rest/🐍️.py check S/🧫️fixtures/💡️inferences/🧊️element-solids` | `check: ok`, 6 cases, 46 rows |
| `cd 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test && SEMIO_TEST_LEVEL=quick bun ./📜️script.ts oracle quick --case 🧊️infer-bim-1-solids-rest` | cases=1 executed=1 passed=1 |
| `… bun ./📜️script.ts parity quick --case 🧊️infer-bim-1-solids-rest` (Rust subject vs shapely oracle) | cases=1 executed=2 passed=2 parity=1/1 |

Third-party cross-validation: the oracle (shapely 2.1.2, numpy) never sees the subject's meshes. It recomputes from the snapshot: polygon areas and bounds (`Polygon.area`, `bounds`, `affinity`), roof eaves (mitred `buffer`), gable ridge (`intersection`), hip ridge rise (an exact numpy linear program audited by `shapely.ops.polylabel`), hip ridge length (negative mitred `buffer`),
convexity (`convex_hull`), stair prisms (documented `stair-runs` contract plus shapely union), railing posts (`LineString.interpolate`). The unit tests compare volume, bounds, closedness (every directed edge balanced), family and fallback code with the committed table, plus closed forms hard-coded in Rust.
Laws tested per family: analytic volumes, parametric law (storey height to column height, beam/slab/roof/railing lift, stair riser count 17 to 19 with 3.0 to 3.4 m), determinism (`ModelInference::infer` twice), default (empty snapshot plans nothing), absence of degenerate elements, parents order (own storey, then the constrained storey), diff gating
(`infer_field_after_diff`: a material edit serves the stored solids, a storey height edit re-infers them).

Subject parity through the platform (Rust adapter `🦀️.rs` answering `{case: {id: {volume, min, max}}}` of the planar elements through the `frame-solids` table) is green: parity 1/1 within 1e-9. Elements with tessellated arcs are audited inside the unit tests within the chord tolerance.

## 6. Open issues and follow-ups

1. `meshes` of my fixtures is empty: the walls agent's three.js oracle (`🧊️infer-bim-1-solids-three`) may add committed meshes; my `expected` rows carry `volume/min/max/curved/...` and null for absent elements, a different shape from the walls fixtures, so that oracle should skip the six cases of these families.
2. Stringers/nosings, railing profiles and concave roofs (straight skeleton) need authored parameters or a skeleton in the framework geometry. 3. `🗑️generated/i-solids-rest` removed; private build dirs are the gate's slots.
