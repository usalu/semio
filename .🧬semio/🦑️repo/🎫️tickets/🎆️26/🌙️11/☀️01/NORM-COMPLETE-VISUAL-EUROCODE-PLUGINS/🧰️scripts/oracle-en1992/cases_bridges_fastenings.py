def c(id, annex="De", tol=1e-6, **groups):
    return {"id": id, "annex": annex, "tol": tol, **groups}

ANCH = {"fck": 30.0, "hef": 120.0, "cracked": 1.0, "dense_reinforcement": 0.0}
CASES = [
    c("BR-01", num={"fck": 45.0, "prestressed": 1.0}), c("BR-01", "En", num={"fck": 80.0}),
    c("BR-02", num={"fck": 35.0, "phi": 20.0, "design_life": 100.0, "c_provided": 60.0}, text={"exposure": "XD3"}), c("BR-02", "En", num={"fck": 45.0, "phi": 25.0, "precast": 1.0, "c_provided": 55.0}, text={"exposure": "XC4"}),
    c("BR-03", num={"fck": 30.0, "bw": 300.0, "z": 405.0, "v_ed": 200e3, "bi": 800.0, "sigma_n": 0.5, "rho": 0.003}, text={"roughness": "rough"}),
    c("BR-04", num={"lambda1": 0.8, "lambda2": 1.1, "lambda3": 0.9, "lambda4": 1.0, "delta_sigma_71": 70.0, "delta_sigma_tendon": 90.0}, text={"kind": "straight", "tendon_kind": "post_curved_steel"}),
    c("BR-05", num={"fck": 30.0, "b": 1000.0, "h": 400.0, "wk": 0.25, "n_ed": 0.0, "act": 200000.0, "fyk": 500.0}, text={"exposure": "XC4"}), c("BR-05", num={"fck": 30.0, "b": 1000.0, "h": 400.0, "act": 200000.0, "prestressed_bonded": 1.0}, text={"exposure": "XC1"}),
    c("BR-06", num={"m_total": 900e6, "p": 2.0e6, "e_p": 300.0, "sag": 0.2, "l": 8.0}),
    c("BR-07", num={"fck": 35.0, "d": 250.0, "h": 300.0, "q_wheel": 300e3, "wheel_print": 400.0, "rho_ly": 0.01, "rho_lz": 0.01}), c("BR-07", "En", num={"fck": 35.0, "d": 250.0, "h": 300.0, "q_wheel": 300e3, "rho_ly": 0.01, "rho_lz": 0.01}),
    c("BR-08", num={**ANCH, "b": 800.0, "h": 800.0, "n_ed": 3000e3, "l0": 9000.0, "as": 2000.0}),
    c("LQ-01", num={"class": 1.0, "hd_over_h": 20.0, "h": 400.0, "wk": 0.1}, text={"exposure": "XC2"}), c("LQ-01", num={"class": 3.0, "hd_over_h": 40.0, "h": 150.0, "x_compression": 40.0}), c("LQ-01", num={"class": 0.0, "hd_over_h": 1.0, "wk": 0.25}, text={"exposure": "XC1"}),
    c("LQ-02", num={"rax": 0.6, "eps_ca": 5e-5, "eps_cd": 1e-5, "alpha_c": 1e-5, "delta_t": 15.0, "sr_max": 400.0, "wk_lim": 0.2}), c("LQ-02", num={"rax": 0.8, "eps_free": 1.5e-4, "sr_max": 350.0}),
    c("LQ-03", num={"fck": 30.0, "b": 1000.0, "h": 400.0, "d": 350.0, "as": 2000.0, "c": 40.0, "phi": 16.0, "m_qp": 120e6, "alpha_e": 6.5, "class": 1.0, "hd_over_h": 10.0}, tol=1e-5),
    c("LQ-04", num={"wk": 0.15, "hd_over_h": 3.0}), c("LQ-04", num={"wk": 0.15, "hd_over_h": 12.0}),
    c("LQ-05", num={"fck": 35.0, "phi": 16.0, "design_life": 50.0, "c_provided": 55.0}, text={"exposure": "XC4"}), c("LQ-05", "En", num={"fck": 35.0, "phi": 16.0, "c_provided": 55.0}, text={"exposure": "XC4"}),
    c("LQ-06", num={"pressure": 0.5, "radius": 8000.0, "t": 300.0, "pm_t_per_m": 6000.0}),
    c("FA-01", num={"n_ed": 40e3, "m_x": 6e6, "m_y": 2e6, "v_ed": 20e3}, list={"ax": [-100.0, 100.0, -100.0, 100.0], "ay": [-75.0, -75.0, 75.0, 75.0]}),
    c("FA-06", num={"fck": 30.0, "c1": 60.0, "hef": 200.0, "dh": 30.0, "d": 16.0, "s2": 80.0, "n_ed": 20e3}), c("FA-06", num={"fck": 30.0, "c1": 150.0, "hef": 200.0, "dh": 30.0, "d": 16.0}),
    c("FA-11", num={**ANCH}, list={"ax": [-100.0, 100.0, -100.0, 100.0], "ay": [-75.0, -75.0, 75.0, 75.0], "ni": [10e3, 30e3, 0.0, 20e3]}),
    c("FA-12", num={"delta_n_rk_s_0": 20e3, "delta_n_ed": 8e3, "alpha_seis": 1.0}),
    c("FA-13", num={**ANCH, "v_ed": 20e3}, list={"ax": [0.0], "ay": [0.0]}), c("FA-13", num={**ANCH, "hef": 50.0}, list={"ax": [-100.0, 100.0], "ay": [0.0, 0.0]}),
    c("FA-14", num={"n_ed_re": 60e3, "fyk": 500.0, "as_provided": 160.0, "l_bd": 300.0, "l_s": 350.0}),
]
