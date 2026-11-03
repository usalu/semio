# Report QA6: Visual QA of M2 Machinery

## Round 1
- **Driver** `T/🧰️QA6/all.ts`:
  - The check reports no problems.
  - `coverage.py --consistency` is ok for 27 of 27, including the crane after the depthM change.
  - The three-views vitest passes 19 of 19.
- **Inspected:** a subset of scales per asset, listed in QA6's final message. The pumpjack stroke, plate-heat-exchanger flap, scissor lift, wheel-loader bucket, crane hook and sorption thermal read correctly.
- **Open:**
  - Remaining scales and the solar-thermal-collector sheets are not inspected.
  - The wheel-loader plan bucket shows ribs only, with no outline.
  - The crane `parked-weathervane` pose cannot slew: the rig has no axial-twist DOF, so the jib does not turn.
  - The scissor arms are hidden in plan, and the sorption rotation is invisible because its axis is lateral. Both are accepted.

## Decision
A framework gap is confirmed: there is no twist (yaw) DOF. It shows in the crane slew, bar-stool swivel, office-chair swivel and bicycle steering. TS implements `twistDeg`, a rotation about the bone's own axis.

## Round 2
- **Inspected:** every asset and scale (dark-line and thermal sheets, all poses and 3 clip instants per clip, stacked sheets at 100 to 150 dpi), including the solar-thermal-collector. Crane 1:50 was checked in tiles at 100 dpi.
- **Wheel-loader plan bucket:** now a true top view, an `extruded` solid on the `bucket` bone using the elevation profile across the full width, plus the cutting edge at 1:50 and 1:100. It follows lifting and tilting in every pose and clip (checked at carry, dig, dump-high and load-cycle). Interior ribs cannot be drawn: the outline pass merges same-region shapes into one silhouette, so ribs inside the bucket region vanish. They would need an own thermal region, so they are dropped; the decision texts say so. The 1:500 `shape-role-undrawn` finding for the rib detail is gone with them.
- **Crane slew (`parked-weathervane`, jib turned 60 degrees):** the jib and counter-jib are bound as `followTip` rects and polys in front (the 1:100 jib is stretched 47 m sideways) and as `followTip` capsules in plan (chords, rails, pendant line; silhouette at 1:500). The lattice diagonals and the jib tip sheave are no longer drawn in plan, because a plan polyline cannot follow a slewed jib (plan `followTip` exists for capsules only). Rigid platform, cab and ballast turn correctly. Checked at 1:50, 1:100, 1:200 and 1:500 in all three views.
- **Check:** clean for all eight assets. **Consistency:** 27 of 27 ok. **Three-views vitest:** 19 of 19.
- **Remaining doubts:** crane plan lattice diagonals are omitted; the wheel-loader bucket has no ribs; scissor arms hidden in plan and the lateral sorption rotation are accepted.

## Round 3
- **Tower crane:** the plan is back to the original rigid shapes (lattice diagonals, jib tip sheave, polylines, bound to `jib`, `counter-jib`, `slewing`); no plan `followTip`. In plan they rotate exactly about the vertical in `parked-weathervane` at 1:50, 1:100, 1:200 and 1:500. Front: with fully rigid shapes the guard fails (`spread-front-x-plan-y` at every scale: front x [-13.5, 3.1] m vs plan y [-14.6, 48.0] m), because a jib seen end-on is 1.6 m wide and a rigid shape cannot reach 47 m sideways. The front jib and counter-jib shapes therefore keep `followTip` (the front lateral stretch rule from fw-status); with it the guard passes and the front reads correctly at all scales. Please confirm with TS whether rigid front shapes should have passed.
- **Wheel-loader bucket:** outline and cutting edge as extruded solids, ribs as unfilled detail strokes (z 3.3 above the body, pen 0.18 mm; 4, 3, 2 and 0 ribs at 1:50, 1:100, 1:200 and 1:500). Before the FW same-region fix, the strokes inside the filled bucket were swallowed by `outlineRuns` in `S/🔨️modules/🎨️depiction/🧩️shapes/🟦️.ts`: the `joined(p, q)` term (line 192, applied at line 198) hid the strokes of every shape in the region of a filled occluder. With the fix they show at 1:50, 1:100 and 1:200 (dump-high checked).
- **Results:** check clean, consistency 27 of 27 ok, three-views vitest 19 of 19.

## Round 4
- **Crane front, 3D definition:** the front jib and counter-jib are `extruded` solids from their bone-local elevation profile (jib rect 0..55 by ±0.8 with z ±0.8, counter-jib 0..16.5 by ±0.7 with z ±0.7; the 1:500 silhouettes the same, the jib tip sheave z ±0.1). No `followTip` remains on the crane. The end-on crossing diagonals of the front jib frame are dropped: an extruded solid only carries the profile's height range, and the end-on X exists only at the default pose. The guard passes at all scales and `parked-weathervane` reads correctly in all three views at 1:50, 1:100, 1:200 and 1:500.
- **Loader bucket plan lines:** the vertical lines are the creases of the tilted bucket profile (lip, floor, back, rim; the extruded solid has one edge per profile vertex), which are physically visible from above. The hood strokes now visible (louvre lines, inner hood outline) were authored as plan detail and were hidden before the same-region fix. All stay.
- **Results:** check clean, consistency 27 of 27 ok, three-views vitest 19 of 19. Asset JSON is written atomically.

## Round 4 (Coordinator Summary)
- **Crane front:** jib, counter-jib and the 1:500 silhouette are `extruded` solids from their elevation profiles. No `followTip` remains on the crane. `parked-weathervane` reads correctly at all scales. The end-on X bracing of the front jib face is dropped (accepted).
- **Loader:** bucket creases in plan are physical edges; the hood louvres are now visible after the same-region fix.
- **Checks:** check clean, consistency 27 of 27, three-views vitest 19 of 19.

## Round 5
- **Wheel-loader front:** the lift arms, the lift and tilt cylinders with their rods, and the bucket are `extruded` solids from their bone-local elevation profiles (arms z 0.9 to 1.2 and -1.2 to -0.9, cylinders at ±0.6, tilt cylinder on the axis, bucket ±1.475). They now connect body and bucket in every pose; verified at 1:50, 1:200 and 1:500 at `raising`, `start-and-lift@5`, `load-cycle@6` and `dump-high`. The plan arms, cylinders and bucket use the same solids, so no `followTip` is left on the loader.
- **Scissor-lift front:** the ten arms are `extruded` solids from the arm profile (z = lateral position of the arm plus or minus half its width). At 1:50 and 1:100 they form continuous strips from the wheels to the platform in `mid-raise`, `raised-working` and `raise-cycle@12`.
- **Results:** check clean, consistency 27 of 27 ok, three-views vitest 19 of 19.
