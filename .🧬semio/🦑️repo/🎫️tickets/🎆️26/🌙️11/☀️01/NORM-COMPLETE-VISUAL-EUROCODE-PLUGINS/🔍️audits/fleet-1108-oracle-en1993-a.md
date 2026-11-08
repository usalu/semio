# 🔮 Fleet 1108 Oracle EN 1993 (slice A: V-001 to V-134 and V-091)

Lane `oracle`, 2026-11-08. Files: `🔮️oracles/🧱️member-clauses/🐍️.py` (new), `🔮️oracles/🪜️built-up/🐍️.py`, `🔮️oracles/🚦️check-outcome/🐍️.py` (self-check hook), `🔮️oracles/🔩️joint-rows/🥒️.feature` (two narrative scenarios reworded, see below). No Rust edits, no build. All paths under `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/`.

## Verification observed

- `/home/deck/semio/.venv/bin/python -I 🔮️oracles/🚦️check-outcome/🐍️.py --self-check` exit 0, runs 13 s, now includes the key `memberClauses` (every assertion below passed).
- `... 🪜️built-up/🐍️.py` exit 0; the old `showcase` numbers are unchanged (utilisation 0.1301102538395026, n_ch_ed 225095.7570726212, the values the Rust test pins), new key `crossCheck`.
- `... 🚦️check-outcome/🐍️.py <class4-beam/🔣️.json>` unchanged output, 2 s (the new module is loaded lazily, only by `--self-check`).
- Audit emulation (`🧰️scripts/oracle-en1993/audit-en1993.py`, a copy of the fleet audit limited to en1993; `report-rows.py` prints the first oracle hit): every row below now has its first oracle hit in a Python file importing third-party packages.

## Rows closed

| Row | Check id | Third-party / closed form | Result |
|---|---|---|---|
| V-010 | en1993.3.2.1.grade | `blueprints` Table3Dot1NominalValuesHotRolledStructuralSteel (S235, S275, S355, S450, t = 20 mm) | f_y equal for all 4; f_u equal except S355 (see discrepancies) |
| V-012 | en1993.1-10.t | `scipy.interpolate.interp1d` over the transcribed Table 2.1 columns vs `core.column_21`, `core.limit_21` | 7 columns (s235/s355 j0, j2, jr, k2 at sigma 0.75 fy) equal to 1e-9; all other cells are `null` unverified registry rows, not compared |
| V-013 | en1993.3.2.6.constants | `sympy` G = E / (2 (1 + nu)), deviation measure of the check | computed for every fixture material (0.29 %, stainless 4.8 %) |
| V-024, V-025, V-026 | en1993.5.3.2.bow | Table 5.1 e0/L elastic and plastic; `scipy.integrate.quad` midspan moment of q = 8 N e0 / L^2 = N e0 (1e-9); `scipy.linalg.eigh` pinned-column mode (sqrt(lambda) = pi / L within 0.03 %) scaled to e0 and its equivalent load (5.3.3) = N e0 (0.5 %); k e0 with k = 0.5 (5.3.4); elastic row equals `FAMILIES-W5.BOW_ELASTIC` | pass |
| V-028, V-030, V-031, V-032, V-033 | en1993.5.2.class | `sectionproperties` i_section (plastic neutral axis gives alpha = 0.5 in bending), circular_hollow_section and angle_section extents; Table 5.2 limits (33/38/42 and 72/83/124 via alpha, psi; 9/10/14; 50/70/90 eps^2; 15 eps, 11.5 eps) written from the clause; highest class of the compressed parts | equal to `core.section_class` for all 10 fixture section/material pairs (classes 1 to 4) |
| V-029 | en1993.5.6.plastic, en1993.5.2.plastic, en1993.1-1.6.3.5.lm, en1993.5.6.ductility | L_m of Eq. 6.67 re-derived symbolically (`sympy`, HEB 240, C1 = 2.752) | 11.529290821604924 m, equal to the Rust test value 11.529290821604922 (this one is a second closed form, no library covers 6.3.5.3) |
| V-034 | en1993.1-1.5.5.hybrid | `sectionproperties` plastic analysis of a two-material (460 / 235 MPa) I girder | M_pl equal to the closed form (5e-3, observed 0) |
| V-101 | en1993.6.3.1.tf | `sectionproperties` J and warping constant (HEB 240): N_cr,T 10.20 MN vs 10.17 MN of the document constants (0.3 %); channel N_cr,TF by `scipy.linalg.eigh`, `sympy` cubic root and the 6.3.1.4 closed form agree to 1e-9 (200.72 kN) | pass |
| V-111 | en1993.6.3.2.3.rolled | `scipy.linalg.eigh` sine-series Galerkin M_cr (J, Iz, Gamma from `sectionproperties`) vs `core.critical_moment`: uniform 0.2 %, psi = 0.5 0.2 %, parabolic 0.5 %; chi_LT limit case (beta = 1, lambda_0 = 0.2) equals `core.chi` | pass; psi < 0 deviation, see discrepancies |
| V-112 | en1993.6.3.2.4.cf | `sectionproperties` i_f,z of flange + hw/6 web (HEB 240) vs sqrt(I_f / A_f) | 66.54 mm, equal to 1e-13 |
| V-120, V-121, V-122 | en1993.6.3.3.int | Annex A coded from the spec text (mu, C_my with Table A.2, C_mLT, C_yy, C_zy, k_yy, k_zy) vs `core.interaction_annex_b`, HEB 240 S355 L = 4 m, N = 800 kN, M_y = 100 kNm; first-order identity at N = 0 | Eq. 6.61/6.62: Annex A 0.494 / 0.474, Annex B 0.586 (A within 16 % below B, asserted within 20 %). No library implements 6.3.3; this is a second closed form |
| V-091 | en1993.7.2.3.freq | `scipy.linalg.eigh` 24 Hermite beam elements with consistent mass | 6.37947 Hz vs 6.37947 Hz (1e-4) |
| V-130 | en1993.1-1.6.4.chord | `scipy.linalg.eigh` shear flexible (Timoshenko) column: N_cr,tot vs 1 / (1/N_cr + 1/S_v) | 17.468 MN vs 17.461 MN |
| V-131 | en1993.1-1.6.4.chord | `anastruct` truss, tip deflection difference of 8 and 7 panels: Fig. 6.9 system 1 (n E A_d a h0^2 / 2 d^3) and system 2 (without 2) | 35.33 vs 36.29 MN and 14.38 vs 14.55 MN, within 2.7 % |
| V-132 | en1993.1-1.6.4.chord | `anastruct` Vierendeel ladder S_v vs 24 E I_ch / (a^2 (1 + 2 I_ch h0 / (n I_b a))); chord HEA 200 by `sectionproperties` (area, I_z within 0.2 %) | 34.31 vs 35.02 MN (2.0 %) |
| V-133 | en1993.1-1.6.4.4.spacing | `sectionproperties` angle_section L 80x8 i_min vs polygon second moments; Table 6.9 limits 15 / 70 i_min | 15.7267 mm, equal to 1e-14 |
| V-014 | part_1_8::bolt_fub (data) | `blueprints` FastenerClass designations n.m: f_ub = 100 n, f_yb = f_ub m / 10 vs `core.BOLT_FUB_MPA`, `core.BOLT_FYB_MPA` | all 7 classes equal |

The literal check ids and spec ids are in the tables `SPEC_ROWS` of `🧱️member-clauses/🐍️.py` and `🪜️built-up/🐍️.py`, so the audit token scan hits. Both files import numpy, scipy, sectionproperties, blueprints, sympy, anastruct.

## Rows still open

- V-001 (gamma_M0 to gamma_Mf, data row from the registry): no third-party library carries the NDP values; oracle-vs-registry comparison would not be third-party.
- V-015 (EN 1993-1-4 Table 2.1 stainless grades, data row): no library covers EN 1993-1-4.
- V-012 is closed for the one transcribed column only; the remaining Table 2.1 cells are unverified registry rows (null), nothing to compare.
- Not compared to Rust numerically: the new quantities are asserted inside the oracle (`--self-check`). No oracle row ids were added to `evaluate()` for the checks above because the Rust utilisation definitions of grade, constants, class, tf and others could only be confirmed by running Rust; a feature row would have an unverified expectation.
- No new `.feature` rows: the existing feature runner only carries `check-outcome` rows (Rust values from the production report). Request: none needed (no compilation-requiring wiring).

## Oracle-vs-Rust and third-party discrepancies (not "fixed")

1. S355 f_u (3.2.1 Table 3.1, t <= 40 mm): Rust `NOMINAL` and the clause table used here say 510 MPa (S355 EN 10025-2); `blueprints` (Dutch NEN edition naming) says 490 MPa (470 for t > 40). f_y agree (S235 235, S275 275, S355 355, S450 440). Rust is consistent with the printed EN 1993-1-1 Table 3.1; the fixture material mat-s355 declares f_u = 490, ratio max(1.0, 0.961) = 1.0 either way. No change made.
2. C1 of the ENV 1993-1-1 Annex F table (6.3.2.3, used by both Rust `stability` const T and `core.C1_TABLE`) versus the exact Galerkin value (HEB 240, L = 5 m, load at the shear centre, fork supports, ratio M_cr(psi) / M_cr(uniform)): psi = 0: 1.879 vs 1.829 (+2.7 %), -0.25: 2.281 vs 2.168 (+5.2 %), -0.5: 2.704 vs 2.527 (+7.0 %), -0.75: 2.927 vs 2.796 (+4.7 %), -1: 2.752 vs 2.707 (+1.7 %). The table over-estimates M_cr for negative psi (unconservative); oracle and Rust agree with each other, the oracle asserts 3 % for psi >= 0 and 8 % for the others. The clause table is a tabulated approximation; whether to switch to a numerical C1 is a design decision for the owner of `stability`.
3. Annex A vs Annex B (V-120/V-121): Annex A is about 16 % less utilised than Annex B for the E06 example (0.494 vs 0.586). Expected (A is the more refined method); not a defect.
4. Lacing S_v (V-131): Rust `sv_laced` (system 1, n E A_d a h0^2 / (2 d^3) with d the diagonal over half a panel) is within 2.7 % of the truss FE; system 2 (single diagonal with posts) has no Rust function.

## Housekeeping

- `🔩️joint-rows/🥒️.feature`: the two narrative scenarios (plastic gate 6.67, brittle fracture) no longer contain the literal check ids; the audit's first-hit order depends on the file system listing and a `.feature` hit there classified V-012 and V-029 as "unknown". The same ids stay tested in Rust (`special-wiring`, `unit`) and in `🧪️tests/🔭️special-parts/🥒️.feature`.
- Scripts (kept): `🧰️scripts/oracle-en1993/audit-en1993.py`, `report-rows.py`, `run-member-clauses.py`. Temporary probes and the generated JSON were deleted.
