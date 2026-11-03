# 🔩 EN 1993 Norm Completion Report (partial: first pass)

Artifact: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993`. Subset root `S = 🏅️standards/🔖️1/🪆️subsets/✳️any`.
Checks root `C = S/🧬️schema/💡️inferences/⚖️checks`.

## 0. Status in one paragraph

A new schema-neutral clause library `C/🧮️clauses` (pure per-entity functions, mm/N/MPa, one entity in, numbers out, split mechanical into `CheckUnit`s) implements most of the EN 1993-1-1 catalogue and parts of 1-2, 1-5, 1-8, 1-9 and 1-10. It is validated standalone (17 tests, E01..E15, all green, see section 4). The engine (`check_full_steel_structure`) was rewired to it and the listed defects were fixed. **The integrated crate build (`@semio-tech/norm-en1993-rs:check`/`:test`) was queued behind host memory pressure and had not run when this report was written; the engine edits are therefore not compiled or test-verified yet** (section 5). Several catalogue groups still need schema entities (frame, supports, parametric sections, joint geometry) and are NOT implemented as engine checks (section 6): this is not norm complete.

## 1. Fixed defects (from `en1993-inventory.md` and coordinator list)

| # | Defect | Fix |
|---|---|---|
| 1 | Fatigue assigned to immutable `params` (compile error) | assignment removed, `gmf` used directly |
| 2 | Only one governing ULS row per member | every ULS row is checked (`uls_rows`); ids stay `...<combination>` |
| 3 | No standalone Mz, torsion | `en1993.6.2.5.mz.<row>`, `en1993.6.2.7.t.<row>` (St. Venant share) |
| 4 | LTB interaction used `max(Mb,Mc)` | uses `chi_LT`-based M_b,Rd only (chi_LT = 1 for members not susceptible) |
| 5 | Simplified k_ij without chi | Annex B tables B.1/B.2 (`interaction_annex_b`) and Annex A (`interaction_annex_a`), checks `...intb.<row>`, `...inta.<row>` |
| 6 | Plate reduction dropped the psi term (1-3, 1-5, class 4) | `rho_internal` with psi, threshold `0.5+sqrt(0.085-0.055 psi)` |
| 7 | chi_fi not used, wrong formula, step functions k_y/k_E | linear interpolation of Table 3.1, chi_fi with (lambda-0.2) form and member slenderness, check `en1993.1-2.nfi.<fire>` |
| 8 | Steel temperature with constant c_a = 600, dt 0.5 | temperature-dependent c_a (4.2.5.1), dt 5 s, reproduces E12 within 3 K |
| 9 | Storage/congregation/snow psi wrong | EN 1990 Table A1.1: storage 1.0/0.9/0.8, congregation 0.7/0.7/0.6, snow 0.5/0.2/0, snow>1000 m 0.7/0.5/0.2; obsolete DE-snow test replaced |
| 10 | Pile limit 0.9 fy vs remedy 0.6 fy | named constants, remedy text states the k_red = 1 stress |
| 11 | alpha_v = 0.6 for all bolt classes | alpha_v by class (Table 3.4), thread/shank flag |
| 12 | Directional weld equal share, no 0.9 fu limit | envelope over force angle, both criteria (4.5.3.2) |
| 13 | beta_w S420/S460 = 0.9 | 1.0 (Table 4.1) |
| 14 | Mcr with ad hoc k_g, C1 1.13 for uniform | ECCS closed form with C1 (table), C2 and z_g; uniform moment C1 = 1 |
| 15 | Class from bending limits only; class-4 area ad hoc | Table 5.2 envelope (bending and compression), `section_class_for_action`, effective area/W_eff by EN 1993-1-5 4.4 |
| 16 | Buckling curve S460 collapsed to a0 for all | Table 6.2 S460 column |
| 17 | Shear only max(Vy,Vz) against one direction | separate `...6.2.6.vz`/`...vy`; shear buckling check `en1993.1-5.5.vbw.<row>` when h_w/t_w > 72 eps/eta |
| 18 | Shear-tension interaction, spacing missing | `en1993.1-8.3.6.interaction`, `en1993.1-8.3.5.<e1,min|...>` (Table 3.3) |
| 19 | HSS dummy utilisation, stainless remedy suggested carbon grades | class-dependent M_c,Rd; stainless grade options |
| 20 | LTB remedy dead loops | removed |
| 21 | Hole d0 = d+2 fixed | d0 per EN 1090-2 (d+1/+2/+3) |
| 22 | Oracle covered only 6.2.4/6.3.1 with hard-coded values | new independent oracle `S/🔮️oracles/🧮️clauses/🐍️.py` |

## 2. Implemented catalogue ids

Implemented as tested pure functions (`C/🧮️clauses`) and, where marked E, wired into the engine:

- **1-1 section 2/3/5**: V-001 (factor set, existing `AnnexParams`), V-020 alpha_cr, V-021 amplification, V-022 analysis-mode decision, V-023 sway phi and H_equiv, V-024 Table 5.1 bow ratio and inclusion criterion, V-027 bracing q_d (module `🌐️global`).
- **Classification**: V-030, V-031, V-032 (CHS, angle class 3 test), V-033 (E, envelope), V-034 not implemented (reduced eps), V-040 effective widths internal and outstand, I-section W_eff and e_N (E, class 4 resistance).
- **6.2**: V-060 tension incl. staggered holes, V-061, V-062 (E), V-063 all A_v cases, V-064/V-066 warping torsion by Fourier series (fork supports), V-065 V_pl,T,Rd, V-067 M_y,V,Rd (E), V-068 M_N for I, RHS, CHS, solid, bi-axial exponents (E for I), V-070 class 3/4 interaction functions, V-073 von Mises.
- **1-5**: V-367 shear buckling (k_tau, lambda_w, chi_w rigid/non-rigid, V_bw, V_bf, interaction 7.1) (E, unstiffened web).
- **6.3**: V-100 (E), V-101 N_cr,T and N_cr,TF functions, V-110, V-111 with Table 6.5, kc and f (functions; E uses 6.3.2.2 curves), V-112 6.3.2.4 functions, V-113a ECCS Mcr, V-113b Mcr by Hermite FE for arbitrary M(x), q(x), z_g, supports, springs (function, not yet engine), V-120 Annex A (E), V-121 Annex B (E), V-123 6.3.4 general method (function), V-124 L_m (function), V-126 effective-length nomogram.
- **6.4**: V-131, V-132, V-133 (functions).
- **7**: V-090 limits, V-091 frequency (functions).
- **1-8**: V-400 (alpha_v, long joints, packing), V-401 (all k1/alpha_b variants), V-402 incl. punching, V-403 (E), V-404 with tension relief, V-405 block tearing and angles, V-406 spacings (E), V-407 bolt group, V-410/V-411 welds (E), V-420 T-stub modes, V-431 k5/k10/Sj,ini, V-433 stiffness class, strength class, column web panel, web compression, M_j,Rd summation (functions).
- **1-9**: V-500..V-503 S-N with cut-off, multi-block Miner, shear and combined, rainflow (function).
- **1-2**: V-600..V-605 gas curves, c_a(theta), unprotected and protected temperature, k_y/k_p/k_E, chi_fi, N_b,fi,t,Rd (E), M_fi, kappa factors.
- **1-10 / misc**: V-900 reference temperature and thickness interpolation (function), V-905 Barlow, p_cr, Spangler (functions).

## 3. Tests added

| File | Content |
|---|---|
| `C/🧮️clauses/🧪️tests/🔬️unit/🦀️.rs` | 17 tests: E01 to E15 plus nomogram/torsion/6.3.4 sanity |
| `S/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` | `clause_oracle_matches_rust_clause_functions`, `every_ultimate_row_is_checked_and_minor_axis_bending_is_reported`, `psi_factors_follow_en_1990_table_a1_1` (replaces the DE-snow test) |
| `S/🔮️oracles/🧮️clauses/🐍️.py` | independent stdlib oracle |
| `S/🧪️tests/🧮️clauses/🥒️.feature` | language-agnostic vectors |

Independence of the oracle: chi via quadratic root (not the 1/(Phi+sqrt) form), Mcr by sine-series Galerkin with its own Jacobi solver (the Rust side uses a Hermite FE with Cholesky + Jacobi), four-point rainflow (Rust: ASTM three-point), Heun integration at 1 s (Rust: Euler at 5 s). No third-party Python library is installed on this host (numpy, scipy, sympy, rainflow, sectionproperties all absent, no pip), so the "same output with a third-party library" rule is NOT met; this is a blocker owned by the environment.

## 4. Test results

Standalone validation of `C/🧮️clauses` only (bare `rustc --test` on the nightly toolchain in the scratchpad, std only, no cargo): `17 passed; 0 failed`. This was done because the repo MCP and the first `repo run` were blocked by dependency-lease starvation (35 min wait, then abort) and later by the memory-pressure queue.

Integrated crate: `repo run '@semio-tech/norm-en1993-rs:check'` was started in the background (log `🗑️generated/en1993/check.txt`) and was still queued behind 15 jobs.

## 4b. Second pass (after coordinator course correction)

- New pure modules: `🧊️plates-shells` (1-5 patch loading, shear lag, reduced stress method; 1-6 cylinder axial buckling per Annex D with Q, Janssen, hoop), `❄️cold-stainless` (1-3 f_ya, distortional chi_d and stiffener spring; 1-4 grade table, Ramberg-Osgood, buckling parameters), `🏗️parts-2-6` (2 fatigue lambda, 1-11 cable resistance, 3 tower utilisation, 5 net thickness and sheet pile, 6 crane l_eff and sigma_oz, 1-8 base plate f_jd, c, F_c,Rd). Standalone tests: 19 passed.
- Engine: EN 1993-1-6 check now uses the Annex D alpha_x(Q = 25) and the plateau chi_x.
- Oracle now imports numpy/scipy (repo `.venv`, run through `python_oracle::command()`): `scipy.linalg.eigh` generalized eigenproblem reproduces the stdlib Galerkin Mcr to 1e-15, `scipy.optimize.brentq` on the Ayrton-Perry equation reproduces chi and chi_LT to 1e-12; the Rust Hermite-FE Mcr (uniform moment, UDL, top-flange load z_g = 150 mm) agrees with scipy within 0.5 %. `sectionproperties` and `structuralcodes` are not installed in `.venv` (only numpy, scipy, shapely), so section-constant checks are still missing.
- Build: `:check` is still queued (15 jobs ahead). No compile error has been seen or fixed yet because the compile has not started.

## 4c. Review fixes (review-en1993-en1994.md, saved, not yet compiled in the crate)

- S1-1 T-stub prying: `prying = L_b <= L_b*` in `🧮️clauses/🔗️joint-components`; special-parts tests swapped (L_b 100 prying, 300 none), oracle now carries `jc_prying_at_100/300` independently, E10 added through `t_stub_modes` (F_T,Rd = 245.956 kN, mode 2). Dead duplicate `joints::t_stub/TStub/omega_transverse/fc_wc_rd/m_j_rd/BoltRow` removed; E10 unit and oracle tests use `joint_components`.
- S1-2 bending-shear: `shear_reduced_m_y_rd` uses |V_Ed|, `shear_rho` and `m_y_v_rd_i` (class 1-2), class 3/4 not reduced; the old `M(1-rho)` function is gone. Regression: IPE300 at 0.9 V_pl,Rd gives 126.95 kNm.
- S1-3 section kinds: `buckling_curve_rolled_i` dispatches through Table 6.2 (`rolledI`, `weldedI`, `hss` as cold hollow, S460 column); `section_class_rolled_i` classifies `hss` as RHS (`classify_rhs`).
- S1-4 class 4: b_e2 now lies next to the zero-stress point; girder 1000x8 + 300x20 S355 gives W_eff = 7007 cm3 and e_N = -11.2 mm (test). Still open: ψ iteration and N·e_N feeding (e_N under uniform compression is 0 for doubly symmetric I, so ΔM = 0 there; hollow/mono-symmetric sections still use crude factors).
- S2: fatigue category is the Δσ_C value for any category (no 71 MPa default); μ0 >= 1 gives θ_cr = 20 °C; class 4 uses θ_cr = 350 °C; `kappa_1` is an enum (all sides 1.0, unprotected 3 sides 0.70, protected 3 sides 0.85).
- Requests for the shared compliance crate (negative capacity passes) and en1990 (combination layer) were appended to `📌️important/requests.md`.
- Open from the review: joint group effects and per-row column T-stub (special-parts wiring), k10 per row vs per bolt, bolt β_Lf/β_p and γ_M3,ser slip, the γ_Mf 2x2 matrix (needs field-meta and fixture changes), legacy tower/pile/silo formulas, shear-buckling interaction duplicate, class 3 signed stress sum, Annex A/B wiring items (b)-(e), fire parse_rating and k_0.2p.
- Standalone check of all `🧮️clauses` modules (including special-parts): 20 unit tests passed.

## 4d. Catalogue constants and crate split (2026-11-02)

- S1 from `oracle-third-party.md`: HEB 240/260/280/300 `it` (27-37 % too small) and `iw` (10x too large) replaced by the sectionproperties values (it 103.7/125.9/145.4/187.7 cm4, iw 4.762e-7/7.362e-7/1.107e-6/1.651e-6 m6), HEB 240 W_pl,z 4.985e-4 m3 (was 5.01e-4). M_cr of an HEB 240 over 6 m (C1 = 1) is now 501.6 kNm (was 867.6). All 169 fixtures and assets (mutation vectors, example snapshots, `.dsl.semio` rows) were hand-updated; the fixture HEB 260 `it` 8.53e-7 became 1.259e-6. The only section catalogue in the repo is the four HEB rows; other kinds (IPE, RHS, CHS, channels, angles) have no stored table and are computed by formulas.
- New tests: `blessed_third_party_cases_feed_the_evaluator` feeds the inputs of `third-party.json` (sections, M_cr, N_pl, M_c, V_pl, A_v, M_y,V, M_N, M_b, alpha_cr) to the Rust evaluator and compares with `expected` under the case profile; `heb240_catalogue_torsion_and_warping_give_the_fe_critical_moment`.
- Crate split: `-checks` exceeded the Cargo source caps (32 files, 262144 bytes, 4000 active lines). New crates `semio-s-artifact-norm-en1993-clauses` (`🧮️clauses`) and `semio-s-artifact-norm-en1993-parts` (`📏️parts`, `AnnexParams`); `-checks` re-exports both (12 files, 233 KB, 3767 active lines, 233 lines of headroom).
- Build history: runs 0-2 infrastructure aborts, run 3 refused by the source boundary (fixed by the split), run 4 SIGKILL in graph planning, run 5 stopped when builds were put on hold. No rustc diagnostics exist yet for the artifact.

## 5. Spec deviations found while testing

- E06 NA-DE values: the spec lists eq. 6.61 = 0.4552 and 6.62 = 0.6442. With gamma_M1 = 1.1 also inside n = NEd/(chi NRk/gM1) in k_yy/k_zy (as the standard requires) the independent oracle and the Rust code give 0.4565 and 0.6418. The spec values reused the EN k factors. Tests use 0.4565/0.6418.
- E12 theta_a: oracle (Heun, dt 1 s) 239.3/491.7/645.9 versus spec 237.9/491.5/646.3: within the stated +-3 K.

## 6. Remaining gaps (not empty)

Blocked on schema work (new entities and twins: frame/supports/restraints, parametric sections, joint geometry, load stations) or not yet written:

- Engine wiring for V-020..V-027 (needs frame/storey entity), V-113b/V-123/V-124 (functions exist; need support/restraint entities), V-130..V-133 (need built-up member entity), V-090 with real load patterns (SLS still uniform-load equivalent).
- 1-8: component method as engine checks (T-stub, stiffness, M_j,Rd functions exist; need geometric joint entity), hollow section joints V-440.., weld groups.
- 1-3 (V-250..V-260: effective section with edge stiffeners, distortional buckling, purlin), 1-4 (V-300..V-305: stainless table, Ramberg-Osgood, buckling), 1-5 (V-360..V-376 other than V-367: shear lag, stiffened plates, reduced stress method), 1-6 shells V-800..V-806, 3-1/3-2 V-850.., 1-11 V-860.., 5 V-870.., 2 V-880.., 6 V-890..: only the existing simplified checks remain; unchanged by this pass.
- Fire and fatigue models still use scalar entities; rainflow/combined-stress/shear fatigue are functions only.
- 1-10 temperature still fixed at 0 deg C in the engine check; Table 2.1 data table not encoded (function takes a table).
- Third-party validation (see section 3), `.feature` has no step adapter wired into the test platform yet.
- Annex A/B class 4 e_N shift (delta M) is 0 in the engine.
- No schema twins, field-meta or fixtures were changed (no new snapshot fields), so wire-twin tests are unaffected.

## 7. Unverified normative values

- Table 6.2 hot-finished hollow: S420 uses curve a (spec text says a0 for S420 and S460; EN text: a0 only S460). Encoded: a below fy 460, a0 from 460.
- CHS M_N,Rd exponent 1.7 (ECCS form), flagged in code.
- Table B.3 C_m rows for transverse loading (`cm_alpha_s`, `cm_alpha_h`) from memory; engine uses only the linear row, 1.0 (uniform) and 0.95 (UDL, zero end moments).
- Annex A factors as transcribed in the spec ([verify] in spec); only used when lambda_0 above the C1 limit.
- kc values UDL 0.94 / point 0.86 (spec), 0.90/0.91 fixed-end not encoded.
- Table 3.1 (EN 1993-1-2) k_p column and k_sh = 1 in engine fire check (box value not in model).
- E_s = 210000 for stainless not applied; Spangler formula; plate 1-7 coefficient k; fatigue shear cut-off 0.457 Delta tau_C.
- NA-DE: kb, lambda_c0 = 0.5, f with kc as in EN, Mcr for trapezoidal sheeting: not implemented.
- Brittle fracture Table 2.1 thickness data: not encoded (no verified source in repo).
