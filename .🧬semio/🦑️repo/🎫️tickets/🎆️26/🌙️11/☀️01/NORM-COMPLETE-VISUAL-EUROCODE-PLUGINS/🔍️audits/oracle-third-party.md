# Third-Party Oracle Cases

Status 2026-11-02. Library coverage: `oracle-library-coverage.md`. Harness: `oracle-harness.md`. Requests to norm agents: `📌️important/requests.md`, section "third-party oracle agent". Nothing was built (no rustc/cargo of the repo); the Rust edits are compile-checked only where stated.

## Result

- 84 blessed cases, 6 artifacts, 8 libraries. `python -I 🐍️.py --cases <all files>` prints 84 `ok` in 24 s; pytest `🏃️harness/🧪️tests/🐍️.py` 17 green.
- Reference values come from the libraries, never from the Rust or the Python compliance mirrors. Where two libraries compute the same quantity, they agree inside `numerical-v1` (5e-3) or better; the judge fails on any spread beyond the profile ("oracle disagreement").
- Our implementation (Rust formulas transcribed in `🧰️scripts/oracle-third-party-diff.py`, not executed) agrees with the libraries on 99 of 119 compared quantities. The 20 differences are 12 Table 3.1 roundings (information) and 8 real defects in the EN 1993 section catalogue (It and Iw of HEB 240-300); two Python-mirror defects were found by running the mirror itself, not the Rust.

## Cases per artifact (files `🗿️artifacts/<enXXXX>/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🔮️oracle-cases/`)

| artifact | file | cases | libraries | content |
|---|---|---|---|---|
| en1990 | `third-party.json` | 1 | sympy + scipy | reliability index beta from P_f (Annex C) |
| en1992 | `third-party.json` | 36 | structuralcodes, concreteproperties, blue-prints, scipy | Table 3.1 laws C30/C60/C90; f_cd; M_Rd of rectangles (normal, over-reinforced, hardening k = 1.08, C60); M-N points (5 axial forces, with and without hardening); cracked x, I_cr, M_cr, sigma_c, sigma_s; V_Rd,c (EN and NA-DE constants, N_Ed, thin slab, rho cap); V_Rd,max, V_Rd,s, V_Ed,max (cot 1, 2.194, 2.5); 7.8, 7.9, 7.11; 5.13N, 5.14, 5.34, 6.6N, 6.30, 6.38, 7.16a/b, 8.3 |
| en1992 | `third-party-mechanics.json` | 1 | PyNiteFEA, anastruct | propped cantilever (reactions, M_hog, M_sag, deflection) |
| en1993 | `third-party.json` | 22 | sectionproperties, blue-prints, scipy | HEB 240/260/280/300 and IPE 300 constants; RHS 200x100x8; CHS 139.7x5 (also exact annulus integrals); PFC 200 channel (with shear-centre offset); L100x10 angle (principal moments); M_cr of IPE 300 (C1 = 1, 1.127) from FE I_z, I_t, I_w; N_pl, M_c,Rd, V_pl,Rd, A_v (I, RHS, CHS, channel), M_y,V,Rd, M_N,y,Rd, M_b,Rd, alpha_cr |
| en1993 | `third-party-mechanics.json` | 2 | PyNiteFEA, anastruct | fixed-base portal sway and base reactions; simple beam |
| en1994 | `third-party-mechanics.json` | 4 | PyNiteFEA, anastruct | two-span beam: full UDL, first-span UDL, midspan forces; simple beam deflection |
| en1995 | `third-party.json` | 1 | blue-prints | 7.5 natural frequency |
| en1995 | `third-party-mechanics.json` | 4 | PyNiteFEA, anastruct | as en1994 with timber E and section |
| en1997 | `third-party.json` | 11 | groundhog | N_q, N_c, N_gamma (EN D.4 via Vesic - 4 tan phi) at 20/30/35 deg; Rankine and Coulomb K_a, K_p; Kulhawy-Mayne phi from CPT (3); s_u (Rad-Lunne); I_c (Robertson-Wride) |
| shared | `🧪️tests/🧫️fixtures/🔮️oracle-cases/numerical.json` | 3 | scipy + sympy | Ayrton-Perry chi (symbolic root), Euler N_cr (sin(kL) = 0), parabola-rectangle flexure |

EN 1991, 1996, 1998, 1999 and the DIN artifacts: no library covers them (see coverage audit); they keep the independent-formulation oracles.

Tolerance profiles: library formula versus library formula `eurocode-quantities-v1` (1e-4); FE, fibre integration, meshes `numerical-v1` (5e-3); Coulomb K_p and I_c (library-specific conventions) `method-spread-v1` (3e-2). NA parameters (alpha_cc, gamma_c, C_Rd,c, k1, ...) are case inputs; the libraries never supply them.

## What was built

- Adapters (`✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️harness/🔌️adapters/`): `structuralcodes`, `concreteproperties`, `sectionproperties`, `blueprints` (table-driven, 26 checks: 25 single formulas plus the V_Rd,c chain), `groundhog`, `sympy`, `pynite`, `anastruct`; `numerical` got the exact CHS integrals. All import their library at module top, so a missing library fails the run.
- Harness: `--bless` takes each quantity from the first reference that computes it; non-finite library output raises (found because groundhog returns NaN without a message when an argument is out of its validated range); coverage script now includes PyNite and anastruct, no row cap, groundhog stub.
- groundhog 0.15.0 imports `plotly`, `pyproj`, `jinja2` at module level without declaring them. The adapter registers an inert import finder for exactly those three roots (documented in its docstring). Alternative: add them to the `test` group (request filed).
- Registry: per artifact `enXXXX-1-<library>` entries (kind `third-party-library`, pinned version, SPDX licence, `testOnly`, capability `enXXXX-1-third-party-cases`, rationale naming the quantities) in `🔮️oracles/🔣️.json` of en1990, en1992, en1993, en1994, en1995, en1997; root entries `norm-eurocode-pynite` and `norm-eurocode-anastruct`. Files round-trip byte-exact through `json.dumps(indent=2)`.
- Rust: `python_oracle::{harness, verify_cases, verify_cases_above}` (std only; the file was compiled standalone with the repo nightly as a library, zero errors) and one test `third_party_libraries_reproduce_the_blessed_oracle_cases` appended to an existing python-oracle test file of each artifact (en1990 and en1994 `💡️inferences/🧪️tests/🔬️compliance-report`, en1992/en1993/en1995 `🧪️tests/⚖️compliance`, en1997 `⚖️compliance/🔮️oracle-and-remedy-laws`). It runs the harness over the artifact's case directory with the repo interpreter and fails loudly with the report on any mismatch. Not compiled in the repo build; "ready for rebuild: third-party oracle cases" appended to `🗑️generated/build-en{1990,1992,1993,1994,1995,1997}/latest.md`.
- Scripts (inputs, kept): `🧰️scripts/oracle-third-party-gen.py` (case definitions), `oracle-third-party-bless.sh` (generate, bless, verify all), `oracle-third-party-register.py` (registry), `oracle-third-party-diff.py` (our formulas against the blessed values).

## Disagreements between our implementation and the libraries

Compared by `oracle-third-party-diff.py`: 119 quantities, Rust sources transcribed with file:line, never executed. Full prose in `requests.md`.

| # | severity | area | finding |
|---|---|---|---|
| 1 | S1 | EN 1993 catalogue `📸️snapshot/🦀️.rs:787-890` | HEB 240/260/280/300: `it` is 27-37 percent too small (65.3 against 103.7 cm4 for HEB 240), `iw` is ten times too large (4.87e-6 against 4.76e-7 m6). M_cr of an HEB 240 over 6 m (C1 = 1): 867.6 kNm with the catalogue constants, 501.7 kNm with the FE constants, +73 percent unsafe. A, I_y, I_z, W_el, W_pl agree within 0.1 percent (W_pl,z 0.5 percent). `mcr_closed_form` itself matches the NCCI SN003 formula to 1e-16 on library inputs. |
| 2 | S1 (verify) | EN 1992 Python compliance mirror `🧪️tests/⚖️compliance/🐍️.py:242-263` | Rectangular block with x <= 0.45 d: M_Rd = 561.5 kNm for A_s = 3500 mm2 (b 300, d 450, C30); structuralcodes and concreteproperties: 402.1 kNm (+39.6 percent). Under-reinforced +0.4 to +0.9 percent. Rust uses strain compatibility and probably gives the library value; the case files are the test. |
| 3 | S2 | EN 1992 Python mirror `🐍️.py:191-207` | NA-DE k1 0.15 instead of 0.12 (+6.6 percent with N_Ed), cap 0.2 f_ck instead of 0.2 f_cd, A_c = b d. Rust `✂️shear-torsion/🦀️.rs:5-19` is correct; Rust helper `🧰️helpers-and-annex-parts:131-135` assumes h = d / 0.9 for sigma_cp. |
| 4 | info | EN 1992 `🧱️materials-and-time/🦀️.rs:23-50` | Table 3.1 rounded values against formulas: f_ctm +0.1 to 1.0 percent, f_ctk,0.05 up to 1.7 percent, f_ctk,0.95 up to 0.9 percent, E_cm up to 0.8 percent. Correct per table; use `table-exact-v1` there. |
| 5 | info | EN 1992 catalogue E01 | A_s,req 1487 mm2 against 1483.5 mm2 (both libraries, k = 1.08, eps_ud = 45 permille): Rust 0.24 percent conservative, test tolerance 2e-3. |
| 6 | info | EN 1997 | groundhog has no EN D.4 N_gamma; EN value checked as Vesic - 4 tan phi. Coulomb K_p cases lie outside the plane-surface validity. |

Agreement worth recording: V_Rd,c (EN and NA-DE constants), V_Rd,max, V_Rd,s, V_Ed,max, 7.16 l/d, torsion T_Rd,max, cracked x/I_cr/sigma_c/sigma_s (7.2), 6.18 shear areas A_v of rolled I, RHS, CHS, channel, M_y,V,Rd (6.30), M_N,y,Rd (6.36), M_cr, N_q, N_c, N_gamma, Rankine and Coulomb coefficients, Kulhawy-Mayne phi: all to 1e-4 or better (mostly 1e-15). Two-span beam coefficients, UDL and midspan forces (exact values in requests.md) confirm the review findings for EN 1994/1995 two-span load arrangements.

## Not done / limits

- Rust formulas were not executed: the comparison is a transcription. The Rust tests only run the harness over the blessed library values; the next step for each norm agent is to feed `inputs` of a case to its evaluator and compare with `expected` under the case profile (the case format is language-agnostic).
- Mechanics cases are in the artifact directories but no kernel test consumes them yet.
- `🔒️dependencies.json` baseline still lacks the six oracle libraries (needs the native `repo verify` command); the lock/sync state is in `oracle-deps-lock.md`.
- blue-prints EN 338 tables and EN 1993-1-8 tables 3.1/3.4 are available but not wired; EN 1992 `fpr` 2023 and EN 1993-1-1:2022 formulas are out of scope (editions 2004/2005).
- groundhog and anastruct are GPL-3.0 (test-only, never linked into shipped code), `cytriangle` (sectionproperties, concreteproperties, blue-prints stack) has no linux-arm64 wheel.
