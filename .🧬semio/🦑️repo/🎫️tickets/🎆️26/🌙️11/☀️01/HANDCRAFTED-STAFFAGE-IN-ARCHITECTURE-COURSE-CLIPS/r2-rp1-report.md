# r2-rp1: physics fundamentals, first half

Beats covered: Beat1_Leistung, Beat2_EnergieImAlltag, Beat3_Energieerhaltung, Beat4_Waermepumpe (rp2: Beat5-8).
File: `E/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py`. Each scene was rendered at ql, 15 fps, de, one at a time, and frame sheets plus zoomed crops were inspected. Beats 1, 2 and 4 were rendered twice (before and after the fixes).

## Findings and fixes
1. Beat1, device card (`device_frame`): the shower (Durchlauferhitzer) card had no icon while vacuum and kettle had one. Added `shower_icon` (`physics-fundamentals-shower`, fitted 0.8 x 0.65, animated), faded in with the shower and out with the formula. Kettle and vacuum icons read correctly (true proportions, kettle warm).
2. Beat2, clerk (`_person("physics-fundamentals-clerk")`): the figure floated about half a unit above the house ground and was 1.0 units tall regardless of the house. It now stands on `hg.get_bottom()` and has the true height `1.75 * 0.72 * house["w_height"] / 5` (5 m two-storey house section). Grounded and at house scale in the re-render.
3. Beat2, shower (`_shower_head`, drops, cable): the hose loop ended below the floor line, so the head looked low and the hose crossed the floor. The nozzle is now at y 1.7 (was 1.3), drop paths and feed cable follow; the hose bottom clears the floor line. Scale unchanged (`SHOWER_UNIT` 2.2).
4. Beat4, heat-pump radiator: the heating-water radiator was the side section with a wall stub, standing free, and the water pipes ended in the void. New cast `physics-fundamentals-radiator-front` (front viewpoint, manifest `V/🧍️staffage/🔣️.json`) at unit 1.2 on the floor-level foot; `feed` and `back` pipes now end at its two stubs, the water loop runs through the radiator, label moved below the return pipe. The old cast `physics-fundamentals-radiator` stays because Beat5 (rp2) still uses it with a window crop.
5. Beat4, final energy house (`fe_heater`): was a thin side-section plate with a gap to the wall. Now the front cast at true metre scale standing on the floor (ripples above it). Removed the unused `_radiator` helper.
6. Guard `🔣️staffage_only.json`: `_thermometer` allowlisted (chart gauge, same class as `_thermo`).

## Checked, no change needed
- Beat3: lamp (thermal pendant, warms with glow rings), standing occupant (grounded, 1.75 m of the 2.5 m room, breathing), labels clear of the figure, sun rays and bars.
- Beat4: evaporator cold blue and condenser hot red, compressor spins when powered, expansion valve sits on the loop, labels (Verdampfer, Verdichter, Verflüssiger, Expansionsventil, Strom, Außenluft) do not overlap.
- Kettle and pot/hob appear in Beat1 (kettle) and Beat7 (rp2); handcrafted look (thermal view, same palette) consistent across Beats 1 to 4. Blind is Beat5 (rp2), boiling/steam hob and pot are Beat7 (rp2).

## Tests
- `staffage_only` (guard): 18 tests, 4 failures remain, none in this file: `cooling/system-sizing::_vent_unit`, `demand/system-losses::_meter`, `demand/primary-energy::_meter_glyph`, `demand/practice::_gas_meter` (other agents' areas; before my allowlist entry there were 5, `_thermometer` was the fifth).
- `scenes`: 11 OK. `integration`: ran without failure output (tail showed only the pydub warning, exit 0).
- Catalog check not needed: no catalog asset was changed, only a new manifest cast row.

## Open
- The shower icon in the Beat1 card is narrow (the asset is tall); it reads, but a wider fit would need a closeup pose of the head.
- Staffage-only guard failures listed above belong to other series.
