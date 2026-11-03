# G2 report: physics fundamentals and DEC hardware (H6 to H9, clock, kettle)

S = `🎓️teaching/🏛️architecture/🧍️staffage`, V = `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals`, T = this ticket folder. Generators and helper scripts: `T/🧰️g2/` (inputs, kept; `g2.mjs` shared builder, one `*.mjs` per asset, `build.mjs`, `check.sh`, `cast.sh`, `sheet.sh`, `render.sh`, `rows.mjs`, `insert_rows.py`, `patch_fund.py`).

## New handcrafted assets (9, all `completion: authored`, en/de text, thermal bindings, poses and two clips each)

| Asset | Category | Size | Poses | Clips | Thermal / transport |
|---|---|---|---|---|---|
| `gas-hob` | appliances | 0.30 x 0.088 m domino hob, front view | off, simmer, high | flicker, light-and-burn | flame 1500 W, pan support, burner head, plate (`burner` key); flame plume |
| `cooking-pot` | objects | 0.30 x 0.13 m, cut open, 3 L water | cold, heating, boiling | heat-to-boil, boil | water, wall, base (`heat` key); steam vent and plume |
| `electric-kettle` | appliances | 0.245 x 0.24 m jug kettle | off, heating, boiling | boil, rattle | plate, shell, water gauge, lid (`power` key); steam at the spout |
| `scroll-compressor` | machinery | 0.30 x 0.43 m hermetic shell with sight glass on the orbiting scroll (`rotor` bone turns) | off, part-load, full-load | spin, start-up | hot dome and discharge stub, cold suction stub; dome plume |
| `expansion-valve` | machinery | 0.19 x 0.09 m brass TXV with sensing bulb | closed, half-open, open | modulate, open-up | warm liquid inlet, cold body, outlet and bulb |
| `finned-heat-exchanger` | machinery | 0.205 x 0.52 m coil section (fin hatch, 23 tube circles, headers) | cold, idle, hot | reverse, warm-up | one `duty` key from -1 (evaporator or cooling coil) to +1 (condenser or heating coil) |
| `sorption-wheel` | appliances | 0.62 m face-on honeycomb wheel (rotor bone) | off, slow, rated | spin, start-and-run | warm matrix, hot regeneration sector |
| `plate-heat-exchanger` | appliances | 0.62 x 0.63 m cross-flow pack with bypass damper | off, partial, full | bypass, warm-up | plate pack, collars |
| `evaporative-humidifier` | appliances | 0.5 x 0.6 m pad humidifier, cut open | dry, wetting, running | drip, start-wetting | pad cools below air, air vent emitter |

- `heating-coil` is not a separate asset: the H9 heater uses `finned-heat-exchanger` (clip `warm-up`, idle to hot), as allowed by the brief. The same asset serves as evaporator and condenser in the heat pump, so cold and hot read from the thermal view.
- Representations: 1:50, 1:100 and 1:200 authored individually for hob, pot, kettle, compressor, coil, wheel, plate exchanger and humidifier; 1:500 unavailable everywhere (reason, limiting feature, alternatives). The valve is authored at 1:100 and 1:50 only (1:200 and 1:500 unavailable).
- Fin packs and honeycomb use hatches or hex cells with a stated, enlarged pitch (decisions say so).
- Heat flux values (`heatW`) are authored estimates calibrated to realistic surface temperatures against 21 degC room air (fins -9 / +51 degC, dome about 80 degC, ...); the pot water and kettle window values are tagged `illustrative` (burner power is carried by the hob asset).
- Manifest: 9 rows added before `storage-crate-stack` in `S/🗂️catalog/🔣️.json` (script `insert_rows.py`, rows from the generated assets).
- Casts added to `V/🧍️staffage/🔣️.json`: `physics-fundamentals-{hob,pot,kettle,compressor,evaporator,condenser,valve,blind}` and `cooling-summer-strategy-{wheel,exchanger,humidifier,heater}`.

## Call sites

`🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py` (the shower site and the `energy_tank` calls were left to F2 and G1; the only touch next to the shower is `FadeOut(kettle_icon)` in the shower swap play):
- **H6** `_hob` and `_flame` deleted; Beat7 uses `physics-fundamentals-hob` (flicker) and `physics-fundamentals-pot` (clip `heat-to-boil`, `pot.rate = POT_BOIL_RATE` starts when Q starts to rise). The hand-drawn water rectangle and its recolouring are gone; the steam and bubble particles stay as diagram streams. Constants `HOB_UNIT`, `HOB_PAN`, `POT_INNER`, `POT_HEIGHT`, `POT_BOIL_RATE`.
- **H7** `_blind` now reads the lamellae of the `external-venetian-blind` cast `physics-fundamentals-blind` (pose `lowered-cut-off-45`, mirrored; same pattern as heating solar-gains `BLIND_CAST`): `_blind(x, y_top, y_bot)` returns the cast group, lamella faces, centres and angle; `_blind_hit` intersects rays with the real lamella faces; the outer and the inner blind beats fade the cast group.
- **H8** heat-pump schema: evaporator and condenser are `finned-heat-exchanger` casts (cold and hot), the compressor is `scroll-compressor` (turns while the compressor beat runs: `comp.rate = 1.0` replaces the old `Rotate`), the valve is `expansion-valve`. The refrigerant loop, serpentine paths, arrows and labels stay diagram; the hand-drawn exchanger blocks, fins, tubes, compressor body, rotor and funnel valve were removed.
- **Clock** `_clock` is now `_manim_module.clock_glyph` (identical API, counted as a diagram instrument).
- **Kettle** the device row shows an icon: the vacuum cleaner cast first, then the `electric-kettle` cast (clip `boil`, `KETTLE_RATE` 60), faded out when the shower label replaces it.

`❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py` Beat2 (H9): the circle with spokes is `sorption-wheel` (`wheel.rate = 1.0` replaces the spoke `Rotate`), the box with diagonals is `plate-heat-exchanger`, the two humidifier boxes with drops are `evaporative-humidifier` (dripping clip), the heater rectangle with zigzag is `finned-heat-exchanger` (idle until the regeneration step sets `heater.rate = 3.0`, then it heats up). Process arrows, lanes, labels and readouts are unchanged.

## Verification

- Catalog check (`S/🔨️modules/🗂️catalog/🩺️check`): each of the nine assets prints a check mark (office-laser suitable at every authored scale); the full catalog run printed `✓ manifest` and 49 assets with 0 failures before it was left running (it continues over the remaining assets in the background).
- Staffage vitest (`T/q2-vitest-alias.config.ts`): 51 of 51 files, 994 passed, 10 skipped.
- Visuals Python tests (`staffage_only` guard, `🐍️`, `icons`, `integration`, `scenes`, `sections`, `section_consumers`, `transforms`, `semio-style`): 69 tests OK.
- Cooling clip tests (`❄️cooling/🎬️clip/🧪️tests`, internal-gains): 7 OK. There is no physics clip test of its own; the authoring clip tests that mention physics (`🏗️authoring/🎬️clip/🧪️tests`) ran 31 tests OK (1 skipped).
- Rendered one scene at a time (quality l, 15 fps) and looked at frames: cooling summer-strategy Beat2 (wheel, plate exchanger, humidifiers, heater; frames 3 to 23 s, regeneration beat after 28 s not sampled), fundamentals Beat7 (hob and pot 2 to 18 s), Beat4 (heat pump 3 to 34 s; valve and condenser beats), Beat5 (blind, incl. a zoom of the outer blind with rays), Beat1 (vacuum, kettle, shower swap). Cast sheets of hob, coil, compressor and valve were inspected too.
- Not re-rendered: Beat2 and Beat6 clocks (the alias is the identical function), Beat5 inner-blind frames at full size (outer blind and its rays were zoomed).

## Observations

- The cast blind is real-size: in the radiation section (about 0.75 units per metre) the lamellae are small, so the blind reads as a thin zigzag column; the old drawing used 0.5 unit lamellae. The ray strikes still end on the lamella faces.
- `FadeOut` on a `staffage_motion` removes it at the end; fade-ins and fade-outs of motion groups work, but a restyled `set_stroke` on a motion is overwritten by the next frame swap (the outer blind recolour only affects the static cast).
- The condenser, evaporator and compressor sit over the old loop line (the line passes behind them); the compressor stub positions do not coincide with the loop attachment points.
- Python writes to emoji-path files fail intermittently with `Errno 22`; the scripts retry.
