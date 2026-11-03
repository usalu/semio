@capability-en1993-1-evaluate
@oracle-en1993-1-compliance-independent
@comparison-numerical-v1
Feature: EN 1993 special-part clause functions and their wired checks reproduce the independent oracle
  The Python oracle (numpy and scipy: brentq buckling factors and thresholds, fixed-point distortional iteration, linprog base plate equilibrium,
  quad Janssen integrals, interp1d tables) and the Rust clause functions in 🧮️clauses must agree for every vector, in any implementation language.
  The wired checks are evaluated on the special-parts showcase subject (a compliant HEB 240 frame extended by one populated row of every special part).

  Further specification assertions, asserted by the Rust unit tests of the artifact and not by the platform adapters:
  - every special-part check of the showcase subject is present once: Given the special-parts showcase subject with the DIN annex When the compliance report is evaluated Then the report contains exactly one check for each of the 48 special-part check ids And every failing special-part check carries at least one remedy And a check that rests on an unverified clause value is reported as an unverified basis

  @id-clause-vector
  @level-quick
  @mode-differential
  Scenario Outline: <vector> <quantity> equals the oracle value
    Given the clause vector "<vector>" of "<part>"
    When the quantity "<quantity>" is evaluated
    Then it equals the independent oracle value within relative tolerance <tolerance>
    Examples:
      | id                                            | vector        | part                | quantity           | tolerance |
      | special-parts-cf-chi-d-en-1993-1-3            | special-parts | EN 1993-1-3         | cf_chi_d           | 1e-6      |
      | special-parts-cf-lambda-d-en-1993-1-3         | special-parts | EN 1993-1-3         | cf_lambda_d        | 1e-6      |
      | special-parts-cf-sigma-cr-s-en-1993-1-3       | special-parts | EN 1993-1-3         | cf_sigma_cr_s      | 1e-6      |
      | special-parts-cf-a-s-en-1993-1-3              | special-parts | EN 1993-1-3         | cf_a_s             | 1e-6      |
      | special-parts-cf-i-s-en-1993-1-3              | special-parts | EN 1993-1-3         | cf_i_s             | 1e-6      |
      | special-parts-cf-fbv-en-1993-1-3              | special-parts | EN 1993-1-3         | cf_fbv             | 1e-9      |
      | special-parts-cf-vb-rd-en-1993-1-3            | special-parts | EN 1993-1-3         | cf_vb_rd           | 1e-9      |
      | special-parts-cf-rw-rd-en-1993-1-3            | special-parts | EN 1993-1-3         | cf_rw_rd           | 1e-9      |
      | special-parts-st-rho-int-cold-en-1993-1-4     | special-parts | EN 1993-1-4         | st_rho_int_cold    | 1e-12     |
      | special-parts-st-rho-int-weld-en-1993-1-4     | special-parts | EN 1993-1-4         | st_rho_int_weld    | 1e-12     |
      | special-parts-st-rho-out-cold-en-1993-1-4     | special-parts | EN 1993-1-4         | st_rho_out_cold    | 1e-12     |
      | special-parts-st-rho-out-weld-en-1993-1-4     | special-parts | EN 1993-1-4         | st_rho_out_weld    | 1e-12     |
      | special-parts-st-es-en-1993-1-4               | special-parts | EN 1993-1-4         | st_es              | 1e-12     |
      | special-parts-st-k02-650-en-1993-1-4          | special-parts | EN 1993-1-4         | st_k02_650         | 1e-12     |
      | special-parts-st-ke-650-en-1993-1-4           | special-parts | EN 1993-1-4         | st_ke_650          | 1e-12     |
      | special-parts-st-chi-flex-en-1993-1-4         | special-parts | EN 1993-1-4         | st_chi_flex        | 1e-9      |
      | special-parts-st-chi-lt-welded-en-1993-1-4    | special-parts | EN 1993-1-4         | st_chi_lt_welded   | 1e-9      |
      | special-parts-st-k-int-en-1993-1-4            | special-parts | EN 1993-1-4         | st_k_int           | 1e-12     |
      | special-parts-st-ro-strain-en-1993-1-4        | special-parts | EN 1993-1-4         | st_ro_strain       | 1e-12     |
      | special-parts-st-ro-tangent-en-1993-1-4       | special-parts | EN 1993-1-4         | st_ro_tangent      | 1e-5      |
      | special-parts-pl-rho-c-en-1993-1-5            | special-parts | EN 1993-1-5         | pl_rho_c           | 1e-12     |
      | special-parts-pl-beta-hog-en-1993-1-5         | special-parts | EN 1993-1-5         | pl_beta_hog        | 1e-12     |
      | special-parts-pl-beta-hog-hi-en-1993-1-5      | special-parts | EN 1993-1-5         | pl_beta_hog_hi     | 1e-12     |
      | special-parts-pl-patch-a-en-1993-1-5          | special-parts | EN 1993-1-5         | pl_patch_a         | 1e-9      |
      | special-parts-pl-patch-b-en-1993-1-5          | special-parts | EN 1993-1-5         | pl_patch_b         | 1e-9      |
      | special-parts-pl-patch-c-en-1993-1-5          | special-parts | EN 1993-1-5         | pl_patch_c         | 1e-9      |
      | special-parts-pl-k-tau-en-1993-1-5            | special-parts | EN 1993-1-5         | pl_k_tau           | 1e-9      |
      | special-parts-pl-k-tau-low-en-1993-1-5        | special-parts | EN 1993-1-5         | pl_k_tau_low       | 1e-9      |
      | special-parts-pl-k-tau-floor-en-1993-1-5      | special-parts | EN 1993-1-5         | pl_k_tau_floor     | 1e-9      |
      | special-parts-pl-mv-en-1993-1-5               | special-parts | EN 1993-1-5         | pl_mv              | 1e-12     |
      | special-parts-pl-ist-en-1993-1-5              | special-parts | EN 1993-1-5         | pl_ist             | 1e-12     |
      | special-parts-pl-alpha-ult-en-1993-1-5        | special-parts | EN 1993-1-5         | pl_alpha_ult       | 1e-12     |
      | special-parts-sh-omega-en-1993-1-6            | special-parts | EN 1993-1-6         | sh_omega           | 1e-12     |
      | special-parts-sh-sigma-cr-en-1993-1-6         | special-parts | EN 1993-1-6         | sh_sigma_cr        | 1e-12     |
      | special-parts-sh-lambda-en-1993-1-6           | special-parts | EN 1993-1-6         | sh_lambda          | 1e-12     |
      | special-parts-sh-alpha-en-1993-1-6            | special-parts | EN 1993-1-6         | sh_alpha           | 1e-12     |
      | special-parts-sh-chi-en-1993-1-6              | special-parts | EN 1993-1-6         | sh_chi             | 1e-12     |
      | special-parts-sh-sigma-rd-en-1993-1-6         | special-parts | EN 1993-1-6         | sh_sigma_rd        | 1e-12     |
      | special-parts-sh-theta-cr-en-1993-1-6         | special-parts | EN 1993-1-6         | sh_theta_cr        | 1e-12     |
      | special-parts-sh-theta-chi-en-1993-1-6        | special-parts | EN 1993-1-6         | sh_theta_chi       | 1e-12     |
      | special-parts-sh-interaction-en-1993-1-6      | special-parts | EN 1993-1-6         | sh_interaction     | 1e-12     |
      | special-parts-sh-omega-short-cx-en-1993-1-6   | special-parts | EN 1993-1-6         | sh_omega_short_cx  | 1e-12     |
      | special-parts-tf-t-ed-en-1993-1-10            | special-parts | EN 1993-1-10        | tf_t_ed            | 1e-12     |
      | special-parts-tf-tmax-en-1993-1-10            | special-parts | EN 1993-1-10        | tf_tmax            | 1e-12     |
      | special-parts-tf-tmax-s355j2-m25-en-1993-1-10 | special-parts | EN 1993-1-10        | tf_tmax_s355j2_m25 | 1e-12     |
      | special-parts-tf-tmax-s355k2-m15-en-1993-1-10 | special-parts | EN 1993-1-10        | tf_tmax_s355k2_m15 | 1e-12     |
      | special-parts-tf-tmax-s355k2-r50-m10-fallback-en-1993-1-10 | special-parts | EN 1993-1-10        | tf_tmax_s355k2_r50_m10_fallback | 1e-12     |
      | special-parts-tf-cells-total-en-1993-1-10 | special-parts | EN 1993-1-10        | tf_cells_total | 1e-12     |
      | special-parts-tf-cells-unset-en-1993-1-10 | special-parts | EN 1993-1-10        | tf_cells_unset | 1e-12     |
      | special-parts-tf-blend-r60-en-1993-1-10 | special-parts | EN 1993-1-10        | tf_blend_r60 | 1e-12     |
      | special-parts-tf-blend-below-r25-en-1993-1-10 | special-parts | EN 1993-1-10        | tf_blend_below_r25_conservative | 1e-12     |
      | special-parts-tc-ernst-en-1993-1-11           | special-parts | EN 1993-1-11        | tc_ernst           | 1e-12     |
      | special-parts-tc-sag-en-1993-1-11             | special-parts | EN 1993-1-11        | tc_sag             | 1e-12     |
      | special-parts-tc-pin-m-en-1993-1-11           | special-parts | EN 1993-1-11        | tc_pin_m           | 1e-12     |
      | special-parts-tc-pin-comb-en-1993-1-11        | special-parts | EN 1993-1-11        | tc_pin_comb        | 1e-12     |
      | special-parts-tc-rope-en-1993-1-11            | special-parts | EN 1993-1-11        | tc_rope            | 1e-12     |
      | special-parts-jc-lb-star-en-1993-1-8          | special-parts | EN 1993-1-8         | jc_lb_star         | 1e-12     |
      | special-parts-jc-f1-m1-en-1993-1-8            | special-parts | EN 1993-1-8         | jc_f1_m1           | 1e-12     |
      | special-parts-jc-f2-en-1993-1-8               | special-parts | EN 1993-1-8         | jc_f2              | 1e-12     |
      | special-parts-jc-ft-rd-m1-en-1993-1-8         | special-parts | EN 1993-1-8         | jc_ft_rd_m1        | 1e-12     |
      | special-parts-jc-f1-m2-en-1993-1-8            | special-parts | EN 1993-1-8         | jc_f1_m2           | 1e-12     |
      | special-parts-jc-ft-noprying-en-1993-1-8      | special-parts | EN 1993-1-8         | jc_ft_noprying     | 1e-12     |
      | special-parts-jc-z-eq-en-1993-1-8             | special-parts | EN 1993-1-8         | jc_z_eq            | 1e-12     |
      | special-parts-jc-k-eq-en-1993-1-8             | special-parts | EN 1993-1-8         | jc_k_eq            | 1e-12     |
      | special-parts-jc-omega-en-1993-1-8            | special-parts | EN 1993-1-8         | jc_omega           | 1e-12     |
      | special-parts-jc-omega-15-en-1993-1-8         | special-parts | EN 1993-1-8         | jc_omega_15        | 1e-12     |
      | special-parts-jc-mj-en-1993-1-8               | special-parts | EN 1993-1-8         | jc_mj              | 1e-12     |
      | special-parts-jc-mu-en-1993-1-8               | special-parts | EN 1993-1-8         | jc_mu              | 1e-12     |
      | special-parts-jc-sj-ini-en-1993-1-8           | special-parts | EN 1993-1-8         | jc_sj_ini          | 1e-12     |
      | special-parts-jc-bp-a-en-1993-1-8             | special-parts | EN 1993-1-8         | jc_bp_a            | 1e-9      |
      | special-parts-jc-bp-b-en-1993-1-8             | special-parts | EN 1993-1-8         | jc_bp_b            | 1e-9      |
      | special-parts-jc-bp-c-en-1993-1-8             | special-parts | EN 1993-1-8         | jc_bp_c            | 1e-9      |
      | special-parts-jc-bp-d-en-1993-1-8             | special-parts | EN 1993-1-8         | jc_bp_d            | 1e-9      |
      | special-parts-jc-fjd-en-1993-1-8              | special-parts | EN 1993-1-8         | jc_fjd             | 1e-12     |
      | special-parts-jc-cs-leff-free-en-1993-1-8     | special-parts | EN 1993-1-8         | jc_cs_leff_free    | 1e-12     |
      | special-parts-jc-cs-fc-free-en-1993-1-8       | special-parts | EN 1993-1-8         | jc_cs_fc_free      | 1e-12     |
      | special-parts-jc-cs-leff-plate-en-1993-1-8    | special-parts | EN 1993-1-8         | jc_cs_leff_plate   | 1e-12     |
      | special-parts-jc-cs-fc-plate-en-1993-1-8      | special-parts | EN 1993-1-8         | jc_cs_fc_plate     | 1e-12     |
      | special-parts-jc-cr-free-en-1993-1-8          | special-parts | EN 1993-1-8         | jc_cr_free         | 1e-9      |
      | special-parts-jc-cr-plate-en-1993-1-8         | special-parts | EN 1993-1-8         | jc_cr_plate        | 1e-12     |
      | special-parts-br-l2-en-1993-2                 | special-parts | EN 1993-2           | br_l2              | 1e-12     |
      | special-parts-br-l3-en-1993-2                 | special-parts | EN 1993-2           | br_l3              | 1e-12     |
      | special-parts-br-l4-en-1993-2                 | special-parts | EN 1993-2           | br_l4              | 1e-12     |
      | special-parts-br-l1-en-1993-2                 | special-parts | EN 1993-2           | br_l1              | 1e-12     |
      | special-parts-br-ds-en-1993-2                 | special-parts | EN 1993-2           | br_ds              | 1e-12     |
      | special-parts-br-cat-en-1993-2                | special-parts | EN 1993-2           | br_cat             | 1e-12     |
      | special-parts-br-ks-en-1993-2                 | special-parts | EN 1993-2           | br_ks              | 1e-12     |
      | special-parts-tw-vcrit-en-1993-3-1-3-2        | special-parts | EN 1993-3-1/3-2     | tw_vcrit           | 1e-12     |
      | special-parts-tw-sc-en-1993-3-1-3-2           | special-parts | EN 1993-3-1/3-2     | tw_sc              | 1e-12     |
      | special-parts-tw-freq-en-1993-3-1-3-2         | special-parts | EN 1993-3-1/3-2     | tw_freq            | 1e-12     |
      | special-parts-tw-cycles-en-1993-3-1-3-2       | special-parts | EN 1993-3-1/3-2     | tw_cycles          | 1e-12     |
      | special-parts-tw-ovalling-en-1993-3-1-3-2     | special-parts | EN 1993-3-1/3-2     | tw_ovalling        | 1e-12     |
      | special-parts-ct-ph-en-1993-4-1-4-2-4-3       | special-parts | EN 1993-4-1/4-2/4-3 | ct_ph              | 1e-12     |
      | special-parts-ct-nx-en-1993-4-1-4-2-4-3       | special-parts | EN 1993-4-1/4-2/4-3 | ct_nx              | 1e-9      |
      | special-parts-ct-pv-en-1993-4-1-4-2-4-3       | special-parts | EN 1993-4-1/4-2/4-3 | ct_pv              | 1e-12     |
      | special-parts-ct-tank-t-en-1993-4-1-4-2-4-3   | special-parts | EN 1993-4-1/4-2/4-3 | ct_tank_t          | 1e-12     |
      | special-parts-ct-pipe-t-en-1993-4-1-4-2-4-3   | special-parts | EN 1993-4-1/4-2/4-3 | ct_pipe_t          | 1e-12     |
      | special-parts-ct-pipe-ax-en-1993-4-1-4-2-4-3  | special-parts | EN 1993-4-1/4-2/4-3 | ct_pipe_ax         | 1e-12     |
      | special-parts-ct-pcr-en-1993-4-1-4-2-4-3      | special-parts | EN 1993-4-1/4-2/4-3 | ct_pcr             | 1e-12     |
      | special-parts-pi-loss-30-en-1993-5            | special-parts | EN 1993-5           | pi_loss_30         | 1e-12     |
      | special-parts-pi-loss-3-en-1993-5             | special-parts | EN 1993-5           | pi_loss_3          | 1e-12     |
      | special-parts-pi-fy-red-en-1993-5             | special-parts | EN 1993-5           | pi_fy_red          | 1e-12     |
      | special-parts-pi-vpl-en-1993-5                | special-parts | EN 1993-5           | pi_vpl             | 1e-12     |
      | special-parts-cr-leff-en-1993-6               | special-parts | EN 1993-6           | cr_leff            | 1e-12     |
      | special-parts-cr-soz-en-1993-6                | special-parts | EN 1993-6           | cr_soz             | 1e-12     |
      | special-parts-cr-tau-en-1993-6                | special-parts | EN 1993-6           | cr_tau             | 1e-12     |
      | special-parts-cr-vm-en-1993-6                 | special-parts | EN 1993-6           | cr_vm              | 1e-12     |
      | special-parts-cr-hertz-en-1993-6              | special-parts | EN 1993-6           | cr_hertz           | 1e-12     |
      | special-parts-cr-lambda-en-1993-6             | special-parts | EN 1993-6           | cr_lambda          | 1e-12     |
      | special-parts-cr-phi2-en-1993-6               | special-parts | EN 1993-6           | cr_phi2            | 1e-12     |
      | special-parts-sec-ang-a-en-1993-1-1 | special-parts | EN 1993-1-1 | sec_ang_a | 1e-9 |
      | special-parts-sec-ang-iu-en-1993-1-1 | special-parts | EN 1993-1-1 | sec_ang_iu | 1e-6 |
      | special-parts-sec-ang-iv-en-1993-1-1 | special-parts | EN 1993-1-1 | sec_ang_iv | 1e-6 |
      | special-parts-sec-ang-alpha-en-1993-1-1 | special-parts | EN 1993-1-1 | sec_ang_alpha | 1e-6 |
      | special-parts-sec-ang-wel-u-en-1993-1-1 | special-parts | EN 1993-1-1 | sec_ang_wel_u | 1e-6 |
      | special-parts-sec-ang-wel-v-en-1993-1-1 | special-parts | EN 1993-1-1 | sec_ang_wel_v | 1e-6 |
      | special-parts-sec-ang-wpl-u-en-1993-1-1 | special-parts | EN 1993-1-1 | sec_ang_wpl_u | 1e-5 |
      | special-parts-sec-ang-wpl-v-en-1993-1-1 | special-parts | EN 1993-1-1 | sec_ang_wpl_v | 1e-5 |
      | special-parts-sec-ang-it-en-1993-1-1 | special-parts | EN 1993-1-1 | sec_ang_it | 4e-2 |
      | special-parts-sec-ang-class-en-1993-1-1 | special-parts | EN 1993-1-1 | sec_ang_class | 1e-12 |
      | special-parts-sec-chs-a-en-1993-1-1 | special-parts | EN 1993-1-1 | sec_chs_a | 2e-3 |
      | special-parts-sec-chs-i-en-1993-1-1 | special-parts | EN 1993-1-1 | sec_chs_i | 3e-3 |
      | special-parts-sec-chs-wel-en-1993-1-1 | special-parts | EN 1993-1-1 | sec_chs_wel | 3e-3 |
      | special-parts-sec-chs-wpl-en-1993-1-1 | special-parts | EN 1993-1-1 | sec_chs_wpl | 3e-3 |
      | special-parts-sec-chs-it-en-1993-1-1 | special-parts | EN 1993-1-1 | sec_chs_it | 3e-3 |
      | special-parts-sec-chs-class-en-1993-1-1 | special-parts | EN 1993-1-1 | sec_chs_class | 1e-12 |

  @id-oracle-check
  @level-quick
  @mode-differential
  Scenario Outline: the check <check> reports the oracle utilisation
    Given the showcase subject "special-parts" with the DIN annex
    When the compliance report is evaluated
    Then the utilisation of the check "<check>" equals the oracle quantity "<quantity>" within relative tolerance <tolerance>
    Examples:
      | id                       | check                    | quantity   | tolerance |
      | en1993-1-3-6-1-5-cf-1    | en1993.1-3.6.1.5.cf-1    | w_cf_shear | 1e-9      |
      | en1993-1-6-8-thin-silo-1 | en1993.1-6.8.thin.silo-1 | w_thin_shell | 1e-9    |
      | en1993-1-10-t-member-b1  | en1993.1-10.t.member-b1  | w_brittle  | 1e-9      |
      | en1993-4-1-5-2-ph-silo-1 | en1993.4-1.5.2.ph.silo-1 | w_silo_ph  | 1e-9      |
      | en1993-4-2-6-silo-1      | en1993.4-2.6.silo-1      | w_tank     | 1e-9      |
      | en1993-5-5-2-il-pile-1   | en1993.5.5.2.il.pile-1   | w_pile_il  | 1e-9      |
      | en1993-6-5-7-vm-crane-1  | en1993.6.5.7.vm.crane-1  | w_crane_vm | 1e-9      |
