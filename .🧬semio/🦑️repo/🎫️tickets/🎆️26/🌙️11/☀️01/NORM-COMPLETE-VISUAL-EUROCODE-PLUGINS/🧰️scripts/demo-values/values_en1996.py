"""Demo values of the EN 1996 catalogue beyond the vocabulary examples: a 365 mm clay masonry wall 2.75 m high (unit strength 20 MPa, general purpose mortar 10 MPa, execution class 2), lintels, a 3 m two-way panel and a 24 m long building. SI units as the vocabulary. Physically plausible demo inputs, explicitly not normative. Lookup order: `ENTRY:key`, `key@scope`, `key`."""
V = {
    "a1_m": 0.5, "a_m2": 0.365, "M-81:a_m2": 0.24, "a_theta_m": 0.03, "ab_m2": 0.073, "alpha1": 0.05, "anchor_rk_n": 5000.0,
    "ap_m2": 140e-6, "beam_a_m2": 0.0375, "as_m2": 600e-6, "asw_m2": 100e-6, "av_m": 0.5, "b_m": 1.0, "basement": 0.0, "bearing_m": 0.2,
    "bed_joint_m": 0.012, "cavity_m": 0.08, "chase_depth_m": 0.03, "chase_length_m": 1.2, "chase_width_m": 0.05, "chase_x_m": 1.0,
    "chase_y_m": 0.5, "class": 2.0, "col_a_m2": 0.0625, "col_as_m2": 452e-6, "col_min_m": 0.25, "col_spacing_m": 4.0, "cover_m": 0.03,
    "cpe": 0.8, "d_m": 0.3, "M-61:d_m": 0.25, "deflection_m": 0.005, "diameter_m": 0.012, "e0_m": 0.01, "e_pa": 6.0e9,
    "ehe_m": 0.02, "ep_pa": 195e9, "eps_cs": 3e-4, "fb_decl_pa": 25e6, "fb_pa": 20e6, "fbok_pa": 2e6, "fd_pa": 5.957e6,
    "fk_pa": 8.935e6, "M-81:fk_pa": 5.654e6, "flange_t_m": 0.1, "flanges": 1.0, "fm_pa": 10e6, "fpk_pa": 1860e6, "frost_class": 2.0,
    "fvk0_pa": 0.2e6, "fvk_pa": 0.3e6, "fxk1_pa": 0.2e6, "fxk2_pa": 0.6e6, "M-31:fxk1_pa": 1.5e6, "M-31:fxk2_pa": 1.5e6, "fyd_pa": 435e6,
    "gamma_m": 1.5, "gamma_soil_n_m3": 18000.0, "group": 1.0, "h_m": 2.75, "M-31:h_m": 1.0, "h_trib_m": 2.5, "h_wall_m": 2.75,
    "hc_m": 2.75, "he_m": 2.0, "hef_m": 2.0625, "height_total_m": 2.75, "hole_pct": 20.0, "hydrostatic": 0.0, "joint_x_m": 6.0,
    "k0": 0.5, "k_e": 1000.0, "k_diaphragm_n_m": 5.0e7, "k_theta": 0.5, "l_m": 3.0, "M-20:l_m": 2.0, "M-31:l_m": 1.0, "la_m": 2.0, "lb_avail_m": 0.8,
    "lc_m": 1.5, "lefm_m": 1.0, "limit_ratio": 250.0, "m_bot_nm": 2000.0, "m_ed_nm": 50e3, "m_hm_nm": 1000.0, "m_mid_nm": 2500.0,
    "m_top_nm": 4000.0, "mu_fi": 0.7, "n_bot_n": 200e3, "n_ed_fi_n": 150e3, "n_ed_n": 500e3, "M-20:n_ed_n": 584e3, "M-42:n_ed_n": 100e3,
    "M-43:n_ed_n": 200e3, "M-44:n_ed_n": 300e3, "n_max_n": 400e3, "n_mid_n": 200e3, "n_min_n": 100e3, "n_per_m2": 3.0, "n_pier_n": 200e3,
    "n_top_n": 200e3, "ned_c_n": 500e3, "opening_w_m": 1.2, "overlap_m": 0.12, "p0_n": 200e3, "phi": 0.8, "phi_inf": 1.5,
    "pier_area_m2": 0.4, "pier_l_m": 1.0, "plaster_m": 0.015, "provided_m": 1.0, "q": 2.0, "qk_pa": 2000.0, "qp_pa": 800.0,
    "rei": 90.0, "rho_e_n_m3": 18000.0, "rho_relax": 0.025, "ring_tie_n": 20000.0, "s_anchor_m": 1.2, "s_m": 0.2, "sides": 2.0,
    "sigma_d_pa": 0.5e6, "sigma_pa": 0.3e6, "sigma_s_pa": 200e6, "simplified": 1.0, "spacing_m": 12.0, "span_m": 2.0,
    "stiff_len_m": 1.0, "stiff_t_m": 0.175, "storeys": 3.0, "t1_m": 0.1, "t2_m": 0.1, "t_m": 0.365, "M-30:t_m": 0.175,
    "M-31:t_m": 0.1, "tie_rd_n": 1000.0, "unit_h_m": 0.25, "M-70:unit_h_m": 0.238, "unit_w_m": 0.175, "v_ed_n": 200e3,
    "M-41:v_ed_n": 50e3, "M-43:v_ed_n": 100e3, "M-82:v_ed_n": 50e3, "w_ed_pa": 960.0, "M-31:w_ed_pa": 1000.0, "M-83:w_ed_pa": 1000.0,
    "web_spacing_m": 3.0, "z_m3": 0.06,
}
EXTRA = {"M-15": {"density": 1800.0}}
TEXTS = {"edges": "SSSS", "orientation": "vertical", "system": "confined", "wall": "wall-1"}
VECTORS = {
    "bar_a_m2@M-42": [200e-6, 200e-6], "bar_y_m@M-42": [0.05, 0.35],
    "l_m@M-21": [2.0, 3.0], "t_m@M-21": [0.365, 0.365], "vrd_n@M-21": [150e3, 200e3], "x_m@M-21": [0.0, 2.0],
}
