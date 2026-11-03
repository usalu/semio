# Report O2: Objects (retractable-bollard, shopping-trolley, stepladder, storage-crate-stack, street-lamp, tealight, waste-bin)

## State
- Front and plan exist at every authored scale. storage-crate-stack and tealight 1:500 stay unavailable. `presentation.viewpoints` is removed, and the en/de decisions are extended.
- Builders: `T/🧰️O2/build_*.py`. The driver `T/🧰️O2/all.ts` runs check, consistency and previews in one process.
- **Consistency sweep:** OK for all 27 asset/scale combinations, covering every pose and clip instant.
- **Check CLI:** OK under `office-laser`. The stepladder print gaps at 1:500 and 1:50 were fixed.
- **Solids:** the bin lid and trolley gate are `extruded`. The temporary helper bones were removed, and no rig is changed.
- **Not run:** the vitest suite and `coverage.py`.

## Doubts (handed to QA10)
- **bollard:** the plan cannot show the rise (physically correct).
- **trolley:** the handle U shape is invented, because the elevation does not determine it.
- **stepladder:** the rig is rigid, so every pose draws the same.
- **crate stack:** only the top crate shows in plan (physically correct).
- **street-lamp:** the same-region merge removes the pole capsule's interior outline.
- **tealight:** the unlit flame disc stays in plan.
- **waste-bin:** the lid corners are square in plan.

## QA5 follow-up (audit-sheets-O12)
- **stepladder:** height is 1.500 at every scale in elevation and front (front-rail top vertices clamped to 1.5, top-cap bezier peak 1.5033 control, 1:200 rail foot to 0, 1:500 foot line to y 0 and rail corner to x -0.41); plan depth 0.47 at 1:500 (rail y0 0.19 to 0.185). 1:200 front foot shapes enlarged (0.075 m) for the new feature-width measure (`foot-front` was 0.184 mm). Builders `build_ladder.py` updated for the plan rail and top cap.
- **shopping-trolley:** the plan grip no longer overshoots the elevation (-0.57 to -0.55 at 1:50 to 1:200, capsules shifted by 0.02, arm starts moved); 1:500 elevation handle raised to 1.00. `build_trolley.py` updated.
- Check, extents probe and three-views vitest pass (swing, ladder, trolley, pot: 8 of 8 tests).
