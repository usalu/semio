# Content report NW-O (new objects, v2)

Assets (new): `S/🗂️catalog/🪑️objects/{patio-heater,charcoal-grill,stepladder,shopping-trolley,drinking-fountain,retractable-bollard,bicycle-stand}/🔣️.json`.
Scratch: `T/🧰️scripts/NW-O/` (`lib.py` helpers with a bounds printer, one builder per asset, `pv.sh` preview, `runcheck.sh`). Generated output: `T/🗑️generated/NW-O/`.

## Status (updated while working)

| asset | check |
|---|---|
| bicycle-stand | pass (checked 2026-10-29) |
| stepladder | iterating (gap fixes) |
| patio-heater, charcoal-grill, shopping-trolley, drinking-fountain, retractable-bollard | first check run queued |

Lessons that shaped all drawings: the filled-feature measure reads about 25 % lower than the nominal width (0.28 mm bands failed at 0.2 mm required), ink gaps count pens, parts must overlap or sit 0.15 mm apart in every named pose (also folded or lowered poses), and a 1:200 drawing may not be a subset of a finer one (copy detection).
