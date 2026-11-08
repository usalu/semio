def c(id, annex="De", tol=1e-6, **groups):
    return {"id": id, "annex": annex, "tol": tol, **groups}

COL = {"fck": 30.0, "b": 400.0, "h": 400.0, "n_ed": 1500e3}
CASES = [
    c("A-03", num={"l_m": 6.0, "m": 2.0, "n_ed": 1000e3}), c("A-03", "En", num={"l_m": 3.0, "m": 1.0, "l0": 2800.0}), c("A-03", num={"l_m": 12.0, "m": 4.0}),
    c("A-04", num={"bw": 300.0, "l0": 6000.0}, list={"b_i": [900.0, 1200.0]}), c("A-04", num={"bw": 300.0, "l0": 2000.0}, list={"b_i": [100.0, 150.0]}),
    c("A-05", num={"b": 300.0, "h": 600.0, "d": 550.0, "as": 1800.0, "fck": 30.0, "m": 200e6}, tol=1e-5),
    c("A-06", num={"fck": 30.0, "xu_d": 0.3, "delta": 0.8}), c("A-06", "En", num={"fck": 30.0, "xu_d": 0.3, "delta": 0.8}), c("A-06", num={"fck": 60.0, "xu_d": 0.25, "delta": 0.9}, text={"steel_class": "A"}),
    c("A-07", num={"fck": 30.0, "xu_d": 0.2, "theta_pl": 0.005, "b": 300.0, "h": 500.0, "d": 450.0}), c("A-07", num={"fck": 60.0, "xu_d": 0.2, "theta_pl": 0.005, "b": 300.0, "h": 500.0, "d": 450.0}),
    c("A-08", num={"e_d": 5000e3, "r_m": 7000e3}), c("A-08", "En", num={"e_d": 4000e3, "r_m": 7000e3}),
    c("A-09", num={**COL, "l0": 5000.0, "phi_ef": 1.5, "as": 1600.0, "rm": 0.4}), c("A-09", "En", num={**COL, "k1": 0.3, "k2": 0.5, "l": 4000.0, "lambda_y": 40.0, "lambda_z": 30.0}, text={"bracing": "braced"}), c("A-09", num={**COL, "k1": 0.2, "k2": 0.4, "l": 4000.0}, text={"bracing": "unbraced"}),
    c("A-11", num={**COL, "l0": 5000.0, "as": 1600.0, "is_moment": 3.0e7, "phi_ef": 1.2, "m_0ed": 80e6}, tol=1e-5), c("A-11", "En", num={**COL, "l0": 4000.0, "as": 400.0, "is_moment": 1.0e7, "m_0ed": 60e6, "c0": 9.6}, tol=1e-5),
    c("A-12", num={"n_rd": 2000e3, "m_rdy": 150e3, "m_rdz": 120e3, "n_ed": 800e3, "m_edy": 60e3, "m_edz": 50e3, "amp_y": 1.2, "amp_z": 1.1}),
    c("A-14", num={"f_v_ed": 1.2e6, "sum_ecd_ic": 3.0e16, "l": 30000.0, "n_s": 8.0}), c("A-14", "En", num={"f_v_ed": 3.0e6, "sum_ecd_ic": 3.0e16, "l": 30000.0, "n_s": 3.0}),
]
