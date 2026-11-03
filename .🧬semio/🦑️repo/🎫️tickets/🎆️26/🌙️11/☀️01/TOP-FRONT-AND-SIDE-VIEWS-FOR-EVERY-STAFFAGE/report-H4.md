# Report H4: Humans (sleeping-person, standing-adult, standing-phone-user, tour-guide, walking-commuter, wheelchair-user, worker-carrying-box)

## State
All seven assets have front and plan views at all four scales, and `presentation.viewpoints` is removed.

**Last full check:** it ran on the final geometry, every pose and clip sample non-empty and finite.
- Six assets pass at every scale: standing-adult, walking-commuter, tour-guide, worker-carrying-box, wheelchair-user and sleeping-person.
- standing-phone-user passes at 1:50, 1:100 and 1:200 but failed at 1:500: its plan x was 0.07 short because the ponytail was missing at that scale. The ponytail is now drawn at 1:500 too, but the asset has not been re-checked.

**Fixed:**
- adult plan width 0.49 → 0.40
- adult plan x at 1:100, 1:200 and 1:500
- commuter plan x at 1:50
- sleeper depth 0.47 → 0.40
- tour-guide front height 2.275

**Changes from the brief:**
- Tour-guide and worker shoes are each one `extruded` solid bound to the foot bone.
- Scale gating uses a lookup table, so no conditions are inverted.

**Previews inspected:**
| Asset | Scale | Poses and clips |
|---|---|---|
| standing-adult | 1:50 | 4 poses |
| wheelchair-user | 1:100 | 4 poses |
| sleeping-person | 1:100 | 3 poses and the breathing clip |
| worker-carrying-box | 1:50 | 4 poses |
| tour-guide | 1:50 | wave and point |

## Method
- **Coordinates:** the rest frame, with the root at the origin.
- **Front:** `followTip` polys for the limbs; the head and accessories are rigid.
- **Plan:** `followTip` capsules for the limbs, a trunk capsule, and ellipses.
- **Scales:** 1:500 is one silhouette, 1:200 shows masses, 1:100 adds lines, and 1:50 has full detail.
- **Decisions:** extended in en and de.

## Doubts (handed to QA5)
- In plan, the commuter's bag capsule stretches as the bag swings.
- The sleeper's side and curled poses cannot show the body rolling. This needs TS's lateral channel.
- Print margins are designed but not yet measured.
- Previews at the other scales and the clip instants have not been inspected.
- The full test suite has not been run.
