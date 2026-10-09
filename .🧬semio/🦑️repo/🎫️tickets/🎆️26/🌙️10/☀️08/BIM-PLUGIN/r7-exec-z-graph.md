# r7 execution report: z-graph (one model graph, real parents, incremental session)

`T` = this folder. `I` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences`. Design: `r7-design-model-graph.md`. API for `z-consumers`: `r7-api-model-session.md`. Facet generator: `r7-z-graph-facets.mjs`.

## Result

Every derived value of the model is a node of ONE `InferredField` (`I/🕸️model-graph`, id `s.bim.model.inference.model-graph`), keyed by the typed enum `ModelNode` (Storey, Band, Cut, WallLayout, CurtainLayout, Host, OpeningFrame, StairRun, Solid(family,id), Room(storey), Plan(storey), Quantity, Totals(scope), Diagnostics(scope)), valued by `ModelValue { node, data: Data }` (Arc payloads). `plan()` emits the topological order with real parents (storey -> band/wall layout -> host -> opening frame -> solid/quantity/totals; storey + layouts -> room -> plan; levels + layouts + frames + runs + rooms -> diagnostics). `compute` dispatches to the pure functions of the leaf modules and receives its parents' VALUES. No consumer recomputes layout, host, top, validity, run or room any more. The framework engine data model is unchanged (the engine already hands all parent values to `compute`; one value enum removes the same-field limit), one additive engine change for F12.

`ModelInference` keeps its ten `#[derived]` fields, now projections of the node values; `compute_*` helpers are thin projections (`infer_selected::<MASK>` = graph restricted to the wanted kinds and their ancestors). `ModelInferenceSession::{update(&snapshot,&diff), refresh(&snapshot), inference(), report()}` runs `infer_field_after_diff` with an enabled `InferenceCache` and copies only changed entries into the held inference.

## Findings fixed

| Finding | How |
|---|---|
| F01 | Wall layouts take the `Band` of the wall and of every wall within two touches as parents (`joins::touching`: one sweep over inflated axis boxes per storey, exact test per candidate pair; `neighbourhood` = T2). `storey_bands`/`layout_of`-per-wall scans are gone from production (`storey_bands` stays `#[cfg(test)]` as the whole-storey reference; test `a_layout_over_the_neighbourhood_equals_the_layout_over_the_whole_storey` is bit-exact on house and the 30-wall join model). |
| F02/F03/F04 | `layout_of` only in the `WallLayout` node; `HostExtent::of_wall/of_curtain` only in the `Host` node (one mullion depth: `plan_kit::depth_of`); `storey_levels::{top_of, vertical_of, constraint_storeys, target_of}` is the only top resolver (copies in wall_layout, element_solids, bodies, stair_runs deleted). |
| F05 (partial) | `plan_kit` owns `point, mark, seg, extents_of, depth_of, bulged, placed, rectangle`; copies in bodies/plan-linework/columns/wall-layout/opening-frames removed. |
| F06 | One rule `opening_frames::host_issues` (tolerance `LENGTH_EPS` 1e-9, join-trimmed extent `HostExtent.trim` from `wall_layout::face_ends`, top): new `OpeningIssue::OutsideTrimmedExtent` (+ `HostDegenerate`, F22). The wall solid cuts exactly the `valid` frames (`walls::cuts_of`), fillers are built for exactly those, quantities subtract exactly those, diagnostics report `OpeningOutsideTrimmed` (en + de). |
| F07 | `quantities::cut_area` = union of the valid cut rectangles (`union_area`), tests: window at wall end, two overlapping windows, take-off volume equals solid volume. |
| F11 | Session with enabled cache and diff gate in production; node-count tests. |
| F12 | Engine: `dep_input` and hashing only with an enabled cache (test `dependencies_are_hashed_only_when_a_cache_can_use_them`); `Diagnostics(Model)` depends on `ReferenceView` (ids and reference fields only); storey/building/plan/room dependencies are scoped to the records and the types they name (`dep_records` without names, `dep_types`, `present`). |
| F13 | Table in the design; plan test per kind. |
| F14 | `RoofFallback` rides the `Solid(Roof)` value, `Diagnostics(Storey)` reports `RoofFlat{Curved,NonConvex,Degenerate,Pitch}` / `RoofOverhangCollapsed` as warnings, en + de rows. |
| F17 | `Plan` has `Room(storey)` as parent and draws rooms from it (second boolean arrangement removed). |
| F18/F19 | host -> openings index built once in `plan::build`; frames take sibling `Cut` parents; wall solids/quantities read hosted frames from parents; obstacles are the `WallLayout` footprints with a precomputed bounding box tested before any boolean. |
| F21 | Gating tests for storey-levels and quantities; determinism/default for stair-runs and spaces; one cache-transparency test covering every projection (cold = warm = uncached on the IFC house and the join model). |
| F22 | `HostDegenerate` issue instead of the silent `(1,0)` tangent. |
| F25 | `SolidSource` removed; `SolidKey { family: SolidFamily, id }`. |

## Changed expectations (legitimate output changes)

* A window/door flush with a wall end, a joined corner or the top is now invalid (`OutsideTrimmedExtent`): no hole, no filler, no quantity deduction; formerly `valid` with silently no hole. The committed oracle fixtures contain no such opening, no table changed (all ten `*infer-bim-*` oracle cases agree).
* Quantities no longer subtract holes the solid never cuts, and overlapping siblings are both invalid (no double count). Test `an_opening_outside_its_host_is_clipped_and_a_broken_type_is_ignored` was replaced by `an_opening_the_wall_does_not_cut_is_not_subtracted` (its old expectation encoded the clipping).
* `DiagnosticCode::ALL` 54 -> 60 (`OpeningOutsideTrimmed`, 5 roof fallbacks); `OpeningIssue` +2. Facets (json/ts/graphql/proto) regenerated by `r7-z-graph-facets.mjs`.
* `rooms_of(snapshot, storey, level)` ignores `level` (the graph derives it); `compute_quantities(snapshot, &inferred)` ignores its second argument.

## Commands and results

| Command | Result |
|---|---|
| `cargo test --manifest-path ✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/Cargo.toml -p semio-s-artifact-bim-model --lib -- --skip editor::` (gate, debug) | **3469 passed, 0 failed** (219 filtered = editor::, the known close stall owned by z-close) |
| oracle `quick` for 🎲️solids-three, 🏠️spaces, 📦️solids-rest, 🔳️wall-solids, 🗺️plan-and-diagnostics, 🛤️stair-runs, 🧮️quantities, 🧱️wall-joins, 🪜️levels-and-wall-heights, 🪟️opening-frames | each 1 case executed, all passed (0 failed) |
| moving one wall, 511 walls + 64 windows (debug build, temporary `[DEBUG]` test `zz_debug_timing`) | 88 of 2385 nodes computed (1 band, 23 wall layouts, 3 hosts, 3 frames, 26 solids, 26 quantities, 1 plan, 2 diagnostics, 3 totals); result equals a fresh inference |

## Timing

Temporary [DEBUG] test (removed), 511 walls on one storey + 64 windows, `cargo test --release`, same machine, other agents building:

| Measure | Release | Debug profile |
|---|---|---|
| uncached `ModelInference::infer` (whole graph, no hashing) | **84 ms** (r4 baseline for the same walls, solids only: 142 ms, wall-layout alone 145 ms; O(W^2) gone) | 320-630 ms |
| session first `update` (hash + compute + cache fill, 2385 nodes) | 354 ms | 0.85 s |
| session `update` after moving ONE wall (88 of 2385 nodes computed) | **353 ms** | 0.6 s |
| fresh uncached infer of the edited model | 92 ms | 0.31 s |
| plan build alone | 32 ms | 26 ms |

Honest reading: the number of COMPUTED nodes is proportional to the neighbourhood (88 of 2385), but the wall-clock of an update is currently NOT better than a fresh uncached inference for this size, because the engine walks and hashes the dependency of every node on every run (dependency construction + JSON encoding of 2385 nodes about 66 ms in release, plan 32 ms, the rest is engine bookkeeping over about 15k parent edges: step/parent clones, hex round trips, merkle sort). The hex helper of the engine was made allocation-free (additive, same output). Next step (not done, tree was not compiling when I wanted to profile the engine split): let the engine skip hashing for keys the diff cannot touch (a `touched(key, diff)` hook), or make dependencies smaller/shared (wall record serialised 4x per wall: band, layout, solid, quantity). Compute work itself is already minimal.

The wasm32-wasip2 check was not run: the tree stopped compiling because of in-flight edits of other agents (new mutation leaves/ceilings/ramps/schedules mounts) right after my last green run; my files compile (last full green run: debug lib, 3469 passed, 0 failed, editor:: skipped).

## Open

* Per-run cost of an incremental update is dominated by the engine hashing every node's dependency (O(N) per edit, 185 ms of ~600 ms in debug for 2385 nodes); compute is O(neighbourhood). Smaller dependencies (or an engine skip list for untouched keys) is the follow-up.
* Clashes (`Diagnostics(Building)`) recompute for the whole building when any body of it changes.
* Editor `🔮️inference` session, `render` memo and IFC export still call the thin projections; `z-consumers` migrates them to `ModelInferenceSession` and then deletes `compute_*`, `rooms_of`.
* Final rerun of the lib test after my last edits (removal of temporary [DEBUG] timers; no logic change) was blocked by other agents breaking the tree (`📋️schedules` mount missing a file); re-run the gate once it compiles.
