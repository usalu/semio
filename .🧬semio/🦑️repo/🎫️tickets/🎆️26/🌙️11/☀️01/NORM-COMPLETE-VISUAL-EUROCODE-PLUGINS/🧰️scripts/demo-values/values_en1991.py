"""Demo values of the EN 1991 catalogue beyond the vocabulary examples: a plausible two-storey office building with a 20 m duopitch roof in a temperate site, a 40 m concrete road bridge, a 6 m concrete silo section. Physically plausible demo inputs, explicitly not normative. Lookup order: `ENTRY:key`, `key@scope`, `key`."""
V = {
    "a_ref": 30.0, "a_t": 130.0, "a_v": 5.0, "altitude": 300.0, "b": 12.0, "E11.08:b": 0.2, "E11.09:b": 0.2,
    "c_d": 0.95, "c_dir": 1.0, "c_e": 1.0, "c_esl": 2.0, "c_f": 1.3, "c_f0": 1.3, "c_fr": 0.04, "c_fx": 1.3,
    "c_o": 1.0, "c_pe": 0.8, "c_pi": -0.3, "c_s": 0.9, "c_season": 1.0, "c_t": 1.0, "ceiling_height": 3.0,
    "charge_kg": 100.0, "dc": 6.0, "delta": 0.1, "delta_q2": 1.0, "delta_s": 0.05, "distance": 10.0,
    "dominant_ratio": 1.0, "dt": 15.0, "dt_m": 10.0, "dt_n": 20.0, "duration_days": 60.0, "eps_m": 0.8,
    "fire_area": 20.0, "floor_area": 400.0, "frequency": 2.0, "gamma_g": 1.35, "gamma_q": 1.5,
    "h": 10.0, "E15.05:h": 1.5, "E13.05:h": 1.5, "E13.06:h": 2.0,
    "h_eq": 2.0, "h_over_d": 0.5, "hc": 12.0, "height": 10.0, "E12.04:height": 3.0, "E12.09:height": 4.0, "hp_over_h": 0.05,
    "k@E4.03": 0.5, "k@E4.04": 0.5, "k@E4.05": 0.5, "k_b": 0.09, "k_sur_cool": 1.0, "k_sur_heat": 1.0,
    "key_area": 20.0, "l_phi": 40.0, "length": 40.0, "E15.05:length": 6.0, "E15.06:length": 6.0, "E14.11:length": 20.0,
    "loaded_length": 40.0, "mu@E4.03": 0.4, "mu@E4.04": 0.4, "mu@E4.05": 0.4, "n_1": 1.5, "n_years": 50.0,
    "north_german_lowland": 1.0, "phi_1": 1.1, "phi_7": 1.5, "psi": 0.5, "psi_lambda": 0.9,
    "q_fd_mj": 600.0, "q_fk_mj": 511.0, "q_p": 800.0, "radius": 300.0, "rho": 1.25, "resistance_min": 90.0,
    "s_k": 650.0, "skew_rad": 0.0, "slab_thickness": 0.25, "span": 40.0, "speed": 15.0, "E17.08:speed": 15.0,
    "st": 0.2, "standoff": 30.0, "storeys": 3.0, "t_0_c": 10.0, "t_f_c": 800.0, "t_lim_min": 20.0,
    "t_max_c": 35.0, "t_min_c": -15.0, "theta_g_c": 600.0, "theta_m_c": 300.0, "thickness": 0.25,
    "thickness2": 0.06, "thickness3": 0.02, "v_b": 25.0, "v_b0": 25.0, "v_h": 0.5, "v_kmh": 100.0, "v_m": 20.0,
    "wall_height": 2.8, "wall_thickness": 0.175, "width": 12.0, "E2.01:width": 7.0, "E2.02:width": 7.0, "E2.12:width": 3.0, "E12.09:width": 4.0,
    "z": 10.0, "E4.03:z": 6.0, "E12.05:z": 3.0, "E14.02:z": 10.0,
}
VECTORS = {
    "consequence@E17.07": [2.0e6, 5.0e6], "p_d_h@E17.07": [0.1, 0.05], "p_h@E17.07": [0.01, 0.001],
    "q_ci@E16.06": [20000.0, 15000.0, 30000.0], "temps_c@E12.10": [20.0, 400.0, 700.0, 900.0, 800.0],
    "times_min@E12.10": [0.0, 5.0, 15.0, 45.0, 90.0],
}
