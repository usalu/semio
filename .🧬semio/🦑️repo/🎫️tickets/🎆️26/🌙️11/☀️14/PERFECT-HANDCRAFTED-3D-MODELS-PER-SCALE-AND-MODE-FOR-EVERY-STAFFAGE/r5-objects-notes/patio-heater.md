
## Round 5 handcraft (2026-11-21)

Scripts: `r5-patio-heater-kit.py`, `-volumes.py`, `-draw.py`, `-roots.py`, `-texts.py`, `-areas.json`, `-build.py` (the round-3 chain carried over; `python3 r5-patio-heater-build.py --source --apply`, then `volumes-insert.py`). The r5 chain reproduced the catalog byte for byte before any change. Backup: `🗑️generated/r5-patio-heater/catalog-before-r5.json`.

### Reference (modelled to)

| quantity | reference | source |
|---|---|---|
| overall height | 2.32 m (class 2.25-2.32 m) | Amazon Basics 46,000 BTU commercial heater, 91.3 in |
| reflector diameter | 0.815 m | same, 32.1 in |
| base pan | 0.467 m | same, 18.4 in |
| housing height | 0.73 m | same, base assembly 28.76 in |
| post | 57-60 mm stainless tube, three sections | Crown Verity CV3050 manual |
| control knob | about 47 mm | product class |
| bottle | 11 kg propane, 300 x 580 mm | EN 1442 class |

### Model against reference (1:50)

| measure | model | reference | deviation |
|---|---|---|---|
| height | 2.25 m | 2.32 m | -3.0 % (class value of the asset kept; at the limit) |
| reflector | 0.80 m | 0.815 m | -1.8 % |
| pan | 0.47 m | 0.467 m | +0.6 % |
| housing height | 0.73 m | 0.73 m | 0 % |
| post | 60 mm | 57-60 mm | 0 % |
| knob | 47 mm | 47 mm | 0 % |
| 1:500 knob projection from the valve housing | 49 mm (75 mm boss, half sunk) | 47 mm knob on its boss | +4 % (75 mm size floor) |

### Grades (opened: body, thermal cut, structure at 1:50, 1:100, 1:200, 1:500; poses high, low, off-afternoon, high-in-breeze; clips ignite-and-warm-up t 360 s, turn-down-and-shut-off t 720 s; 2D 1:50 to 1:500)

| scale | body before → after | thermal | structure |
|---|---|---|---|
| 1:50 | A- → A- | A- | A- |
| 1:100 | A- → A- | A- | A- |
| 1:200 | A- → A- | A- | A- |
| 1:500 | B (knob a 75 mm cylinder sticking 69 mm out of the head; housing lid a cone rising 55 mm to the post) → A- | B+ → A- (housing envelope follows the new lid) | B+ → A- |

### Changes

- 1:500 knob: a rounded boss of 75 mm (the size floor) from -30 to +45 mm along the knob bone, half sunk into the valve housing, with a rounded face (body and the elevation 2D, which takes the same profile).
- 1:500 housing: the lid has the rounded shoulder and nearly flat top of the pressed 1:50 lid (0.695/0.225, 0.718/0.21, 0.73/0.18, 0.735/0.075 m) instead of a cone; the post socket rises from it to 0.755 m. Body, structure (the same profile) and the 2D of all three views changed together.
- Decisions and the 1:500 silhouette text rewritten in both languages.
- `r3-validate.ts --file` of the candidate with `--volumes`: VALID; catalog written and spliced.

### Remaining

- Height 2.25 m against 2.32 m of the retail reference is 3.0 %; the asset's class value is kept because no data sheet of a 2.25 m model was fetched.
- The emitter screen perforation, hose, regulator and burner internals stay unmodelled (below every feature or hidden, see round 3).
- The 1:500 dome is a four-segment cone shell; the slight kinks read at sheet size, not on paper.
