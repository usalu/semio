# Polish F2 report

S = `🎓️teaching/🏛️architecture/🧍️staffage`, V = `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals`, E = `🎓️teaching/🏛️architecture/⚡️energy`, T = this ticket folder. Every scene was rendered one at a time at quality l, 15 fps, and frames were looked at.

## 1. Shower head (physics fundamentals, Beat2 `_shower_head`)

- Cause: unit 1.1 made the 1.12 m rail 1.2 units tall and the 0.11 m head a speck; the spray was only five thin drop lines.
- Fix in `E/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py`: `SHOWER_UNIT` 2.2 (true scale against the 1.12 m rail, rail now about 2.4 units), `SHOWER_NOZZLE` constant replaces the hand-tuned anchor offset so the water leaves exactly at the nozzle, a cone of 7 cyan drop streams starting at the nozzle, three rising white vapour streams (the asset's `steam-plume`), litre read-out moved left so it does not touch the rail.
- Decision on the asset: not changed. Cast variants with `transport` (parcels, pathlines, condensation) were exported and looked at: the plumes extend metres beyond the head, inflate the cast bounds (the fitted scale would shrink the head) and read as orange blobs. The asset keeps A10's choice (no spray shapes, warmed head, hose and valve in thermal); water and vapour are drawn by the scene's flow animation. The head, rail and hose warm in the thermal view during the clip (rate 40).
- Looked at: Beat2 frames 44 to 59 s: the shower is legible, spray and vapour visible at 53 s, thermometer and read-outs clear.

## 2. Split unit cooling (cooling heating-comparison Beat3)

- Cause (measured): the clip lasted 120 s but the front panel (2900 J/m2K, about 6 minutes time constant) only cooled 25 % in that time, and `COOLER_RATE` 15 showed 36 s of it; only the vane turned blue, late.
- Asset `S/🗂️catalog/🔌️appliances/split-ac-indoor-unit`: casing cooling -15 W to -60 W, vane -8 W to -25 W (U of about 3.7 W/m2K behind a 14 degC air flow, vane washed by the jet; assumption text en/de updated), `start-cooling` lengthened 120 s to 600 s (duration, last key, contacts, phase; description en/de updated). Cold jet emitter and operating binding unchanged. No `transport` in the cast on purpose (it changes the bounds of the fitted unit).
- Scene: `COOLER_RATE` 45 (`E/❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py`).
- Looked at: Beat3 frames 10 to 30 s: the unit stays orange until the coil beat, then turns light blue within about 8 s, the vane first.

## 3. Reversing pendulum fan (ventilation mechanical Beat3)

New handcrafted asset `decentralised-hrv-fan` (appliances, manifest row after `duct-fan`, generator `T/🧰️f2/hrvfan.mjs` + `build.mjs`, scales 1:50, 1:100, 1:200 authored individually, 1:500 unavailable with reason and alternatives):
- 0.26 m square push-pull unit with the inner cover off: sleeve ring, dark opening, ceramic honeycomb disc (hexagonal cell pattern, 25 mm cells at 1:50, 44 mm at 1:100, merged at 1:200), five swept blades and hub on a revolute rotor bone; four screws.
- Poses `off`, `extract` (blades angle 0), `supply` (angle 720). Clips `pendulum-cycle` (6 phases of 70 s, 420 s: the rotor runs two revolutions one way and back with a stop at each reversal, operating keys and the two emitters `exhaust-out` -21 K and `supply-in` -5 K switch with the phase) and `reverse-spin` (8 s).
- Thermal: core region (0.3 mm half cell wall of ceramic, contact with 21 degC air in the extract pose and -5 degC air in the supply pose) stores and gives up heat; casing 1 W and hub 3 W on the `fan` key.
- Honest limit: the thermal palette separates the core only moderately: the core goes from light blue to a deeper blue during the supply phase and back (fills `#a9d8e8` to `#81b5d3`); it is never orange because the extract air is at room temperature and the heat swing is 26 K at most. I did not tune further.
- Scene (`E/🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py`): the ad-hoc orange `core` rectangle and the old fan glyph are gone; `_pendulum_unit` fits the cast at the wall, `_pendulum_half` seeks the clip window of the current 70 s phase over the duration of the stream half (`UpdateFromAlphaFunc`), so the spin reverses together with the air stream (extract is the even phase, supply the odd; from the seventh half on the last two phases repeat so the core state does not jump). Looked at 8 consecutive frames: blade phase differs between frames and alternates with the streams.
- Manifest: `ventilation-mechanical-pendulum-fan`.
- Catalog: `--asset decentralised-hrv-fan` passes, office-laser suitable at all three scales; `--category appliances` all green.

## H5 central MVHR unit (coordinator addition)

The RoundedRectangle housing and the fan glyph are replaced by the handcrafted `mvhr-unit` (`_mvhr`, cast `ventilation-mechanical-mvhr`, clip `boost-spin`, thermal, spun by `_spin`), 0.62 units tall at `(p2 + 0.15, 0.6)`; the label "WRG Zentral" now sits above it, the room ducts and the fresh/exhaust paths attach to its real left, right and bottom edges. Looked at in Beat3 (20 to 22 s): spigots, window and impeller read, streams meet the sides.

## 3.3 Laptop badge (coordinator addition)

`shared-watt-laptop` is now `clip: work-and-sleep`, thermal, `office-neutral`, dark; `_WATT_RATES["laptop"]` 6 (a pose renders no warmth; at rate 6 the badge stays in the 10 to 300 s full-load phase for its 22 s life, before the lid closes). Looked at: the laptop is a thin side-on figure, so at 0.8 units the cream outline dominates; the fills are warm (`#f08c59`, `#fcab60`) but only legible as a faint tint. Honest result: legible as before, no longer pure dark-line. It could only get warmer-looking with a larger badge glyph.

## 4. Scenes M9 had not inspected

- Mechanical pendulum scene: rendered and looked at (see 3); label "70 s Zuluft" sits slightly over the wall corner next to the unit, unchanged layout from before.
- Heating-comparison person badge (Beat1 WinterGains, 16 to 28 s): thermal standing adult, legible and warm; nothing to fix. (The laptop badge next to it was the issue above.)
- Heating fundamentals Beat9 heater badge (60 s): thermal heater, warm orange, "ungedaemmt" 2000 W badge reads well; nothing to fix.

## Tests

- Catalog check: `split-ac-indoor-unit`, `decentralised-hrv-fan`, `shower-head` pass; `--category appliances` green.
- Staffage vitest (ticket-local alias configs): library 51 of 51 files, 994 passed, 10 skipped; React 17 of 17 files, 107 passed, 1 skipped.
- V Python tests (`🐍️`, `icons`, `staffage_only`, `integration`, `scenes`, `sections`, `section_consumers`, `transforms`): 60 tests OK, including the guard `staffage_only` (no orphan casts: the two new casts are used). Staffage Python package: 24 OK. Cooling clip tests and ventilation `🧪️architecture`: 9 OK.

## Files

- New: `S/🗂️catalog/🔌️appliances/decentralised-hrv-fan/🔣️.json`, `T/🧰️f2/{hrvfan.mjs,build.mjs}`, scripts `f2_*.{sh,py,json}` in T.
- Edited: `S/🗂️catalog/🔣️.json` (manifest row), `S/🗂️catalog/🔌️appliances/split-ac-indoor-unit/🔣️.json`, `V/🧍️staffage/🔣️.json` (two casts, laptop cast), `V/🐍️.py` (laptop rate), `E/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py`, `E/❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py`, `E/🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py`.
- `T/🗑️generated/f2/` deleted.
