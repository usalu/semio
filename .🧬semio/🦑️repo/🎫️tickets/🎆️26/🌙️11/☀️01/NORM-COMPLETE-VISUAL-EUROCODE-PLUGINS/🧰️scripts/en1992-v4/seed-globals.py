"""Writes demo example values (column 5) and corrected units (column 4) into the EN 1992 input vocabulary source, then regenerates the vocabulary. Values are seeded demo inputs, not normative. Run: python3 -I seed-globals.py"""
import pathlib, subprocess, sys
HERE = pathlib.Path(__file__).resolve().parent
TSV = HERE.parent / "catalogue-bridge-en1992-inputs.tsv"
OUT = next(p for p in pathlib.Path("/home/deck/semio").glob("✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🗄️registry/📚️inputs"))
G = {
 "fck": 30, "fck_t": 20, "fyk": 500, "es": 200000, "k": 1.08, "eps_uk": 50, "fuk": 800, "fpk": 1860, "fp01k": 1640, "ep": 195000, "ecm": 33000, "ecm_t": 32000, "e_p": 100,
 "b": 300, "h": 500, "d": 450, "d2": 50, "bw": 300, "bf": 1200, "hf": 150, "hw": 400, "bt": 300, "bi": 300, "b_e": 300, "z": 405, "z0": 300, "c": 35, "c0": 35, "c1": 400, "c2": 400, "a": 40, "az": 40, "a_c": 150,
 "d_nom": 16, "d_f": 250, "t_f": 150, "t_w": 12, "h_s": 400, "h_ct": 100, "h_ed": 50, "h_min": 150, "h_member": 500, "hcr": 250, "hd_over_h": 10, "hef": 100, "hef_eff": 100, "hf": 150,
 "l": 6000, "l0": 6000, "l0t": 6000, "l0_fi": 3000, "l_m": 6, "l_available": 800, "l_bd": 600, "l_s": 500, "span": 9000, "e0": 20, "e_d": 50, "e_tot": 50, "e1": 100, "e2": 100, "e_nx": 0, "e_ny": 0, "e_par": 100, "e_v": 0,
 "u": 1600, "u1_reduced": 1600, "x": 150, "x_min": -100, "theta": 500, "x_max": 100, "y_min": -100, "y_max": 100, "x_compression": 150, "y_compression": 150, "z_compression": 150, "z_h": 300, "z_i": 100, "xu_d": 0.3, "xy_d": 0.2, "y_n": 150, "x_max": 100,
 "w": 100, "w_el": 12000, "w_before": 5, "wk": 0.3, "wk_lim": 0.3, "sag": 40, "slip": 6, "radius": 5000, "radius_m": 5, "r_m": 5, "capital_l": 0, "opening_w": 0, "opening_a": 0, "opening_clear": 0, "wheel_print": 400, "width": 300, "width_m": 6, "wall_length_m": 8, "phi_m": 0.02, "pm_t_per_m": 100000, "alpha_m": 0.02,
 "dim": 300, "dg": 16, "dh": 30, "clear": 60, "clear_distance": 60, "delta_x": 1000, "s": 200, "s2": 200, "s_surf": 200, "sr": 300, "st": 300, "sr_max": 300, "sr_y": 300, "sr_z": 300, "amp_y": 10, "amp_z": 10, "t_min": 90,
 "as": 1571, "as_provided": 1571, "asl": 1257, "as2": 628, "ap": 1000, "as_main": 1000, "as_secondary": 400, "as_span": 600, "as_support": 600, "as_through": 400, "as_req": 1200, "as_surf": 1500, "as_surf_provided": 1500, "asv": 400, "asw": 157, "asw_s": 0.9, "asw_s_provided": 0.9, "a_b": 200, "a_t": 600, "a_links": 400, "ac": 90000, "ac0": 40000, "ac1": 360000, "ac_per_m": 1000, "act": 40000, "act_ext": 3000, "sum_ast": 600, "sum_ast_min": 300, "u": 1600,
 "m_ed": 250e6, "m_edy": 150e6, "m_edz": 120e6, "m_qp": 150e6, "m_char": 180e6, "m_max": 200e6, "m_min": 50e6, "m_total": 600e6, "m_x": 2e6, "m_y": 1e6, "m_0ed": 100e6, "m_edge": 50e6, "m_ed_fi": 100e6, "m1": 100e6, "m2": 200e6, "m_rdy": 235.6e6, "m_rdz": 235.6e6, "mx": 20000, "my": 20000, "mxy": 5000, "t_ed": 20e6, "bimoment": 4e9,
 "n_ed": 500e3, "n_rd": 3574e3, "n_ed_fi": 500e3, "n_ed_re": 100e3, "nx": 100, "ny": 50, "nxy": 20, "nz": 0, "sx": 2, "sy": 1, "txy": 3, "t": 200,
 "v_ed": 200e3, "v_ed0": 100e3, "v_ccd": 0, "v_td": 0, "v_max": 200e3, "v_min": 100e3, "v_rd_c": 80e3, "v_rd_cp": 80e3, "v_rd_max": 500e3, "v_rd_s": 100e3, "f_v_ed": 100e3, "f_ed": 300e3, "f_td": 150e3, "f_bt": 20e3, "f_cr": 100e3, "q_wheel": 150e3, "strut_force": 300e3, "n_s": 5, "p": 500e3, "pm0": 500e3, "pm_t": 450e3, "pmax": 1e6, "p_force": 500e3, "n_rd_s": 100e3, "n_rd_p": 100e3, "n_rd_c": 100e3, "n_rd_sp": 100e3, "n_rd_cb": 100e3, "n_rk_p": 80e3, "n_rk_p0": 80e3, "delta_n_ed": 5e3, "delta_n_rk_s_0": 40e3,
 "sigma_c": 10, "sigma_cp": 1, "sigma_s": 250, "sigma_sd": 250, "sigma_p": 1000, "sigma_pi": 1300, "sigma_pm0": 1400, "sigma_pm_inf": 1100, "sigma_jack": 1400, "sigma_c_max": 12, "sigma_c_min": 2, "sigma_c_qp": 8, "sigma_gd": 0.5, "sigma_n": 0.5, "sigma_s_allowed": 200, "delta_sigma": 30, "delta_sigma_c": 3, "delta_sigma_equ": 120, "delta_sigma_71": 162.5, "delta_sigma_tendon": 80, "fbd": 3, "fct_eff": 2.9, "fctd": 1.2, "fctd_t": 1.1, "tendon_angle": 0, "alpha_deg": 90, "alpha_v_deg": 0, "theta_m": 300, "theta_pl": 0.01, "step_deg": 15, "r_class": 90, "mu_fi": 0.7, "delta_t": 20, "design_life": 50, "t_hours": 1000, "psi_fi": 0.5,
 "rh": 50, "h0": 187.5, "t0": 28, "ts": 3, "phi_inf": 2.4, "eps": 2.0, "eps_ud": 20, "eps_ud_p": 20, "eps_ca": 5e-5, "eps_cd": 1e-4, "eps_cs": 3e-4, "eps_free": 2e-4, "eps1": 0.5, "eps2": 1.5, "alpha_c": 0.00001, "alpha_e": 6.06, "alpha_l": 1, "alpha1": 1, "alpha3": 1, "alpha_seis": 0.8, "beta": 1.15, "beta_cc_t0": 0.6, "beta_computed": 1.15, "delta": 0.8, "delta_fd": 1, "delta_v": 0.5, "chi": 0.8, "xi": 0.5, "xi_lim": 0.617, "ks": 0.8, "kc": 0.4, "k1": 0.8, "k2": 0.8, "k_conf": 0.7, "lambda_y": 40, "lambda_z": 40, "lambda_shear": 3, "lambda_max": 1.4, "lambda1": 0.8, "lambda2": 1, "lambda3": 1, "lambda4": 1, "phi_fat": 1.0, "gamma_f_fat": 1.0, "gamma_g": 1.35, "gamma_q": 1.5, "gamma_inst": 1.2, "gk": 20, "qk": 10, "cot_theta": 2, "cot_theta_f": 1.2, "cot_alpha": 0, "mu": 0.19, "rho": 0.002, "rho1": 50, "rho1000": 2.5, "rho_ly": 0.01, "rho_lz": 0.01, "beff_over_bw": 1, "q": 20, "pressure": 50, "i": 1, "li": 1, "ic": 8e9, "i_b": 1, "rm": 1, "ei": 1e13, "sum_ecd_ic": 1e14,
 "phi": 20, "phi_l": 25, "phi_s": 16, "phi_p": 12, "phi_ef": 2, "cd": 45, "nb": 1, "n_bars": 4, "bars": 4, "at_laps": 50, "n_tendons": 4, "steps": 20, "points": 24, "kappa_points": 12, "m": 1, "ni": 4, "dc_gamma": 0, "dc_st": 0, "dc_add": 0, "dc_dev": 10, "c_provided": 45, "zcp": 100, "air_entrained": 0, "u1_reduced": 1600, "tw": 200, "w_strut": 100, "node_x": 0, "node_y": 0, "psi_h_nb": 1, "s_first": 0, "p_transverse": 0, "psi_h_nb": 1, "rax": 0.5, "bisect": 40, "member_i": 0, "member_j": 1, "support_node": 0, "support_dof": 0, "load_x": 0, "load_y": 100e3, "eps_cu": 3.5,
}
FLAGS0 = ["box", "brittle_partitions", "circular", "decompression", "dense_reinforcement", "fatigue", "full_discontinuity", "grout", "imposed_only", "inclined", "is_moment", "isolated", "lever", "narrowing", "nonlinear_creep_proven", "precast", "prestressed", "prestressed_bonded", "pure_tension", "slab", "special", "tension_flange", "thin_early_age", "transient", "welded", "confinement", "cracked", "splitting_reinforcement", "direct_support"]
for k in FLAGS0: G[k] = 0
for k, v in {"cracked": 1, "splitting_reinforcement": 1, "direct_support": 0}.items(): G[k] = v
UNITS = {"phi": "mm", "phi_l": "mm", "phi_s": "mm", "phi_p": "mm", "p": "N", "p_force": "N", "pm0": "N", "pm_t": "N", "pmax": "N", "cd": "mm", "t_f": "mm", "t_w": "mm", "t_ed": "Nmm", "tw": "mm", "ic": "mm4", "rho1": "1", "s_surf": "mm", "hd_over_h": "1", "sx": "1", "sy": "1", "txy": "1", "w_strut": "mm", "wheel_print": "mm", "eps_uk": "1", "zcp": "mm", "node_x": "mm", "node_y": "mm", "load_x": "N", "load_y": "N", "sum_ecd_ic": "1", "t0": "1", "ts": "1", "ei": "1"}
lines = TSV.read_text(encoding="utf-8").splitlines()
out, n, seen = [], 0, set()
for line in lines:
    if not line or line.startswith(("#", "@")):
        out.append(line); continue
    f = line.split("|"); f += [""] * (10 - len(f))
    key, kind = f[0], f[2]
    if kind == "n":
        if key in UNITS: f[3] = UNITS[key]
        if key in G:
            f[4] = repr(float(G[key])); n += 1; seen.add(key)
    out.append("|".join(f))
TSV.write_text("\n".join(out) + "\n", encoding="utf-8")
print(n, "examples written; unused dict keys:", sorted(set(G) - seen))
subprocess.check_call([sys.executable, "-I", str(HERE.parent / "catalogue-bridge-gen-inputs.py"), str(OUT), str(TSV), "s.norm.en1992"])
