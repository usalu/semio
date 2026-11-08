def c(id, annex="De", tol=1e-6, **groups):
    return {"id": id, "annex": annex, "tol": tol, **groups}

CASES = [
    c("SL-03", num={"fck": 30.0, "b": 300.0, "h": 500.0, "d": 450.0, "as": 1500.0, "m_ed": 120e6}, tol=1e-5), c("SL-03", num={"fck": 30.0, "b": 300.0, "h": 500.0, "d": 450.0, "as": 1500.0, "m_ed": 20e6}, tol=1e-5),
    c("SL-03", num={"fck": 35.0, "b": 250.0, "h": 600.0, "d": 550.0, "as": 2000.0, "m_ed": 200e6, "phi_inf": 2.0}, tol=1e-5),
    c("SL-04", num={"fck": 30.0, "b": 1000.0, "h": 250.0, "n_ed": 0.0, "act": 125000.0, "as_provided": 600.0, "fyk": 500.0}), c("SL-04", num={"fck": 30.0, "b": 400.0, "h": 600.0, "n_ed": 300e3, "h_member": 600.0, "sigma_s_allowed": 280.0}),
    c("SL-04", num={"fck": 25.0, "b": 300.0, "h": 400.0, "pure_tension": 1.0, "fct_eff": 1.8, "h_member": 900.0}), c("SL-04", num={"fck": 30.0, "b": 2000.0, "h": 200.0, "f_cr": 120e3, "act": 200000.0}, text={"part": "flange"}),
    c("SL-05", num={"sigma_s": 240.0, "wk_lim": 0.3, "phi": 14.0, "s": 180.0}), c("SL-05", num={"sigma_s": 300.0, "wk_lim": 0.4, "phi": 12.0, "s": 150.0}), c("SL-05", num={"sigma_s": 190.0, "wk_lim": 0.2, "phi": 20.0, "s": 100.0}),
]
