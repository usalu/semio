# Report M1: Machinery

**Assets:** concrete-mixer-truck, diesel-generator, evaporative-humidifier, excavator, expansion-valve, finned-heat-exchanger, forklift, heat-pump-outdoor-unit.

## State
- 26 representations have front and plan views. Unavailable: valve 1:200 and 1:500, humidifier 1:500, coil 1:500. `presentation.viewpoints` is removed, and the en/de decisions are extended. No rig bones were added; earlier helper bones were dropped.
- **Final round** (`T/🧰️M1/all.ts`):
  - Check ok for all eight, and every scale is suitable under `office-laser`.
  - Default-pose consistency is ok.
  - The sweep of every pose plus 13 instants per clip is non-empty and finite.
  - The three-views vitest passes 16 of 16.
- **Solids:** `extruded` is used for arm parts, cylinders, mast stages, tines, carriage, pallet, lids, flaps and the mixer chute.
- **Previews:** main poses and some clip instants were inspected at each scale for all eight assets.

## Doubts (handed to QA12)
- **Mixer drum:** M1 assumed `revolved` spins only about a vertical axis, so it drew the drum front as a tilted-circle capsule. But FW's revolved solid is bone-local and placed with the full bone transform, so the 14° axis should be expressible.
- **Mixer lateral extent:** ±1.35 m against `depthM` 2.55, because of the mirror arms. Verify this against tolerance, or make `depthM` include the mirrors.
- **Hidden in front or plan** (physically correct): valve inlet, humidifier pad and sump, coil fins, mixer chute when stowed.
- The heat-pump fan blades are not drawn. The excavator welds are omitted.
- `lib.py` normalised the formatting of the diesel, forklift and heat-pump files.
