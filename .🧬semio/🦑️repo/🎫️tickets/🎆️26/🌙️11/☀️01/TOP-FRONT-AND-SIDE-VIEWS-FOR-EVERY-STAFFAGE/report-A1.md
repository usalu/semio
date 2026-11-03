# Report A1: Animals (horse, dairy-cow, domestic-cat, goat, hedgehog, herring-gull, labrador-dog)

## Scope
- 24 authored representations now have front and plan views:
  - 4 scales: horse, cow, goat, dog
  - 1:50, 1:100 and 1:200: cat, gull
  - 1:50 and 1:100: hedgehog
- The unavailable scales stay unavailable.
- `presentation.viewpoints` is removed.
- The scripts are in `T/🧰️A1/`: `kit.py`, `legs.py`, one module per asset, `build.py`, `verify.ts` and `render.py`.

## Method
- **Reference frame:** the rest frame with the root at the origin.
- **Lateral sign:** far-side bones are +, near-side bones are −.
- **Bindings:**
  - Plan limbs are `capsule` shapes with `followTip`.
  - Front limbs are `poly` columns with `followTip`.
  - Cat and dog tails are bead chains.
- **Occlusion order:** hind legs < forelegs < trunk < neck < head < ears < eyes.
- **Thermal regions and decisions:** regions are copied from the elevation per bone; the decisions texts are extended in en and de.
- **Detail per scale:**
  - 1:50: full detail, including the mane, udder, horns, tongue, spine skirt and webbed feet.
  - 1:100: the same masses without detail lines.
  - 1:200: the main masses only; cat legs are drawn 1.15× wider.
  - 1:500: a composed silhouette.

## Checks
- **Extents:** ok for all 24 representations. The tightest is the gull's plan depth: 0.18 against 0.20, with a tolerance of 0.021.
- **Poses and clips:** every pose plus 8 samples per clip were evaluated at every scale, with 0 bad results. That is 252 queries in total.
- **Catalog check:** ok and suitable under `office-laser` (min feature 0.25–0.71 mm). The 1:500 violations from round 1 are fixed.
- **Previews inspected:** horse, cow, goat and dog at 1:100, cat and hedgehog at 1:50, and the gull. Each covered the main poses and the clip instants.

## Doubts
1. **Gull wing spread:** front and plan lateral coordinates are static. Spread wings need a pose-dependent lateral channel, which is a framework follow-up assigned to TS (lateral spread).
2. **Hedgehog roll-up:** it reads as a sagittal contraction.
3. **Merged leg groups:** at 1:200 and 1:500 they are bound to the middle bone.
4. **Late changes:** the tail beads, the narrower legs and the 1:500 fixes were verified numerically but not re-inspected by eye.
5. **Direct runs:** after the capacity queue stalled, `bun verify.ts` was run directly instead of through `repo`.
