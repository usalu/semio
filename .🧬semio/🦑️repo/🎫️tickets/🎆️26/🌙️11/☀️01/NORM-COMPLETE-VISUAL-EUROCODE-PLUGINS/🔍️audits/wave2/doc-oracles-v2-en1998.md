# Doc oracles v2: EN 1998

Python side dry-run green (`/home/deck/semio/.venv/bin/python -I`), 28/28 rows; nothing compiled or run on the Rust side.

## Oracle
`🫨️en1998/…/✳️any/🔮️oracles/🚦️check-outcome/🐍️.py` (`--json` default, `--self-check`). Written from EN 1998-1 3.1.2, 3.2.2.2, 3.2.2.5, 3.2.4, 4.2.3, 4.3.3.2, 4.3.5, 4.4.2, 4.4.3.2, 5.4.3, 6.5, Tables 3.2, 3.3 and DIN NA Table NA.4. Families per spec-trace `check` row: SEI-11/46 `baseShear`, `storeyForces`, `overturning`, `componentCombination`; SEI-20 `planRegularity`, `elevationRegularity`, `plan.slenderness`, `plan.envelope`, `elevation.setback`; SEI-30 `qLimit`; SEI-35 `t1Given` (and Rayleigh period for `baseShear`); SEI-41 `torsion`; SEI-44 `pdelta`; SEI-45 `drift`; SEI-47 `joint.gap`; SEI-49 `nonstructural.force`; SEI-50/51 `capacityDesign.nodes`, `rc.rho`, `rc.omegaWdBeam`, `rc.noSteelClass`, `steel.class`. Not covered: ground class/site response (SEI-01/02, status checks), time history (SEI-06), importance (informative), diaphragms, masonry.simple, bridges/silos/tanks/towers, member checks with `.minimum` utilisation (`minB`, `omegaWd`, `rhoPrime`).

## Third-party validation
`--self-check`: shear-building Rayleigh period against `scipy.linalg.eigh` (generalised eigenproblem; Rayleigh/exact 0.9933 for the 4-storey chain, equal for one storey), design-spectrum branch against `scipy.optimize.brentq`.

## Case and fixtures
`🧪️tests/🚦️check-outcome-en1998-1`: 4 old rows kept (same values), 24 new (15 pass, 9 fail): fixtures `🧪️tests/🧫️fixtures/🚦️check-outcome/seismic-rich-pass|seismic-rich-fail/🔣️.json` (two 4-storey RC buildings, importance III, Rayleigh period, joint gap, non-structural element, capacity design, steel member; fail: weak base shear, soft storey, large drift/P-delta, set-back, narrow joint, rho above max, steel class 4). Generator `🧰️scripts/doc-oracles-v2-en1998-fixtures.py`; manifest sentence via `🧰️scripts/doc-oracles-v2-manifest.py`.

## Mismatches (reading the Rust; `📌️important/requests.md`, section "from: doc-oracles v2 (en1998)")
1. Accidental torsion delta computed and dropped: storey shears 1.3x too low (s2 0.1574 against 0.2046).
2. `planDims` utilisation Lx/Ly (1.5) with forced Pass.
3. `elevationRegularity` only under plan dimensions > 0.
4. `planRegularity` max e0 against min r instead of per direction.
5. Old EN oracle branch wrong (S 1.2/TC 0.4 for all grounds, no gamma_I, q floor 1.0): replaced; its numbers differed from the Rust on 13 of 262 documents, all explained by these oracle gaps (q_max cap, gamma_I, 0.8 irregularity factor, Rayleigh, EN table).
No disagreement of a check value with the Rust is known (not run).
