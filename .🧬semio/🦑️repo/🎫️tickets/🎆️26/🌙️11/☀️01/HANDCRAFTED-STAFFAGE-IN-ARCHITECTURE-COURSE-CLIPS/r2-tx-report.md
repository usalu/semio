# R2 TX: D4 case-data move and visual check of na's changes

## Task A (D4)
- Sibling test folders keep case JSON beside the test as `🔣️.cases.json` (`🧪️tests/🎭️presentation`, `🧪️tests/📇️registry`).
- Moved `📊️excel/🎬️clip/🧪️cases/🧍️occupants/🔣️.json` to `📊️excel/🎬️clip/🧪️tests/🧍️occupants/🔣️.cases.json`; the test path on line 14 of `🧪️tests/🧍️occupants/🐍️.py` now points there. The empty `🧪️cases/🧍️occupants` folder is removed.
- `🧪️cases/⚡️anchors` and `🧪️cases/🔣️.json` are tracked files of someone else and stay.
- `T/m9_tests.sh` on the occupants test: 4 tests OK.

## Task B (render checks, ql 15 fps, one scene at a time)
The clips render German on screen for every audience (`set_vo_language("de")`, English is voice only), so the de variant is what each frame shows. The `SEMIO_TEACHING_LANGUAGE=en` run of the cooling scene produced the same German text. The English narration strings are covered by na's parity audit, not by these renders.

| Scene | Result |
|---|---|
| cooling internal-gains Beat2_HumanFactor | Pass. Scale axis 0, 300, 600, 900 W. Readouts 70, 120, 300, 900 W alternate above and below the axis, no overlap. 70 W and 120 W dots sit close but their labels do not touch. Activity row (70/120/300/900 W) fits. The 120 W anchor sits under the three people next to "n = 3". |
| physics Beat1_Leistung (vacuum) | Pass. The P = 0.7 kW, t = 1.43 h = 86 min, E = 1.00 kWh lines fit the box; the area rectangle runs past the 1 h tick to 1.4 h. The power ladder shows Staubsauger 700 W, Mensch 100 W, Teelicht 40 W; the last two are close but legible. The shower rectangle is clipped at the 2 kW axis (18.6 kW, pre-existing, not touched). |
| physics Beat3_Energieerhaltung (lamp) | Pass. P_el = 60 W, Licht 3 W, Wärme 57 W, "3 W + 57 W", "60 W Wärme", all fit. The mid-animation "Licht 2 W / Wärme 45 W" is the count-up. |
| ventilation natural Beat2_StossOderKipp (radiator on, `night`, 600 W) | Pass. The label reads 600 W, 50 %, "Heizkörper 300", total 827 Wh then 830 Wh with 2.4 x, and the bars fit the row. The radiator pose and its glow are small and inside the radiator, with no spill. No overlap with the tilted window. |
| heating conduction Beat4_WattVergleich (toaster 900 W, tealight 40 W) | Pass. "Toaster 900 W", "Teelicht 40 W", the compare labels (0.8 Toaster, 1.2 Mensch in Ruhe, 1.4 Teelicht) and the 715, 120, 55 W lines all fit. |

## Fixes
None needed. No source changed besides the D4 move.

## Not checked
- Cooling Beat1 and Beat3, the heating-comparison scene and the storage-standby scene were not rendered. A single beat costs 5 to 35 minutes at this memory budget.
- A real English on-screen render does not exist, since the clip text is German only.

## Tests
- `staffage_only` guard run after the move: 22 tests OK.

Scripts: `T/🧰️tx/render.sh`, `montage.py`, `crop.py`. Generated output was removed.
