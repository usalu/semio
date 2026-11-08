def c(id, annex="De", tol=1e-6, **groups):
    return {"id": id, "annex": annex, "tol": tol, **groups}

CASES = [
    c("S-05", num={"n_rd": 2000e3, "m_rdy": 150e3, "m_rdz": 120e3, "n_ed": 800e3, "m_edy": 60e3, "m_edz": 50e3}), c("S-05", num={"n_rd": 2000e3, "m_rdy": 150e3, "m_rdz": 120e3, "n_ed": 1700e3, "m_edy": 40e3, "m_edz": 30e3}),
    c("S-08", num={"h": 400.0, "n_ed": 1000e3, "m_ed": 10e6}), c("S-08", num={"h": 900.0, "n_ed": -500e3}),
    c("S-09", num={"b": 1000.0, "hw": 200.0, "fck": 25.0, "l0": 2500.0, "e_tot": 15.0, "n_ed": 300e3}), c("S-09", "En", num={"b": 1000.0, "hw": 150.0, "fck": 30.0, "l0": 3000.0}),
    c("V-01", num={"av": 600.0, "d": 450.0, "v_ed": 300e3}), c("V-01", "En", num={"av": 150.0, "d": 450.0, "v_ed": 300e3}), c("V-01", "En", num={"av": 1200.0, "d": 450.0, "v_ed": 300e3}),
    c("V-05", num={"fck": 30.0, "bi": 1000.0, "z": 400.0, "v_ed": 300e3, "beta": 0.5, "sigma_n": 0.5, "rho": 0.003, "alpha_deg": 90.0, "fyk": 500.0}, text={"roughness": "rough"}),
    c("V-05", num={"fck": 40.0, "bi": 800.0, "z": 500.0, "v_ed": 900e3, "rho": 0.005, "alpha_deg": 60.0, "sigma_n": -1.0}, text={"roughness": "indented"}),
    c("V-05", num={"fck": 30.0, "bi": 800.0, "z": 500.0, "v_ed": 400e3, "fatigue": 1.0, "sigma_n": 0.3}, text={"roughness": "smooth"}),
    c("V-06", num={"fck": 30.0, "hf": 150.0, "delta_fd": 250e3, "delta_x": 2000.0, "cot_theta_f": 1.8, "fyk": 500.0}), c("V-06", "En", num={"fck": 40.0, "hf": 120.0, "delta_fd": 100e3, "delta_x": 1500.0, "cot_theta_f": 1.9, "tension_flange": 1.0}),
    c("T-03", num={"t_ed": 30e6}, list={"stiffness": [1.0, 3.0, 4.0]}),
]
