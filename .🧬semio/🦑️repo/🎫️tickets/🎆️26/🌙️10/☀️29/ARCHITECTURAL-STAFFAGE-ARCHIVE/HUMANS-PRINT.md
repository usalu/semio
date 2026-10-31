# Humans print and validation status

Office-laser limits used by `measureSuitability`: stroke ≥ 0.10 mm, gap ≥ 0.15 mm, feature ≥ 0.20 mm, hatch spacing ≥ 0.40 mm.

`validateAsset` is clean for all 24 human assets. Every asset has `presentation.defaultScale`. Role pens, shape pens, and hatch pens that were under 0.10 mm were raised (cyclist and elderly-with-cane hatch pens in this pass). `running-child` extents match `dimensions` within tolerance.

`parent-with-stroller` passes `validateAsset` (child operating / passenger / breath).

Force residuals sampled the same way as `checkAssetDetailed` (named poses plus 9 clip instants) are clear for `person-with-rollator`, `pregnant-person`, and `runner-with-blade-prosthesis` with `structure.dynamics` of `quasi-static`.

## Fully suitable authored scales

- walking-commuter: 1:500, 1:200, 1:100, 1:50
- kneeling-gardener: 1:100, 1:50 (1:500 and 1:200 are unavailable)
- parent-carrying-toddler: 1:100, 1:50 (1:500 and 1:200 are unavailable)

## Still unsuitable

Ink gaps below 0.15 mm (and a few features below 0.20 mm) remain on the other authored scales. Joins (ink gap ≤ 0) are not reported. Raising pens slides the measured band along a continuous range of outline separations, so it does not clear every gap. Pushing edges apart or across each other reduced one pair and opened others, so those edits were reverted. `leaning-person` 1:50 is suitable; 1:500 still has `upper-arm-l+figure` 0.091 mm and `toe-r` feature 0.150 mm.

Scratch fixer: `🧰️scripts/HUM-FIX/fix.ts`.
