# r2 cp2: cooking-pot redesigned for 1 L / 1 kg

The physics fundamentals scene teaches m = 1 kg, 419 kJ, so the asset now follows it. Scripts in `🧰️cp2/` (geometry, thermal, probe, render).

## Changes
- `S/🗂️catalog/🪑️objects/cooking-pot/🔣️.json`: a 1 L saucepan, 0.172 m outer / 0.16 m inner, 0.10 m tall, handles to 0.25 m. Water 1 L / 1 kg at 50 mm (bones, comY, shapes at 1:50/100/200, texts, masses 1.64 kg total, areas, anchors, emitter diameters).
- Water region `areaM2` 0.0201, `heatCapacityJM2K` 208 150 (= 1 kg x 4186 / area, half the true capacity of the 2 L design). Heat into the pot 1200 W: water 1173, base 12, wall 15.
- Uses the solver's new phase change (water material `phaseChange` boiling 100 degC, 2.257 MJ/kg; steam vent with `region: water`); the 0.42 throttle and the vapour ratio tuning are gone, operating level stays 1. Same state ph found (r2-ph-report).
- Clip `heat-to-boil`: 379 s, boil at 319 s (about 5.3 min), events, phases and plume track retimed.
- `S/🗂️catalog/🔣️.json`: heightM 0.1, widthM 0.25 for cooking-pot.
- Scene `🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py`: `POT_INNER` 0.172, `POT_HEIGHT` 0.10, `POT_BOIL_RATE` 30.5 (319 s / 10.45 s of scene time), bubble paths fitted to the smaller pot.

## Solver probe (heat-to-boil)
Water 16.8 degC at 0 s (evaporative cooling), 44 at 100 s, 70.8 at 200 s, 96 at 300 s, 100.00 from 319 s; phase-change power 904 W (0.40 g/s of vapour), base 124 degC, wall 53 degC. The 331 kJ for 79 K plus losses land in the 5-6 min window.

## Render
`Beat7_SensibelLatent` rendered once (480p15) and looked at: the pot is smaller and the water fills about half of it; Q = 419 kJ and T = 100 degC read at 11 s, as the 419 kJ formula panel appears.

## Tests
- Catalog check: cooking-pot and gas-hob both pass (office-laser suitable at 1:50, 1:100, 1:200; 1:500 unavailable as declared). gas-hob needed no change (pot still takes 1200 W).
- Staffage vitest (`q2-vitest-alias.config.ts`): 51 of 53 files pass, 1001 tests passed, 10 skipped. The 2 failing suites (`🌡️thermal-span`, `🎭️cast-export`) fail at load on an ajv strict-mode error in the `semio.staffage.cast/v1` schema (`required property "transport"`), from another agent's concurrent cast/schema work, not from this change.

## Open
- r2-cp-report still describes the old 2 L design; this report replaces it.
- The scene's `m = 1 kg` label now matches the asset.
