# Oracle wiring v2 (nothing compiled; values re-derived by hand and with the repo python venv)

## Done
- Harness path: `python_oracle::verify_cases_above` and `oracle_driver::case_files_above` now also test `<ancestor>/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🔮️oracle-cases` at every level, so the old test (artifact crate under `📦️packages/🦀️rust`) reaches its own cases for en1991/1992/1997/1999 instead of the shared `numerical.json`.
- Registries exposed: `pub mod oracle_wiring` under `cfg(any(test, feature = "oracle-wiring"))` plus `pub fn registry() -> &'static Registry` in all ten checks crates; Cargo feature `oracle-wiring` (enables compliance `compliance-testing`, optional mechanics). Cargo.lock edges unverified.
- en1992: 47 adapters (was 30). New public clause functions: `creep_strain`, `creep_terms/creep_phi_rh/creep_beta_h/creep_phi_from/adjusted_loading_age/notional_size_ac_u/age_adjusted_modulus/eps_cd_0`, `Conc.eps_cu1`, `nu1_strut`, `a_sw_max`, `torsion_shear_flow`, `torsion_longitudinal`, `t_rd_max`, `v_ed_max_unreinforced`, `beta_moment`, `beta_rectangular`, `v_ed_punching`, `v_rd_c_from`, `strain_difference(_alpha)`, `sr_max_closed`, `crack_width_value`, `curvature_534`, `f_bd_from`, `alpha_235`, `l_b_min_m`; SL-04 outcomes `k_c`, `k`; AJ-03 outcome `theta`. `crack_width`, `nominal_curvature`, `f_bd`, `l_bd_m`, `shrinkage`, `creep_phi` now delegate to them. All 17 gaps and the 32 missing quantities are wired; `mechanics.beam-continuous` uses the new mechanics API `solve_continuous_beam` (`🧮️mechanics/🌉️continuous-beam`). Anchorage adapter now applies the 0.7 floor via `alpha_235` (the earlier "capped" mismatch was an adapter issue, not a bug).
- en1993: 27 adapters; added `global::sway_alpha_h/sway_alpha_m`, `resistance::{a_w_hollow, tau_ed_i}`, `parametric_sections::{rhs_minor_moduli, chs_torsion}`; rhs-sharp, chs, sway cases now complete. Eq. 5.2 vs eigenvalue: test `eq_5_2_overestimates_the_eigenvalue_for_a_documented_reason` with `EQ_5_2_VS_EIGENVALUE_REASON` (driver known differences cannot express a gap between two blessed methods because our evaluators reproduce each side).
- en1997: `part_1::{bulk_unit_weight, buoyant_unit_weight}`, phase-relations adapter (2 cases). en1991: `part_1_4::mean_wind_velocity`, `v_m` added (3 cases).

## Expected-mismatch review (en1992 known differences, 8 entries + chi)
- creep rapid/slow phi_0, phi (4 entries): oracle bug, adapter omits `t0_adj`; library with t0_adj equals ours (1.4157/1.4012, 1.7215/1.7049). Routed to oracles.
- min-reinforcement deep beam k_c, A_s_min: oracle bug, N_Ed passed in N to a kN function. Routed to oracles (k_c 0.572).
- cracked-c30 M_cr (0.8 vs 0.5 percent): library definition not isolated; documented, routed.
- KK chi, E_adj (c30): registry chi 0.8 unverified vs 0.864 relaxation; E_adj inherits it (4.7 percent > 3 percent). c50 stays inside the spread.
- Risk: cracked-c30 I_cr predicted 0.46 percent (limit 0.5), from E_cm table 33 vs formula 32.84 GPa.

## Still unwired
- en1993 (7 checks, 9 cases): tee, mono-i, ehs, rhs-rounded, channel, angle, IPE300 row, mcr-i x2: need an FE torsion/warping solver (routed to shared). en1999 free outline It/Iw and en1994 Iw share the need.
- en1990 (fatigue cut-off, exact beta, beam envelope), en1991 (tandem, LM71, drift), en1997 (anchor elongation/resistance, pile group, oedometer), en1999, en1994, en1995 T classes, en1996: see their wiring reports and requests.md.
