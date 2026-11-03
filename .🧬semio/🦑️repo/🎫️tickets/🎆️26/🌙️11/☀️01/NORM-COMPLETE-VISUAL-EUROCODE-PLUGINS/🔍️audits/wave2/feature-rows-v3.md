# Feature rows v3 (wave 2): EN 1990, EN 1995, EN 1997 (+ new families)

Python only, niced, one artifact at a time, dry runs via `heavy.sh`. No Rust, no git writes. Scripts in `🧰️scripts/feature-rows/`: `gen1995.py`, `gen1997.py` (fixture generators, copy a passing fixture, cross one input, `--write`), `cover.py` (per-family pass/fail coverage from the feature file, counts `pass-v2-NN`, `fail-v2-NN` too; `spec_families.py` missed them and the mutation-fixture sources, `spec_families3.py` fixes both), `rows199{0,5,7}-v3.tsv` (spec fed to the idempotent `append_rows.py`). Expected values come from the committed Python check-outcome oracles (no hand guesses); statuses `unverifiedBasis` taken from the oracle.

## Result (families with a pass AND a fail row / families in the feature)
| code | before | after | rows | dry run |
|---|---|---|---|---|
| EN 1990 | 25 / 29 | 29 / 29 | 84 -> 88 | 88/88 ok |
| EN 1995 | 17 / 38 | 38 / 38 | 58 -> 80 | 80/80 ok |
| EN 1997 | 14 / 30 | 30 / 30 | 62 -> 78 | 0 non-ok lines (all 78 ok) |

## New fixtures (`🧫️fixtures/🚦️check-outcome/`)
- EN 1990 `annex-limits-fail` (copy of annex-all-fail: RC3, DSL1, IL1, working life 100 years, rdFat 6000): `annex-b.dsl` 3.0, `annex-b.il` 3.0, `fat.beam-B1` fail, `table-2.1.design-working-life` 2.0.
- EN 1995 (generator `gen1995.py`): `overload-fail` (small slender beam 80x160 over 7 m, loads x4, c90 force, closely spaced bolts, big holes, 15 degree taper), `column-overload-fail` (N x3, buckling lengths x2), `vibration-fail` (f1 over 8 Hz, light mass, tiny damping), `panel-overload-fail`, `floor-overload-fail`, `deck-wheel-fail` (wheel x40), `deck-roomy-pass` (plate side 0.5 m, plate_bearing 0.346).
- EN 1997 (generator `gen1997.py`) `geo-overload-fail` (from geotechnical-rich-pass: investigation depth 1 m, sand phi 45 with weak CPT/SPT, F1 4.2 MN with 2.6 MN horizontal, F2 150 kN horizontal, pore pressure 400 kPa, head 5 m over 3 m flow, pile group 9 MN, thin shaft cohesion, wall embedment 0.5 m).

## Coverage tables (rows added per family, utilisation of the new row)
EN 1990 (fail rows): annex-b.dsl 3.0, annex-b.il 3.0, fat.beam-B1, table-2.1.design-working-life 2.0. The traffic-groups pass family was already paired.

EN 1995 (fail rows from `overload-fail` unless noted): 6.1.5.c90 2.73, 6.1.7.shear 8.6, 7.2.winst 46.7, 7.2.wfin 97.9, 7.2.wfreq, 7.3.f1 6.25, 7.3.stiffness 1.131, 8.spacing.a1 2.0, 8.spacing.a4t 3.0, t-18.hole_limit 1.5, t-18.net_section 13.0, t-30.taper_angle 1.5; 6.3.2.compression 5.48 (`column-overload-fail`); 7.3.velocity 2.64 (`vibration-fail`); t-122.wheel_spread 7.2 (`deck-wheel-fail`); t-122.plate_bearing pass 0.346 `unverifiedBasis` (`deck-roomy-pass`); t-97.bending 14.2 / rolling_shear 6.0 / gross_shear 1.22 fail (`panel-overload-fail`) and gross_shear pass 0.0508 (`unverifiedBasis`, mutation before); 7.2.floor-creep 101.8 and t-63.f1 12.97 (`floor-overload-fail`). Fail rows carry status `fail`, the plate_bearing and gross_shear pass rows `unverifiedBasis` as the oracle yields.

EN 1997 (all fail rows from `geo-overload-fail`): 2.4.2.investigation 6.0, 2.phi.cpt 1.38, 2.phi.spt 1.47, 2.phi.derived 1.47, 6.5.bearing F1 7.7, 6.5.3.sliding F1 1.46, 6.5.4.eccentricity F2 2.25, 6.6.tilt.el 1.18, 10.hyd.upl-U1 6.67, gi-12.phi_layer 1.36, gi-17.phi_assumed 1.54, hyd-01.heave (new v11 heave family) 3.03, pile-06.block, pile-06.max_pile 1.64, rw-07.embedment 5.06, rw-12.embedment 10.25. Heave had a pass row (0.30); now both.

## Families added tonight, by code
- EN 1991 v12 (footbridges, railDynamics, loadDispersals, bulkSolids, rotatingMachines, craneHoists): the check-outcome oracle does not emit them (ids emitted: 1-1 self-weight/imposed/partitions/concentrated/floor-force, 1-2, 1-3, 1-4 wind/dyn, 1-5, 1-6, 1-7). They are covered by clause-level oracle cases `third-party-v12-rows.json` (scipy numeric), not by check-outcome pass/fail rows. Open: needs the shared/oracle owner to emit entity checks.
- EN 1993 v13 (fin plate, cover plates, base plate): not emitted by the check-outcome oracle (zero matches); the plate-and-foundation oracle has its own feature (`🔮️oracles/🧩️plate-and-foundation`). Not extended; en1993 has no check-outcome fixture directory.
- EN 1994 v11 tension stiffening: not emitted by any oracle; no change (also untouched while frozen).
- EN 1992 v13 outward punching: already paired (`punching`, `punching-max`, `uout` have pass and fail rows incl. `slab-FS-*-outward-*`).
- EN 1997 v11 heave: done above.

## Notes
- Tolerance 1e-6 on all new rows, same Python path as the oracle (self-comparison through the step layer; Rust unrun).
- Boundary of method: `cover.py` families derive from the instance regex per code; near-limit rows (u 0.9 to 1.1) are not used on purpose (sliding F2 0.95 not used).
- ready for rebuild: feature rows v3 (appended to build-en1990, build-en1995, build-en1997 latest.md).
