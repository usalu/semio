def c(id, annex="De", tol=1e-6, **groups):
    return {"id": id, "annex": annex, "tol": tol, **groups}

PS = {"ap": 1500.0, "fpk": 1860.0, "fp01k": 1640.0}
CASES = [
    c("PR-01", num={**PS, "sigma_jack": 1400.0}), c("PR-01", "En", num=PS),
    c("PR-02", num={**PS, "pm0": 1.9e6}), c("PR-02", "En", num=PS),
    c("PR-06", num={"sigma_pm_inf": 1000.0, "fpk": 1860.0, "fp01k": 1640.0}), c("PR-06", "En", num={"sigma_pm_inf": 1000.0, "fpk": 1860.0, "fp01k": 1640.0, "delta_sigma": 80.0}),
    c("PR-07", "En", num={"pm_t": 1.5e6}, text={"bond": "post_bonded"}), c("PR-07", "En", num={"pm_t": 1.5e6}, text={"bond": "pre"}),
    c("PR-08", num={"p": 1.2e6, "sag": 0.15, "l": 6.0}), c("PR-08", num={"p": 1.2e6, "radius": 8000.0}),
    c("PR-09", num={"pmax": 2.0e6, "a": 200.0, "h": 600.0, "fyk": 500.0, "as_provided": 900.0}), c("PR-09", "En", num={"pmax": 1.5e6, "a": 150.0, "h": 500.0}),
    c("PR-10", num={"sigma_c": 14.0, "fck_t": 30.0}), c("PR-10", num={"sigma_c": 14.0, "fck_t": 30.0, "nonlinear_creep_proven": 1.0}),
    c("F-01", text={"structure": "bridge"}), c("F-01", text={"structure": "building"}),
    c("F-02", num={"b": 300.0, "h": 600.0, "d": 550.0, "as": 1800.0, "alpha_e": 6.5, "m_max": 250e6, "m_min": 120e6}, tol=1e-5), c("F-02", num={"b": 300.0, "h": 600.0, "d": 550.0, "as": 1800.0, "alpha_e": 6.5, "m_max": 250e6, "m_min": -10e6}, tol=1e-5),
    c("F-03", text={"kind": "straight"}), c("F-03", "En", text={"kind": "welded"}), c("F-03", text={"kind": "post_curved_steel"}),
    c("F-05", num={"fck": 30.0, "sigma_c_max": 12.0, "sigma_c_min": 3.0, "beta_cc_t0": 0.9}), c("F-05", "En", num={"fck": 60.0, "sigma_c_max": 18.0, "sigma_c_min": 2.0}),
    c("F-06", num={"fck": 30.0, "v_max": 80e3, "v_min": 30e3, "v_rd_c": 110e3, "cot_theta": 3.0}), c("F-06", num={"fck": 60.0, "v_max": 80e3, "v_min": 30e3, "v_rd_c": 110e3}),
    c("F-07", num={"lambda1": 0.8, "lambda2": 1.1, "lambda3": 0.9, "lambda4": 1.0, "lambda_max": 1.4, "delta_sigma_71": 70.0, "phi_fat": 1.2}, text={"kind": "straight"}), c("F-07", "En", num={"lambda1": 1.6, "lambda2": 1.1, "lambda3": 1.1, "lambda4": 1.0, "lambda_max": 2.0, "delta_sigma_71": 60.0}, text={"kind": "welded"}),
]
