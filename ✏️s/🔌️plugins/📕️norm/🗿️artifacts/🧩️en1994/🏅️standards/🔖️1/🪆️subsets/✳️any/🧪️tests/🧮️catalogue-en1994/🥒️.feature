@capability-en1994-1-evaluate
@oracle-en1994-1-compliance-independent
@comparison-numerical-v1
Feature: EN 1994 catalogue formulas reproduce the hand-checked worked examples
  Every row is one hand-checked quantity of the EN 1994 target specification (examples E1-01..E1-12).
  The Rust catalogue (`⚖️checks/🧮️catalogue`) and the stdlib-only Python oracle
  (`🔮️oracles/🧮️catalogue/🐍️.py`, numeric strip integration, numeric double integration of the curvature and
  RK4 time stepping) both evaluate the same input and must agree with the published value within the tolerance.

  Further specification assertions, asserted by the Rust unit tests of the artifact and not by the platform adapters:
  - Stud demand follows the applied moment: Given a composite beam whose steel alone resists the design moment Then no concrete force is required of the connection And raising the design moment above the bare steel resistance raises the required concrete force monotonically up to full connection
  - The partial-connection stress block never falls below the linear interpolation: Given the section of example E1-03 and any degree of connection between 0 and 1 Then the stress-block resistance is at least the linear-interpolation resistance

  @id-published-example
  @level-quick
  @mode-differential
  Scenario Outline: <example> <quantity> is reproduced
    Given the worked example "<example>" with its published inputs
    When the quantity "<quantity>" is evaluated
    Then it equals <expected> within relative tolerance <tolerance>
    Examples:
      | id                   | example | quantity       | expected   | tolerance |
      | e1-01-beff-simple    | E1-01   | beff_simple    | 2.0        | 1e-9      |
      | e1-01-beff-end       | E1-01   | beff_end       | 1.5        | 1e-9      |
      | e1-02-mpl-full       | E1-02   | mpl_full       | 423900     | 5e-4      |
      | e1-03-mpl-linear     | E1-03   | mpl_linear     | 323490     | 1e-3      |
      | e1-03-mpl-partial-sb | E1-03   | mpl_partial_sb | 364350     | 1e-3      |
      | e1-03-eta-min-8m     | E1-03   | eta_min_8m     | 0.49       | 1e-9      |
      | e1-04-prd1           | E1-04   | prd1           | 81656      | 1e-4      |
      | e1-04-prd2           | E1-04   | prd2           | 83332      | 1e-3      |
      | e1-04-prd-sheet      | E1-04   | prd_sheet      | 69410      | 1e-3      |
      | e1-04-prd-two-per-rib | E1-04  | prd_two_per_rib | 57159.393 | 1e-6      |
      | e1-04-stud-gap       | E1-04   | stud_gap       | 1.1764706  | 1e-6      |
      | e1-05-asf-req        | E1-05   | asf_req        | 0.000626   | 2e-3      |
      | e1-06-vpl            | E1-06   | vpl            | 526330     | 1e-3      |
      | e1-06-rho-shear      | E1-06   | rho_shear      | 0.2704     | 1e-3      |
      | e1-07-ncr-col        | E1-07   | ncr_col        | 20863000   | 1e-3      |
      | e1-07-chi-col        | E1-07   | chi_col        | 0.9174     | 1e-3      |
      | e1-08-mmax-rhs       | E1-08   | mmax_rhs       | 211850     | 1e-3      |
      | e1-08-mpl-rhs        | E1-08   | mpl_rhs        | 206940     | 2e-3      |
      | e1-08-mu-1700        | E1-08   | mu_1700        | 0.690      | 2e-3      |
      | e1-09-slab-full      | E1-09   | slab_full      | 43770      | 1e-3      |
      | e1-09-slab-partial   | E1-09   | slab_partial   | 29770      | 2e-3      |
      | e1-09-slab-mk        | E1-09   | slab_mk        | 26000      | 1e-9      |
      | e1-09-slab-vv        | E1-09   | slab_vv        | 46980      | 2e-3      |
      | e1-10-i-n0           | E1-10   | i_n0           | 0.00029353 | 2e-3      |
      | e1-10-i-nl           | E1-10   | i_nl           | 0.00022162 | 2e-3      |
      | e1-10-defl           | E1-10   | defl           | 0.01034    | 5e-3      |
      | e1-11-as-min         | E1-11   | as_min         | 0.0008352  | 1e-3      |
      | e1-12-iso-60         | E1-12   | iso_60         | 945.3      | 1e-3      |
      | e1-12-iso-30         | E1-12   | iso_30         | 841.8      | 1e-3      |
      | e1-13-mnl-ipe300     | E1-13   | mnl_ipe300     | 422131     | 3e-3      |
      | e1-13-kappa-ipe300   | E1-13   | kappa_u_ipe300 | 0.050581   | 3e-3      |
      | e1-14-end-mb-cracked     | E1-14   | cont_end_mb_cracked         | 64047.6    | 1e-3      |
      | e1-14-end-defl-cracked   | E1-14   | cont_end_defl_cracked       | 0.0021625  | 1e-3      |
      | e1-14-int-defl-cracked   | E1-14   | cont_interior_defl_cracked  | 0.0014511  | 1e-3      |
      | e1-14-cant-tip-cracked   | E1-14   | cont_cantilever_tip_cracked | 0.0051473  | 1e-3      |
      | e1-14-end-unequal-mb     | E1-14   | cont_end_unequal_mb         | 65000      | 1e-5      |
      | e1-15-enc-mb             | E1-15   | enc_mb                      | 342880     | 1e-3      |
      | e1-15-enc-md             | E1-15   | enc_md                      | 407639     | 1e-3      |
      | e1-16-fire-core-60       | E1-16   | fire_core_60                | 24.4885    | 1e-3      |
      | e1-16-fire-flange-60     | E1-16   | fire_flange_60              | 74.7581    | 1e-3      |
      | e1-16-fire-corner-60     | E1-16   | fire_corner_60              | 941.389    | 1e-3      |
      | e1-17-beta1-k01          | E1-17   | beta1_k01                   | 0.93985    | 1e-4      |
      | e1-17-beta2-k01          | E1-17   | beta2_k01                   | 0.62814    | 1e-4      |
      | e1-18-cross-ribbed       | E1-18   | girder_cross_ribbed         | 0.055      | 1e-9      |
      | e1-18-cross-solid        | E1-18   | girder_cross_solid          | 0.088      | 1e-9      |
      | e1-18-as-min             | E1-18   | girder_as_min               | 0.0005     | 1e-9      |
      | e1-18-delta-sigma-ts     | E1-18   | delta_sigma_ts_example      | 9654893.85 | 1e-9      |
      | e1-18-end-plate-open     | E1-18   | end_plate_open              | 931128.3   | 1e-6      |
      | e1-18-end-plate-capped   | E1-18   | end_plate_capped            | 510000     | 1e-9      |
      | e1-18-crack-band-k6      | E1-18   | crack_band_k6               | 0.44996    | 2e-3      |
      | e1-19-fire-steel-60      | E1-19   | beam_fire_steel_60          | 368.8639   | 3e-3      |
      | e1-19-fire-soffit-60     | E1-19   | beam_fire_soffit_60         | 775.9448   | 1e-3      |
      | e1-19-fire-mid-60        | E1-19   | beam_fire_mid_60            | 125.9835   | 1e-3      |
      | e1-19-fire-mfi-120       | E1-19   | beam_fire_mfi_120           | 245753.88  | 3e-3      |
      | e1-20-stud-endurance     | E1-20   | fatigue_stud_endurance      | 429444868.57 | 1e-6    |
      | e1-20-stud-damage        | E1-20   | fatigue_stud_damage         | 0.011642938  | 1e-6      |
      | e1-20-rebar-damage       | E1-20   | fatigue_rebar_damage        | 0.089049251  | 1e-6      |
      | e1-20-steel-util         | E1-20   | fatigue_steel_util          | 1.2290452  | 1e-6      |
      | e1-21-ts-fatigue         | E1-21   | delta_sigma_ts_fatigue_example | 4827446.923 | 1e-9   |
      | e1-21-sigma-max-f        | E1-21   | fatigue_rebar_sigma_max     | 117327446.923 | 1e-9    |
      | e1-21-rebar-util-pass    | E1-21   | fatigue_rebar_util_pass     | 0.32352983 | 1e-6      |
      | e1-21-rebar-util-fail    | E1-21   | fatigue_rebar_util_fail     | 1.18342354 | 1e-6      |
      | e1-21-rebar-damage-ts    | E1-21   | fatigue_rebar_damage_ts     | 3.8835503e-05 | 1e-6   |
      | e1-22-chi-lt-lambda      | E1-22   | chi_lt_rolled_lambda        | 0.57735027 | 1e-6      |
      | e1-22-chi-lt-rolled      | E1-22   | chi_lt_rolled               | 0.89837760 | 1e-6      |
      | e1-22-chi-lt-welded      | E1-22   | chi_lt_welded               | 0.72554695 | 1e-6      |
      | e1-22-patch-f-rd         | E1-22   | patch_f_rd                  | 517480.1439 | 1e-6     |
      | e1-22-patch-chi          | E1-22   | patch_chi                   | 0.52359574 | 1e-6      |
      | e1-22-patch-ly           | E1-22   | patch_ly                    | 0.348      | 1e-9      |
      | e1-22-patch-interaction  | E1-22   | patch_interaction           | 0.64182830 | 1e-6      |
      | e1-23-vib-mass           | E1-23   | vibration_mass              | 1478.588509 | 1e-9     |
      | e1-23-vib-f1             | E1-23   | vibration_f1                | 7.164758   | 1e-4      |
      | e1-23-vib-utilisation    | E1-23   | vibration_utilisation       | 0.418718   | 1e-4      |
      | e1-24-ltb-ks             | E1-24   | ltb_ks                      | 70476.48   | 1e-6      |
      | e1-24-ltb-mcr            | E1-24   | ltb_mcr                     | 216307.9   | 1e-6      |
      | e1-24-ltb-lambda         | E1-24   | ltb_lambda                  | 1.0050900  | 1e-6      |
      | e1-24-ltb-chi            | E1-24   | ltb_chi                     | 0.66204368 | 1e-6      |
      | e1-24-ltb-mbrd           | E1-24   | ltb_mbrd                    | 144666.8   | 1e-6      |
      | e1-25-creep-ns           | E1-25   | creep_ns                    | 15.188859  | 1e-6      |
      | e1-25-creep-i-ns         | E1-25   | creep_i_ns                  | 0.00024733105 | 1e-6   |
      | e1-25-creep-defl-load    | E1-25   | creep_defl_load             | 0.010348671 | 2e-3     |
      | e1-25-creep-defl-shrink  | E1-25   | creep_defl_shrinkage        | 0.0074134533 | 1e-5    |
      | e1-25-creep-defl-total   | E1-25   | creep_defl_total            | 0.017762124 | 2e-3     |
      | e1-26-fire-crit-mu05     | E1-26   | fire_crit_mu05              | 584.66526  | 1e-6      |
      | e1-26-fire-crit-mu03     | E1-26   | fire_crit_mu03              | 663.77669  | 1e-6      |
      | e1-26-fire-protected-90  | E1-26   | fire_protected_90           | 507.88182  | 3e-3      |
