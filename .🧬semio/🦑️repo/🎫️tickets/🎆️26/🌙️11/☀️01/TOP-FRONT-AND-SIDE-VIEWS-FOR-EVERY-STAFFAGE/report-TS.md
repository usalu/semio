# Report TS: Three-Views Guard

## Built
- `S/🧫️fixtures/🧊️three-views/🔣️.json` holds the coordinate contract, the tolerance table, a synthetic asset on the sample-figure rig and six consistency cases. The hand-computed values are a 0.96 m hip height, a stretched front thigh of 0.417233 m and a plan capsule 0.168573 m ahead.
- `S/🧪️tests/🧊️three-views/` holds the test files:
  - `🥒️.feature`
  - `🟦️.ts` (vitest)
  - `🧰️.ts` (sweep and consistency rules)
  - `🐍️.py` (shapely oracle, run through `uv --with shapely`)
- `T/🧰️TS/` holds `coverage.py`, `consistency.ts` and `vitest.three-views.config.ts`. The commands are in [ts-status.md](ts-status.md).

## What it checks
- **Sweep.** For every asset, authored scale, pose and 6 instants of every clip, all three views are authored, non-empty and finite.
- **Consistency at the default pose.** With tolerance max(0.02 m, 3 % of the largest dimension) the views must agree:
  - elevation x = plan x
  - elevation y = front y
  - front x = plan y
  - x extent = widthM, lateral extent = depthM, height = heightM
- **Oracle.** Shapely agrees within 1 mm on the hand-computed vectors and on both complete reference assets.

## State at hand-off
- 169 of 320 tests pass. The 151 failures are expected: almost all are `front-missing` or `plan-missing`.
- In the first coverage run, 14 of 531 authored representations had all three views.
- The brief's reading of `dimensions` is confirmed by `split-ac-indoor-unit`.

## Inconsistencies found in existing front and plan content
- `bicycle-stand`: the front is offset by y −0.024 m.
- `city-car`: the plan width is 1.88 m against a depthM of 1.68 m.
- `jogger`: several checks fail at 1:500.
- `panel-radiator`: the front width is 1.05–1.07 m against 1.0 m.
- `scroll-compressor`: the front height is 0.34 m against 0.43 m.
- `seated-office-worker`: the plan width is 1.40 m against a depthM of 0.60 m.
- `standing-adult`: the plan width is 0.49 m against 0.40 m.
- `tilt-window`: the front width is 0.94 m against 0.90 m.
- `walking-commuter`: plan x fails at 1:50.

## Front y is relative to the rest root
`restWorld` solves the rest pose with the rig root at the origin. Front shapes are therefore authored relative to the rest root, not the ground.

- **Effect:** for a figure whose pelvis is lifted by the root, a front rect at y 0.8–1.0 evaluates to 1.76–1.96.
- **Authoring rule:** author front shapes with the root at y = 0, so the feet of a standing figure sit near −0.96.
- **Guard:** it measures evaluated values, so it holds whichever reference is used.

## Lateral Spread (Final)
- **New field:** `BoneOffset.abductionDeg` on revolute joints, −180 to 180, positive towards the asset's left. It accumulates down the chain. `solveSpread` lives in the rig module, and `spreadShape`/`placeShape` live in evaluation. Animation interpolates it, and the `joint-limit` validator covers it.
- **Tests:** fixture `🧫️fixtures/🧊️lateral-spread` with hand-computed vectors, plus `🧪️tests/🧊️lateral-spread` (Gherkin, vitest, numpy+shapely oracle). Running `T/🧰️TS/vitest.lateral.config.ts`, 101 of 103 pass.
- **Red tests (not caused by TS):** plan-projection for standing-adult arm span (0.406 vs 0.45) and city-car `body-shell-plan`. These are old tests against content that was handcrafted since; FW is updating them.
- **Demonstration:** herring-gull glide, approach, landing and touchdown poses spread their wings (near wing negative).
- **Open, then decided:** at first the elevation was not foreshortened by cos φ. The coordinator then decided the elevation must foreshorten too, so that the views agree in every pose. TS is reworking this.

## Elevation Foreshortening (Final)
- **`solveRig`:** uses a 3D `solveFrames` chain whenever any bone is spread, and the elevation is its orthographic projection. Mass, thermal, kinematics and grounding read the same chain. The structure harness builds its own chain and is not affected.
- **`spreadShape`:** adds only the lateral offset.
- **Fixture:** re-derived by hand; a new test pins elevation-to-front agreement.
- **Guard:** new spread checks `spread-elevation-x-plan-x`, `spread-elevation-y-front-y` and `spread-front-x-plan-y` run at every pose and clip instant with a non-zero spread.
- **Results:** the lateral config passes 104 of 104. tsc is clean except for `d3-format` (FW). In the three-views run, 12 of 322 fail: assets still being authored, plus herring-gull spread poses at 1:50 that are 0.03–0.08 m off.
- **Gull:** the remaining mismatch comes from the authored silhouettes (wing span, low point, forward reach). It is handed to QA1.

## Axial Twist (Round 1)
- **New field:** `twistDeg` rotates a bone (on a revolute joint) about its own axis, after its sagittal rotation and spread. It accumulates down the chain and is solved in the same 3D `solveFrames`. Schema, TS twin, animation and the `joint-limit` validator all cover it.
- **Tests:** `🧫️fixtures/🧊️axial-twist` (hand vectors) and `🧪️tests/🧊️axial-twist`, sharing the numpy+shapely oracle in `🧊️lateral-spread/🐍️.py`. The ticket-local config passes 195 of 195. tsc is clean except for `d3-format`.
- **Guard:** the spread checks also run for twisted bones.
- **Tower crane:** a revolute `turntable` bone is added at 39 m (the physical slewing ring), and `slewing` is reparented to it. `parked-weathervane` sets a 60° twist. The guard then fails: rigid plan shapes do not slew.
- **Decision:** rigid front and plan shapes must follow the full 3D transform of their bone, so a twisted subtree rotates in plan exactly. Authors must not work around this with `followTip`. TS is fixing it.

## Full 3D Placement (Final)
- **Rig:** `solveFrames` returns 3D frames plus an `affected` flag. `carry` maps `now.o + now.M·rest.M⁻¹·(p − rest.o)`.
  - **Rigid shapes:** rotate exactly, about the vertical in plan and about the forward axis in front.
  - **`followTip` shapes:** carried the same way when the bone has at least half its length in view.
  - **Extruded solids:** swept along the lateral axis, then hulled.
  - **Revolved solids:** revolve about their axis bone's carried vertical, so a tilted axis works.
  - Unaffected chains keep the old code path.
- **Schema:** new `Bone.z`, the lateral rest offset of a bone origin (default 0). It is the pivot for bones beside the symmetry plane; the gull wings use ±0.068.
- **Tests:** 194 of 194 pass; tsc is clean except for `d3-format`. The crane `parked-weathervane` passes the guard with no crane edits.
- **Open:**
  - Solid paths under spread and twist have no dedicated test yet; TS will add one.
  - Gull at 1:50: the default pose and landing@3.2 s overshoot the plan x by about 0.02 m. This goes to QA1.

## Solids, Bone.z, Consumers
- **Fixtures:** solids under spread and twist (extruded slab, 14° revolved drum, 105° lid), with oracle and Gherkin. The config passes 206 of 206.
- **Validator:** new code `bone-lateral` (within depthM/2 + tolerance).
- **Consumers:** `loop-closing` now compares spread and twist. All other consumers read `solveRig` or the evaluated shapes. Bones seen exactly end-on give degenerate statics in the structure module (no NaN).
- **Correction ordered:** a single 3D path for every bone, so revolved solids always follow their axis bone's frame.

## Single 3D Path, Elevation Solids, One Outline (Landed)
- **Carry:** every front and plan shape on every bone uses the full 3D carry; the separate old path is removed. Revolved solids always revolve about their axis bone's carried frame (plain 105° lid gives a tilted dome).
- **Elevation solids:** allowed. New validator code `elevation-lateral`.
- **Outlines:** solids project to one union outline via the new `🔨️modules/🔺️polygon` (`unionPolygons`, no external library). Vectors (L-bracket, slab) agree with shapely `unary_union`.
- **Gull:** the wings are extruded solids in all three views. touchdown, landing@1.92 s and flare-key pass. landing@2.56 s still needs a far-leg elevation shape (QA1).
- **Results:** the config passes 208 of 208; tsc is clean except for `d3-format`.
- **Whole-catalog guard:** 9 of 322 fail.
  - sleeping-person (QA5).
  - Default-pose consistency for parent-with-stroller, person-with-rollator, person-with-umbrella (H2), pregnant-person, running-child, seated-reader and skateboarder (H3). These are probably rigid shapes that now rotate with their bones; they are handed to H2 and H3.

## Height of Flat Shapes (QA11 Question, Landed)
- A flat plan shape is carried at the height of the middle of its elevation twin at rest (same id, same bone, not a solid). A flat front shape is carried at that twin's forward position. With no twin it falls back to the bone origin; such a part should be authored as a solid.
- New vector: a pedestal on a torso tilted back 14° shifts back 0.229826 m in plan (the old origin carry gave 0.048515). The numpy/shapely oracle applies the same rule and agrees. Ticket vitest: 209 of 209.
- **Whole-catalog guard:** 8 of 322 fail. sleeping-person and the gull at landing@2.56 now pass. Newly failing: the mallard-duck sweep (sent to QA8). Still failing: the herring-gull sweep (QA1), and default-pose consistency for six humans (H2, H3).

## Revolve About the Axis Bone's Rest Direction (Landed)
- A revolved solid revolves about its `axisBone`'s own `u` (rest direction), carried by the pose, with no special cases; `axisX`/`axisZ` shift it at rest. The oracle uses the axis bone's rotation-matrix column. New vector: `hub` in `🧫️fixtures/🧊️axial-twist` (horizontal rest axis: a 0.1 m disc in front, a 0.22 × 0.2 m band in plan and elevation). Ticket vitest 210 of 210.
- Authoring rule: `axisBone` must lie along the part's symmetry axis (pole, mast, trunk, drum, hub).
- Guard 8 of 322 fail. Newly failing: market-parasol (QA9), bar-stool (QA7), date-palm storm@1:50 (QA3). Now passing: herring-gull, mallard-duck, parent-with-stroller, person-with-rollator. Still open: person-with-umbrella (H2), pregnant-person, running-child, seated-reader (H3).
