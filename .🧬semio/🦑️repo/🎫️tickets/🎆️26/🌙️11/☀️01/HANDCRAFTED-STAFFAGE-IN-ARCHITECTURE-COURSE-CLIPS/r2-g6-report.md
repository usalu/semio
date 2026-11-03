# r2-g6 report: strengthened staffage guard

Files: `V/🧪️tests/🐍️staffage_only.py` (implementation), `V/🧪️tests/🔣️staffage_only.json` (rules, allowlists, vectors). Scripts: `🧰️g6/g6_run.sh`, `g6_loop.sh`.

## Rules (in the JSON, Python implements them)
- **Rule A** (`rule_a`): AST pass over every function in a `🎬️clip` script under `🏛️architecture`. A name matching the object-noun token regex, a primitive call (`Circle`, `Rectangle`, `Line`, ...) and no staffage call (`staffage`, `fitted_staffage`, `_cast`, `pendant_lamp`, `watt_anchor`, ...) fails unless `path::function` is in `rule_a.allowed`. Every entry carries a reason; a stale entry (function no longer a hit) also fails.
- **Rule B** (`rule_b`): every cast an `⚡️energy` clip names must have manifest view `thermal` or be in `rule_b.allowed` (12 furniture, storage and landscape ids, each with a reason). Cast ids are resolved from literals, module string constants (`BLIND_CAST`), `PREFIX + x` and f-strings, with a dynamic suffix expanded to every manifest id under the prefix. Allowlist entries must exist in the manifest and be non-thermal. A separate test requires all `V._WATT_CASTS` to be thermal.
- **forbidden_casts**: added `shared-watt-bulb`, `shared-watt-monitor`, `shared-watt-vacuum`.
- Vectors: 8 Rule A sources (violations and clean: staffage-built, mixed, non-object name, no primitive), 7 Rule B resolution vectors, 5 Rule B violation or clean vectors, with an in-JSON mini manifest.

## Current violations (first run, 27 failures)
- Rule A, to be fixed by g4 (not allowlisted): `_vent_unit` (system-sizing), `_meter` (system-losses), `_meter_glyph` (primary-energy), `_thermo` and `_gas_meter` (practice), `_thermometer` (physics fundamentals). Also at first run `_valve_dial` through `_clock` duplicates and `energy_tank` store were reported by hz; the clocks and tank are already gone.
- Rule A, allowlisted as sections, structure, charts or tables: 15 entries (see JSON).
- Rule B: only furniture hit (12 ids, allowlisted). Blinds and lamps (g5) were already thermal: no genuine Rule B violation.

## Results
See the end of this file for the final run.

## Final state
- Guard run (18 tests) at last check: 13 pass, 5 fail, all genuine Rule A violations that g4 is converting: `_vent_unit` (system-sizing), `_meter` (system-losses), `_meter_glyph` (primary-energy), `_gas_meter` (practice), `_thermometer` (physics fundamentals). Rule B is green (no genuine violation; blinds and lamps are thermal).
- Per the coordinator, the allowlist entries other agents added for `_meter`, `_meter_glyph`, `_gas_meter`, `_thermo` and `_thermometer` were removed. Only sections, structure, charts, tables, the clock-glyph dial (`_year_clock`) and the vent dial mark remain. Re-run `🧰️g6/g6_run.sh` (about 4 min) after g4 lands; it must print `OK`.
- Other visuals tests (`🐍️icons`, `🐍️scenes`, `🐍️sections`, `🐍️transforms`, `🐍️`): 37 tests OK. `🐍️scenes.py` has no `_radiator` reference.
- The 90-minute watch ended before g4's conversion landed; the guard re-run is the open item.

## Correction note (OI-127, r3-reg, 2026-11-03)
- The counts above are superseded. The current guard JSON `V/🧪️tests/🔣️staffage_only.json` holds `rule_b.allowed` = 8 entries (this report says 12) and `rule_a.allowed` = 18 entries (this report says 15).
- The guard JSON is the source of truth. The counts in this report were taken from an earlier state of the file, and the allowlists have changed since. The rule definitions in the Rules section are unchanged.
