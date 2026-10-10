# r12-exec-w2-f3-graph: component and MEP inference (label `w2-f3-graph`)

T = ticket folder, S = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`, I = `S/🧬️schema/💡️inferences`. API for the other agents: `T/r12-w2-f3-api.md` (published early, matches the code).

## Verification state (read first)

Ran and green (exact results):

| Command | Result |
|---|---|
| `cargo check -p semio-s-artifact-bim-model --lib` (gate `w2-f3-graph`) | finished, 0 errors (`🗑️generated/w2-f3-graph/x.txt`) |
| `cargo check ... --lib --tests` | finished in 6m04s, 0 errors (`t.txt`); includes the unit-test fix `wanted: u64` in `🕸️model-graph/🧪️tests/🔬️unit` that the widening of the kind masks to u64 had broken |
| `cargo test ... --lib -- inferences::components inferences::mep element_solids::components element_solids::mep` (run 1, 25 tests) | 24 passed, 1 failed (`the_faults_of_a_placement_...`: my expectation counted 2 override issues, there are 4: the parameters `width` and `depth` and the solid `s-table` that reads them) |
| same filter (run 2, 37 tests incl. the new graph tests) | **36 passed, 1 failed**: `components::component::graph_tests::editing_one_run_recomputes_that_run_its_solid_the_clash_scan_and_no_component` (my gating expectation was wrong: raising the end of the duct by 0.4 m still left it 0.15 m from the pipe, closer than the reach 0.2) |
| python oracle `🐍️.py write` / audit on `🪑️components/🏠️room` (`.venv`, shapely 2.1.2) | `16 components, 6 MEP elements, 11 findings`, `oracle agrees` |
| platform: `SEMIO_TEST_LEVEL=quick bun ./📜️script.ts oracle quick --case 🪑️infer-bim-1-components` (test module dir) | `executed=1 passed=1 failed=0 errored=0` (the subject table equals the python table) |
| `bun r3-f1-check-names.ts` | 15 problems, all pre-existing (sheets/energy/psets/mutation fixtures); none under `🪑️`/`🌀️` names |
| `bun r3-f1-gen-oracle.ts`, `bun r3-f1-gen-feature.ts` | ran (`196 kinds, 1565 scenarios`) |

Written after those runs and therefore NOT compiled or run (the framework crates were broken by peers' work in progress for the rest of the session: `semio-framework-os-kernel`, `stdio-zip`, `framework-2d`, `framework-plugin`, `framework-pixels`; my six retries through `🗑️generated/w2-f3-graph/retry.sh` all stopped there):

* the one-line fix of the failing gating test (`z: 3.6` in `🪑️components/🧪️tests/🕸️graph`),
* `ReferenceView` rows for components and MEP elements (`RefElementStorey`, duplicate ids) in `⚠️diagnostics/🔗️references`,
* the facet script run (`r12-w2-f3-graph-facets.mjs`, ran, JSON parses),
* registering the element-solids case `🪑️components-mep` (`CASES` 7 -> 8 in `🧊️element-solids/🦀️.rs`) and its bless.

To run once the framework builds:

```
gate=T/🚦️gate.sh
"$gate" w2-f3-graph -- cargo check --manifest-path <A>/Cargo.toml -p semio-s-artifact-bim-model --lib --tests --message-format short
"$gate" w2-f3-graph -- env BIM_BLESS=1 cargo test --manifest-path <A>/Cargo.toml -p semio-s-artifact-bim-model --lib -- the_committed_expectations_and_meshes_match_the_inference   # writes the meshes of 🪑️components-mep (check git diff: only that case may change)
"$gate" w2-f3-graph -- cargo test --manifest-path <A>/Cargo.toml -p semio-s-artifact-bim-model --lib -- inferences::components inferences::mep element_solids::components element_solids::mep   # expect 37 passed
"$gate" w2-f3-graph -- cargo test ... --lib   # exact counts
"$gate" w2-f3-graph -- cargo check ... --lib --target wasm32-wasip2
cd 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test && SEMIO_TEST_LEVEL=quick bun ./📜️script.ts oracle quick --case 🎲️infer-bim-1-solids-three   # three.js volumes of the component and MEP solids
```

## What was built

Inference modules (all new, mounted in `A/🦀️.rs`):

* `I/🪑️components` (`ComponentValue`, `ComponentEntry`, `component_of`, `fit_to_wall`, `overrides_of`, `dependency`) with `🩺️findings`: placement of the instance (free-standing, mirrored, overridden, hosted fit to the wall face with `+y` away from the wall), footprint = oriented rectangle of the bounds of the visible family solids, bounds, volume, connector in the colour of its system, typed issues.
* `I/🌀️mep` (`MepValue`, sections, palette `key/colour/rgb/part_colour`, `mep_of`, `dependency`) with `💥️clash` (sweep and prune on x, y/z box test, segment distance, pairs of different systems closer than the sum of the reaches, each pair once) and `🩺️findings`.
* Element solids: `I/🧊️element-solids/🪑️components` (family meshes mirrored/turned/moved, winding reversed by the mirror) and `🌀️mep` (one mitred prism per segment, upright rectangle on slopes and risers, circle tessellated with the families chord tolerance 1e-4, one group named after the system). `SolidFamily::{Component, Mep}`.
* Plan: `PlanKind::{ComponentOutline, ComponentFront, ComponentConnector, MepAxis, MepBand, MepDrop}` and the arms `🗺️plan-linework/{🪑️components,🌀️mep}` (cut/projection/hidden by the vertical span against the cut plane; band mitred; drops as circle or rectangle); `Inputs.components/meps`.
* Quantities: `QuantityKind::{Component, Mep}`, `component_quantity`, `mep_quantity`, `ElementQuantity.groups`, `QuantityTotals.groups` (`component-category:<c>`, `component-system:<s>`, `mep-system:<s>`, `mep-size:<label>`), kinds/types totals; the dependency reads all densities for components.
* Diagnostics (en+de rows, `row_of`): `ComponentOutsideStorey, ComponentInWall, RefComponentFamily, RefComponentHost, ComponentOverride, MepDegenerate, MepClash, TerminalUnconnected`; `Inputs.components/meps/clashes`.
* Model graph: `NodeKind::{Component, Mep, MepClash}` (34 kinds), `ModelNode`, `Data::{Component, Mep, MepClashes}`, plan steps (parents: storey, family node when it exists, host layout when the wall exists; clash node per storey), compute arms and `Index` fields, projection into `ModelInference.components` and `.mep`, `dirty::touched` rules, `READS`, `kinds::{COMPONENTS, MEPS}`, `Anonymous` for the two records.
* Renderers: `family_color` and the system colour of a MEP group in `🖌️render/🧊️world`, `kind_of` and `MaterialKey::System` in the glTF export, `kind_class` of the new plan kinds in the SVG style (minimal arms so the crate stays exhaustive; the assets agent refines).
* Tables/IO: `🚪️io/📝️text/💡️inferences/🪑️components` (`table_json`, slug `components` of `encode_inference_projection_json`).
* Facets: `T/r12-w2-f3-graph-facets.mjs` (the four facets of the inference schema family; ran, idempotent).

## Tests

* `🪑️components/🧪️tests/🔬️unit` (9): free-standing placement incl. footprint corners and CCW, overrides, mirror, hosted fit on three walls (south, north mirrored, east rotated), position only chooses side and station, terminal connector colour, typed faults, shared family value (`Arc::ptr_eq`), dependency honesty.
* `🪑️components/🧪️tests/🕸️graph` (12): cache transparency warm = cold = uncached (16 component, 6 mep, 1 mep-clash nodes), topological plan for 7 selections, family edit recomputes the family and exactly its 2 override components / 3 sharing components, unrelated edit and renames compute nothing, moving one component, editing one run, moving a host wall, every issue code with en+de text, plan symbols (13 outlines, 6 connector strokes, 5 bands, 1 drop, styles cut/projection/hidden, band area, tick position), quantities and group totals, the text table equals the oracle table.
* `🌀️mep/🧪️tests` (6), `🧊️element-solids/🌀️mep/🧪️tests` (5), `🧊️element-solids/🪑️components/🧪️tests` (5).
* Oracle: `S/🧪️tests/🪑️infer-bim-1-components/{🐍️.py,🥒️.feature,🦀️.rs}`, scenario `@id-components-room`, manifest row `bim-1-shapely-geometry`; fixtures `S/🧫️fixtures/💡️inferences/🪑️components/🏠️room/{📸️snapshot,💡️inference/🪑️components}/🔣️.json` (written by `T/r12-w2-f3-graph-fixtures.ts` and the oracle). Python recomputes the family under overrides (families oracle), the host fit and placement (numpy), footprints, wall overlaps and distances (shapely), MEP closed forms, the clash pairs (independent segment distance), the quantities per element and group and the findings; its audit checks GEOS areas and three metamorphic laws.
* three.js: case `🪑️components-mep` (`T/r12-w2-f3-graph-solids-case.py`, 16 expected closed-form volumes) added to `🎲️infer-bim-1-solids-three` (`🟦️.ts`, `🦀️.rs`, `🥒️.feature`); meshes still to bless.

## Justified exclusions and open items

1. `ScheduleCategory::{Component, Mep}` and the properties/classification holders: `ScheduleCategory` and `TemplateTarget` are generated schema enums with exhaustive uses in schedules, editor and IO; adding variants is a schema change of the lead (`r3-f1-gen-model.ts`). Components and MEP appear in the quantity totals (kind, family, category, system, size), which the schedule vocabulary can already group by through `Kind`/`Type` fields once the lead adds the categories.
2. `ViewCategory` (view category filters) has no components/MEP entry; primitives of the new plan kinds are always drawn (`category_of_kind` falls through). Same reason: schema enum owned by views.
3. A curved host wall: the Rust fit supports arcs (closest point on the arc, radial normal), the python oracle audits straight hosts only and raises `NotImplementedError` otherwise; the room corpus has none.
4. `ComponentInWall` against the join-trimmed wall footprints of the layouts; the python oracle uses untrimmed bands and refuses a component within 0.3 m of a wall end.
5. The MEP clash is a node (`MepClash(storey)`) that is cancellable only between nodes by the stepped driver; one scan of a storey is not interruptible inside.
6. Pipe volumes are tessellated: solid volume < `area * length` by about 3e-3 at 100 mm; the closed form is in `MepValue.volume` and `ElementQuantity.gross_volume`, the solid one in `net_volume`.
7. `cargo test --lib` full counts, wasm32-wasip2 check and the bless of `🪑️components-mep`: see the verification table (blocked by peers' framework edits).
