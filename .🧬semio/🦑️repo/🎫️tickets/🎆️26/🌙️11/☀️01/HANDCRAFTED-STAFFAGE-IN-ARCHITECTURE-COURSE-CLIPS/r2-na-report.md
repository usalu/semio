# R2 NA: narration numbers aligned with the casts

Re-run of `T/🧰️nn/audit_narration_numbers.py` (patched by `T/🧰️na/patch_audit.py`): 27 matches, 10 justified, 0 minor deviations, 0 mismatches, en/de parity of all sentences yes (C26 fixed). The audit file is `T/audit-narration-numbers-r2.md`.

Path per row: T = text follows the cast (asset is what is depicted and cannot run at the stated value), M = manifest changed to an operating state, J = justified, no change. No asset rating was changed.

| Claim | Before | Now | Path and reason |
|---|---|---|---|
| C1, C2b, C3, C4 seated or standing person | 100 W vs 124 W | 120 W | T. Seated 1.1 MET is 124 W, no 100 W pose exists. cooling internal-gains narration, scale dots, `qp` line, single-person label, anchor; the 50-person total is now 6000 W (`watt_anchor(6000)`), de "sechstausend" |
| C2a sleeping | 80 W | 70 W | T. Sleeper 0.7 MET is 71 W |
| C2c walking | 200 W | 300 W | T. Walking commuter poses are 2.6 MET (286 W), no slower pose |
| C2d sport | up to 800 W | up to 900 W | T. Jogger 8 MET is 895 W. Scale axis 0 to 900, ticks 0, 300, 600, 900; `🔣️.cases.json` `metabolic_watts` is now [70, 120, 300, 900] |
| C5 heating persons | 80 W = 4/5 of full output | 80 W = 2/3 of full output | T. 80 W is the sensible share; 2/3 of 124 W is 83 W. The cast runs at its full 124 W. Anchor in `_WATT_DEVICES` now compares to 120 W (0.67x) |
| C6a, C6b, C7, C19b, C22b resting person | about 100 W, 70 + 30 W, 45 g/h | unchanged | J. All say "about"; 117 W (-14 %) and 48 g/h (-6 %) are textbook rounding of the resting person |
| C8, C9 lamp | 100 W | 60 W | T. The pendant lamp is rated 60 W. Sub-lines now 3 W light + 57 W heat (5 % and 95 % of 60 W), `p_el` 60, labels, sum panel, sink |
| C10, C19c vacuum | 1 kW | 700 W | T. Cast is a 700 W canister and has no boost pose. Beat1: P tracker 0.7 kW, the area step runs 1.4 h so the area stays 1 kWh (`SX` 3.4), kettle text "almost three times as high, a third as wide", ladder entry 700 W. Beat2: vacuum runs 86 min, tag 0.7 kW, narration "about an hour and a half"; kWh ladder "Staubsauger, 1,4 Stunden" |
| C20 vacuuming time | almost 3 h | almost 4 h | T. 2.7 kWh / 0.7 kW is 3.9 h; the clock runs to 3.9 h |
| C18 tealight energy | 0.1 kWh | 0.15 kWh | T. 12.8 g paraffin is 0.156 kWh; ladder entry and its dimension arrow moved to 0.15 |
| C19a, C22a tealight | 30 W | 40 W | T. Region heat of the tealight is 40 W (summary says about 35 W). Ladder, conduction refs and kinds |
| C22c toaster | 1000 W | 900 W | T. Toaster 902 W. Conduction: refs, kinds, narration; the compare sentence is now 0.8 toasters and "eineinhalb" / "about one and a half" tealights (715 W / 900 W, 55 W / 40 W). `_WATT_DEVICES` toaster 900 W, standing person 120 W, seated person 120 W; heating-comparison person anchor 120 W |
| C25 radiator | 500 W vs rated 1.2 kW | 600 W | M. Manifest cast `ventilation-natural-radiator-on` uses the existing pose `night` (51.5 % of 1.2 kW is 618 W). `P_RAD` 600 W: half is 300 Wh, tilted total about 830 Wh (was 250 and 780), scale unit 8/830. The cast regenerated on first test run |
| C26 storage standby | 1.5 kWh/day (de also 300 kWh) | 1.8 kWh/day, 360 kWh in both languages | T. Cylinder standby is 1.82 kWh/day. `_STORE_KWH_D` 1.8, on-screen term; the total stays about 14 000 kWh (13 360 + 703). Parity row now yes |
| C28 coil air drop | 32 to 18 degrees (14 K) | 32 to 19 degrees (13 K) | T. Split unit air drop is 13.3 K; `coil_temp` goes to 19 |
| C12 shower heater | 20 kW | unchanged | J. No heater in the catalog; 1 kWh in 3 min is arithmetic and 18 to 24 kW is the real range |
| C13a shower time | 30 L in about 3 min | unchanged | J (open for the coordinator, +20 %). The beat states a throttled 10 L/min; the head's 12 L/min is its maximum and the beat draws no flow rate. Changing to 2.5 min would contradict the 20 kW line of Beat1 |
| C14 3520 kJ | unchanged | J. Pure arithmetic |
| C23, C24 | unchanged | J. The person asset stores no latent split or air demand; the values are textbook planning figures |

## Tests
- `T/m9_tests.sh`: cooling internal-gains tests, cooling clip tests, ventilation architecture test, `staffage_only` guard: 32 tests OK.
- `T/m9_tests.sh`: visuals icons, staffage tests, authoring clip tests, ventilation volume: 77 tests OK (1 skipped, pre-existing).
- `ast.parse` OK on all touched clip modules. Audit re-run reports no missing strings.

## Not verified and open
- No scene rendered (Beat1_Leistung vacuum and kettle area diagram, Beat2 cooling scale layout, Beat3 lamp, conduction Beat4, natural Beat2 are the visual changes). The cooling scale now spreads 70 and 120 W at x of 0.8 and 1.4 on a 10.4 wide axis; a look at the readout overlap is advised.
- Quiz json `🧲️physics/❓️quiz/🔣️.json` is not a clip and still says 35 W for the tealight and 100 W elsewhere; not touched.
- Changed manifest cast `ventilation-natural-radiator-on` means other agents' cached renders of that cast are stale by design (fingerprint).
- Scripts in `T/🧰️na/`: `mets.py`, `dev.py`, `pose.py` (asset inspection), `patch_audit.py`.
