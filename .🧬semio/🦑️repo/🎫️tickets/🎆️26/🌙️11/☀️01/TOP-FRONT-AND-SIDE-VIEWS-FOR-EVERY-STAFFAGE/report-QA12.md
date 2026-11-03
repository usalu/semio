# Report QA12: Machinery visual QA

- Mixer drum: revolved cannot express the tilted drum (the axis is always the rest-world vertical through the axis bone origin, carried by the bone's rest-to-now change; the drum's 14 degree tilt is a rest angle; the plan of a revolved solid is a circle). Front and plan are now the exact envelopes of the tilted circles (`a_mixer.py` drum_front / drum_plan).
- Mixer chute twist: twist turns a bone about its own longitudinal axis; the chute axis is 40..165 degrees, so twist 40 moved the tip by 0.0 m sideways (checked with `T/🧰️QA12/dbg.ts`). A yaw needs a vertical pivot bone, which does not exist. Poses unchanged.
- Mixer depth: lateral extent 2.55 m (tyres) at 1:200/1:500 and 2.70 m with mirror arms at 1:50/1:100; tolerance 0.27 m, depthM 2.55 kept (legal body width).
- Heat pump: static swept fan blades (7 at 1:50, 5 at 1:100) as unfilled lines.
- Renderer rule found: filled shapes of one region on one bone hide each other's inner outlines (`joined` in `outlineRuns`). Unfilled detail lines of such a region vanished (fan rings, spokes, mixer flights, end door, steering wheel). `lib.py` now drops region on unfilled shapes; forklift pallet z raised in front and plan; mixer lamps moved to cab-glazing.
- Final round: check ok (all suitable), consistency 28/28 ok, three-views vitest 16/16. Evidence: `T/🗑️generated/QA12/consistency.txt`.

## Round 2
- Chute swivel: new revolute bone `chute-swivel` (vertical, at the chute pivot, limits 0..0, twist free) carries `chute`; poses and clip keys swing it with `twistDeg` (30 degrees at full discharge, proportional to the chute luff). Guard (three-views vitest 16/16) and check pass.
- lib.py: region dropping reverted; saves are atomic (temp file + os.replace).
- Heat pump 1:50: spokes removed, 5 blades.
- Pending: revolved drum (needs the rest-tilt rule from TS), and an evaluator regression: the new `solid3` extruded branch emits one ring per profile edge, the line renderer outlines every ring, so curved extruded profiles (excavator boom, stick, bucket in front and plan at 1:50/1:100) render as a ladder of strokes.

## Round 3 (final)
- Revolved drum: with the rest-axis revolve (fw-status "revolved axis = the axis bone's own rest direction") the mixer drum is now one `revolved` solid on the `drum` bone in front and plan at every scale (`a_mixer.py` drum_solid; envelope code removed). In memory it matched the earlier exact envelopes to 0.01 m (front +-1.12 x 1.251..3.950, plan x -3.46..1.94 by +-1.12). Mixer sheets inspected at 1:50, 1:200 and 1:500 (every pose, discharge instants), plan shows the tapered outline.
- Excavator ladder regression is gone (1:50 and 1:100 sheets).
- Humidifier (1:50 to 1:200), valve (1:50, 1:100) and coil (1:50 to 1:200) sheets read well after the single 3D path; every pose and three instants per clip.
- Final: check ALL OK, suitable under office-laser; `coverage.py --consistency` 28/28 ok (`T/🗑️generated/QA12/consistency.txt`); three-views vitest 16/16.
