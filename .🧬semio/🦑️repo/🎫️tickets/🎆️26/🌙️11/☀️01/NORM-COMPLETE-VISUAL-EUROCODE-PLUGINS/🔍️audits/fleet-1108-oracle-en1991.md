# fleet-1108 oracle lane: EN 1991 oracle depth audit

Scripts: `🧰️scripts/en1991-oracle-depth.py` (token scan + AST depth), `🧰️scripts/en1991-catalogue-oracle-check.py` (runs oracle on every catalogue feature row), `🧰️scripts/en1991-oracle-grid-rows.py` (emits blessed grid rows).

## Method
- Rule class (task rule): row has an oracle hit when its id or check prefix occurs as a token in a non-test Python file under an `oracle` path; third-party when that file imports a non-stdlib package. Files scanned: `🔮️oracles/⚖️compliance`, `🚦️check-outcome`, `🌪️hazard-families`, `🧮️closed-forms` (new), `🎬️scene/🔮️oracles`.
- Deep class (stricter, mine): AST of the `if entry == "Exx.yy"` branch; third-party only when the branch (transitively through module functions) really uses numpy/scipy/sympy. The rule class is optimistic because `⚖️compliance/🐍️.py` imports numpy/scipy at file level while most branches were pure `math`.
- Before: rule 103 third-party / 0 stdlib-only / 6 none (of 109); deep: 19 third-party entries. After: rule unchanged (103/0/6); deep: 57 third-party entries (57 per scan, see table).
- The 6 `none` rows are `not-modelled` by spec (E2.18, E2.21, E2.22, E3.06, E4.06, E4.09): nothing to cross-check.
- Separate JSON oracle-case fixtures (`🧪️tests/🧫️fixtures/🔮️oracle-cases/*.json`, 11 files, 33 check ids: LM1 tandem/LM71/tandem envelope/braking/centrifugal/fatigue/footbridge/rail dynamic/groups/wind net/cpe/peak pressure/snow shapes+drift/fire iso834+parametric+hasemi/bridge temperature) already carry `norm-eurocode-pynite`, `-anastruct`, `-scipy-numerics` references (blessed values, consumed by `🎯️oracle-comparison-en1991-1`); they are third-party-backed but not hit by the Python token scan (ids live in JSON).

## What was added
1. `🔮️oracles/🧮️closed-forms/🐍️.py`: sympy closed forms written from clause text plus scipy (interp1d, quad, brentq) for E12.04 E12.05 E12.07 E13.01 E13.03 E13.05 E13.06 E13.07 E13.08 E13.10 E13.12 E14.01 E14.03 E14.04 E14.05 (c_r inverted with brentq) E14.06 E14.07 E14.10 E14.11 E14.13 E14.14 E14.16 E15.03 E15.06 E16.01 E2.03 E2.04 E2.07 E2.08 E2.12 E2.14 E2.16 E2.17 E3.01 E3.04 E3.05 E3.07 E4.02 E4.03 (sympy dsolve of the Janssen ODE + quad for N_wf) E4.08 E17.08 E17.09.
2. `🔮️oracles/⚖️compliance/🐍️.py` `--catalogue` now merges the closed forms over the stdlib/scipy value (third-party value is what Rust is compared against) and exits 3 with `[oracle-disagreement]` lines when the two oracle formulations differ (tolerance 1e-7, 2e-3 for the 0.577 return-period entries, 2e-4 E14.01). Existing Rust test `python_oracle_agrees_with_the_catalogue` therefore uses it with no Rust change.
3. 168 new grid rows appended to `🧪️tests/🏋️en1991-catalogue/🥒️.feature` (ids `oracle-e*`): terrain categories 0-4 x z in 1/5/10/25/60/150 m for c_r, I_v, q_p (EN 1991-1-4 4.3.2-4.5), orography, c_prob over p, E14.01 zones, snow mu1 (0..75 deg), mu3, F_s, s_e, Annex D factor, Phi_2/Phi_3 over L_phi, rail Q_tk over radius, LM1 braking Q_lk over loaded length, footbridge q_fk over span, rail f/Q_tk over speed, hoist classes. Values come from the sympy/scipy path; the Rust catalogue test `feature_rows_match_the_reference_values` compares them (tolerance 1e-6, looser where the clause rounds).
- Verified: oracle run over all 339 feature rows: 339 within row tolerance, 0 internal disagreement stdlib vs sympy/scipy, exit 0.

## Discrepancies
- None between oracle and Rust known yet (Rust not run, no builds permitted). Oracle-internal note: EN 1991-1-3 D.1 prints the Gumbel constant as 0.577; the hand-checked feature value (E16.01 snow_factor 0.632221285) uses the exact Euler constant 0.5772157; with 0.577 the value is 0.632233 (relative 2e-5, beyond row tolerance 1e-5). The oracle uses the exact constant.

## Open
- Rust confirmation of the 168 new rows needs a build of `s-plugin-norm-artifact-en1991-...catalogue` tests (see requests.md `## fleet-1108 → en1991`).
- Stdlib-only (deep) entries remaining: E11.x (table lookups), E13.02/09/11/13 (NA snow zones), E14.02 (NA profile), E14.15, E15.01/02/04/05/07/08, E16.02/06, E17.01/02/05/06/07/10, E2.01/02/05/06/09/10/11/13/15, E3.02/03; these are table lookups or products with no library closed form; candidates for structuralcodes (not installed in the venv) or sympy products next.
- Traffic LM1/LM71 beam effects (tandem moment envelopes) stay in the JSON fixtures with pynite/anastruct; no new rows added.

## Spec-trace depth table (109 rows)
| id | clause | check | rule class | deep class |
|---|---|---|---|---|
| E11.01 | EN 1991-1-1 Annex A | en1991.1-1.self-weight | third-party | stdlib |
| E11.02 | EN 1991-1-1 5.2 | en1991.1-1.partitions | third-party | stdlib |
| E11.03 | EN 1991-1-1 6.3.1.2 | en1991.1-1.imposed,en1991.1-1.floor-force. | third-party | stdlib |
| E11.04 | EN 1991-1-1 6.3.1.2 | en1991.1-1.imposed | third-party | stdlib |
| E11.05 | 6.4 Table 6.12-6.13 | design-case | third-party | stdlib |
| E11.06 | EN 1991-1-1 6.3.4 | en1991.1-1.imposed | third-party | stdlib |
| E11.07 | 6.3.3 | design-case | third-party | stdlib |
| E11.08 | 4.4 Figure 6.2 | design-case | third-party | stdlib |
| E11.09 | EN 1991-1-1 6.5 | en1991.1-1.concentrated | third-party | stdlib |
| E12.01 | EN 1991-1-2 3.2.1 | en1991.1-2.gas-temperature,en1991.1-2.fire.none | third-party | third-party |
| E12.02 | EN 1991-1-2 3.3 | en1991.1-2.gas-temperature,en1991.1-2.opening-factor,en1991. | third-party | third-party |
| E12.03 | EN 1991-1-2 3.3.2 | en1991.1-2.fire-load | third-party | third-party |
| E12.04 | EN 1991-1-2 Annex F | en1991.1-2.fire-load | third-party | third-party |
| E12.05 | Annex C | en1991.1-2.local-flame.,en1991.1-2.local-flux. | third-party | third-party |
| E12.06 | EN 1991-1-2 3.4 | en1991.1-2.h-net | third-party | third-party |
| E12.07 | 4.3.1 + 4.2 (E_fi,d) | design-case | third-party | third-party |
| E12.08 | Annex B | design-case | third-party | third-party |
| E12.09 | Annex G | design-case | third-party | third-party |
| E12.10 | 3.3.3 + zone models | design-case | third-party | third-party |
| E13.01 | EN 1991-1-3 5.2 | en1991.1-3.snow. | third-party | third-party |
| E13.02 | EN 1991-1-3 4.1 | en1991.1-3.snow. | third-party | stdlib |
| E13.03 | EN 1991-1-3 5.3.2 | en1991.1-3.roof-surface-mu. | third-party | third-party |
| E13.04 | EN 1991-1-3 5.3.4 | en1991.1-3.snow. | third-party | third-party |
| E13.05 | EN 1991-1-3 5.3.5 | en1991.1-3.snow. | third-party | third-party |
| E13.06 | EN 1991-1-3 5.3.6 | en1991.1-3.roof-step-drift.,en1991.1-3.roof-parapet-drift. | third-party | third-party |
| E13.07 | 5.3.7 | design-case | third-party | third-party |
| E13.08 | 5.3.8 | en1991.1-3.snow-guard. | third-party | third-party |
| E13.09 | EN 1991-1-3 4.3 | en1991.1-3.snow-exceptional | third-party | stdlib |
| E13.10 | Annex D | design-case | third-party | third-party |
| E13.11 | Annex E | design-case | third-party | stdlib |
| E13.12 | EN 1991-1-3 4.2 | en1991.1-3.snow-exceptional | third-party | third-party |
| E13.13 | 6 | design-case | third-party | stdlib |
| E14.01 | 4.2 (4.1)-(4.3) | design-case | third-party | third-party |
| E14.02 | EN 1991-1-4 4.3.2 | en1991.1-4.wind. | third-party | stdlib |
| E14.03 | 4.3.3 + Annex A.3 | design-case | third-party | third-party |
| E14.04 | EN 1991-1-4 4.4 | en1991.1-4.wind. | third-party | third-party |
| E14.05 | EN 1991-1-4 4.5 | en1991.1-4.wind. | third-party | third-party |
| E14.06 | 4.2(2) NOTE, 1991-1-6 | design-case | third-party | third-party |
| E14.07 | EN 1991-1-4 7.2.2 | E14.07 | third-party | third-party |
| E14.08 | 7.2.3 Tables 7.2-7.5, Fig 7.6-7.9 | design-case | third-party | third-party |
| E14.09 | EN 1991-1-4 7.2.9 | en1991.1-4.wind. | third-party | third-party |
| E14.10 | 7.4-7.5 | design-case | third-party | third-party |
| E14.11 | 7.8-7.10 | design-case | third-party | third-party |
| E14.12 | EN 1991-1-4 Annex B | E14.12 | third-party | third-party |
| E14.13 | EN 1991-1-4 8 | en1991.1-4.bridge-wind. | third-party | third-party |
| E14.14 | EN 1991-1-4 Annex E | E14.14 | third-party | third-party |
| E14.15 | EN 1991-1-4 Annex F | E14.15 | third-party | stdlib |
| E14.16 | 7.6-7.13 | design-case | third-party | third-party |
| E15.01 | EN 1991-1-5 6.1.3 | en1991.1-5.section-range. | third-party | stdlib |
| E15.02 | EN 1991-1-5 6.1.4 | E15.02 | third-party | stdlib |
| E15.03 | 6.2 | design-case | third-party | third-party |
| E15.04 | EN 1991-1-5 5 | en1991.1-5.thermal,en1991.1-5.combination | third-party | stdlib |
| E15.05 | EN 1991-1-5 Annex C | E15.05 | third-party | stdlib |
| E15.06 | EN 1991-1-5 Annex A | en1991.1-5.isotherm | third-party | third-party |
| E15.07 | A.1 | design-case | third-party | stdlib |
| E16.01 | EN 1991-1-6 3 | en1991.1-6.stage-return-period.,en1991.1-6.stage-wind. | third-party | third-party |
| E16.02 | EN 1991-1-6 4.11 | en1991.1-6.stage-load.,en1991.1-6.construction | third-party | stdlib |
| E16.03 | 4.13 Table 4.? | design-case | third-party | third-party |
| E16.04 | EN 1991-1-6 4.12 | en1991.1-6.falling-load. | third-party | id-in-docstring/other |
| E16.05 | EN 1991-1-6 4.9 | en1991.1-6.execution-imperfection. | third-party | id-in-docstring/other |
| E16.06 | EN 1991-1-6 Annex A.2 | en1991.1-6.stage-combination. | third-party | stdlib |
| E17.01 | EN 1991-1-7 4.3 | en1991.1-7.impact-force.,en1991.1-7.accidental.none | third-party | stdlib |
| E17.02 | EN 1991-1-7 4.5 | en1991.1-7.impact-force. | third-party | stdlib |
| E17.03 | EN 1991-1-7 4.4 Tables 4.4-4.5 | en1991.1-7.derailment-force. | third-party | id-in-docstring/other |
| E17.04 | EN 1991-1-7 4.6 Annex C.4 | en1991.1-7.impact-force. | third-party | id-in-docstring/other |
| E17.05 | EN 1991-1-7 4.7 | en1991.1-7.helicopter-force. | third-party | stdlib |
| E17.06 | EN 1991-1-7 Annex A | en1991.1-7.tie-force. | third-party | stdlib |
| E17.07 | Annex B | design-case | third-party | stdlib |
| E17.08 | EN 1991-1-7 Annex C | en1991.1-7.impact-force.,en1991.1-7.impact. | third-party | third-party |
| E17.09 | EN 1991-1-7 Annex D | en1991.1-7.gas-explosion.,en1991.1-7.explosion. | third-party | third-party |
| E17.10 | EN 1991-1-7 Annex E | not-modelled | third-party | stdlib |
| E2.01 | EN 1991-2 4.2.2 Table 4.1 | en1991.2.notional-lanes,en1991.2.lane-width.,en1991.2.lane-i | third-party | stdlib |
| E2.02 | EN 1991-2 4.3.2 Table 4.2 | en1991.2.lm1-tandem,en1991.2.lm1-tandem-position,en1991.2.lm | third-party | stdlib |
| E2.03 | EN 1991-2 4.3.3 Table 4.2 | en1991.2.lm2 | third-party | third-party |
| E2.04 | EN 1991-2 4.3.4 | en1991.2.lm3,en1991.2.lm3-vehicle.,en1991.2.lm3-vehicle-leng | third-party | third-party |
| E2.05 | EN 1991-2 4.3.5 | en1991.2.lm4 | third-party | stdlib |
| E2.06 | 4.3.6 | en1991.2.dispersal-footprint. | third-party | stdlib |
| E2.07 | EN 1991-2 4.4.1 | en1991.2.braking | third-party | third-party |
| E2.08 | EN 1991-2 4.4.2 | en1991.2.centrifugal | third-party | third-party |
| E2.09 | EN 1991-2 4.5 | en1991.2.group.selected,en1991.2.footway | third-party | stdlib |
| E2.10 | 4.6 | en1991.2.fatigue-axle.,en1991.2.fatigue-udl. | third-party | stdlib |
| E2.11 | 4.7 | en1991.2.collision-x.,en1991.2.collision-y. | third-party | stdlib |
| E2.12 | 5 | en1991.2.footbridge-load,en1991.2.footbridge-horizontal,en19 | third-party | third-party |
| E2.13 | 5.7 | en1991.2.footbridge-walking. | third-party | stdlib |
| E2.14 | EN 1991-2 6.3.2 | en1991.2.rail.lm71-axle,en1991.2.rail.lm71-udl | third-party | third-party |
| E2.15 | EN 1991-2 6.3.3 | en1991.2.rail.sw0,en1991.2.rail.sw2 | third-party | stdlib |
| E2.16 | 6.4 | en1991.2.rail-dynamic. | third-party | third-party |
| E2.17 | 6.5 | design-case | third-party | third-party |
| E2.18 | 6.7 | not-modelled | none | - |
| E2.19 | 6.7.1 (the spec numbers it 6.8) | en1991.2.derail-line-load.,en1991.2.derail-length.,en1991.2. | third-party | id-in-docstring/other |
| E2.20 | 6.8.2 Table 6.11 (the spec numbers it 6.9) | en1991.2.rail-group-axle.,en1991.2.rail-group-udl.,en1991.2. | third-party | id-in-docstring/other |
| E2.21 | 6.10 | not-modelled | none | - |
| E2.22 | Annex F | not-modelled | none | - |
| E3.01 | 2.5 | en1991.3.hoist-release. | third-party | third-party |
| E3.02 | EN 1991-3 2.7 | en1991.3.crane-vertical,en1991.3.crane-position,en1991.3.cra | third-party | stdlib |
| E3.03 | EN 1991-3 2.7 + 2.8 | en1991.3.crane-horizontal | third-party | stdlib |
| E3.04 | 2.9 | en1991.3.hoist-test-static.,en1991.3.hoist-test-dynamic. | third-party | third-party |
| E3.05 | 2.12 | design-case | third-party | third-party |
| E3.06 | Annex B | not-modelled | none | - |
| E3.07 | 5 | en1991.3.machine-force. | third-party | third-party |
| E4.01 | 2.2 | en1991.4.silo-class | third-party | third-party |
| E4.02 | Annex E | en1991.4.bulk-k-mean. | third-party | third-party |
| E4.03 | EN 1991-4 5.2 + 6 | en1991.4.silo-pressure,en1991.4.silo-wall-friction,en1991.4. | third-party | third-party |
| E4.04 | 5.3 | en1991.4.silo-discharge,en1991.4.silo-discharge-friction | third-party | third-party |
| E4.05 | EN 1991-4 5.2.4 | en1991.4.silo-patch,en1991.4.silo-eccentricity | third-party | third-party |
| E4.06 | EN 1991-4 (clause not stated by the spec) | not-modelled | none | - |
| E4.07 | EN 1991-4 / EN 14491 (clause not stated by the spec) | en1991.4.dust-vent-area.,en1991.4.dust-pressure. | third-party | id-in-docstring/other |
| E4.08 | EN 1991-4 7 | en1991.4.tank | third-party | third-party |
| E4.09 | Annex A, B, C | not-modelled | none | - |
