# r11-w01-depth-infer execution report (WP-01: authored depth parameters made effective)

Label `r11-w01-depth-infer`, logs `T/🗑️generated/r11-w01-depth-infer/` (`test3.txt` = the one full run of my areas, `test12.txt` = last failed attempt). `S` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`,
`I` = `S/🧬️schema/💡️inferences`, `E` = `I/🧊️element-solids`.

## Status
All audit tasks implemented. Python oracles green (exact output below). Rust: **one** compiled run of my areas (14:26, after `r13-compiles.flag`):
`cargo test --lib -- <my module filters>`: **262 passed, 19 failed**. I triaged all 19, fixed those that are mine (after that run), and then could not
rebuild: from ~15:00 on `semio-framework` / `semio-framework-os-kernel` fail with 39 resp. 4 errors `ActorInstance*/JobCheckpoint/ColdPairIngressStatus:
serde::Serialize/Deserialize is not satisfied ... multiple different versions of crate serde_core (1.0.228 vs 1.0.229)` (root `Cargo.lock` pins 228, the model
`Cargo.lock` 229), and peers' editor/IFC4/component edits break the crate in between. So my post-run fixes are **written but not recompiled**.
wasm32-wasip2 check: **not run** (same reason; r13 reported lib/tests/wasm green at 14:09).

## What changed (before -> after)
| Area | Before | After |
|---|---|---|
| Stairs unit tests (`E/🪜️stairs`) | used removed `steps_of`/prism volumes | rewritten: oracle table, riser law, per-part volumes vs oracle, analytic tread/riser volumes, nosing, closed/open/mono band areas (hand integrals), landings with `landing_depth`, stringer plan footprints, spiral ignores stringer + diagnostic, dependency honesty, session edit == fresh inference, quantity == sum of parts |
| `stair.stringer-ignored-on-spiral` | no caller | emitted in the validity stair loop; code `StairStringerIgnored`, en+de text, `ignored_stringers` in `storey_dependency` |
| Plan cut height | const `CUT_HEIGHT` | `storey_cut_height` = `storey.cut_height.unwrap_or(DEFAULT_CUT_HEIGHT)`; storey record already in the plan dependency; tests: authored/default heights, low cut opens and high cut closes the poche gaps, session edit redraws exactly that storey's plan (`plan == 1`, no `wall-layout`) and equals a fresh inference |
| Roofs | plane envelope, convex only | framework straight skeleton (`roof_surface_controlled`) for Hip/Gable/Mansard on any simple straight footprint (concave L/T/U), layers = `shell(t)`, lines = skeleton lines; fallbacks: curved, **skeleton** (new code `roof.fallback-flat.skeleton`, replaces non-convex), degenerate (self-crossing = no solid), invalid pitch (incl. mansard without break), overhang collapsed, **`roof.gable-ends-adjust-to-hip`** (new); pitch 0 = flat; `roof_geometry_controlled`/`roof_solid_controlled` pass a cancellation control into the skeleton |
| Quantities | railing volume | `surface_area` = infill area, new optional `balusters` (omitted when 0, so committed tables are unchanged) |
| Railings | no baluster count | `baluster_count(&Railing)`; tests for parts vs oracle, baluster stations, glass/panel material, infill area, dependency |
| IFC export | one brep | stairs: `IfcMember` `<id>:stringer` aggregated beside the flights; railings: `IfcMember` `<id>:baluster` + `IfcPlate` `<id>:infill` aggregated by the `IfcRailing` (IFC 2x3 has no such entity types) |
| Facets | 74 diagnostic codes | +`StairStringerIgnored`, +`RoofGableToHip`, `RoofFlatNonConvex`->`RoofFlatSkeleton` in `I/🔗️.graphql`,`🔣️.json`,`🟦️.ts`,`🛰️.proto` (renumbered), `⚠️diagnostics/🟦️.ts`; `ElementQuantity.balusters` in the same files and `🧮️quantities/🟦️.ts` |

Oracles (third-party: shapely 2.1.2, numpy, `py_straight_skeleton`): `📦️infer-bim-1-solids-rest` (stairs, railings, roofs rewritten; **bug fixed**: case lookup `"/%s/" % name in uri` never matched the emoji folders, both oracle and Rust adapter compared empty tables; now by folder suffix), `🛤️infer-bim-1-stair-runs` (`landing_depth`), `🗺️infer-bim-1-plan-and-diagnostics` (storey `cut_height`, new scenario `@id-plan-metrics-cut-heights` in feature + python + Rust adapter). Fixtures via `T/r11-w01-depth-infer-fixtures.py` (idempotent): +7 stairs, +5 railings (+glass material), +8 roofs and `r-gable-rotated` made a rotated rectangle, +2 deep-landing stair-runs stairs, new plan case `🔪️cut-heights`; expected tables written by the oracles. Feature descriptions of solids-rest and stair-runs extended.

## Commands and exact results
* oracles: stair-runs `write`/`check`: `oracle agrees` (15 stairs); solids-rest `write` then `check`: `check: ok` (roofs 20 rows/2 absent, stairs 16/2, railings 11/1; skeleton roofs carry a `1e-7` tolerance because the framework merges event points within `1e-9` of the extent, observed volume error 1.3e-8); plan-and-diagnostics `check`: `oracle agrees` (cut-heights ground `WallCut.area` 8.5275 vs house 8.1675 = the 1.2 x 0.3 window gap closed by the 0.5 m cut).
* `"$T/🚦️gate.sh" r11-w01-depth-infer -- cargo test --manifest-path <model Cargo.toml> -p semio-s-artifact-bim-model --lib --no-run`: exit 0 (14:26). Whole-lib run: aborts with `STATUS_STACK_OVERFLOW` in `bim-plan-window-isolation` (not mine, r13-stack).
* filtered run (`inferences::element_solids inferences::plan_linework inferences::diagnostics inferences::quantities inferences::stair_runs export::ifc::circulation`): **262 passed, 19 failed**. Triage of the 19:

| failing test | owner / action |
|---|---|
| `roofs::..::roofs_reproduce_the_third_party_oracle_table` (r-gable volume 6.299999986 vs 6.3), `roofs_have_the_analytic_volume...`, `layers_stack_upward...` | mine: skeleton corner error 1e-8; oracle tolerance for skeleton roofs 1e-7, tests relaxed 1e-12 -> 1e-7 (written, not rerun) |
| `roofs::..::the_upward_surface_of_a_pitched_roof...` (r-gable.surface_area) | mine: tolerance 1e-9 -> 1e-6 (written, not rerun) |
| `railings::..::the_infill_is_a_slab_per_bay...` (r-panel infill area) | mine: round posts are tessellated, their inset differs by ~1e-5; tolerance 1e-4 (written, not rerun) |
| `stairs::..::a_deeper_landing_pushes_the_second_flight...` | mine: test asserted the wrong axis (the second flight of a left turn grows along x); now `max.x` grows by 0.8 (written, not rerun) |
| `diagnostics::..::index::tests::every_code_is_listed_once_in_all...` | a peer inserted `RefViewportSheet/View/RefRevisionSheet` before `RailingHostUnresolved` in the enum but after it in `ALL`; I reordered `ALL` (verified by script: 111 = 111, same order) |
| `walls::..::attached::a_wall_under_a_gable_roof...`, `a_window_in_a_trimmed_wall...`, `wall_sweeps::..::the_inferred_attic_equals_the_table...`, `..committed_attic_meshes..`, `diagnostics::..::wall_depth::a_fully_covered_attach_is_silent` | wall-depth package (peer) built on my new `roofs::underside_pieces` (the peer added it to my roofs file); the wall under a gable end stays at 3.0 (`WallAttachUnreached`). Cause not found without a debugger run (I added a `[DEBUG]` print test, removed again after the framework build broke); the roof side is correct per the roof oracle. First thing to check: `underside_pieces` vs the attach sampler after the module move `🔗️attach` -> `🧲️attach` |
| `element_solids::..::the_committed_expectations_and_meshes_match_the_inference` (curtain-grid meshes stale), `plan_kit::..::the_planar_projection...` (beams case: 13 rows now), `beams_reproduce...` (b-arc-incline), `ramps::..::a_curved_ramp...`, quantities `a_ramp_measures...` and `the_subject_reproduces_the_third_party_oracle_table` (finishes), `diagnostics::..::viewports_and_revision_rows...` | not mine (curtain/beams/ramps/finishes/views packages; need BIM_BLESS or their own fixes) |

## Re-run list once the framework builds again
1. `cargo test ... --lib -- inferences::element_solids inferences::plan_linework inferences::diagnostics inferences::quantities inferences::stair_runs export::ifc::circulation` (expect my 8 above green; the attach/wall-sweeps 5 need the cause above), then `--target wasm32-wasip2 --lib` check.
2. Platform harness `oracle quick --case <📦️infer-bim-1-solids-rest|🛤️infer-bim-1-stair-runs|🗺️infer-bim-1-plan-and-diagnostics>` (was blocked by the value refactor at 03:31, not retried).
3. Generators `r3-f1-gen-mutation-facets.ts`, `r3-f1-gen-oracle.ts`, `r3-f1-gen-feature.ts` (no leaf of mine changed; `r3-f1-check-names.ts` ran: 2 problems, neither mine).
Unverified-by-compiler items most likely to need a fix: mansard/gable line counts in `the_ridge_and_the_hips...` (ridge 1, break 4, hip 8, gable verge 4), the IFC circulation tests (compiled in the 14:26 run: they passed, no failure listed).

## Open items / out of scope
* Model-graph `value` has no control channel, so the graph calls the uncontrolled roof variants; the controlled ones are tested (a cancelled skeleton gives a flat roof + `roof.fallback-flat.skeleton`).
* Plan still draws the roof outline only: the plan node has no roof-solid parent and `PlanKind` has no roof-line kinds (lines exist in `RoofGeometry::lines`).
* Hosted railings (`rail_hosts`, `ramps::railing_meshes`) use the extents of the authored sections only: no baluster row, no infill on sloped paths.
* Curved footprints under pitched shapes fall back to flat (reported). Pitch rules now: Hip/Gable pitch 0 = flat (silent); Mansard needs lower, upper > 0 and break > 0.
* Longest new path 189 characters (limit 256). No `[DEBUG]` lines left; `__pycache__` created by oracle runs deleted.
