# Report O1: Objects (bicycle-stand, chalkboard, charcoal-grill, cooking-pot, drinking-fountain, market-parasol, patio-heater, playground-swing)

## State
- Front and plan exist at every authored scale. chalkboard and cooking-pot 1:500 stay unavailable. `presentation.viewpoints` is removed.
- **Final check:** validation, `office-laser` suitability, default-pose consistency, and all pose and clip samples are non-empty in all three views. It passes for all eight.
- **Not run:** the TS vitest and `consistency.ts`. Their rule ran in `T/🧰️O1/batch.ts`.
- Builders: `T/🧰️O1/` (`lib.py` plus one builder per asset). Every scale is an explicit block, with no scale comparisons.

## Changes
- **bicycle-stand:** front offset (−0.024 m) and the 1:500 height fixed; the front now spans 0 to 0.8 at all scales.
- **Solids:** `revolved` for the parasol canopy, ribs and stretcher and the grill lid; `extruded` for the swing seat, chains and shackles.
- **Rig:** helper bones `lid-axis` and `lid-handle` added under the grill `lid`. **This violates the no-helper-bones decision**; QA9 must remove them.

## Doubts (handed to QA9)
- **Framework (FW):** `representation-copied` scales SVG arc flags (`A`), so O1 avoided arcs.
- **grill:** the open lid in plan is a widest-reach disc; the damper tilt and the knob pointer are not shown; a faint seam remains on the mirrored lid.
- **parasol:** the plan hem is a circle instead of an octagon, and only the two in-plane ribs are drawn in plan.
- **fountain:** jet and button are absent in plan (deliberate).
- **swing:** the elevation of `pendulum-forward` shows no chain or seat. This is existing elevation data and must be fixed.

## QA9 Final Round
- **Grill lid:** `axisBone: "lid"`, `axisX: 0` in `T/🧰️O1/charcoal_grill.py`, no helper bones. Open lid at 105° is a tilted dome: plan lens 0.29 m deep (x −0.58 to −0.29) and 0.585 m wide, hinged at the bowl rim; front a face-on disc above the bowl with the handle on top. Closed-lid views unchanged.
- **Re-check after the single 3D path:** check and sweep ok at every scale of all eight; `coverage.py --consistency` 30 of 30; three-views vitest 16 passed.
- **Parasol:** 8 spokes in plan, furled as a 0.30 m disc; the hem stays a revolved circle.
- **Swing:** the chain overlaps the right A-frame leg in the side view at ±30°. This is the true side projection (chain and leg lie in the same line of sight); accepted.
- Heater, fountain, bicycle stand, chalkboard and pot sheets not re-read after the change (Haiku reader dispatched).

## QA5 follow-up (audit-sheets-O12)
- **playground-swing `pendulum-forward`:** not missing. Chains (`chain-left-link`: x 1.33..2.26, y 2.245..0.649) and seat (`seat-plank` x 2.04..2.46, y 0.557..0.615) are in the side view at every scale. The chain line runs exactly along the near A-frame leg because the pendulum angle (29 degrees) equals the leg angle; the seat sits on that line. No data change, motion kept.
- **Plan/front depth drift fixed:** the extent is now 1.24 m (plates at +-0.62) at every scale: 1:50 beam caps shortened from +-0.635 to +-0.62, 1:500 beam widened from +-0.60 to +-0.62 (front and plan). Builder `T/🧰️O1/playground_swing.py` updated the same way.
- **cooking-pot:** handle extent +-0.125 at 1:50, 1:100 and 1:200 (elevation and plan), pot body +-0.086 at every scale (1:100 front/plan/elevation, 1:200 elevation was +-0.1). Plan top outline at 1:100 and 1:200 is now a 48-point polygon (was an ellipse). At 1:200 the 1.7 mm ring plus the 0.045 m lugs still reads as a small lens; that is the scale, not a coarse polygon.
- Check, consistency (extents probe) and the three-views vitest pass for these assets (swing, pot).
