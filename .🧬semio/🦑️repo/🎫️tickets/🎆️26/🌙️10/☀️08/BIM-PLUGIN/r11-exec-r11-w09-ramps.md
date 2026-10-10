# R11 execution report: r11-w09-ramps (WP-09 ramps finisher)

T = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN`, S = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`, I = `S/🧬️schema/💡️inferences`.
Gate label `r11-w09-ramps`, logs in `T/🗑️generated/r11-w09-ramps/` (`check-lib.txt` is the last `cargo check --lib` of the artifact).

## Status in one paragraph

Everything on the audit list is written. The third-party pieces that need no Rust build ran green (ramp-runs oracle, platform run of the new case). **No cargo test result exists**: from the start of the run until the end,
`cargo check` of `semio-s-artifact-bim-model` stopped in the framework (owner commit 677 retirement refactor in `os 🏪️store`, `📖️playbook`, `♾️infinite/dag`, then the `🔌️plugin ♻️cancellation` move, 16 to 800 errors
flapping while `r11-store*` agents worked). None of the errors is in a file of this package; the last run before this report was `check-lib.txt`. So every Rust test below is **WRITTEN BUT UNVERIFIED**, and the
committed IFC export of the new ramps case, the blessed `🗣️.dsl.semio` text of the house and the oracle table of the IFC ramps case could not be produced yet (exact commands under "To run once the build is back").

## 1. Compile error

`S/✏️editor/🧵️gestures/🧱️chain/🦀️.rs:120` (`ctx.name_of(|labels| labels.kind_ramp, count)`) was already correct when I re-read it (r11-baseline/merge fixed it): `kind_ramp` exists in the terminology table, `name_of` in the session. Nothing to do.

## 2. Files touched (before -> after)

Created
- `I/🛝️ramp-runs`: tests extended (see 3). `S/🚪️io/📝️text/💡️inferences/🛝️ramp-runs/🦀️.rs` (new, `table_json`; follows upstream's move of per-inference `table_json` into `io/text/inferences/*`, mounted in `io/text/inferences/🦀️.rs`); `table_json` removed from the schema module (the merge agent had already wired the `"ramp-runs"` slug of `encode_inference_projection_json`).
- Fixture `S/🧫️fixtures/💡️inferences/🛝️ramp-runs/🏞️ramps/{📸️snapshot,💡️inference/🛝️ramp-runs}/🔣️.json`: 17 ramps (straight, at the exact 1:12 limit, steep, L-bent, zigzag, smooth join, quarter arc, to-storey constrained long and short, on the next storey, descending, no sloped run, flat, merged landings, degenerate path, wide, ramp on a missing storey). Snapshot written by `T/r11-w09-ramps-fixtures.py`, table written by the oracle.
- Oracle case `S/🧪️tests/🛝️infer-bim-1-ramps/{🐍️.py,🥒️.feature,🦀️.rs}`: second independent implementation of the run (arc length from the bulge, corners from tangent angles, merged landing intervals, storey levels from the sibling levels oracle) audited with shapely 2.1.2: GEOS length of the 4096-chord path, `shapely.ops.substring` cut of every flight and landing (they tile the path), mitred buffer area = width x length, continuous heights, slope identity, and the metamorphic storey-raise law. Scenario `@id-ramp-runs-ramps`, tags `capability-bim-1-infer`, `oracle-bim-1-shapely-geometry`, `comparison-floating-point-v1` (the existing manifest row is reused, like stair-runs).
- IFC case `ramps`: oracle/feature/adapter extended (see 4); fixture dir `S/🧫️fixtures/🏗️ifc/🛝️ramps/` will hold the export (not yet written).
- `T/r11-w09-ramps-examples.ts` (house entrance ramp + 2 hosted railings, validated: references, slope limit, plot), `T/r11-w09-ramps-fixtures.py`.

Updated
- `I/🛝️ramp-runs/🧪️tests/🔬️unit/🦀️.rs` 178 -> about 300 lines (+6 tests, gating/cache transparency).
- `I/⚠️diagnostics/🧪️tests/🔬️unit/🦀️.rs` +4 tests (RampSlope, RampNoRun, DegenerateAxis/NonFinite apart, relaxing/lengthening clears).
- `I/🧮️quantities/🧪️tests/🔬️unit/🦀️.rs` +5 tests (`ramp_quantity`).
- `I/🗺️plan-linework/🧪️tests/🔬️unit/🦀️.rs` +7 tests (ramp linework).
- `S/🚪️io/📤️export/🏗️ifc/🛝️ramps/🦀️.rs`: a ramp whose path has no length is skipped (`x.skip`), not written empty. Tests +3 (counts, committed file is the current export, subject report equals oracle table).
- `S/🚪️io/📤️export/🏗️ifc/🔬️projection/🦀️.rs`: `COUNTED` 46 -> 48 (`IfcRamp`, `IfcRampFlight`, appended). Python `COUNTED` likewise. `S/🧫️fixtures/🏗️ifc/🏠️house/🔬️measure/🔣️.json` got the two zero counts (that table was already behind `COUNTED` from other waves; whoever runs `🐍️.py write` fixes the rest).
- `S/🧪️tests/🏗️export-bim-1-ifc/{🐍️.py,🥒️.feature,🦀️.rs}`: new case `🛝️ramps` and scenario `@id-export-ifc-ramps`.
- Examples: `T/r4-x-examples-gen.ts` (merges `rampsFor`, verifies ramp/host references), regenerated `S/🖼️assets/{🏡️house,🏢️office}/📸️snapshot.json` (only `ramps` and `railings` changed); `S/📚️examples/🧰️checks/🦀️.rs` (ramps in storey/material/top/host dangling checks, owner set, `ramp_runs` completeness and compliance in `infer`, `create-ramp` in the replay table before the railings it hosts); `S/📚️examples/🏡️house/🧪️tests/🧩️example/🦀️.rs` (railing count 4, two new tests).
- Views: `S/✏️editor/🎮️commands/🔭️create-view/🦀️.rs` `extents` and `T/r10-w12-views-examples.ts` `extents` now include the points of ramp paths (otherwise a ramp outside the building footprint was cut by the elevations, sections and the 3D camera). Both sides were changed together; w12 owns these, please keep.

## 3. The tests (all unrun)

- `ramp-runs`: oracle table reproduced; graph parents (own and target storey, orphan not planned, parents first); gating (no-op diff gated, material edit and ramp rename recompute no run, a width edit recomputes exactly one `ramp-run`, an unrelated storey none, a ground-height edit moves only the ramps that follow the next storey, incremental == fresh); cache transparency cold == warm == uncached for ramps and ramps with hosted railings over ramp_runs/solids/plans/quantities/diagnostics, thin projection == whole; a hosted railing follows its ramp (only that railing re-inferred).
- diagnostics: `RampSlope` exactly for r-bent, r-steep, r-to-first-short with slope/limit/rise/run values, severity, key, en and de text, r-at-limit clean; `RampNoRun` for r-no-run only, `DegenerateAxis` for the empty path, merged-landing flat ramp clean; lengthening or relaxing clears, shortening raises; NaN width is `NonFinite` once.
- quantities: path/width/rise, strip as gross and net area, perimeter, slab volume, walking-surface area `1.2 x (3 + sqrt(49.25))`, mass, one layer row; side railings do not count as slab; bent strip 13.2 m2; descending rise -0.5; totals per kind/material/storey; follows the target storey height; degenerate path has no volume, missing storey no row.
- plan: outline (closed, 4 vertices, area 12, projection style), landing lines at 1.5 and 8.5 across the width, up arrow with head, tag `7.1 %` / `1:14.0` at the middle with the slope as measure; cut / projection / hidden by the cut plane; bent ramp (6-vertex mitre, 4 landing lines, bent shaft, tag at 5.5 m); arcs stay arcs and a descending arrow points down; degenerate and orphan draw nothing; unique ids; a ramp edit redraws exactly the plan of its storey.
- examples: the house has `rp-entry` (11 m bent path, 6.5 m sloped, 0.5 m rise from -0.5 to 0.0, 2 flights, 3 landings, ends on the door axis) and two hosted railings, all compliant, with solids and quantities; replay through `create-ramp` reproduces the example; an unconnected ramp keeps its rise when the storey height changes.

## 4. IFC (ifcopenshell)

`🐍️.py`: new case `🛝️ramps` (snapshot and committed table are the shared ramp-runs fixture), `ramp_problems`: `IfcRamp` set == ramps the shapely-adjudicated table resolves with a path, `ShapeType` by the buildingSMART rule (spiral / straight / two straight / user defined), container storey, `get_parts` -> one `IfcRampFlight` per flight and one `LANDING` `IfcSlab` per landing, one `IfcRailing` per ramp with a side railing, flight and landing `Length`/`Width` quantities, ramp `Length`, `Width`, `Height` (= rise) and `GrossArea`, kernel volume of the parts summing to `GrossVolume` (1e-6), authored `Semio_Authoring` ramp record equal to the snapshot, no orphan flights or landing slabs. Generic audit adapted: `IfcRailing` = authored railings + ramps with sides, ramps are known containment tags; `committed` is now a dict (`annotations`, `ramp-runs`); `write` accepts case names.

## 5. To run once the build is back (not run)

```
gate=T/🚦️gate.sh ; M=S/../../../../.. (the artifact manifest)
"$gate" r11-w09-ramps -- cargo test --manifest-path <artifact>/Cargo.toml -p semio-s-artifact-bim-model --lib ramp
"$gate" r11-w09-ramps -- env BIM_BLESS=1 cargo test ... --lib the_committed_ramps_file_is_the_current_export
"$gate" r11-w09-ramps -- env BIM_BLESS=1 cargo test ... --lib bless_the_house_text
.venv/Scripts/python.exe S/🧪️tests/🏗️export-bim-1-ifc/🐍️.py write S/🧫️fixtures/🏗️ifc 🛝️ramps       (then check)
cd 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test && SEMIO_TEST_LEVEL=quick bun ./📜️script.ts oracle quick --case 🛝️export-... (infer-bim-1-ramps already green; export-bim-1-ifc after the write)
cargo test --lib (exact counts) ; cargo check --target wasm32-wasip2 --lib
bun T/r3-f1-gen-mutation-facets.ts ; bun T/r3-f1-gen-oracle.ts ; bun T/r3-f1-gen-feature.ts ; bun T/r3-f1-check-names.ts
```
Likely first findings to expect: the walking-surface area of the quantity test, the kernel volume of open per-piece breps (if `kernel_volume` cannot tessellate an open flight brep, the ramp oracle reports it; then the volume check should compare only the ramp's own aggregate), and `ModelDiff` computed-by-kind counts in the gating test.

## 6. Commands that did run

| Command | Result |
|---|---|
| `python 🐍️.py write` / `check` of `🛝️infer-bim-1-ramps` (`.venv`, shapely 2.1.2) | `oracle agrees`, 17 ramps, no problem |
| platform: `SEMIO_TEST_LEVEL=quick bun ./📜️script.ts oracle quick --case 🛝️infer-bim-1-ramps` (test module) | `executed=1 passed=1 failed=0` |
| `bun T/r4-x-examples-gen.ts` | house and office written; only `ramps`/`railings` differ from before |
| `cargo check --lib` through the gate (many times) | blocked by framework errors outside this package, never reached the BIM crate |
| `cargo test`, wasm32-wasip2 check, IFC oracle on the new case | NOT RUN |

## 7. Open items

1. Run the list in section 5 and fix whatever the unrun Rust tests find.
2. Generators not re-run (`gen-mutation-facets`, `gen-oracle`, `gen-feature`, `check-names`): nothing of mine changes a schema or a mutation leaf; run them with the verification pass. Emoji uniqueness of my new folders was checked by construction (`🛝️` is new among its siblings in `🧪️tests`, `🧫️fixtures/💡️inferences`, `🧫️fixtures/🏗️ifc`, `io/text/inferences`).
3. The ramp case is not registered in the taxonomy members list (`🔣️taxonomy.json`), like the other recent cases.
4. All path lengths of the new files are below 256 characters (longest file path of the BIM package and this ticket measured at 240).
