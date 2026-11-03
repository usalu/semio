# Report M2: Machinery

**Assets:** oil-pumpjack, plate-heat-exchanger, scissor-lift, scroll-compressor, sorption-wheel, tower-crane, wheel-loader, solar-thermal-collector.

## Results
- **Consistency (`coverage.py --consistency`):** all 27 representations are ok. The crane result was computed before the depthM change.
- **Three-views vitest:** 19 tests pass for these assets.
- **Driver `all.ts`:** consistent; no empty or non-finite view at any pose or clip instant; `checkAsset` reports no problems.
- **Detail conditions:** written as `<=` or `==`, so fine detail does not leak into coarser scales.

## Per asset
- **scroll-compressor:** the rotor is edge-on and the same in every pose (the axis is lateral). Front and plan are drawn per scale.
- **plate-heat-exchanger:** the flap is an `extruded` solid. The `damper-lift` bone is deleted.
- **scissor-lift:** the arms use `followTip`. In plan they are hidden under the deck (physically correct, accepted).
- **wheel-loader:** arms, cylinders and bucket use `followTip`. In plan the bucket ribs number 5, 4, 3 and 2 by scale.
- **oil-pumpjack:** beam, horsehead, pitmans and crank are `extruded`, so the stroke reads in front and plan.
- **tower-crane:** the lattice is end-on in front and drawn as chords with diagonals in plan. depthM is corrected from 2.0 to 3.5 to match the real ballast extent.
- **sorption-wheel:** casing and rim are extruded, and the hot sector sits on the near rim. The rotation cannot show, because its axis is lateral (accepted).
- **solar-thermal-collector:** already consistent; only `presentation.viewpoints` was removed.

## Previews inspected
- full-load 1:50
- partial 1:50
- raised-working 1:50
- dump-high 1:50 and 1:500
- bottom-of-stroke 1:50
- max-reach 1:100
- rated 1:50

## Open (QA6)
- The remaining poses and clip instants have not been inspected, including the crane at 1:500 and the bypass clip.
- The full suite has not been run.

## QA6 Round 5 (Haiku Audit Findings)
- **wheel-loader:** lift arms, lift and tilt cylinders, rods and bucket are extruded solids in front and plan; no `followTip` is left. The arms connect body and bucket at 1:50, 1:200 and 1:500 in raising, start-and-lift@5, load-cycle@6 and dump-high.
- **scissor-lift:** the ten arms are extruded solids at their lateral positions. Continuous strips were verified at 1:50; 1:100 and 1:200 were not read (sent to a Haiku reader).
- Check clean, consistency 27 of 27, three-views vitest 19 of 19.

## Haiku Re-Audit (audit-sheets-M2b.md)
- 81 sheets: wheel-loader at 1:50, 1:100, 1:200, 1:500 and scissor-lift at 1:50, 1:100, 1:200 (no 1:500), every pose and three instants per clip. All three views present; arms, cylinders and bucket connect body to bucket, scissor arms connect wheels to deck in front and plan; extents match the elevation. The detached-bucket finding is resolved. M2 is closed.
