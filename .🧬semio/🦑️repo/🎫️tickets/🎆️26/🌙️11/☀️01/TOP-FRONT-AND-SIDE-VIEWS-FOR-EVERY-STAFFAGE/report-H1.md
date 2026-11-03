# Report H1: Humans (child-on-kick-scooter, courier-with-hand-truck, cyclist, elderly-with-cane, jogger, kneeling-gardener, leaning-person)

## State
- 26 authored representations now have front and plan views. The kneeling-gardener exists at 1:50 and 1:100 only; 1:200 and 1:500 stay unavailable.
- `presentation.viewpoints` is removed.
- **Check CLI** (run with bun directly): ✓ for all seven, and all scales are suitable under `office-laser`. The minimum feature is 0.200 mm and the minimum gap 0.19 mm.
- **Probe:** all ranges match at the default pose. Every pose plus 9 instants of every clip is non-empty and finite in all three views.
- **Not run:** the three-views vitest and `consistency.ts`.
- **Previews inspected:**
  - **1:50 and 1:100:** the main poses and one clip instant per asset.
  - **1:200 and 1:500:** mostly checked numerically. The courier sheets predate the last edits.

## Method
Everything is drawn in the rest frame with the root at the origin. The scripts in `T/🧰️H1/` compute coordinates only.

- **Limbs:** in plan, `followTip` capsules at their own lateral positions; in front, `followTip` polys with round caps.
- **Torso:** in front, a `followTip` poly; the hips are rigid.
- **Feet:** an outline with a sole drop where the foot bone is horizontal at rest.
- **Scales:**
  - 1:500: no part narrower than 0.106 m.
  - 1:200: masses.
  - 1:100: articulated.
  - 1:50: full detail.
- **No inverted scale gates.** The thickening of tyres, tubes and cranks at coarse scales is deliberate.
- **Decisions** are extended in en and de.

## Elevation fixes
- **jogger 1:500:** head too low; the limbs were in the wrong frames. Redrawn.
- **cyclist 1:200:** the crank pen was fixed.

## Doubts (handed to QA4)
- **Courier parcels in plan:** rigid; during tilt-and-settle they drift by up to 0.1 m. Fix with `extruded` solids.
- **Cane ferrule in plan:** cannot follow the bone tip. Fix with an `extruded` solid.
- **Gardener hat brim in front:** a flat ellipse while the head pitches.
- **Not re-rendered:** the 1:500 and 1:200 sheets after the last edits.
