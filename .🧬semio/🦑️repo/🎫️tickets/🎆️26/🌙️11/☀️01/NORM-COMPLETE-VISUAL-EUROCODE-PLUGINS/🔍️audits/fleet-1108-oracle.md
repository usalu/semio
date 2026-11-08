# 🔮️ Fleet 1108, lane oracle: third-party oracle cross-checks (EN 1992 first)

Date 2026-11-08. Python-side verification with `/home/deck/semio/.venv/bin/python -I`. No Rust was compiled by this lane; the Rust comparison runs in the next en1992 build (see requests.md `## fleet-1108 → en1992`).

## EN 1992 (done by this lane)

Mechanism, no new Rust wiring beyond one test edit:

- New oracle modules `…/en1992/…/🔮️oracles/⚖️compliance/📐️clause-forms/<domain>/🐍️.py` (domains: `🧱️materials`, `✂️shear-and-sections`, `🪟️serviceability`, `🕳️punching`, `🧵️prestress-fatigue`, `🏛️analysis-models`, `🔗️detailing`, `🔥️fire`, `🪝️fastenings`, `🌉️bridges-and-liquid`; shared inputs in `🧰️shared`). Each exports `HANDLERS = {catalogue id: fn(case) -> {key: SI value}}`; `🔮️oracles/⚖️compliance/🐍️.py` merges them (`load_clause_forms`) so the existing `--catalogue` and `{"cases":[…]}` entry points answer them.
- Libraries used: `structuralcodes.codes.ec2_2004` (fcd/fctd/fyd, Table 3.1 laws, Eq. 3.2-3.5 time development, k and k_c of 7.3.2, A_s,min Eq. 7.1), `shapely` (punching control perimeters as clipped buffers of the loaded area, foundation perimeter areas, anchor cone footprints), `scipy.optimize` (foundation punching scan), `numpy` (transformed-section neutral axis roots, load distribution of anchors, interpolation of Figures/Tables). Elsewhere the clause text transcribed independently (Tables 4.3N, 4.4N, 4.4DE, E.1DE, 5.2a, 5.4, 5.5, 5.6, 5.8-5.10, 6.3N/6.4N, 7.2N/7.3N, EN 1992-1-2 3.1/3.2a).
- Fixtures: `…/🧬️schema/🧪️tests/🧫️vectors/catalogue.json` grew from 67 to 277 cases (builder + case sources in `TICKET/🧰️scripts/oracle-en1992/`: `apply.py`, `cases_*.py`; `apply.py --write` regenerates the JSON).
- Test edit (needs the next en1992 build): `…/🧬️schema/🧪️tests/📇️catalogue/🦀️.rs`, `independent_python_oracle_matches_the_rust_catalogue` now reads oracle keys as `name`, `name@util` (utilisation) and `name@cap` (capacity), the old `util*` prefix rule stays.
- Full run of the oracle over all 277 vectors: 721 compared values, 21 s (the slow cases are the pre-existing S-04, S-06, SG-01).

### Catalogue entries now covered by an independent oracle (68 handlers added)

M-01, M-03, M-04, M-06, M-07, M-09, M-10, M-12, S-12, M-13, M-14, D-01, D-02, D-03, D-04, D-05, D-06, D-07, S-05, S-08, S-09, V-01, V-05, V-06, T-03, SL-03, SL-04, SL-05, P-01, P-05, P-06, P-07 (and `P-03@clause` for BR-07), PR-01, PR-02, PR-06 … PR-10, F-01, F-02, F-03, F-05, F-06, F-07, A-03, A-04, A-05, A-06, A-07 (only the x_u/d limit), A-08, A-09, A-11, A-12, A-14, B-04, B-05, B-06, B-08, B-09, DT-02 … DT-08, DT-10 … DT-13, R-01, R-02 (ISO 834 gas curve only), R-06, R-07, R-08, BR-01 … BR-08, LQ-01 … LQ-06, FA-01, FA-06, FA-11, FA-12, FA-13, FA-14.

### Still without a numeric oracle (EN 1992)

- A-13 (general method, fibre integration + bisection): needs `section_of` semantics of the Rust section kernel; not done.
- R-02 temperature field (only the gas curve is checked), R-03 (500 °C isotherm method, M_Rd,fi), R-05 (column in fire): need an independent 2D transient heat solver with the EN 1992-1-2 Annex A properties; not done.
- A-07 plastic rotation capacity θ_pl,d: the Rust model (κ_u − κ_y) l_pl k_λ is not clause text; only the limit x_u/d (5.6.2(2)) is checked.
- LQ-07 (checklist counter), DI-01 (data integrity, no numeric content): n/a.
- Entries that already had an oracle before this lane and were not touched: T-02, T-04, A-10, S-03, B-03, A-15, S-11, AF-01 … AL-01, M-05, S-04, S-06, A-01, A-02, B-02, V-02, V-03, P-03, SL-06, SL-07, FA-03, PA-01, PR-04, F-04, ST-01, ST-03, ST-04, SG-01, SG-02, R-04, B-07, DT-09, PR-05, M-02, M-08, B-01, A-16, DT-14.

## Oracle-versus-Rust candidates (not "fixed" in the oracle; confirm in the next build)

Values are the ones the Rust source computes by reading, the oracle values from the clause. None of these cases is in `catalogue.json` unless stated.

1. B-08 (links), mandrel diameter: Rust `4φ` for every φ; EN 1992-1-1 Table 8.1N (also for links) 4φ for φ ≤ 16 mm and 7φ for φ > 16 mm. Example φ = 20 mm: Rust 80 mm, oracle 140 mm. Vectors use φ = 10 and 12 mm only.
2. V-05 (interface shear), annex `En`: Rust takes c = 0.025 / 0.20 / 0.40 / 0.50 (DIN NA) for very smooth / smooth / rough / indented for both annexes; EN 1992-1-1 6.2.5(2) gives 0.25 / 0.35 / 0.45 / 0.50 (μ 0.5 / 0.6 / 0.7 / 0.9). Example: roughness rough, `En`, f_ck 30, σ_n = 0, ρ = 0: Rust c f_ctd with c = 0.40, oracle 0.45. Vectors use annex `De` only.
3. PR-07 (characteristic prestress): rows `ndp.en1992.p-r-{sup,inf}.{post-bonded,pre-or-unbonded}` have `naDe = null`; `ndp_number` returns NaN for the annex `De` (Rust) and the registry reader of the oracle raises (`KeyError … has no number for De`). The feature rows `pr-07-*-de` of `🧪️tests/📇️en1992-catalogue/🥒️.feature` expect the recommended values (1.10 / 0.90 / 1.05 / 0.95) for `De`; the ndp lane has to give `naDe` the same numbers. Vectors use annex `En`.
4. P-07 (prestressed flat slab punching): Rust `util = (β V_Ed/(u_1 d) − V_p/(u_1 d))/v_Rd,c`; oracle `β (V_Ed − V_p)/(u_1 d)` (the net force enters before β, 6.4.3(…) reading). Example `En`, f_ck 30, d 210, c_1 400, V_Ed 700 kN, ρ 0.01, P = 200 kN, θ = 0.1 rad, σ_cp = 0.5: oracle util 1.116645. The case is in `catalogue.json` (last P-07 row); a mismatch there is this discrepancy.
5. V-01 (support reduction), annex `De`, a_v < 0.5 d: Rust gives β = 1, EN 1992-1-1 6.2.2(6) / DIN NA uses a_v = 0.5 d (β = 0.25). Vectors avoid the region for `De`; the `En` row with a_v = 150 mm, d = 450 mm (β = 0.25) agrees.
6. D-04 (nominal cover): Rust has two structural-class reduction flags (`quality` and `special`, each −1); Table 4.3N has one ("special quality control"). Vectors use `special` only.

## EN 1993, EN 1994, EN 1998, EN 1999

Delegated to three sub-agents of this lane (same rules, own reports):

- `fleet-1108-oracle-en1993-a.md` (V-001 … V-134, V-091)
- `fleet-1108-oracle-en1993-b.md` (V-2xx … V-9xx)
- `fleet-1108-oracle-en1994-98-99.md` (EN 1994 12 rows, EN 1998 9 rows, EN 1999 A-60, A-62, A-70, A-72)

Observation for EN 1999 A-60, A-62, A-70, A-72: the Python oracle already contains an independent second implementation (`sheet_checks`, `shell_checks` in `🔮️oracles/⚖️compliance/🐍️.py`), but builds the check ids dynamically (`f"en1999.1-4.{cid}.{id}"`), so the audit token scan finds neither the ids nor a third-party import (the file says "only the standard library").
