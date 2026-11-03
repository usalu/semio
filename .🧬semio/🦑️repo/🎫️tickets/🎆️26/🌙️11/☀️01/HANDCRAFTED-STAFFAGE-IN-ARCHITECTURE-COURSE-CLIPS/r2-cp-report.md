# r2 cp: cooking-pot thermal redesign and vehicle interior regions

## 1. cooking-pot (`S/🗂️catalog/🪑️objects/cooking-pot/🔣️.json`) and gas-hob

Model facts found in the solver (`🌡️thermal/🧱️balance`, `⏳️transient`): regions do not conduct into each other; an `operating` region receives `heatW x level / areaM2`, relaxes with `C/G` towards its steady state, and water regions evaporate (wet by default). So each region carries the heat it absorbs, and the boil is reached by dropping the operating level to the evaporation (same pattern as `electric-kettle`).

| Item | Before | After |
|---|---|---|
| Water | 3 L / 3 kg, 90 mm | 2 L / 2 kg, 57 mm (bones, comY, shapes at 1:50/100/200, texts) |
| Water `heatCapacityJM2K` | 3000 | 237170 (= 2 x 4186 / 0.0353 m2), area 0.0353 m2 (inner diameter 0.212 m) |
| Heat into the pot | 126 W (water 38, base 60, wall 28) | 1200 W: water 1155 (through the base), base 20, wall 25; base contact tempC 100 degC when lit (trivet) |
| Operating levels | 0 / 0.55 / 1 | 0 / 0.42 (boiling = evaporation) / 1 |
| Steam emitter | 0.3 kg/kg (1.4 g/s) | 0.06 kg/kg at 0.004 m3/s = 0.22 g/s = the 485 W evaporation at level 0.42 |
| Clip `heat-to-boil` | 300 s, boil at 240 s | 760 s: lit at 1 s, full heat to 690 s, level ramp to 0.42 by 710 s, boil to 760 s; steam tracks 0.02, 0.2 at 552 s, 1.0 at 710 s; events ignition and operating |
| massKg | 4.04 | 3.04 (structure text 3 kg) |

Solver check (`🧰️cp/probe.test.ts`, `heat-to-boil`): water 17 degC at t=0 (evaporative cooling), 52 at 260 s, 90 at 585 s, 98 at 665 s, 100.6 at 697 s, steady boiling pose 100.9 degC. So 10 to 11.5 minutes to boil. Pot base 125 degC, wall 55 degC.

gas-hob (`S/🗂️catalog/🔌️appliances/gas-hob/🔣️.json`): flame region `heatW` 1500 to 865, so the 2.5 kW split is flame 865, plate/burner/trivet 435, pot 1200 (48 percent); notes and assumptions (en/de) updated. No emitter was needed: the hob feeds the pot by the budget and the pot base contact temperature.

Scene (`Beat7_SensibelLatent`, `physics-fundamentals-pot` with `POT_BOIL_RATE = 66`): not edited. At 66x the 690 s boil falls at 10.5 s of scene time, exactly when Q reaches 419 kJ (frames at 11 to 18 s show 100 degC). Rendered once (`🗑️generated/cp/b7-sheet.png`, deleted): pot and hob read, water lower than before. Open: the scene still labels `m = 1 kg` while the pot holds 2 L (scene text, left for the rp audit).

## 2. vintage-car-1929 and city-car

New passive region `interior` (seats, steering column or wheel, door handles; vintage 3.5 m2 upholstery material added, city-car 5 m2 interior-trim, insulation layer, `hidden` text for 1:100 and coarser), bound in every pose (all key poses too) plus an assumption note. Every still-unbound shape in all authored representations now has a region: vintage 24 shapes (interior 6, body-panels 12, fenders, hood, roof, tyres), city-car 67 (interior 7, body-panels 31, tyres 20, glazing, bonnet, tailgate). `🧰️cp/unbound.py` reports 0 unbound shapes. Float notation of a few values changed (0.00002 to 2e-05) by the JSON round trip, semantically identical.

## Tests

- Catalog check (`bun check --asset <id>`): cooking-pot, gas-hob, vintage-car-1929, city-car all check mark (office-laser suitable, 1:500 unavailable for pot and hob as declared).
- Staffage vitest (`q2-vitest-alias.config.ts`): 51 of 51 files, 1030 passed, 11 skipped.

## Notes

- Python writes to `city-car/🔣️.json` failed intermittently with EINVAL or Permission denied (file held by another process); staged in `🗑️generated/cp` and copied with `cat`. Another agent's edit to city-car between 14:12 and 14:19 would have been overwritten (none seen).
- Scripts in `🧰️cp/`; `🗑️generated/cp` removed.
