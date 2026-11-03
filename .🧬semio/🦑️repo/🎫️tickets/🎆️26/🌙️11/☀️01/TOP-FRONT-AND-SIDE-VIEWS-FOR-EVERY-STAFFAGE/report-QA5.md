# Report QA5: Visual QA of H4 Humans

## Round 1
- **Fixed:**
  - Commuter bag is now an `extruded` solid (the strap still uses `followTip`).
  - Phone-user 1:500 plan length.
  - Sleeper plan length.
  - Wheelchair 1:500 footplate thickness.
  - Plan hand extension.
- **Spread added:** tour-guide wave and gesture (`upper-arm-r` −15°, too small to read as waving) and sleeper limbs.
- **Removed after guard failures:** sleeper torso roll, tour-guide turning-to-group twist, and standing-adult pointing spread (plan 0.911 vs front 0.751).
- **Tests:** the three-views vitest passes 13 of 14. Sleeper turn-over still fails: 1:100 at 4.8 s by 0.001 m over tolerance, and 1:200 at 3.2 s, where front y is 0.843 vs 0.781 (front shapes).
- **Inspected:** about 10 of about 340 sheets.

## Decision
A guard failure means the front and plan shapes are not bound to the 3D chain. It is not a reason to drop the motion. Re-author on full 3D placement with shapes centred on their bones, using `Bone.z` for the shoulder and hip offsets. Then restore physically right values: pointing sideways about −85°, a waving arm raised sideways, the torso roll, and the turn.
