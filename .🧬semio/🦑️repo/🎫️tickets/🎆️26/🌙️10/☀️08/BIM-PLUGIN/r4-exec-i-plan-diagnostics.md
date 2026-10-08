# r4 execution report: i-plan-diagnostics (Wave I, BIM model artifact)

`T` = ticket folder `BIM-PLUGIN`, `S` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`, `I` = `S/🧬️schema/💡️inferences`. Consumer API: `T/r4-api-plan-diagnostics.md`.

## 1. Result

Two new fields of `ModelInference`, both `InferredField`s over the storey-rooted DAG, plus one shared helper leaf:

* `🗺️plan-linework` : `plan_linework: BTreeMap<storey id, PlanLinework>`. Per storey the architectural plan at the storey cut (constant `CUT_HEIGHT = 1.2` m above the storey elevation, the documented plan convention; no authored override yet) as typed primitives in building coordinates: filled regions with holes, polylines whose vertices carry a bulge, text anchors, each with a style class (`Cut`, `Projection`, `Hidden`, `Annotation`), a `PlanKind` and the id of its element for picking.
* `⚠️diagnostics` : `diagnostics: Vec<Diagnostic>` with severity, stable code (54), element ids, message key `bim.diagnostic.<slug>`, English and German texts, `missing` ids and message numbers.
* `📦️bodies` (helper, not a field): prismatic bodies (footprint regions between two heights) of walls, curtain walls, columns, beams, slabs and stairs, level/scope helpers; shared by both fields.

## 2. Files created

* `I/📦️bodies/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`
* `I/🗺️plan-linework/{🦀️.rs,🟦️.ts}` (types, `Sheet`, field, metrics), submodules `🧱️walls`, `🏛️members`, `🪜️stairs`, `📍️annotations` (each `🦀️.rs`), tests `🧪️tests/🔬️unit/🦀️.rs`
* `I/⚠️diagnostics/{🦀️.rs,🟦️.ts}` (types, key, field), submodules `💥️clashes`, `🔗️references`, `📐️validity`, `🪜️levels`, `💬️messages` (each `🦀️.rs`), tests `🧪️tests/🔬️unit/🦀️.rs`
* Fixtures `S/🧫️fixtures/💡️inferences/🗺️plan-linework/{🏡️house,🌀️curved}/` and `S/🧫️fixtures/💡️inferences/⚠️diagnostics/{🏡️clean,💥️defects}/`: `📸️snapshot/🔣️.json` (authored by `T/r4-i-plan-diagnostics-fixtures.py`) and the expected tables `💡️inference/🗺️plan-metrics/🔣️.json`, `💡️inference/⚠️diagnostics/🔣️.json` (WRITTEN by the oracle, never by hand)
* Oracle case `S/🧪️tests/🗺️infer-bim-1-plan-and-diagnostics/{🥒️.feature,🐍️.py,🦀️.rs}` (4 scenarios, oracle `bim-1-shapely-geometry`)
* In `T`: `r4-api-plan-diagnostics.md`, `r4-i-plan-diagnostics-fixtures.py`, `r4-i-plan-diagnostics-splice.py` (aggregate splice), `r4-i-plan-diagnostics-facets.py` (facet splice), this report

## 3. Shared files touched (surgical, before to after)

| File | Change |
|---|---|
| `A/📦️packages/🦀️rust/Cargo.toml` | `semio-framework-2d` gains `features = ["booleans"]` (region booleans for clashes; another agent added `semio-framework-geometry` at the same moment, my duplicate line was removed) |
| `A/🦀️.rs` mount tree | `+` mounts `inferences::{bodies, plan_linework, diagnostics}` before `element_solids` |
| `I/🦀️.rs` aggregate | `+` fields `plan_linework`, `diagnostics`, their `infer` calls and two `InferenceFieldSpec` rows (`reads = plan_linework::READS / diagnostics::READS`) |
| `I/{🔣️.json,🟦️.ts,🔗️.graphql,🛰️.proto}` | `+` the two fields and all their types (text insertion by `r4-i-plan-diagnostics-facets.py`; enum variant lists are read from the Rust sources; proto tags 9 and 10 taken as `max + 1`) |
| `S/🚪️io/📝️text/📸️snapshot/🦀️.rs` | `encode_inference_projection_json` gains the arms `"plan-metrics"` and `"diagnostics"` |
| `S/🔮️oracles/🔣️.json` | one sentence appended to the rationale of `bim-1-shapely-geometry` (no new oracle row: the case reuses it) |

## 4. Design decisions

1. **Storey-rooted DAG** (`storey-levels::Rooted`): `Element(storey)` is the plan, parents = own storey + the storeys its top constraints target (`bodies::level_storeys`); `dep_input` = `bodies::storey_scope` (all elements of the storey, the openings of its hosts, grid lines of its building, all types, material ids, building placement). A storey height edit re-infers the plans above it; a material colour edit touches nothing.
2. **Diagnostics keys** `Level / Storey / Building / Model`: storey scope (references of its elements, degenerate geometry, openings from `opening-frames`, stairs from `stair-runs`, spaces from `spaces`), building scope (all clashes, level gaps/duplicates/datum, duplicate space numbers), model scope (dangling spatial references, orphaned data, duplicate ids; depends on the whole snapshot).
3. **Poché is exact**: a cut wall is split at the opening gaps into pieces built with `band_loop` on the sub-axis and the join trims of `wall-layout` at the wall ends, so arcs stay arcs and no boolean is needed; area law: pieces tile the join-trimmed footprint minus the gaps.
4. **Style by cutting** `[low, high)` against the plane: cut / below (projection) / above (hidden, dashed); demolished walls hidden; windows above the plane dashed.
5. **Clash model**: bounding-box sweep, height overlap > 1 um, kind table (slab/wall and slab/column excluded as legitimate), exact `region_boolean` intersection area > `AREA_EPS = 1e-3` m2; crossing walls (X join) are not clashes; a beam ending in its partner rests on it. Mesh tests were not used: every checked kind is a prism, so footprint area times shared height is the exact volume; sloped/curved parts are bounded by their extent (documented).
6. **No new dependency edges**: only `semio-framework-geometry` and the `booleans` feature of `semio-framework-2d`, both already framework crates; wasm32-wasip2 unaffected.

## 5. Commands and results

All cargo calls through `T/🚦️gate.sh i-plan-diagnostics`.

| Command | Result |
|---|---|
| `cargo check -p semio-s-artifact-bim-model` | exit 0, no warning from the new files |
| `cargo check -p semio-s-artifact-bim-model --target wasm32-wasip2` (re-run after the last edit) | exit 0 (`Finished dev profile`) |
| `cargo test -p semio-s-artifact-bim-model --lib -- inferences::bodies inferences::plan_linework inferences::diagnostics` | first run 36 passed, 3 failed (two wrong test expectations of mine: mitered footprint is a trapezoid of area `8.0 * 0.3`; the ground plan's stair changes when the storey gets taller; one real bug: a non-finite body was not refused). All three fixed. Full lib run before the last two tests: **3099 passed, 18 failed**, all 18 outside this scope (12 editor, 1 viewer, 3 io/ifc, 1 `spaces::an_island_wall_is_a_hole_of_the_room`, 1 outliner). Final focused run after all edits: **41 passed, 0 failed** (bodies 8, plan_linework 19, diagnostics 14) |
| `python 🐍️.py write` then `check` over `S/🧫️fixtures/💡️inferences` (shapely 2.1.2) | `write: oracle agrees`, `check: oracle agrees` (plan house, plan curved, diagnostics clean, diagnostics defects) |
| `cd 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test && SEMIO_TEST_LEVEL=quick bun ./📜️script.ts oracle quick --case 🗺️infer-bim-1-plan-and-diagnostics` | `executed=4 passed=4 failed=0 errored=0` |

Third-party validation inside the oracle: the poche union is `shapely.unary_union` of mitered/butted bands minus gap strips (8.1675 m2 for the ground storey, 3.0 for the first, closed form 2.421885654 m2 for the arc wall audited by a 4096-chord sector), clash areas are `Polygon.intersection(...).area` (0.06, 0.03, 0.0075 m2 in the defect model), the clean house yields an empty table.

## 6. Tests added (unit, language-neutral fixtures)

`plan_linework` (19): cut height per storey, poché pieces and exact area, opening symbols in the style of their cut, layer lines, members, curtain mullions, stairs (6 solid, 10 dashed, 1 cut line, arrow), spaces (explicit and bounded), open room tagged at its seed, grids and bounds, unique ids, curved walls exact, projection/demolished styles, sill raised closes the gap, wall moved moves only its poché, storey height moves only the cut above, determinism and default, DAG parents, gating (`infer_field_after_diff`). `diagnostics` (14): clean model empty, defect model exactly 13 findings, severity/key/storey/missing/values, moving a wall into a column creates the clash and moving it back removes it, crossing vs overlapping walls, beam resting vs passing through, beam raised into the slab, table consistency (54 codes, same placeholders en/de), rendering en/de/other locale, no datum/level gap/zero height, degenerate loops/profiles/paths/non-finite, space not enclosed/seed in wall, dangling references/orphans/duplicate ids, determinism/default/gating. `bodies` (8).

## 7. Open issues

1. Rust subject adapter `S/🧪️tests/🗺️infer-bim-1-plan-and-diagnostics/🦀️.rs` is `sut`-gated like its siblings and was not compiled by the platform here; the logic it calls (`encode_inference_projection_json` arms) is exercised through the unit tests and the shapely oracle tables, not through `subject`/`parity`. Run `bun ./📜️script.ts parity quick --case 🗺️infer-bim-1-plan-and-diagnostics` once the host package builds.
2. Shapely's `unary_union` dissolves pieces only after snapping to 1e-9 (`shapely.set_precision`); recorded in the oracle.
3. Plan conventions to review by u-editor: coordinates are building-local (no origin/rotation), the cut height is a constant, and the up arrow is a polyline through flights and landings.
4. The `bodies` leaf duplicates the three-line `TopConstraint` resolution (`vertical`) that `wall-layout::top_of` and `element_solids::resolve_vertical` also contain; `walls` and `stairs` use their own, so a later consolidation into one framework-neutral helper is possible.
5. Messages use `m3`/`m2` ASCII units on purpose (Windows shell safety); localisation of number formatting is out of scope (`.` decimal point in both languages).

## 8. Scratch

`T/🗑️generated/i-plan-diagnostics/` (cargo and oracle logs) was deleted. Kept inputs: `r4-i-plan-diagnostics-{fixtures,splice,facets}.py`. No process was killed.
