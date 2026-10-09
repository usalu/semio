# 🔎️ R11 Audit — w01-depth-infer (WP-01)

Status: **PARTIAL**. Stairs, stair-runs and railing solids wired; plan cut height and skeleton roofs not started; railings
unit test broken; diagnostics, quantities, IFC, oracles and feature scenarios incomplete.
`S` = artifact subset, `I` = `S\🧬️schema\💡️inferences`. Last log `T\🗑️generated\w01-depth-infer\check0.txt` (22:54) predates the
railings (23:15) and stairs (23:25) edits.

## Status table
| Item | Present | Wired | Notes |
|---|---|---|---|
| Stairs solid: stringer, nosing, tread_thickness, riser | Yes | Yes | `I\🧊️element-solids\🪜️stairs\🦀️.rs` STEP/RISER/STRINGER/LANDING (148-166, 258-262) |
| Stair-runs landing_depth | Yes | Yes | `I\🪜️stair-runs\🦀️.rs` 139-152 |
| Railings solid: profile, post_profile, baluster, infill | Yes | Yes | `I\🧊️element-solids\🛤️railings\🦀️.rs` 102-163 |
| POST_SIZE/RAIL_WIDTH/RAIL_DEPTH | Removed from prod | — | still used in railings unit test (42, 47, 59) → lib test build breaks; oracle `S\🧪️tests\📦️infer-bim-1-solids-rest\🐍️.py` 54, 66, 538 |
| Plan cut height | No | No | `I\🗺️plan-linework\🦀️.rs:51` `CUT_HEIGHT = 1.2` used at 367, 375 |
| Roofs on framework skeleton | No | No | `🏠️roofs\🦀️.rs` unchanged; framework `📐️geometry\🏠️roof::roof_surface_controlled` available |
| Roof fallback diagnostics | Yes | Yes | `⚠️diagnostics\📐️validity` 250-262, messages 84-87 |
| `roof.gable-ends-adjust-to-hip` | No | No | — |
| `stair.stringer-ignored-on-spiral` | No | No | `stringer_ignored` (stairs 271) has no caller |
| Quantities stair/railing volume | Yes | Yes | from solids |
| Quantities infill area, baluster count | No | No | — |
| Editor property rows | Yes | Yes | `✏️editor\🧩️entities\🦀️.rs:589-605`, cut height 476 |
| IFC stringers/infill/baluster | No | No | `🚪️io\📤️export\🏗️ifc\🪜️circulation` |
| Stair dependency() | Partial | — | lists stringer, nosing, tread_thickness, riser; NOT landing_depth (cache honesty) |

## Tests/oracles
Stairs unit test has no stringer/nosing coverage; solids oracle stair law prism-only (437-506); plan oracle `CUT_HEIGHT`
(46, 554); plan unit test asserts 1.2; no gating/parametric cut tests; no concave roof case; no `.feature` scenarios.

## Remaining tasks (ordered)
1. Railings unit test + oracle → read profiles (rail 0.06×0.04, post 0.05×0.05 per handoff §3).
2. Stairs unit tests (tread, riser, stringer, nosing volumes) + oracle formulas (handoff §2).
3. `stair.stringer-ignored-on-spiral` diagnostic (en+de) emitted from validity stair loop.
4. Plan cut height from `storey.cut_height.unwrap_or(DEFAULT_CUT_HEIGHT)` (`📸️snapshot\✅️validity\🦀️.rs:90`); gating +
   parametric tests; plan oracle; feature scenario.
5. Roofs on `roof_surface_controlled` (handoff §4), `roof.gable-ends-adjust-to-hip`, concave hip/valley fixtures + oracle.
6. Quantities: infill area, baluster count.
7. Model graph parents + landing_depth in stair dependency; gating + cache-transparency tests.
8. IFC stringers as IfcStair flight parts.
9. Feature scenarios (solids-rest, plan-and-diagnostics).
10. Build + lib test.
