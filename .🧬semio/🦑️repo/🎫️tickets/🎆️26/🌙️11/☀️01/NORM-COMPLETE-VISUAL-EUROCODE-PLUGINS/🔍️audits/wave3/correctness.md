# Wave 3 correctness review (W3-CORRECTNESS)

Independent re-derivation of the safety-relevant kernels written in wave 2. Method: read the Rust, recompute by hand or by a separate Python calculation (stdlib, scratch scripts in the session scratchpad), compare with the code. The repo carries no printed standard text (only the spec-trace summaries and the target specs), so clause statements below are from the reviewer's knowledge of the Eurocodes; where that is not certain the item is "Unverifiable". Nothing was compiled or run in Rust (rule). No freeze was active at any edit.

Legend: OK, Issue (fixed = changed by me with a test, open = routed in `📌️important/requests.md`), Unverifiable.

## Summary table

| # | Item | Verdict |
|---|---|---|
| 1 | EN 1993 sway: m of alpha_m over all storeys | Issue, fixed |
| 1b | EN 1993 sway: phi0, alpha_h, alpha_m, phi*V, exemption 0.15, direction, bow load | OK |
| 1c | EN 1993 5.2.2(5)B amplification | OK (conservative) |
| 2 | Base plate M_j,Rd lever convention | Issue, fixed |
| 3 | Base plate c-strip l_eff = plate width | Issue, open |
| 4 | T-stub modes, L_b*, n, Table 6.4 lengths | OK |
| 5 | CHS/angle: curves, classes, chi | OK |
| 6 | Toughness lookup logic | OK; cell values Unverifiable |
| 7 | EN 1994 V-M rho and re-solve | OK |
| 7b | EN 1994 tension stiffening, fatigue reading | OK as arithmetic; factor Unverifiable |
| 8 | EN 1997 characteristic values from CPT (n counts readings) | Issue, open |
| 8b | EN 1997 M2 earth pressure | OK |
| 9 | EN 1992 punching: beta (all rules), W1, v_Rd,cs, openings, stud rows | OK (readings flagged unverified by owner) |
| 10 | Shared fastening cone kernel | OK |
| 11 | EN 1998 design base shear, spectrum, lambda | OK |
| 12 | MO active/passive, water, sliding | OK |
| 13 | Tie beams, slopes (Bishop, pseudo-static) | OK (coefficients Unverifiable) |
| 14 | EN 1998-2 pier EI_eff | OK |
| 15 | EN 1998-3 theta_y (A.10b), theta_um (A.1) | OK |
| 16 | EN 1996 concentrated loads beta | OK |
| 16b | EN 1996 chase tables | OK against recollection |
| 17 | EN 1991 crane test loads | Issue (suspected swap), open |
| 17b | EN 1991 hoist phi_2/phi_3, rail Phi, footbridge q_fk | OK |
| 17c | EN 1991 footbridge walking/Q_flk | Issue (low), open |

## 1. EN 1993 sway imperfection (v21)

File: `🔩️en1993/…/⚖️checks/🕸️frame-envelope/🦀️.rs` (`sway_imperfection`), `🧮️clauses/🌐️global/🦀️.rs`.

OK (hand): phi = phi0 alpha_h alpha_m with phi0 = 1/200, alpha_h = 2/sqrt(h) in [2/3, 1], alpha_m = sqrt(0.5 (1 + 1/m)). Portal h = 4 m, m = 2: 0.005 * 1.0 * 0.866025 = 4.33013e-3, force phi*V = 866.03 N for V = 200 kN per eaves node (matches the unit test). Exemption H_Ed >= 0.15 V_Ed at and above the storey top: index mapping `exempt_below` checked (storey i belongs to level i+1, node at level L looks at storeys[idx(L)-1]). Unfavourable direction: both signs solved, larger sum of second-order peak moments kept: acceptable. Bow imperfection in the cross-section row only (8 N e0/L^2), not double counted with 6.3: consistent with 5.3.2(6).

**Issue 1 (fixed)**: `m` was the number of compressed columns with N >= 0.5 mean over ALL columns of ALL storeys. EN 1993-1-1 5.3.2(3)b: m is the number of columns in a row (per storey; the 50 % rule against the average of that plane). Pooling storeys gives m = 7..8 for a two-storey three-bay frame instead of 4 and lowers alpha_m (0.75 instead of 0.79; 3 storeys of 2 columns: 0.764 instead of 0.866, 12 % less sway force): unconservative.
Fix: `columns_in_a_row(&[(lower level, N)])` groups the compressed columns by their lower node level, applies the 50 % rule per row and takes the smallest row count (largest alpha_m, conservative for all storeys, scalar phi kept so consumers are unchanged). Test `m_counts_the_columns_of_a_row_per_storey_and_the_smallest_row_governs` (hand: ground row 4, upper row with one lightly loaded column 3, pooled 3 versus the old pool 7; alpha_m(3) = 0.8164966, alpha_m(7) lower). The portal test (m = 2, one storey) is unchanged.
Open refinement (not a defect): the standard allows a phi per storey from its own m; the scalar uses the governing one.

**1c OK**: `amplified_profile` multiplies V and M of the whole first-order course by 1/(1 - 1/alpha_cr) for 3 <= alpha_cr < 10 (5.2.2(5)B and 5.2.1(3)); the standard amplifies only the sway part, so multiplying everything is conservative; N unchanged.

## 2. Base plate M_j,Rd (v14)

Files: `🧮️clauses/🔗️joint-components/🦀️.rs` `base_plate_m_rd`, callers `🔗️special-checks/🧩️plate-checks` (mjrd) and `🔭️special-parts-checks` (`base_plate`), anchor statics `base_forces`.

`base_forces` solves T - C = N, M = T z_l + C z_r, i.e. T = (M + N z_r)/(z_l + z_r): tension side lever = `baseLeverLeft`, compression side = `baseLeverRight` (also the remedy text "lever arm of the compression side" for the right lever). `base_plate_m_rd` evaluated the mirrored arrangement (left compression, `n z_r - lo (z_l + z_r)`), i.e. treated z_r as the tension lever.
Hand check with asymmetric levers z_l = 200 mm, z_r = 400 mm, F_t = 150 kN, F_c = 400 kN each side: N = +100 kN: T = 150, C = T - N = 50, M = 150*200 + 50*400 = 50.0 kNm (old code 70.0: 100*400 + 50*600); N = -300 kN: T = 100, C = 400, M = 100*200 + 400*400 = 180 kNm (old code 120.0 kNm, 33 % too low; the error is N (z_r - z_l) in one sense or the other). All committed fixtures have z_l = z_r (checked every fixture with a base moment), so the python oracles and rows never saw it; the oracle formulas were written with the same swap (linprog objective [zl, -zr]).
**Fix**: `base_plate_m_rd` returns `hi (z_l + z_r) - N z_r` (tension left); callers unchanged. Symmetric results identical (60.0 and 150.0 kNm for the rows a and b). Added: unit assertions with the hand values 50e6 and 180e6, oracle rows `jc_bp_c`, `jc_bp_d` (scipy linprog with objective [-zl, zr], independent of the closed form) plus adapter arms and two feature rows `special-parts-jc-bp-c/d-en-1993-1-8`; check-outcome oracle line for `mjrd` adapted.
Not run: the special-parts oracle through `heavy.sh` timed out in the queue (the gate was busy); the two new expected values are verified by hand and by a vertex enumeration (50 000 000 and 180 000 000 N mm). Please let the next build run the special-parts rows.

## 3. Base plate c-strip, T-stub, cone (v14)

- **Issue 3 (open)**: `base_bearing` / `compression_strip` use `F_c,Rd = f_jd * b_eff * l_eff` with `b_eff = t_fc + 2c` but `l_eff = plate width`. EN 1993-1-8 6.2.5/6.2.6.11: the T-stub footprint is (t_f + 2c) x (b_fc + 2c), limited by the plate. A plate wider than b_fc + 2c overstates F_c,Rd and hence M_j,Rd and the bearing utilisation (unconservative). The model has no column flange width on the joint; it must come from the member section. Routed to the en1993 owner.
- OK: c = t_p sqrt(f_y/(3 f_jd gamma_M0)) (Eq. 6.5), f_jd = beta_j F_Rdu/(b_eff l_eff) with F_Rdu = min(A_c0 f_cd sqrt(A_c1/A_c0), 3 A_c0 f_cd) and the load-spread limits b2 <= min(a, 3 b1, b1 + h) (EN 1992-1-1 6.7). gamma_C applied once (f_cd = f_ck/gamma_C).
- OK T-stub (`t_stub_modes`): M_pl,1 = 0.25 l_eff,1 t^2 f_y/gamma_M0, L_b* = 8.8 m^3 A_s n_b/(l_eff,1 t^3), prying: F1 = 4 M_pl,1/m (method 2), F2 = (2 M_pl,2 + n SigmaF_t)/(m + n), F3 = SigmaF_t; no prying: 2 M_pl,1/m; n = min(e, 1.25 m); Table 6.4 single row lengths 2 pi m / 4 m + 1.25 e. The choice of Table 6.4 (inside the flange) for the base plate is stated by the owner; the print was not available.
- OK cone: see item 10. The per-anchor demand T/anchor_count assumes `anchorCount` = anchors in tension (field label says so) and isolated cones (stated in the explanation; spacing < 3 h_ef would reduce the resistance and is not modelled).

## 5. CHS and angle (v19, v22)

OK. Table 6.2 reproduced entry by entry from `curve_table_6_2`: rolled I (h/b > 1.2: a/b for t_f <= 40, b/c for 40..100, S460 a0/a/a), welded I, hot hollow a (a0 for S460), cold hollow c, angle b. Imperfection factors 0.13/0.21/0.34/0.49/0.76; chi (Eq. 6.49) correct. CHS classes d/t <= 50, 70, 90 eps^2, angle class 3 limits h/t <= 15 eps and (b+h)/2t <= 11.5 eps. Hand check of the angle test: L100x100x10 sharp: A = 1900 mm^2, I_u = 2.865833e6, I_v = 7.342544e5 mm^4, S235, L = 3 m: lambda_bar = 1.624984, curve b, chi = 0.3000516 (code test 0.3000516). Unverifiable: Annex G/H of EN 1993-3-1 for angles (not applied, stated), `1 - n^1.7` for CHS.

## 6. Toughness (v18)

OK logic: `select` picks the bracketing stress columns, interpolates linearly in sigma_Ed/f_y, falls back to the next HIGHER stress column (smaller t_max) when the lower column is not transcribed, clamps ratio < 0.25 to the 0.25 column (more favourable but allowed by the table's range) and refuses above 0.75; T_Ed above +10 uses the +10 column (conservative), below -50 is not applicable. Interpolation in T_Ed linear; t-direction never unconservative. Student/other: none.
Unverifiable: the 49 transcribed cells. Recollection (not certain): S235 J2 135 115 100 85 75 65 55 (registry 125 105 90 75 65 55 45, identical to the registry row for S355 K2, which I recall as 110 90 75 65 55 45 40); S235 J0 90 75 60 50 40 35 30 (registry 90 75 65 55 45 40 35). All rows are `unverified`, so the verdict cannot be a clean pass; nevertheless the S355 K2 row would be unconservative if my recollection holds. A human with the print must replace the rows.

## 7. EN 1994 (v11, v12, v13)

OK rho: rho = 0 for V <= V_pl,Rd/2 else min(1, (2V/V_pl - 1)^2) (6.2.2.4(2)). Re-solve with web width scaled by (1 - rho): for a steel-only I section this equals W_pl f_y - rho t_w h_w^2 f_y/4 of EN 1993-1-1 6.2.8, so the plastic neutral axis re-solve and the moment (about the slab top, T = C + N_c, M = T z_t - C z_c - N_c x/2) are consistent; hogging equilibrium cb - ta = fs and M = cb zc - ta zt - fs y_r are right. Interaction is evaluated coincident (M(s), V(s)) for sagging and at the support for hogging. V_pl is the steel plastic shear resistance (slab ignored) as the clause requires.
Tension stiffening: alpha_st = A I/(A_a I_a) = 8 and Delta sigma_s = 0.4 f_ctm/(alpha_st rho_s) reproduce 9 654 893.85 Pa for C30 (f_ctm 2.8965 MPa, rho_s 1.5 %). Factor 0.4 versus 0.2 (EN 1994-2 6.8.5.4) and whether the increment belongs to the range or to both extremes are Unverifiable here; the code adds it to the range (safe side). The 0.8 f_sk limit for sigma_s in the stiffening check is from the owner's recollection of 7.2.2.

## 8. EN 1997 (v12)

- OK: sigma_v0 (total, with gamma_w by annex), c_u = (q_t - sigma_v0)/N_kt, Kulhawy-Mayne phi' = 17.6 + 11 log10(q_t1), (N1)60 = N sqrt(100/sigma'_v0) <= 1.7 N, Hatanaka-Uchida phi' = sqrt(20 (N1)60) + 20; t(0.95) table (6.314 ... 1.725 for 20) correct; X_k = X_m (1 - k_n V_x) with k_n = t/sqrt(n) (mean fractile, local = false); n < 3 gives the lowest value.
- **Issue 8 (open)**: `derive_layer` puts every reading depth of every CPT inside the layer into one sample set. A CPT delivers a reading every few centimetres, strongly autocorrelated; n of 50 to 500 drives k_n to ~0.1 and X_k to the mean. n must be the number of independent samples (one value per investigation point, or the scale-of-fluctuation count). Unconservative for the characteristic c_u and phi'. Routed to the en1997 owner (suggested: per-investigation mean per layer as the sample).
- 8b OK (M2): tan phi_d = tan 30/1.25 gives phi_d = 24.7913 deg and K_p = 2.4442018 (Rankine, delta = 0), equal to the value pinned by the oracle (2.444201814844742); gamma applied once (design strength, A2 actions at 1.0/1.3).

## 9. EN 1992 punching (v13, v14)

OK against EN 1992-1-1: Eq. 6.39 and Table 6.1 k (0.45/0.60/0.70/0.80), W1 of 6.40 (interior) and 6.45 (edge), Eq. 6.43 (1.8 sqrt((e_y/b_z)^2 + (e_z/b_y)^2) with b = c + 4d), Eq. 6.44/6.45/6.46 for edge and corner with inward eccentricity (u1/u1*, k from c1/2c2), v_Rd,cs = 0.75 v_Rd,c + 1.5 (d/s_r) A_sw f_ywd,ef/(u1 d) sin(alpha), f_ywd,ef = 250 + 0.25 d, punching-max at u0 (EN) or 1.4 v_Rd,c at u1 (DE). Outward eccentricity (generalised Eq. 6.39 about the perimeter centroid, never below the inward value) is a conservative reading and is labelled unverified by the owner. Openings: tangents from the centre of the loaded area to the opening, opening within 6 d of the loaded area edge, hand length 2 * 0.65/9 = 0.14444 m for the 200 mm opening at x = 1.0 m (confirmed: tangent y = +-x/9 at x = 0.65). Studs: A_sw = studs in one perimeter, s_r largest radial spacing, reach + k d for u_out. Unverifiable: Figure 6.20 u1* legs and the stud rail layout of Figure 6.22.

## 10. Shared fastening cone kernel

OK: N0_Rk,c = k1 sqrt(f_ck) h_ef^1.5 (k1 8.9 / 12.7 cast-in, 7.7 / 11.0 post-installed), c_cr = 1.5 h_ef, s_cr = 3 h_ef, A0 = 9 h_ef^2, psi_s = min(0.7 + 0.3 c/c_cr, 1), psi_re = min(0.5 + h_ef/200, 1), psi_ec = prod 1/(1 + 2e/s_cr), psi_M = max(2 - z/(1.5 h_ef), 1); group area = union of clipped footprints; pull-out k2 A_h f_ck (7.5/10.5), splitting psi_h,sp, gamma_Ms (1.2 f_uk/f_yk >= 1.4 tension, 1.0 f_uk/f_yk >= 1.25 shear, 1.5 outside 800 MPa / 0.8), steel shear with lever arm 1.2 W_el f_uk (1 - N/N_Rd,s) alpha_M/l. Edge case: one edge c1 = 100 mm, h_ef = 150: ratio (c1 + c_cr)/(2 c_cr) = 0.7222, psi_s = 0.7 + 0.3*100/225 = 0.8333 (the oracle formula and the kernel agree). Unverifiable: k7 (0.8 "with grout" in the kernel; EN 1992-4 ties 0.8 to ductility and grout separately), edge breakout k9. The reduced h_ef rule for narrow members (>= 3 edges) is not modelled.

## 11-15. EN 1998

- Design base shear (v18): S_d of Eq. 3.13-3.16 (beta 0.2 floor on a_g), q = q0 alpha_u/alpha_1 k_w >= 1.5, cap by Table 5.1, 0.8 for irregular elevation (Table 4.1), lambda 0.85 for T1 <= 2 T_C and more than two storeys, F_b = S_d m lambda. Independent: a_g = 1.0 m/s^2, S = 1.35, q = 3 (0.8: 2.4), T1 0.8, T_C 0.6: S_d = 1.35 * 2.5/2.4 * 0.6/0.8 = 1.0547 m/s^2 versus the 0.2 a_g floor 0.2. OK.
- MO kernel (part5-v2): K_AE (E.4) and K_PE (E.5) agree with a force-polygon solution (own implementation, wall vertical, flat backfill) for (phi, delta, k_h, 1 +- k_v) = (30, 0, 0, 1), (30, 15, 0.2, 1.0), (35, 20, 0.15, 0.9), (30, 10, 0.3, 1.1): active K*V = 0.452032 / 0.318430 / 0.580361 and passive 4.12893 / 6.46980 / 3.56986 versus polygon 0.452031 / 0.318428 / 0.580359 and 4.128931 / 6.469807 / 3.569891 (rel. < 1e-5; the 0.333333 and 3.0 Rankine cases exact). Soil thrust 0.5 gamma V H^2 K consistent with this; inertia direction for passive (away from the wall) is the same absolute direction as the toward-the-wall active inertia. Submerged part (gamma - gamma_w, theta_w with gamma_sat or gamma_d inertia) and Westergaard 7/12 k_h gamma_w H^2 as in Annex E. Sliding H_Rd = (V - u) tan delta/gamma_Rd + passive, demand from the same design base shear. OK.
- Tie beams and slopes (v17): N = +-0.3/0.4/0.6 alpha S N_Ed for ground B/C/D (5.4.1.2), larger supported weight is conservative; A/E not applicable (nothing invented); slope: F_H = 0.5 alpha S S_T W, F_V = +-0.5/0.33 F_H by a_vg/a_g > 0.6, simplified Bishop with F_H in the driving moment about the centre (lever (y_c - y_cg)/R), vertical force in the weight, both signs examined, M2 strengths, F_s >= 1. Sign of alpha, moment direction and m_alpha checked by reading. OK; the coefficients 0.3/0.4/0.6, 0.5, 0.33, 0.6 are the owner's recalled registry rows (unverified).
- Pier EI_eff (part2-v3): EI_eff = M_y/phi_y of the fibre section (equals M_Rd/phi_y,ideal by the idealisation of EN 1998-2 Annex C). Independent fibre calculation (400 strips, parabola-rectangle f_c 30, 2 permille / n 2, elastic-plastic steel 500/200 GPa, b 0.8, h 1.2, rho 0.8 %, cover 0.08, N 1 MN): phi_y 0.0029949 1/m, M_y 2469.1 kNm, EI 8.244e8 N m^2 versus oracle 0.0029943, 2465.3 kNm, 8.2333e8 (0.1 to 0.2 %). EI_g C30 (E_c 33 GPa) 3.80e9: ratio 0.22. OK.
- Assessment (v16, part3): A.10b theta_y = phi_y (L_V + a_V z)/3 + 0.0013 (1 + 1.5 h/L_V) + 0.13 phi_y d_b f_y/sqrt(f_c) with phi_y, d_b in consistent length units (m) and A.1 theta_um = 0.016 * 0.3^nu * [max(0.01, omega')/max(0.01, omega) f_c]^0.225 (L_V/h)^0.35 25^(alpha rho_sx f_yw/f_c) 1.25^(100 rho_d)/gamma_el: exponents and structure match. nu = N/(b h f_c), omega = A_s1 f_y/(b d f_c), omega' with A_s2. CF/gamma_m division (ductile mean/CF, brittle mean/(CF gamma_m)) as EN 1998-3. Unverifiable: the strain constants (eps_c2, eps_cu, eps_su) and the 3/4 theta_um SD rule are registry rows; fibre kernel tested against concreteproperties by the owner. OK.

## 16. EN 1996

OK: beta = (1 + 0.3 a1/h_c)(1.5 - 1.1 A_b/A_ef) in [1, min(1.25 + a1/(2 h_c), 1.5)], beta = 1 for A_b > 0.45 A_ef (continuous: at 0.45 the formula gives 1.005); A_ef = l_efm t with the 60 deg spread. Chase tables match my recollection of EN 1996-1-1 8.6.2 Tables 8.1 (depth 30 mm, widths 100/125/150/175/200; formed during construction not modelled) and 8.2 (horizontal: 0/0, 0/15, 0/20, 10/25, 15/30 for unlimited length / <= 1250 mm), the 80 mm x 120 mm lower-third rule for t >= 225 mm, 225 mm spacing and 0.13 L cumulative width; status "calculation required" (warning) outside the table is the right direction. All rows remain `unverified` by the owner.

## 17. EN 1991

- OK: phi_2 = phi_2,min + beta_2 v_h with (1.05, 0.17), (1.10, 0.34), (1.15, 0.51), (1.20, 0.68) (Table 2.5), phi_3 = 1 - dm/m (1 + beta_3) with beta_3 0.5 (grab) / 1.0 (magnet), rail Phi_2 = 1.44/(sqrt(L) - 0.2) + 0.82 in [1.00, 1.67] and Phi_3 = 2.16/(sqrt(L) - 0.2) + 0.73 in [1.00, 2.00], footbridge q_fk = 2.0 + 120/(L + 30) in [2.5, 5.0] kN/m^2 (L 30: 4.0, Q_flk 0.1 * 4 * 30 * 3 = 36 kN), rotating machine F = m e (2 pi f)^2 (formula only; the inputs of the owner's 49.348 kN example were not reconstructed), crane runway envelope by influence line.
- **Issue 17 (open, information checks only)**: registry `test-static` 1.1 and `test-dynamic` 1.25 (taken from spec line E3.04). Crane practice and EN 1991-3 2.10 give the static test 1.25 Q_H and the dynamic test phi_6 Q_H with phi_6 = 0.5 (1 + phi_2) (about 1.1). The spec line is probably inverted. No verdict depends on it (information rows), but the numbers would mislead a user.
- **Issue 17c (open, low)**: the walking row reports "outside 1.7..2.1 Hz" with no consequence; EN 1990 A2.4.3.2 requires the comfort verification for vertical f < 5 Hz regardless. The 10 kN floor of Q_flk has no source (the clause is 10 % of the vertical load, alternatively 60 % of the service vehicle). Both flagged unverified by the owner; check against the print.

## Changes made by this review

| File (under `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/`) | Change |
|---|---|
| `🧬️schema/💡️inferences/⚖️checks/🕸️frame-envelope/🦀️.rs` | `columns_in_a_row` + use in `sway_imperfection` + test |
| `🧬️schema/💡️inferences/⚖️checks/🧮️clauses/🔗️joint-components/🦀️.rs` | `base_plate_m_rd` lever convention |
| `🧬️schema/💡️inferences/⚖️checks/🧮️clauses/🧪️tests/🔭️special-parts/🦀️.rs` | asymmetric rows c, d and hand values |
| `🧪️tests/🔭️special-parts/🦀️.rs`, `🧪️tests/🔭️special-parts/🥒️.feature` | adapter arms and two feature rows |
| `🔮️oracles/🧮️special-parts/🐍️.py` | levers per case, rows c and d |
| `🔮️oracles/🚦️check-outcome/🐍️.py` | `mjrd` M_Rd formula |

Requests appended to `📌️important/requests.md` (five lines, prefix `[wave3-correctness 15:40]`); rebuild line in `🗑️generated/build-en1993/latest.md`. Not run: Rust (no cargo), the special-parts oracle (queued behind the heavy gate); verified by hand instead.

## Caveats on this review

- Clause texts are from memory; items marked Unverifiable need the print.
- The machine example in item 17 uses inputs I did not reconstruct; only the formula was checked.
