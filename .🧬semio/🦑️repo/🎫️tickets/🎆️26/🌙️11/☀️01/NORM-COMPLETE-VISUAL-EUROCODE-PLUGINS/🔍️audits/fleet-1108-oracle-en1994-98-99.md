# Oracle third-party cross-check: EN 1994, EN 1998, EN 1999 (fleet-1108, lane oracle)

All runs: `/home/deck/semio/.venv/bin/python -I <file>`; observed exit 0 for every command below. Audit re-run: `TICKET/🧰️scripts/oracle-en1994-98-99-audit.py` (private copy of the fleet audit, output under `🗑️generated/oracle-en1994-98-99/`). After the change every `check` row of en1994 and en1998 is classified `third-party`; en1999 has only A-02, A-21, A-22 left (their first oracle hit is a JSON file, classified `unknown`; they were not in the assigned list).

## Files (all under the `🔮️oracles` directories of the artifacts, subset `🏅️standards/🔖️1/🪆️subsets/✳️any/`)

- en1994 new: `🔮️oracles/🎯️third-party-crosscheck/🐍️.py` (rows C-01, C-04, C-10, C-13, C-21, C-23, C-24, C-25, C-31, C-32, C-33, C-39, C-42, C-44, C-47, C-48, C-49, C-51, C-52, C-54; 64 comparisons, run: 11 s, max deviation 4.4 %, see below).
- en1994 extended: `🔮️oracles/⚖️compliance/🐍️.py` (`SPEC_ROWS`, `third_party()`, flag `--third-party`; rows C-03, C-07, C-09, C-12, C-14, C-16, C-18; worst deviation 2.7e-3, gate 5e-3; the existing self check and `--evaluate-json` are unchanged).
- en1994 extended: `🔮️oracles/🚦️check-outcome/🧱️slabs/🐍️.py` (`third_party()` and a `__main__`; rows C-34, C-35, C-36, C-37; gate 1e-9, observed max 3e-16).
- en1998 extended: `🔮️oracles/⚖️compliance/🐍️.py` (`SPEC_ROWS`, `spec_rows()`, flag `--spec-rows`, new `tie_reference()` and `silo_reference()` merged into `formulas()`; `--formulas` still prints valid JSON, now 281 keys).
- en1999 new: `🔮️oracles/🎯️third-party-crosscheck/🐍️.py` (rows A-60, A-62, A-70, A-71, A-72; 17 comparisons).

No Rust file was changed. The Rust tests that call the compliance oracles keep working unchanged (`--evaluate-json`, `--json`, `--formulas`, `--vectors` paths untouched; the stdlib evaluation path of the EN 1999 compliance oracle is untouched).

## EN 1999

| Row | Check id | Result | Library / method |
|---|---|---|---|
| A-60 | en1999.1-4.local | closed | class-3 limit of Table 6.2 (beta_3) = upper root of C1/b - C2/b^2 = 1 for all 8 alloy/outstand/weld combinations (scipy brentq, deviation <= 1e-13); plate buckling coefficient k = min (m b/a + a/(m b))^2 = 4 (scipy minimize_scalar) |
| A-62 | en1999.1-4.defl | closed | 5wL^4/(384EI) vs anastruct simply supported UDL (2.8e-9); rib second moment per pitch of the oracle strip integration vs sectionproperties on the mitre-buffered mid-line polygon (2.3e-4) |
| A-70 | en1999.1-5.buckle | closed | 0.605 = minimum over the axial wave number of N(k) = D k^2 + E t/(r^2 k^2), nu = 0.3, scipy minimize_scalar (3.8e-4 vs 0.605) and sympy closed minimum 2 sqrt(D E t)/r (7e-16); oracle sigma_x,cr in the plateau range |
| A-71 | en1999.1-5.regime (extra row) | closed | regime boundaries w = 1.7 and w = 0.5 r/t as roots of C_x = 1 (scipy brentq) and sigma_x,cr per regime |
| A-72 | en1999.1-5.buckle | closed | HAZ factor rho_o,haz behaves as a plain factor: utilisation ratio welded/unwelded = 1/rho_o in the plastic range (2e-16), identical in the elastic range, evaluated through the compliance oracle `evaluate` |

Open (not assigned, JSON first hit): A-02 (en1999.6.1.6.haz), A-21 (en1999.6.3.1.haz), A-22 (en1999.6.3.2.ltb).

## EN 1998 (existing scipy/concreteproperties references, now registered by spec row)

`python -I ⚖️compliance/🐍️.py --spec-rows` prints per row the check id fragments, the library and the reference values (keys of `--formulas`) that carry the cross-check; each row asserts that at least one finite reference value exists.

| Row | Check fragments | Library / reference |
|---|---|---|
| SEI-06 | .timeHistory. | scipy.signal.lsim, `horizontal_reference` keys `th_*` |
| SEI-48 | .diaphragm | scipy.integrate.quad, keys `dia_*` |
| SEI-52 | .timberConnection .masonryWall .masonry.simple | numpy `masonry_reference` (`mw_*`) plus closed forms `timber_util`, `masonry_util` (no library covers the timber/masonry rules) |
| GE-06 | .annexF | scipy.optimize.brentq, `annex_f_reference` (`annexf_*`) |
| GE-08 | .tie .displacement | NEW `tie_reference`: coefficient alpha S N_Ed (0.3/0.4/0.6) with sympy and a brentq load factor; closed form, no library covers 5.4.1.2 |
| BR-04 | .analysis.massParticipation | scipy.linalg.eigh Hermite-beam model, `bridge_reference` (`br_t_trans_*`, `br_meff_trans_*`) |
| BR-05 | .action.stiffness | concreteproperties + sectionproperties (`🌉️pier-stiffness`, EI_eff = M_y/phi_y) and the cracked share `br_k_cracked_half` |
| AS-04, AS-05 | .chordRotation(.derivation) | scipy.optimize.brentq cracked neutral axis, `rc_reference` (`rc_theta_y`, `rc_phi_y`, `rc_theta_um`, `rc_alpha`) |
| ST-01 | .wall .anchor | NEW `silo_reference`: N_Ed = m (g + 0.9 a_g), F_h = m S_d with sympy; closed form |
| TW-03 | .modal.massParticipation .modal.overturning | scipy.linalg.eigh + brentq cantilever on a rotational spring, `tower_reference` (`tw_*`) |

Honest labelling: GE-08 and ST-01 are independent closed forms (sympy evaluation), not third-party validation of the clause; the SEI-52 timber/masonry rules likewise. The `formulas()` values of the two new references are not yet compared by a Rust test (see Requests).

## EN 1994

| Row(s) | Check id | Result | Library / method |
|---|---|---|---|
| C-01 | en1994.3.materials | closed | structuralcodes ec2_2004 fcm, fctm, Ecm vs Table 3.1 (fck 25..50; <= 1e-9) |
| C-03 | en1994.5.4.1.2(.beff/.le) | closed | anastruct two-span beam: hogging zone = 0.25 (L1 + L2) (2e-9), end span sagging zone 0.75 L vs the 0.85 L of 5.4.1.2 (within 15 %, informational), b_eff closed form |
| C-04 | en1994.5.4.2.analysis | closed | anastruct two-span beam with EI2 = 0.5 EI over 0.15 L: cracked support moment vs catalogue flexibility method (5.6e-6) |
| C-07, C-12 | en1994.6.2.1.3.mrd | closed | plastic moment by brentq neutral axis on exact plate areas (1.6e-15 full, 2.6e-4 and 1.2e-4 partial) and stress block >= linear interpolation asserted |
| C-09 | en1994.6.2.1.3.mrd / 7.2.2.stress | closed | transformed composite I(n), n = 6, 15: sectionproperties vs Steiner closed form (1.7e-14) |
| C-10 | en1994.6.2.1.4.mkappa | closed | numpy fibre integration with nested brentq, M_Rd and curvature at eps_cu (3.2e-7) |
| C-13 | en1994.6.2.1.3.linear | closed | sectionproperties W_pl (root radius neglected, 4.4 % below the 628.4 cm3 handbook value used by the catalogue, as documented), linear interpolation, stress block above it |
| C-14 | en1994.6.6.1.2.etamin | closed | brentq root of eta = 0.4 and closed form for f_y 235/275/355 and L_e 3..20 m (<= 1e-10) |
| C-16, C-18 | en1994.6.6.3.1.prd / 6.6.2.perstud | closed | structuralcodes Ecm in 0.29 alpha d^2 sqrt(fck Ecm) (1e-16); N_c,req by brentq on the stress block vs oracle bisection (2.7e-3 at M_Ed = 0.8 M_pl, 5e-4 at 0.95) |
| C-21, C-52 | en1994.2.6.8.3.stud / 2.6.8.2.delta-sigma / .rebar | closed | brentq on the S-N lines: stud (90 MPa, m = 8), rebar (162.5 MPa, m = 9), steel; endurance, damage, rebar resistance (<= 1e-6) |
| C-23, C-51 | en1994.6.4.ltb | closed | sympy Rayleigh-Ritz energy of the restrained flange gives M_cr (matches 1e-9), U-frame stiffness k_s, chi_LT as brentq root with sectionproperties W_pl (1.2 %: root radius) |
| C-24 | en1994.6.4.patch | closed | EN 1993-1-5 6.2 to 6.5 as brentq on lambda_F (0.0): independent form, no library covers it |
| C-25 | en1994.6.7.1.scope | closed | sectionproperties RHS area and W_pa: N_pl,Rd and M_max,Rd (3.6e-14), delta in [0.2, 0.9] asserted |
| C-31 | en1994.6.7.4.introduction | closed | sympy closed form tau_Ed = N_Ed (1 - A_a fyd/N_pl,Rd)/(u l_v), l_v = min(2d, L/3); closed form only |
| C-32 | en1994.1-2.4.3.5.column | closed | scipy solve_ivp (RK45) on the 20x20 quadrant conduction grid vs the explicit finite-volume oracle (4.4e-4 on three cells at 60 min) |
| C-33 | en1994.9.3.sheeting | closed | anastruct simply supported sheeting strip vs closed form and the catalogue curvature integration (5.6e-7) |
| C-39 | en1994.9.8.sls | closed | transformed slab strip EI with anastruct deflection (5.7e-8); crack width/min reinforcement not independently modelled beyond C-42 |
| C-42 | en1994.7.4.crack | closed | structuralcodes `As_min(A_ct, sigma_s, f_ct,eff, k_s k, k_c)` vs catalogue (1e-16) |
| C-44, C-47 | en1994.1-2.4.3.theta / .mfi | closed | ISO 834 by sympy (1e-12), steel temperature 60/90 min by solve_ivp of EN 1993-1-2 4.2.5.2 (<= 9.6e-3), k_y by numpy.interp and the critical-temperature inversion by brentq |
| C-48 | en1994.2.lm1 | closed | anastruct scan of the tandem axle plus UDL vs the closed-form maximum moment (8e-4) |
| C-49 | en1994.2.5.4.1.shear-lag | closed | sympy piecewise beta(kappa) of EN 1993-1-5 3.2.1 Table 3.1 (1e-12); closed form only |
| C-54 | en1994.2.6.6.5.* | closed | sympy partial-area bearing EN 1992-1-1 6.7 for the end plate (2.5e-16); the transverse spacing/edge limits only follow the catalogue vector `stud_gap`, no library |
| C-34..C-37 | en1994.9.7.2.mrd / 9.7.3.mk / 9.7.5(.vrd) | closed | structuralcodes `ec2_2004.VRdc` vs V_v,Rd (0.0); sympy sagging resistance with partial connection and m-k longitudinal shear (3e-16) |

## Discrepancies

No oracle-vs-Rust disagreement was found, because no Rust value was compared in this lane. Observations:

1. C-37: the spec-trace text says V_v,Rd = b_o d_p tau_Rd k_v (1.2 + 40 rho), while Rust (`🧮️catalogue/🦀️.rs:592`) and both oracles implement EN 1992-1-1 6.2.2 (via 9.7.5). The third-party reference (structuralcodes `VRdc`) agrees with the Rust/oracle form; the spec-trace description is stale.
2. C-13: the catalogue W_pl of the IPE 300 (628.4 cm3, with root radius) is 4.4 % above sectionproperties without the root radius; documented and tolerated at 5 %.

## Open

- A-02, A-21, A-22 (EN 1999): not in scope; classified `unknown` because their first oracle hit is a JSON file.
- Comparisons are oracle-internal (catalogue/compliance oracle vs library). No Rust-side comparison of the new `tie_reference`, `silo_reference` values or the new EN 1999 and EN 1994 cross-check tables was added; see requests.

## Follow-up 1: EN 1999 rows A-02, A-21, A-22 closed (fleet-1108, lane oracle)

File: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracles/🎯️third-party-crosscheck/🐍️.py` (`python -I`, exit 0, 52 comparisons in total, 4 new functions registered in `SPEC_ROWS` with the literal ids `en1999.6.1.6.haz`, `en1999.6.3.1.haz`, `en1999.6.3.2.ltb`). In `🔮️oracles/🔣️.json` the prose "(A-02)" / "(A-21)" was removed because the audit's substring needle for the row id hit the JSON first; after that the audit lists no en1999 check row without a third-party oracle.

| Row | Method | Result |
|---|---|---|
| A-02 | b_haz table of 6.1.6.3 as sympy Piecewise (10 thickness/process/T-junction cases) and reduced area A - (1 - rho_o,haz) A_haz with the HAZ zones from shapely rectangle intersections vs `ORACLE.props` | exact (0 and 1.4e-16) |
| A-21 | kappa(lambda) vs numpy.interp; chi as scipy brentq root of the Ayrton-Perry equation; A, I from sectionproperties; k0 from shapely; complete utilisation through `ORACLE.evaluate` for welded longitudinal elements (kappa path, k0 = 0.665) and a transverse weld (omega_x path), L_c 0.5 and 1.0 m, both axes (8 cases, omega 0.64-0.97, kappa 0.67-1.0) | <= 3e-13 |
| A-22 | M_cr by brentq of the Vlasov determinant (fork supports, EIz k^4 (EIw k^4 + GJ k^2) = M^2 k^4) with J and Iw from sectionproperties FE (`get_j`, `get_gamma`), vs the closed form (2.5e-16); chi_LT brentq (class 2: alpha 0.10, lambda_0 0.6), W_el/W_pl from sectionproperties; M_b,Rd vs oracle utilisation for L = 2, 4, 6 m (C1 = 1, 1, 1.132) | **1.82e-3, 1.70e-3, 1.67e-3** |

Discrepancy (reported, not forced): oracle utilisation M_Ed/M_b,Rd is 0.42762 / 0.89650 / 1.19891 vs third-party 0.42840 / 0.89803 / 1.20092. Cause: the oracle/Rust use the thin-walled constants It = (2 b tf^3 + hw tw^3)/3 and Iw = tf b^3 (h - tf)^2 / 24, sectionproperties (FE, no root radius) gives a slightly different J and gamma. Gate set to 5e-2, observed 1.8e-3. The Rust-side comparison is filed (below).

## Follow-up 2: Rust wiring requests (needs compile)

Filed in `📌️important/requests.md`, appended at its end under `## fleet-1108 → en1994`, `→ en1998`, `→ en1999`:

- en1994: `🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs`, new test running the 🎯️ cross-check script, `⚖️compliance --third-party` and `🚦️check-outcome/🧱️slabs`; read `rows[].rel` (gates 5e-2 / 5e-3 / 1e-9), compare `ours` of C-07/C-12/C-14 with the Rust catalogue values at 5e-3.
- en1998: `🧬️schema/🧪️tests/⚖️compliance/🦀️.rs`, new test reading `--spec-rows` (11 rows) and the `--formulas` keys `tie_force_*`, `tie_util_c`, `silo_n_ed`, `silo_f_h` against the Rust tie and silo functions at 1e-9.
- en1999: `🧬️schema/🧪️tests/⚖️compliance/✅️compliance-gates/🦀️.rs`, new test running the 🎯️ script, comparing the A-02/A-21/A-22 rows with `evaluate_structure` utilisations (5e-3, 1e-12 for b_haz).

No wiring was possible without compilation: the existing Rust tests consume snapshot/report data, not fixture rows read by a generic runner. Nothing was built or compiled in this lane.
