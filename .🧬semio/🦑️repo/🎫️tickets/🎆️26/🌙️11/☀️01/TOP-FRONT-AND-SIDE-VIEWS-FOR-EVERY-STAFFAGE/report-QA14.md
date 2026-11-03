# Report QA14: Visual QA of the Components (C2 assets)

**Assets:** steel-column, sliding-door, tilt-window, tied-canopy, wheelchair-ramp, stair-flight, spiral-stair.

## Round 1
- Rendered 444 `--views` sheets (every pose, 3 clip instants plus keys, every authored scale) and read them as montages and zooms. No shape fix was needed.
- Same-region z audit (`T/🧰️QA14/zaudit.py`): only four hidden duplicate posts/plates in the wheelchair-ramp 1:50 front (end-on rows of posts), left as they are.
- Spiral stair: the treads are fixed bones with the 22.5 degree azimuths authored in plan and front, consistent at rest. The walker carrier was wrong: `walker-x` is prismatic along the parent's `u` (the vertical column), so `dx` moved the carrier vertically and its forward and lateral position was never expressed. Fixed with a turn bone, a prismatic radius (`axisDeg` 90, `dy` = -0.6) and `walker-y`.
- Final check, `coverage.py --consistency` and the three-views vitest were green for the seven assets.

## Round 2
1. **Twist limit (clean fix, single `walker-turn` bone).**
   - A revolute joint may declare `twistMinDeg` (-720..0) and `twistMaxDeg` (0..720). Without them the twist stays -180..180; the spread stays +-180.
   - Changes: `S/🔨️modules/🦴️rig/🟦️.ts` `effectiveOffset` clamps to them; the validator `joint-limit` in `S/🔨️modules/🗂️catalog/🟦️.ts` compares the authored twist with the effective one and names the joint's limits; schema `Joint` (json + twin) gained the two fields and `BoneOffset.twistDeg` is -720..720 (the joint decides); README line updated.
   - Test vectors (`S/🧫️fixtures/🧊️axial-twist`, test `S/🧪️tests/🧊️axial-twist/🟦️.ts`, scenario "A joint may declare whole turns" in the feature): arm at torso twist 270 = 0.6 m to the right, at 360 = 0.6 m ahead (hand-computed); limits 300 allowed, 370 refused with `twistMaxDeg` 360, -300 allowed with `twistMinDeg` -360, -190 refused without. The 10 files of `T/🧰️TS/vitest.lateral.config.ts` (216 tests, including schema twin, conformance and asset validation) pass. A note is appended to `T/fw-status.md`.
   - Spiral: one bone `walker-turn` (child of `column`, revolute, angle 0..0, twist 0..360) with `walker-x` prismatic below it. Probe `T/🧰️QA14/walker.ts`: tread 8 pose x -0.588, lateral +0.117; top tread x 0.499, lateral -0.333; climb at 3.5 s x 0, lateral +0.600; at 7.5 s x -0.600, lateral 0. This matches the plan sectors.
2. **Tied canopy.**
   - The construction matches the source (summary and statics): an IPE 160 beam pinned at the wall, with a tube strut hinged at both ends rising from the beam (4 m out) to the wall anchor 2.88 m above; the rod direction in the anchors (144.2 degrees) and the statics agree with the drawn elevation; front and plan follow it.
   - Not changed: the beam sits 0.12 m above the ground line. A real canopy hangs at door-head height, but the asset does not specify it, and raising it would change `dimensions`, the wall and every fixture. Recommendation: owner decision.
3. **Montages read:** spiral climb at 0, 1, 3.5, 7, 10.5 and 14 s (1:50); every 1:500 sheet of all seven assets (all poses and clips, incl. sun-and-shade and the load ramps) and the tilt-window 1:200 set. All three views are non-empty, consistent in extent and unchanged in structure.
4. **Final:** check suitable under `office-laser` at all 27 authored representations (tilt-window 1:500 stays unavailable); `coverage.py --consistency` ok for all; three-views vitest 14/14.

## Remaining doubts
- The climb and walker poses draw nothing in any view (the person is a load point only); all climb instants look identical.
- Steel-column, tied-canopy and wheelchair-ramp poses barely differ visually (tiny deflections), so poses weakly test the binding.
- The tilt-window fully-tilted sash leans about 30 degrees from vertical (authored pose).
- Tied-canopy beam height above ground (see above).

## Round 3: tied canopy raised to door-head height
- **Decision (coordinator):** the beam at 0.12 m above grade is not credible, so the canopy now hangs at door-head height: beam underside 2.25 m, beam centre 2.33 m. The strut keeps its bone-local geometry (4 m out, 2.88 m up, 144.246 degrees), so the anchors, the statics and the structure fixtures are untouched; the wall anchor is now 5.21 m above grade.
- **Changed, all in `T/🧰️C2/tied_canopy.py` (rebuilt from `orig/`, atomic write):**
  - Rig root `y` 0.12 to 2.33; `dimensions.heightM` 3.14 to 5.35 (also in the catalog manifest row); wall-face thermal area +2.21 m2 to 4.87 m2; the en/de summary names the door-head height (underside 2.25 m / Trägerunterkante 2,25 m).
  - Elevation, every scale: wall rects, render and insulation reach from the ground to 5.35 m; the insulation zigzag and the rebar lines are regenerated (same pitch) down to 0.30 m above the ground.
  - Front, every scale: ground-based coordinates, wall 0 to 5.35 m, every other shape lifted by 2.21 m. Plan is unchanged (a top view has no height).
  - The en/de silhouette decisions of all four scales say how the canopy hangs and what the front shows.
- **Checks:** consistency, check (suitable under `office-laser` at 1:500, 1:200, 1:100 and 1:50) and `coverage.py --consistency` are clean for the seven assets; the three-views vitest passes 14/14 for them.
  - Related suites also pass: `🪢️tied-canopy` (its fixture is independent of the asset), catalog-physics, structure-oracle, print-suitability, representation-copied and -resolution, asset-validation, schema-conformance, physical-invariance, thermal-scenarios, cast-export and axial-twist (911 tests pass in that run).
  - Four whole-catalog three-views cases fail, all for other workers' assets that are still being worked on (person-with-white-cane, runner-with-blade-prosthesis, running-child), none for these seven.
- **Sheets read:** all poses and clip instants at 1:500 (13 sheets), snow-load at 1:200, 1:100 and 1:50. The canopy reads as a door-head canopy on a 5.35 m wall; the strut is anchored at the upper plate in all three views.
- **Accepted by the coordinator:** the walker drawing nothing (structural component).
