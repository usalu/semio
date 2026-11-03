# Doc oracles v2: EN 1990 and EN 1991

Nothing compiled or run on the Rust side. Python dry-run (`/home/deck/semio/.venv/bin/python -I`, `🗑️generated/doc-oracles/dry-run.py`): en1990 57/57 ok, en1991 42/42 ok; both oracles `--self-check` green.

## What exists
| artifact | new oracle (`🔮️oracles/🚦️check-outcome/🐍️.py`, `--json` default, `--self-check`) | fixtures (`🧪️tests/🧫️fixtures/🚦️check-outcome/`) | case rows |
|---|---|---|---|
| en1990 | wraps `⚖️compliance/🐍️.py` (brute-force combinations, scipy FORM, Table D1) and divides by the member criterion | `annex-all-pass`, `annex-all-fail` (annex De), `annex-en-610ab-pass`, `annex-en-610ab-fail` (annex En) | 7 old + 50 new |
| en1991 | wraps the same-folder compliance oracle (required values) and adds rows; utilisation = required / assumed | `actions-all-pass` (0.55 to 0.9), `actions-all-fail` (1.15 to 2.0) | 4 old + 38 new |

The case `🐍️.py` of both now points at the new oracle (older rows unchanged). Generators: `🧰️scripts/doc-oracles-v2/{gen-en1990,gen-en1991,add-rows}.py`. Rows lie outside 0.95 to 1.02; tolerance 1e-3.

## Families against spec-trace `check` rows
EN 1990: E90.06/E90.16/E90.22/E90.23 (`str|geo.<m>.6.10|6.10a|6.10b`, `.set-c`), E90.15 (`equ.stab|destab`), E90.17/18 (`6.11b|6.12b`), E90.26 (`a2.transient`), E90.08/09/19 (`sls.<m>.6.14b..6.16b`), E90.24 (`a14.deflection|deflection-w3|vibration`), E90.21 (`fat`, `fat.damage`), E90.28 (`annex-c.beta`), E90.29 (`annex-c.form`), E90.30 (`annex-d`), E90.02/05 (`annex-b.dsl|il`), E90.03/04 (`table-2.1.design-working-life`), E90.10/11/12 (`e10.material`, `e11.geometry`, `e12.analysis`).
EN 1991: E11.01 (`self-weight`), E11.03/04/06 (`imposed`, `floor-force`), E11.02 (`partitions`), E11.09 (`concentrated`), E13.01..05 (`snow` via the older oracle), E14.02/04/05 (`wind` via the older oracle), E12.01/02 (`gas-temperature`, nominal curves), E12.03/04 (`fire-load`), E12.06 (`h-net`), E15.04 (`thermal`), E16 (`construction`), E17.01/02 (`impact`).

## Third-party validation (`--self-check`)
en1990: FORM beta of N(300,30) against N(150,30) = 3.53553 against closed form and scipy.stats.norm (1e-6); Table D1 k_n against scipy.stats.t; anastruct simply supported beam reaction 30.0 kN and PyNite moment 45.0 kNm (q L^2/8) plus the PyNite deflection line at s = 0.25 against the 3.2 s (1 - 2 s^2 + s^3) shape used for the A1.4 stations. Combination, FORM design point and Palmgren-Miner come from the scipy-based compliance oracle. en1991: scipy.integrate spring-mass ODE peak force 932 438 N against v sqrt(k m) 932 440 N (2e-6), scipy.constants Stefan-Boltzmann, ISO 834 curve against numpy log10, c_prob; wind/snow/gumbel pieces in the older oracle use scipy.stats.

## Gaps (not covered by a document row)
EN 1990: Annex A2 traffic groups and bridge SLS (`a2.traffic-groups`, `a2.bridge`, `a2.rail-*`), `annex-b.kfi` (informative), `annex-c.alpha` and `partial-factors` (informative), `a14` frame/storey drift, fatigue `fat` > 1 (Rust Warning), overrides (`en1990.override.`), `.min` reversal (no fixture with a negative effect). EN 1991: parametric fire (`opening-factor`, `validity`), wind bridge/dynamic (`1-4.dyn-*`, `vortex`, `bridge-wind`), thermal sections 1-5 Annex C, stage rows (`1-6.stage-*`), roof surface/step/parapet drift collections, bridge/crane/silo families, explosion. Failing rows for statuses the Rust maps to Warning (fat, annex-b, working life) are intentionally absent.

## Mismatches found by reading (owners in `📌️important/requests.md`, section "from: doc-oracles v2 (en1990/en1991)")
No check value disagrees with the oracle by reading. Findings: (1) en1990 6.11b/6.12b ids are fine in the document report (audit coverage-3 concern concerns only the catalogue key); (2) en1990 `a14.deflection` sampled-max risk and Warning-vs-Fail on fat, DSL/IL, working life; (3) en1991 `construction_qk_pa` machinery/concreting 3.0 kN/m2 not in Table 4.1; (4) en1991 fire load ignores the occupancy fractile (450.3 against 548.1 MJ/m2, +21.7 %); (5) delta_q1 log-interpolation vs linear (1.3408 against 1.2333 at 100 m2); (6) sigma 5.67e-8 vs 5.670374e-8.

## Files
`⚖️en1990|🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/`: `🔮️oracles/🚦️check-outcome/🐍️.py` (new), `🔮️oracles/🔣️.json` (rationale), `🧪️tests/🚦️check-outcome-enXXXX-1/{🥒️.feature,🐍️.py}`, `🧪️tests/🧫️fixtures/🚦️check-outcome/*` (4 + 2 documents); ticket: `🧰️scripts/doc-oracles-v2/*`, `📌️important/requests.md`, `🗑️generated/build-en1990|en1991/latest.md`.
