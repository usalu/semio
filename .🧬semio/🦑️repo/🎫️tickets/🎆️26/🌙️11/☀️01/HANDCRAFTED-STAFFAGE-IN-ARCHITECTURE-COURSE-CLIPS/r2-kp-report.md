# R2 kp: kettle and humidifier thermal fixes

Assets: `S/🗂️catalog/🔌️appliances/electric-kettle/🔣️.json`, `.../evaporative-humidifier/🔣️.json`. Scripts: `T/🧰️kp/` (`kettle_patch.py`, `kettle_poses.py`, `humidifier_patch.py`, `crop.py`).

## Electric kettle
- Water region now carries the element: `heatW` water 1930, plate 40, shell 15, lid 5 (sum 1990 W of 2 kW). Water `heatCapacityJM2K` 349 000 (1 kg x 4186 / 0.012 m2). Fill is 1 L: water bone 1.0 kg, `massKg` 2.22 (notes, structure assumption en/de updated).
- Poses: heating power 1 (was 0.6), boiling power 0.085 (element level equals the evaporation at 100 degC; level list renamed "Heating at 2 kW" / "Boiling (evaporation)"). Steam emitters ramp 0.1 to 0.3/0.4 from t=141 s and to 1 at t=187 s.
- Clip `boil`: 200 s (was 180): power 1 until 171 s, linear to 0.085 at 187 s, boiling to 200 s. Solver (`🧰️tp/temps.ts`, cast physics-fundamentals-kettle): water 17.2 degC at 0 s, 54 at 80 s, 90 at 160 s, 98.5 at 180 s, 99.5 degC held to 200 s. About 3 min to the boil.
- The solver does not clamp at 100 degC (psat > 1 atm), so the power track throttles before 100; a constant 2 kW overshoots to 103.6.
- Fundamentals scene (rate 60): kettle icon goes blue to red within the first seconds, reads right. Frames: `T/r2-kp-kettle-frames.png`.

## Evaporative humidifier
- Defect found: `vapourKgPerKg` is the absolute humidity of the emitted air, not an excess. 0.004 was drier than room air (0.0083) so the "humidifier" dried the air. Now 0.0106 with `temperatureRiseK` -6 (90 % efficiency toward the wet bulb of 22 degC / 50 %, outlet about 16 degC, about 95 % RH). Balance at 0.28 m3/s: -2.0 kW sensible, +2.0 kW latent (2.45 MJ/kg, 0.8 g/s).
- Pad and sump are now passive wet surfaces, no `heatW` (-90 W and -12 W removed): they settle just above the wet bulb by evaporation alone (cast: pad 17.7, sump 18.5 degC at 22 degC air; was 12.5 and 13.1, below the wet bulb of 15.3). Pad `wettedness` 0 / 0.5 / 1 in dry / wetting / running. Sump heat capacity 125 000 J/(m2 K) (6 L of water per 0.2 m2).
- Notes and assumption text (en/de) rewritten.
- DEC scene (summer-strategy Beat2) renders with blue pad, unchanged reading: `T/r2-kp-dec-scene.png`.

## Checks
- Catalog check `--asset electric-kettle` and `--asset evaporative-humidifier`: both pass (1:200, 1:100, 1:50 suitable; 1:500 unavailable).
- Staffage vitest (q2 alias config): 50 of 51 files, 1026 tests passed; the dxf-reopen ezdxf test timed out at 180 s under load, rerun alone: 3 of 3 pass. So 51/51 files.
- Not run: Python unittest for clip modules, hot-water-cylinder untouched per ruling 3.

## Open
- Kettle clip is 200 s, not 180; the fundamentals scene hold time was not changed (it rendered fine).
- Humidifier `drip` clip still non-looping (cast uses loop flag).
