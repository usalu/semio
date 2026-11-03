@capability-en1996-1-evaluate
@oracle-en1996-1-compliance-independent
@comparison-numerical-v1
Feature: EN 1996 catalogue worked examples
  The hand-checked examples X1 to X12 of the target specification are evaluated by every implementation of the catalogue.
  Inputs are SI numbers written name=value, text inputs name:value, separated by semicolons.

  @id-catalogue-example
  @level-quick
  @mode-differential
  Scenario Outline: Catalogue entry <entry> output <key>
    Given the catalogue entry "<entry>" for annex "<annex>" with inputs "<inputs>"
    When the entry is evaluated
    Then the output "<key>" equals <expected> within relative tolerance <tolerance>
    Examples:
      | id              | entry | annex | inputs                                                                                | key    | expected | tolerance |
      | m-02-fk-en      | M-02  | en    | fb_pa=20e6;fm_pa=10e6;material:clay;gamma_m=1.5                                       | fk     | 8.935e6  | 1e-3      |
      | m-02-fd-en      | M-02  | en    | fb_pa=20e6;fm_pa=10e6;material:clay;gamma_m=1.5                                       | fd     | 5.957e6  | 1e-3      |
      | m-06-e-lt-en    | M-06  | en    | fk_pa=8.935e6;material:clay                                                           | e-lt   | 3574e6   | 1e-3      |
      | m-10-hef-en     | M-10  | en    | h_m=2.75;t_m=0.365;rho2=0.75;sides=2                                                  | hef    | 2.0625   | 1e-9      |
      | m-10-rho-n-en   | M-10  | en    | h_m=2.75;t_m=0.365;rho2=0.75;sides=3;l_m=3.0                                          | rho-n  | 0.7126   | 1e-3      |
      | m-10-rho-n-en-2 | M-10  | en    | h_m=2.75;t_m=0.365;rho2=0.75;sides=4;l_m=3.0                                          | rho-n  | 0.5093   | 1e-3      |
      | m-12-n-top-en   | M-12  | en    | t_m=0.365;hef_m=2.0625;fd_pa=5.957e6;n_top_n=200e3;m_top_nm=4000;n_bot_n=200e3        | n-top  | 1881e3   | 2e-3      |
      | m-13-phi-m-en   | M-13  | en    | t_m=0.365;hef_m=2.0625;fd_pa=5.957e6;n_mid_n=200e3;m_mid_nm=2500;phi_inf=1.5;k_e=1000 | phi-m  | 0.88579  | 2e-4      |
      | m-14-beta-en    | M-14  | en    | ab_m2=0.073;a1_m=0.5;hc_m=2.75;t_m=0.365;lefm_m=1.0;fd_pa=5.957e6;ned_c_n=500e3       | beta   | 1.3409   | 1e-3      |
      | m-14-n-rdc-en   | M-14  | en    | ab_m2=0.073;a1_m=0.5;hc_m=2.75;t_m=0.365;lefm_m=1.0;fd_pa=5.957e6;ned_c_n=500e3       | n-rdc  | 583.1e3  | 2e-3      |
      | m-14-beta-eq-6-10 | M-14  | en    | ab_m2=0.146;a1_m=0.5;hc_m=2.75;t_m=0.365;lefm_m=1.0;fd_pa=5.957e6;ned_c_n=500e3 | beta   | 1.117818 | 1e-5      |
      | m-14-n-rdc-eq-6-10 | M-14  | en    | ab_m2=0.146;a1_m=0.5;hc_m=2.75;t_m=0.365;lefm_m=1.0;fd_pa=5.957e6;ned_c_n=500e3 | n-rdc  | 972.2e3  | 2e-3      |
      | m-14-beta-cap-1-5 | M-14  | en    | ab_m2=0.0365;a1_m=4.0;hc_m=2.75;t_m=0.365;lefm_m=1.0;fd_pa=5.957e6;ned_c_n=500e3 | beta   | 1.5      | 1e-9      |
      | m-14-n-rdc-cap-1-5 | M-14  | en    | ab_m2=0.0365;a1_m=4.0;hc_m=2.75;t_m=0.365;lefm_m=1.0;fd_pa=5.957e6;ned_c_n=500e3 | n-rdc  | 326.1e3  | 2e-3      |
      | m-14-beta-cap-1-25 | M-14  | en    | ab_m2=0.0365;a1_m=0.0;hc_m=2.75;t_m=0.365;lefm_m=1.0;fd_pa=5.957e6;ned_c_n=500e3 | beta   | 1.25     | 1e-9      |
      | m-14-beta-above-0-45 | M-14  | en    | ab_m2=0.2;a1_m=0.5;hc_m=2.75;t_m=0.365;lefm_m=1.0;fd_pa=5.957e6;ned_c_n=500e3 | beta   | 1.0      | 1e-9      |
      | m-14-n-rdc-above-0-45 | M-14  | en    | ab_m2=0.2;a1_m=0.5;hc_m=2.75;t_m=0.365;lefm_m=1.0;fd_pa=5.957e6;ned_c_n=500e3 | n-rdc  | 1191.4e3 | 2e-3      |
      | m-20-v-rd-en    | M-20  | en    | l_m=2.0;t_m=0.365;n_ed_n=584e3;v_ed_n=200e3;fb_pa=20e6;fm_pa=10e6;gamma_m=1.5         | v-rd   | 301.7e3  | 2e-3      |
      | m-20-flange-overhang-capped-en | M-20  | en    | l_m=2.0;t_m=0.24;n_ed_n=584e3;v_ed_n=200e3;fb_pa=20e6;fm_pa=10e6;gamma_m=1.5;flange_t_m=0.24;web_spacing_m=4;height_total_m=8;flanges=2;flange_len_m.0=0.5;flange_len_m.1=2.0 | flange-overhang | 0.97 | 1e-6      |
      | m-20-flanged-area-capped-en | M-20  | en    | l_m=2.0;t_m=0.24;n_ed_n=584e3;v_ed_n=200e3;fb_pa=20e6;fm_pa=10e6;gamma_m=1.5;flange_t_m=0.24;web_spacing_m=4;height_total_m=8;flanges=2;flange_len_m.0=0.5;flange_len_m.1=2.0 | flanged-area | 0.9456 | 1e-6      |
      | m-20-flanged-i-capped-en | M-20  | en    | l_m=2.0;t_m=0.24;n_ed_n=584e3;v_ed_n=200e3;fb_pa=20e6;fm_pa=10e6;gamma_m=1.5;flange_t_m=0.24;web_spacing_m=4;height_total_m=8;flanges=2;flange_len_m.0=0.5;flange_len_m.1=2.0 | flanged-i | 0.3452605856 | 1e-6      |
      | m-30-m-rd-en    | M-30  | en    | t_m=0.175;h_m=2.75;w_ed_pa=960;fxk1_pa=0.2e6;gamma_m=1.5                              | m-rd   | 680.6    | 2e-3      |
      | m-31-alpha1-en  | M-31  | en    | t_m=0.1;h_m=1.0;l_m=1.0;w_ed_pa=1;fxk1_pa=1.5e6;fxk2_pa=1.5e6;gamma_m=1.0             | alpha1 | 0.041667 | 1e-4      |
      | m-31-annex-e-en | M-31  | en    | t_m=0.1;h_m=1.0;l_m=1.0;w_ed_pa=1;fxk1_pa=1.5e6;fxk2_pa=1.5e6;gamma_m=1.0             | annex-e | 0.041667 | 1e-4     |
      | m-31-annex-e-entered | M-31 | en | t_m=0.1;h_m=1.0;l_m=1.0;w_ed_pa=1000;fxk1_pa=1.5e6;fxk2_pa=1.5e6;gamma_m=1.0;alpha1=0.05 | annex-e-m-ed | 2500 | 1e-9 |
      | m-70-chase-area-en | M-70 | en | t_m=0.365;chase_depth_m=0.03;chase_width_m=0.05;chase_length_m=1.2                  | chase-area | 0.06 | 1e-9 |
      | m-70-chase-depth-vertical-t115-de | M-70 | de | t_m=0.115;chase_depth_m=0.03;chase_width_m=0.05;chase_length_m=1.0;orientation:vertical | chase | 0.03 | 1e-9 |
      | m-70-chase-width-vertical-t115-de | M-70 | de | t_m=0.115;chase_depth_m=0.03;chase_width_m=0.05;chase_length_m=1.0;orientation:vertical | chase-width | 0.1 | 1e-9 |
      | m-70-chase-width-vertical-t175-en | M-70 | en | t_m=0.175;chase_depth_m=0.03;chase_width_m=0.12;chase_length_m=1.0;orientation:vertical | chase-width | 0.125 | 1e-9 |
      | m-70-chase-width-recess-t365-en | M-70 | en | t_m=0.365;chase_depth_m=0.03;chase_width_m=0.3;chase_length_m=0.4;orientation:recess | chase-width | 0.2 | 1e-9 |
      | m-70-chase-depth-horizontal-short-t365-de | M-70 | de | t_m=0.365;chase_depth_m=0.02;chase_width_m=0.05;chase_length_m=1.0;orientation:horizontal | chase | 0.03 | 1e-9 |
      | m-70-chase-depth-horizontal-long-t365-de | M-70 | de | t_m=0.365;chase_depth_m=0.02;chase_width_m=0.05;chase_length_m=2.0;orientation:horizontal | chase | 0.015 | 1e-9 |
      | m-70-chase-depth-horizontal-long-t240-en | M-70 | en | t_m=0.24;chase_depth_m=0.02;chase_width_m=0.05;chase_length_m=2.0;orientation:horizontal | chase | 0.01 | 1e-9 |
      | m-40-m-rd-en    | M-40  | en    | b_m=1.0;d_m=0.3;as_m2=600e-6;fyd_pa=435e6;fd_pa=5.957e6;m_ed_nm=50e3                  | m-rd   | 72.58e3  | 1e-3      |
      | m-81-phi-s-en   | M-81  | en    | t_m=0.24;hef_m=2.0625;fk_pa=5.654e6;gamma_m=1.5;n_ed_n=500e3                          | phi-s  | 0.7688   | 1e-3      |
      | m-81-n-rd-en    | M-81  | en    | t_m=0.24;hef_m=2.0625;fk_pa=5.654e6;gamma_m=1.5;n_ed_n=500e3                          | n-rd   | 695.4e3  | 2e-3      |
      | m-90-t-min-de   | M-90  | de    | t_m=0.365;rei=90;material:clay;mu_fi=0.7                                              | t-min  | 0.15     | 1e-9      |
      | m-45-lb-de      | M-45  | de    | diameter_m=0.012;provided_m=1.0;fyd_pa=435e6;gamma_m=1.5                              | lb     | 0.97875  | 1e-6      |
      | m-61-limit-de   | M-61  | de    | span_m=2.0;d_m=0.25;deflection_m=0.005;limit_ratio=250                                | lintel-deflection | 0.008 | 1e-9 |
      | m-03-fvk-en     | M-03  | en    | fb_pa=20e6;fm_pa=10e6;material:clay;mortar:gp;group=1;sigma_pa=0.3e6                   | fvk    | 4.2e5    | 1e-9      |
      | m-03-fvk-casi-en | M-03 | en    | fb_pa=20e6;fm_pa=10e6;material:calcium_silicate;mortar:gp;group=1;sigma_pa=0.3e6        | fvk    | 3.2e5    | 1e-9      |
      | m-03-fvk-casi-tl-en | M-03 | en   | fb_pa=20e6;fm_pa=10e6;material:calcium_silicate;mortar:tl;group=1;sigma_pa=0.3e6        | fvk    | 5.2e5    | 1e-9      |
      | m-11-slenderness-de | M-11 | de | hef_m=2.0625;simplified=1;t1_m=0.1;t2_m=0.1;t_m=0.365 | slenderness | 16.3700733484 | 1e-5 |
      | m-15-pier-compression-de | M-15 | de | density=1800;fd_pa=5957000;n_pier_n=200000;opening_w_m=1.2;pier_l_m=1;t_m=0.365 | pier-compression | 1956874.5 | 1e-5 |
      | m-15-lintel-load-de | M-15 | de | density=1800;fd_pa=5957000;n_pier_n=200000;opening_w_m=1.2;pier_l_m=1;t_m=0.365 | lintel-load | 2320.2612 | 1e-5 |
      | m-15-lintel-moment-de | M-15 | de | density=1800;fd_pa=5957000;n_pier_n=200000;opening_w_m=1.2;pier_l_m=1;t_m=0.365 | lintel-moment | 464.05224 | 1e-5 |
      | m-32-q-lat-de | M-32 | de | fd_pa=5957000;la_m=2;t_m=0.365;w_ed_pa=960 | q-lat | 198405.33125 | 1e-5 |
      | m-34-n-min-de | M-34 | de | fd_pa=5957000;h_m=2.75;he_m=2;hydrostatic=0;n_max_n=400000;n_min_n=100000;qk_pa=2000;rho_e_n_m3=18000;t_m=0.365 | n-min | 27123.2876712 | 1e-5 |
      | m-34-n-max-de | M-34 | de | fd_pa=5957000;h_m=2.75;he_m=2;hydrostatic=0;n_max_n=400000;n_min_n=100000;qk_pa=2000;rho_e_n_m3=18000;t_m=0.365 | n-max | 978437.25 | 1e-5 |
      | m-01-delta-de | M-01 | de | fb_decl_pa=25000000;hole_pct=20;unit_h_m=0.25;unit_w_m=0.175 | delta | 1.3 | 1e-5 |
      | m-01-fb-de | M-01 | de | fb_decl_pa=25000000;hole_pct=20;unit_h_m=0.25;unit_w_m=0.175 | fb | 32500000 | 1e-5 |
      | m-04-fxk1-de | M-04 | de | fm_pa=10000000;gamma_m=1.5;sigma_pa=300000;material:clay | fxk1 | 100000 | 1e-5 |
      | m-04-fxk2-de | M-04 | de | fm_pa=10000000;gamma_m=1.5;sigma_pa=300000;material:clay | fxk2 | 400000 | 1e-5 |
      | m-04-mu-de | M-04 | de | fm_pa=10000000;gamma_m=1.5;sigma_pa=300000;material:clay | mu | 0.25 | 1e-5 |
      | m-41-v-rd1-de | M-41 | de | asw_m2=0.0001;av_m=0.5;b_m=1;d_m=0.3;fd_pa=5957000;fvk_pa=300000;fyd_pa=435000000;s_m=0.2;v_ed_n=50000 | v-rd1 | 72000 | 1e-5 |
      | m-41-v-rd2-de | M-41 | de | asw_m2=0.0001;av_m=0.5;b_m=1;d_m=0.3;fd_pa=5957000;fvk_pa=300000;fyd_pa=435000000;s_m=0.2;v_ed_n=50000 | v-rd2 | 58725 | 1e-5 |
      | m-41-v-rd-de | M-41 | de | asw_m2=0.0001;av_m=0.5;b_m=1;d_m=0.3;fd_pa=5957000;fvk_pa=300000;fyd_pa=435000000;s_m=0.2;v_ed_n=50000 | v-rd | 130725 | 1e-5 |
      | m-43-col-area-de | M-43 | de | beam_a_m2=0.0375;col_a_m2=0.0625;col_as_m2=0.000452;col_min_m=0.25;col_spacing_m=4;fd_pa=5957000;fvk_pa=300000;l_m=3;n_ed_n=200000;t_m=0.365;v_ed_n=100000 | col-area | 0.0625 | 1e-5 |
      | m-43-col-reinforcement-de | M-43 | de | beam_a_m2=0.0375;col_a_m2=0.0625;col_as_m2=0.000452;col_min_m=0.25;col_spacing_m=4;fd_pa=5957000;fvk_pa=300000;l_m=3;n_ed_n=200000;t_m=0.365;v_ed_n=100000 | col-reinforcement | 0.000452 | 1e-5 |
      | m-43-shear-de | M-43 | de | beam_a_m2=0.0375;col_a_m2=0.0625;col_as_m2=0.000452;col_min_m=0.25;col_spacing_m=4;fd_pa=5957000;fvk_pa=300000;l_m=3;n_ed_n=200000;t_m=0.365;v_ed_n=100000 | shear | 219000 | 1e-5 |
      | m-44-p-eff-de | M-44 | de | a_m2=0.365;ap_m2=0.00014;e_pa=6000000000;ep_pa=195000000000;eps_cs=0.0003;fd_pa=5957000;fpk_pa=1860000000;fxk1_pa=200000;n_ed_n=300000;p0_n=200000;phi_inf=1.5;rho_relax=0.025;z_m3=0.06 | p-eff | 183070.273973 | 1e-5 |
      | m-50-tie-force-de | M-50 | de | cavity_m=0.08;h_wall_m=2.75;n_per_m2=3;t1_m=0.1;t2_m=0.1;tie_rd_n=1000;w_ed_pa=960 | tie-force | 160 | 1e-5 |
      | m-50-tie-force-second-leaf-de | M-50 | de | cavity_m=0.08;h_wall_m=2.75;n_per_m2=3;t1_m=0.1;t2_m=0.175;tie_rd_n=1000;w_ed_pa=800 | tie-force | 41.93284 | 1e-5 |
      | m-51-anchor-de | M-51 | de | anchor_rk_n=5000;gamma_m=1.5;h_trib_m=2.5;ring_tie_n=20000;s_anchor_m=1.2;w_ed_pa=960 | anchor | 3333.33333333 | 1e-5 |
      | m-51-ring-tie-de | M-51 | de | anchor_rk_n=5000;gamma_m=1.5;h_trib_m=2.5;ring_tie_n=20000;s_anchor_m=1.2;w_ed_pa=960 | ring-tie | 20000 | 1e-5 |
      | m-80-height-limit-de | M-80 | de | h_m=2.75;height_total_m=2.75;l_m=3;qk_pa=2000;span_m=2;storeys=3;t_m=0.365 | height-limit | 4.38 | 1e-5 |
      | m-80-thickness-limit-de | M-80 | de | h_m=2.75;height_total_m=2.75;l_m=3;qk_pa=2000;span_m=2;storeys=3;t_m=0.365 | thickness-limit | 0.365 | 1e-5 |
      | m-82-lc-min-de | M-82 | de | fvk0_pa=200000;gamma_m=1.5;lc_m=1.5;sigma_d_pa=500000;t_m=0.365;v_ed_n=50000 | lc-min | 1.5 | 1e-5 |
      | m-82-v-rd-de | M-82 | de | fvk0_pa=200000;gamma_m=1.5;lc_m=1.5;sigma_d_pa=500000;t_m=0.365;v_ed_n=50000 | v-rd | 146000 | 1e-5 |
      | m-83-wind-panel-de | M-83 | de | basement=0;fd_pa=5957000;fxk1_pa=200000;gamma_m=1.5;h_m=2.75;he_m=2;n_max_n=400000;n_min_n=100000;rho_e_n_m3=18000;sigma_d_pa=500000;t_m=0.365;w_ed_pa=1000 | wind-panel | 14062.6388889 | 1e-5 |
      | m-91-t-red-de | M-91 | de | a_theta_m=0.03;fd_pa=5957000;k_theta=0.5;l_m=3;n_ed_fi_n=150000;phi=0.8;t_m=0.365 | t-red | 0.305 | 1e-5 |
      | m-91-n-rd-fi-de | M-91 | de | a_theta_m=0.03;fd_pa=5957000;k_theta=0.5;l_m=3;n_ed_fi_n=150000;phi=0.8;t_m=0.365 | n-rd-fi | 2180262 | 1e-5 |
      | m-95-q-min-de | M-95 | de | k_diaphragm_n_m=50000000;q=2;system:confined | q-min | 2 | 1e-5 |
      | m-95-q-max-de | M-95 | de | k_diaphragm_n_m=50000000;q=2;system:confined | q-max | 3 | 1e-5 |
      | m-16-phi-reduction-de | M-16 | de | fd_pa=5957000;h_m=2.75;hef_m=2.0625;n_mid_n=200000;phi_inf=1.5;t_m=0.365;w_ed_pa=960 | phi-reduction | 1 | 1e-5 |
      | m-16-n-mid-wind-de | M-16 | de | fd_pa=5957000;h_m=2.75;hef_m=2.0625;n_mid_n=200000;phi_inf=1.5;t_m=0.365;w_ed_pa=960 | n-mid-wind | 1925705.38978 | 1e-5 |
      | m-21-centre-de | M-21 | de | b_m=1;e0_m=0.01;e_pa=6000000000;h_m=2.75;l_m.0=2;l_m.1=3;t_m.0=0.365;t_m.1=0.365;v_ed_n=200000;vrd_n.0=150000;vrd_n.1=200000;x_m.0=0;x_m.1=2 | centre | 1.4405965203 | 1e-5 |
      | m-21-governing-wall-de | M-21 | de | b_m=1;e0_m=0.01;e_pa=6000000000;h_m=2.75;l_m.0=2;l_m.1=3;t_m.0=0.365;t_m.1=0.365;v_ed_n=200000;vrd_n.0=150000;vrd_n.1=200000;x_m.0=0;x_m.1=2 | governing-wall | 200000 | 1e-5 |
      | m-21-flexible-centre-de | M-21 | de | b_m=1;diaphragm:flexible;e0_m=0.01;e_pa=6000000000;edge_hi_m=6;edge_lo_m=0;h_m=2.75;l_m.0=2;l_m.1=3;l_m.2=3;t_m.0=0.365;t_m.1=0.365;t_m.2=0.365;trib.0=0.1666666666667;trib.1=0.5;trib.2=0.3333333333333;v_ed_n=200000;vrd_n.0=30000;vrd_n.1=120000;vrd_n.2=70000;x_m.0=0;x_m.1=2;x_m.2=6 | centre | 3.3496436139 | 1e-5 |
      | m-21-flexible-governing-wall-de | M-21 | de | b_m=1;diaphragm:flexible;e0_m=0.01;e_pa=6000000000;edge_hi_m=6;edge_lo_m=0;h_m=2.75;l_m.0=2;l_m.1=3;l_m.2=3;t_m.0=0.365;t_m.1=0.365;t_m.2=0.365;trib.0=0.1666666666667;trib.1=0.5;trib.2=0.3333333333333;v_ed_n=200000;vrd_n.0=30000;vrd_n.1=120000;vrd_n.2=70000;x_m.0=0;x_m.1=2;x_m.2=6 | governing-wall | 30000 | 1e-5 |
      | m-21-rigid-governing-wall-de | M-21 | de | b_m=1;e0_m=0.01;e_pa=6000000000;h_m=2.75;l_m.0=2;l_m.1=3;l_m.2=3;t_m.0=0.365;t_m.1=0.365;t_m.2=0.365;v_ed_n=200000;vrd_n.0=30000;vrd_n.1=120000;vrd_n.2=70000;x_m.0=0;x_m.1=2;x_m.2=6 | governing-wall | 70000 | 1e-5 |
      | m-05-lb-de | M-05 | de | diameter_m=0.012;fbok_pa=2000000;fyd_pa=435000000;gamma_m=1.5;lb_avail_m=0.8 | lb | 0.97875 | 1e-5 |
      | m-05-lb-avail-de | M-05 | de | diameter_m=0.012;fbok_pa=2000000;fyd_pa=435000000;gamma_m=1.5;lb_avail_m=0.8 | lb-avail | 0.8 | 1e-5 |
      | m-07-gamma-m-de | M-07 | de | class=2;situation:persistent | gamma-m | 1.7 | 1e-5 |
      | m-07-gamma-s-de | M-07 | de | class=2;situation:persistent | gamma-s | 1.15 | 1e-5 |
      | m-07-gamma-m-fire-de | M-07 | de | class=2;situation:persistent | gamma-m-fire | 1 | 1e-5 |
      | m-42-m-rd-n-de | M-42 | de | b_m=1;bar_a_m2.0=0.0002;bar_a_m2.1=0.0002;bar_y_m.0=0.05;bar_y_m.1=0.35;fd_pa=5957000;fyd_pa=435000000;h_m=2.75;l_m=3;m_ed_nm=50000;n_ed_n=100000 | m-rd-n | 174848.049155 | 1e-5 |
      | m-52-spacing-de | M-52 | de | spacing_m=12;material:clay | spacing | 12 | 1e-5 |
      | m-60-mortar-de | M-60 | de | cover_m=0.03;fm_pa=10000000;frost_class=2;exposure:mx2;material:clay | mortar | 10000000 | 1e-5 |
      | m-60-frost-de | M-60 | de | cover_m=0.03;fm_pa=10000000;frost_class=2;exposure:mx2;material:clay | frost | 2 | 1e-5 |
      | m-60-cover-de | M-60 | de | cover_m=0.03;fm_pa=10000000;frost_class=2;exposure:mx2;material:clay | cover | 0.03 | 1e-5 |
      | m-33-we-de | M-33 | de | cpe=0.8;gamma_soil_n_m3=18000;he_m=2;k0=0.5;qp_pa=800 | we | 640 | 1e-5 |
      | m-33-e0-de | M-33 | de | cpe=0.8;gamma_soil_n_m3=18000;he_m=2;k0=0.5;qp_pa=800 | e0 | 18000 | 1e-5 |
      | m-33-earth-resultant-de | M-33 | de | cpe=0.8;gamma_soil_n_m3=18000;he_m=2;k0=0.5;qp_pa=800 | earth-resultant | 18000 | 1e-5 |
