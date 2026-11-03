# EN 1993 Special Parts (second norm agent)

`C = 🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🧮️clauses` under `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993`. All modules are new siblings in `C`; the only shared edit is the module registration in `C/🦀️.rs` (appended `pub mod` lines and `special_parts_tests`). No snapshot, schema, field-meta or engine file was touched, so no twin changes.

## Implemented (pure per-entity functions, mm/N/MPa)

| Module | Catalogue | Content |
|---|---|---|
| `🥫️cold-formed` | 1-3 V-250..V-260 | lip k_sigma, iterated distortional buckling of an edge-stiffened flange (A_s, I_s, K, sigma_cr,s, chi_d, convergence), reduced stiffener, lambda_w/f_bv/V_b,Rd, web crippling R_w,Rd (cat. 1/2), M+F interaction, flange curling, spring series for sheeting |
| `🔬️stainless` | 1-4 V-300..V-305 | rho internal/outstand (cold/welded, thresholds as exact root of the curve), chi flexural/LTB/torsional by member kind, k_y/k_LT, member interaction, secant modulus, fire factors (grade 1.4301) |
| `🧩️plated-ext` | 1-5 V-360..V-376 | sigma_E, alpha_e, chi_c, rho_c interpolation (4.5), shear lag hogging/end/ULS with plate buckling, k_F types a/b/c and longitudinal stiffener, patch loading l_y/chi_F/F_Rd for a/b/c, F+M, k_tau with longitudinal stiffeners, M-V-N interaction, I_st,min, torsion criterion, flange-induced buckling, alpha_ult,k, reduced stress rho and utilisation |
| `🫙️shell-stability` | 1-6 V-800..V-802 | Q classes A/B/C, omega, C_x, sigma_x/theta/tau,Rcr, generic chi(lambda, alpha, beta, eta, lambda0), LS3 design stresses (axial, hoop, shear), LS3 interaction exponents, LS1 von Mises |
| `🌡️toughness` | 1-10 V-900 | Table 2.1 columns (S235/S355, 0.75 f_y), T_Ed, interpolation, `None` outside table (fail-safe) |
| `🪢️tension-components` | 1-11 V-860/861 | rope/strand breaking force, F_Rd, Ernst modulus, sag, threaded rod, pin shear/bearing/bending/combined/serviceability |
| `🔗️joint-components` | 1-8 V-420..V-440 | l_eff (column flange single/group, end plate extended/first-below/inner), T-stub modes 1/2/3 with methods 1 and 2, L_b* and prying, k_wc, b_eff,c,wc, rho_web, omega Table 6.3, F_c,wc,Rd, F_t,wc,Rd, V_wp,Rd (stiffened), top-down row distribution with group limits, M_j,Rd, z/k_eq, k1..k5/k10, S_j,ini, mu, S_j, M-phi curve, frame stiffness and strength classification, base plate M_Rd (Table 6.7 as plastic equilibrium), k13/k15/k16, S_j,ini base, f_jd with concrete confinement |
| `🌉️bridge-fatigue` | 2 V-880 | lambda_2/3/4, lambda_1 interpolation, Delta sigma_E,2, utilisation, category ladder, orthotropic details, size effect |
| `🗼️towers-masts` | 3-1/3-2 V-850/851 | lattice lambda_eff, v_crit, Scruton, amplitude, cantilever frequency, vortex cycles, ovalling, guy stiffness |
| `🏺️containment` | 4-1/4-2/4-3 V-803/804/905 | Janssen p_h/p_w/p_v/n_x, tank course thickness, pipe wall, restrained stress, von Mises, p_cr |
| `🧷️sheet-piling` | 5 V-870 | corrosion allowance (4 environments), reduced properties, M_c,Rd, V_pl,Rd, shear reduction, interlock shear |
| `🚜️crane-runway` | 6 V-890 | phi_2, wheel load, I_rf, l_eff, sigma_oz, tau_oxz, torsion, von Mises web, Hertz, lambda_crane, fatigue utilisation |

## Tests and oracle

- `C/🧪️tests/🔭️special-parts/🦀️.rs`: `sp01..sp06` unit tests (worked and boundary values) and `sp07_special_parts_match_independent_oracle` (about 120 quantities).
- Oracle `S/🔮️oracles/🧮️special-parts/🐍️.py` (numpy/scipy, run through `python_oracle::command()`): independent formulations (brentq for chi and rho thresholds, scipy `fixed_point` for the distortional iteration, `linprog` for the base plate Table 6.7, `quad` for the Janssen wall force, `np.interp` for tables). Rust and oracle agree within 1e-6 (iteration) and 1e-9 .. 1e-12 elsewhere.
- Result: standalone `rustc --test` of the whole `🧮️clauses` module (nightly-2026-07-20, scratchpad harness with stubs only for `serde_json` and `python_oracle`): `26 passed; 0 failed` (19 existing + 7 new), warning-free. The integrated crate build is owned by the primary agent ("ready for rebuild" appended to `build-en1993/latest.md`).
- Not done: third-party library validation for the stainless/shell/joint formulas beyond scipy numerics (no `structuralcodes`/`sectionproperties` in `.venv`); `.feature` vectors for the new quantities (the primary owns `🧪️tests/🧮️clauses/🥒️.feature`; quantities are listed by the `sp07` keys).

## Wiring

See `📌️important/requests.md` section "to: primary EN 1993 norm agent (from: en1993 special-parts agent)" for a function-by-function table with proposed check ids and every new snapshot field required.

## Unverified normative values

- 1-3: lip k_sigma (0.5 .. 0.5 + 0.83 cuberoot((c/b - 0.35)^2)), flange k_sigma = 4 in the iteration, K with kf/b2 as input, web crippling alpha 0.075/0.15 and l_a = 10 mm / s_s, f_bv ranges, flange curling formula.
- 1-4: rho formulas and constants 0.125/0.079/0.231/0.242, alpha/lambda0 (0.49/0.40, 0.49/0.20, LTB 0.34/0.76, torsional 0.34/0.20), k_y/k_LT forms, fire factors of 1.4301 (EN 1993-1-2 Table C.1 recalled from memory); other grades return `None`.
- 1-5: beta hogging, beta_ult = beta^kappa, type b/c l_y expressions, k_tau,sl, I_st,min.
- 1-6: alpha_theta 0.75/0.65/0.50 per class, alpha_tau 0.8, lambda0 = 0.4 for hoop and shear, interaction exponents, C_x thresholds (1.7, 0.5 r/t).
- 1-10 Table 2.1: only the sigma_Ed = 0.75 f_y(t) columns of S235 (JR, J0, J2) and S355 (JR, J0, J2, K2), recalled from memory; stress levels 0.5/0.25 f_y and grades S420..S690 are not encoded.
- 1-8: alpha (Fig. 6.11) is an input; k4/k5 = 0.9 vs 0.85 in the repo; psi = 2.7; L_b* factor 8.8.
- 2: orthotropic detail categories (125, 36, 71, 71, 90) and lambda_1 curves (not embedded; the caller passes the NA table).
- 3: lattice lambda_eff coefficients (0.9, 0.35 + 0.7 lambda, 0.7 lambda), ovalling frequency expression.
- 4/5/6: tank courses use the plain hydrostatic membrane (no one-foot or variable-point refinement); Table 4-1 corrosion losses (soil environments) recalled from memory; lambda_crane (nu k_Q)^(1/m); tau_oxz = 0.2 sigma_oz.

## Remaining gaps (blocked or not covered)

- Engine wiring (primary agent): needs the snapshot entities listed in the requests; none exist yet.
- 1-5 Annex A k_sigma,p for orthotropic plates and Annex D corrugated webs: not implemented (chart based); 1-3 intermediate stiffeners (5.5.3.3), fastener resistances (section 8) and the purlin/sheeting restraint stiffness derivation (10.1.5) are only represented by the series spring helpers.
- 1-6: external pressure/wind buckling of stiffened tanks, rings and wind girders; GMNIA/LBA calibration; 4-1 hoppers and discharge eccentricity patch loads.
- 1-8: hollow section joints (V-450, Table 7.x), weld group torsion (V-412), pure rotation capacity phi_Cd (only classification and curve), alpha(lambda1, lambda2) chart.
- 2: embedded lambda_1 road/rail curves and rail traffic lambda factors; 3-1: exact Annex G/H eccentric angle rules and ice/wind loads; 6: local flange stress sigma_ox and fatigue detail table of EN 1993-6.
- 1-4: grades other than 1.4301 in the fire table.
