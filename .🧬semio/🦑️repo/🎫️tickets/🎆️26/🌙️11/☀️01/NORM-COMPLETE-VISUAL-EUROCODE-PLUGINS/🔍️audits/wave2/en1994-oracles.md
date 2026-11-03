# EN 1994 oracles (wave 2): check-outcome twins for the missing families

Python only; dry runs via `heavy.sh`; no Rust, no git writes, `freeze.txt` absent throughout. New oracle module `🔮️oracles/🚦️check-outcome/🧭️families/🐍️.py` (loaded as sibling by the check-outcome oracle; wired in `beam_checks` through `families()` / `stress()` and in `slab_checks`). Clause text is not in the repo (only spec-trace ids), so formulas are from EN 1994-1-1, EN 1992-1-1 and EN 1993-1-1 as known; Rust read only for ids, inputs and constants.

## Family coverage
| Check id | Oracle | Derivation (independent of Rust) | Third-party | Rows (pass/warn/fail) |
|---|---|---|---|---|
| 5.5.class | yes | Table 5.2 outstand 9/10/14 eps, web in bending 72/83/124 eps, c = (b-tw)/2-r, root radius from catalogue area; util = class/3 | sectionproperties root radius (existing self-check) | 1 / 0 / 1 (class 4 plate girder) |
| 6.6.5.5.spacing | yes | max(s/min(6 hc, 800 mm), 5d/s) | none (closed form) | 2 / 0 / 1 |
| 6.6.1.1.layout | yes (status `warning` when declared > fitting) | floor((L/2)/s) x studs per rib | none | 2 / 2 / 0 |
| 6.6.6.vlrd | yes | EN 1992-1-1 6.2.4, two planes at 2 hf, nu = 0.6(1-fck/250), theta 45 deg; N_c,req from the oracle's strip kernel; util = max(strut, A_sf) | numpy bisection kernel (existing) | 1 / 0 / 2 |
| 6.2.2.4.vm-sagging | yes (simply supported, uniform load) | rho = (2V/Vpl-1)^2 capped at 1; reduced strength (1-rho) fyd in the web strips, plastic axis re-solved, 201 stations of M(x) = 4Ms(1-s), V = V|1-2s| | sectionproperties plastic moment with a reduced-yield web material (agrees to 1e-12) | 2 / 0 / 1 |
| 6.2.2.4.vm (hogging) | yes (end, interior, cantilever) | same reduced-strength strip integration with the bar force at 40 mm | same | 3 / 0 / 1 (rho 0.27) |
| 7.2.2.stress | yes (not cantilever) | staged: M_a on steel alone (unpropped), M_G on n_L = n0(1+1.1 phi), M_Q on n0; max(sigma_a/fy, sigma_c/0.6fck) | sectionproperties transformed composite I (agrees to 1e-12) | 3 / 0 / 1 |
| 9.7.6.punching | yes (slabs with point loads; the bending rows stay off for those) | EN 1992-1-1 6.2.2 v_Rd,c, C_crit = 4a + 2 pi 2 dp at a = 200 mm, F_Ed = 1.5 F | none | 1 / 0 / 1 |

Fixtures added (`🧪️tests/🧫️fixtures/📄️oracle-documents`): `beam-class4-fail`, `beam-stud-spacing-fail`, `beam-hogging-interaction-fail` (end span, Q 22 kPa, V = 0.76 Vpl), `slab-punching-pass` (60 kN), `slab-punching-fail` (200 kN). 25 new rows (feature now 83 rows). Generator `🧰️scripts/feature-rows/gen1994v12.py` (idempotent). Dry run (`wave2-features-dryrun.py`, through heavy.sh): `check-outcome-en1994-1: 83 ok, 0 failed`, no FAIL or NO HANDLER lines. `🐍️.py --self-check` passes with the new `families` block.

## Rust vs oracle (not changed, no clause text in the repo to quote)
1. 6.2.2.4 bending-shear: Rust subtracts `rho tw hw^2/4 fyd` (hogging) and a fixed-axis web integral (sagging) from the unreduced moment; the clause applies (1-rho) fyd to the shear area and the section is re-solved. With bars/concrete the axis moves, so they differ: 0.2 % at rho 0.27, 5 % at rho = 1 (hogging, 3.465 vs 3.650). The `vm` hogging fail row (2.03 vs 2.035) is inside the 1e-2 tolerance.
2. Rust `rho` is not capped at 1: for V_Ed > V_pl,Rd (e.g. `beam-shear-fail`, rho 2.63) the hogging M_V drops far below the physical bare-flange value (Rust-style 500 kNm vs 323 kNm with cap... i.e. web strength cannot go negative). The vpl row already fails there; no vm row uses such a document.
3. 6.6.5.5 spacing: Rust limits to 6 h_c (concrete above the sheeting); the clause text speaks of the total slab thickness, so Rust is the stricter (safe) reading. Left as is.
4. `layout` util is n/fit and the status is `warning` at > 1: rows use exactly that; if Rust reports the 1.6 / 2.0 rows differently (e.g. `unverifiedBasis`) adjust.
5. Status caveat (from oracle-emit): rows say `pass`; if any of these checks reads an unverified registry NDP (gamma_c / gamma_s via annex params for vlrd), Rust would report `unverifiedBasis`.

## Still without a Python twin
2.lm1, 9.8.sls (needs I_eff slab kernel), 6.7.1.scope / 6.7.4.introduction, 3.materials, 5.4.2.analysis / cracked, shear-lag stress, 7.4.crack w_k at check level, 7.2.2.stress for cantilevers, vm-sagging for point-load/continuous profiles, 1-2 table-column (tables absent), 1-2.4.3.5.column, action.single-source, subject.empty (flags). concreteproperties was not used (sectionproperties covers the plastic and elastic cross-checks needed here).

- ready for rebuild: en1994 oracles (fixtures + feature rows only)
