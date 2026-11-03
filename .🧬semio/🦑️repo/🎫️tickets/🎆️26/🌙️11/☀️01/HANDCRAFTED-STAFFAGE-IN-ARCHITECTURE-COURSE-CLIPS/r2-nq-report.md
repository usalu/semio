# R2 nq: en/de number mismatches in architecture clip narration

## Scope and method

- Scanner: `T/🧰️nq/nq_narration_numbers.py` (ast over every `.py` under a `clip` folder below `🎓️teaching/🏛️architecture`, plus json walks over `.json` files with string `en` and `de` values).
- Pairs collected: `tr("en", "de")` calls, `{"en": ..., "de": ...}` dicts, and `(key, en, de)` tuples that sit inside a `NARRATION = [...]` assignment. Other tuples are not narration (cell refs, hex colours, identifiers) and are skipped.
- Numbers come from digits (German `,` decimal, `.` or space thousands, LaTeX `{,}` and `\,`) and from number words (English and German, including compounds such as `fünfundzwanzig`, `dreimal`, `Komma` and `point`). A bare English `one`/`a` counts only before a scale word, a tens word, or `point`, and a bare `1` is dropped from both sides, because `one` is mostly an article.
- Final scan: `pairs=3798 mismatches=167`. Of these, 149 are json (142 in `🏗️bim/.../🧱️model/🔣️.json` model data, 7 in excel `localize`/`errors` cases) and 18 are narration tuples. The `tr` and dict pairs now show 0 mismatches.
- Raw scan output went to `T/🗑️generated/nq/` (deleted at the end of the ticket work).

## Fixed (true en/de gaps)

File: `⚡️energy/🔥️heating/🎬️clip/🧮️calculation/🐍️.py`, `ReviewingHeatLosses` and the annual and remaining-heat beats.

The de narration quoted a figure that the en narration omitted, so the en text had no number to match. Each de figure matches a value the scene computes (verified by importing the module with `uv run`):

| Line | Beat | en figure added | de figure | Computed value |
|---|---|---|---|---|
| 335 | trans | about 2,300 W | rund 2 300 W | `_PHI_T` 2300.0 |
| 338 | vent | about 1,500 W | rund 1 500 W | `_PHI_V` 1500.0 |
| 341 | total | about 3,800 W | rund 3 800 W | `_PHI_LOSS` 3800.0 |
| 433 | gradtag | about 3,500 kelvin-days per year | rund 3 500 Kelvintage | `_GT` 3500.0 |
| 436 | annual | about 16,000 kilowatt-hours per year | rund 16 000 kWh | `_Q_LOSS_SHOWN` 16000.0 |
| 648 | solar | about 3,000 kWh per year | rund 3 000 kWh | `_Q_SOL` 3000.0 |
| 651 | internal | about 2,000 kWh per year | rund 2 000 kWh | `_Q_INT` 2000.0 |
| 654 | total (gains) | about 5,000 kWh per year | 5 000 kWh | `_Q_GAIN` 5000.0 |
| 814 | formula | about 4,500 kWh per year | also 4 500 kWh | `_Q_USE` 4500.0 |
| 808 (mass) | mass | eta h rises to about 0.97 | rund 0,97 | `_ETA_HEAVY` 0.9699928 |
| 1289 | ce | roughly 600 kWh | rund 5 Prozent, etwa 600 kWh | `_Q_CE` 600.0 |
| 1292 | d | 900 kWh | 900 kWh | `_Q_D` 900.0 |
| 1298 | g | about 700 kWh | rund 700 kWh | `_Q_G` 703.2 (rounded in text) |

Edits are English-only, one clause each, in `NARRATION` strings. The German text was not changed.

## Checked, not a mismatch (false positives of the scanner)

- Number words the parser does not model: `thirtyfold` (physics fundamentals:1163, de `dreißigmal` = 30), `a tenth` (physics:2301, de `0,1`), `nineteen-sixty` (heating fundamentals:1384, de `1960`), `eight and a half` (ventilation volume:452, de `8,5`; `a half` is not parsed), `two thin air films` (heating fundamentals:1378, de `beiden`), `almost three times` (ventilation natural:966, de `fast dreifacher`), `more than a factor of two` (primary-energy:1132, de `Doppelte`), `count the zeros` (physics:590, 918: 3, 6, 9, 12 on both sides).
- Unit style: physics:933 `2.4 trillion kWh` and `170 trillion` vs de `2 400 Mrd. kWh` and `170 000 Mrd. kWh`. Both are equal values.
- Pronoun or article `two`/`three`/`null` that one side drops and the other does not: solar-radiation:398, ventilation mechanical:248 and 907, volume:464, natural:310, heating conduction:363 (de "Nettostrom ist null" = stopped), recap:551 and 1180.
- Eta heavy mass: de `0,97` matches the scene (`_ETA_HEAVY` 0.9699928); en `0.90` is the light case `_ETA_LIGHT` 0.9, which is correct.

## Not narration, not changed

- `🏗️bim/🎬️clip/🧱️model/🔣️.json` (142 hits): element and material text in the BIM model data. Every hit is a parser artefact (`two-leaf` vs `zweischalig`, `6.081..15.5`, `±0.15m` joins). Not hand-reviewed item by item.
- `📊️excel/🎬️clip/🧪️cases/🔣️.json` and `🎨️visuals/📗️workbook/🔣️.json`: formula localisations (`SUM(...,1.5)` vs `SUMME(...;1,5)`) and `#NULL!` tips. Equal values, no change.
- `❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py`: not edited (another agent is repairing it). The earlier hits there were `one` pronouns, which the scanner now ignores. No number mismatch.

## Tests

- `T/m9_tests.sh` over all `energy` lines of `T/🧰️ht/modules.txt` ran for 1500 s and was killed by the timeout (`EXIT=124`) before the unittest summary. The log had no `Error` or `FAIL` lines up to that point. The modules list has no test that covers the heating calculation clip, so the heating edits are not exercised by it.
- Load check: `uv run ... python T/🧰️nq/nq_check_heating.py` imports the heating calculation module after the edits and prints the values listed above. The import succeeds.

## Open

- A complete unittest run of the energy modules needs a longer timeout (or a split run). I did not re-run it.
- The scanner does not model number words such as `thirtyfold`, `a half`, `nineteen-sixty`, or `a tenth`. The rule against dropping `1` hides a `one` vs `1` mismatch.
