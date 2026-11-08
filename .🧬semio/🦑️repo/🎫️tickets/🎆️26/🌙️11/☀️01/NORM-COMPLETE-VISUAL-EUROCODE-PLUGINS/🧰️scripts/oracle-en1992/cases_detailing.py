def c(id, annex="De", tol=1e-6, **groups):
    return {"id": id, "annex": annex, "tol": tol, **groups}

CASES = [
    c("B-04", num={"fck": 30.0, "phi": 20.0, "cd": 45.0, "rho1": 50.0, "l_available": 1100.0, "clear_distance": 30.0, "sum_ast": 300.0, "as": 314.0}), c("B-04", "En", num={"fck": 25.0, "phi": 16.0, "rho1": 25.0, "p_transverse": 2.0, "alpha3": 0.8, "fyk": 500.0}, text={"bond": "poor"}), c("B-04", num={"fck": 30.0, "phi": 25.0, "cd": 40.0}, text={"loading": "compression"}),
    c("B-05", num={"phi": 20.0, "nb": 2.0}), c("B-05", num={"phi": 32.0, "nb": 4.0}, text={"situation": "compression"}), c("B-05", num={"phi": 32.0, "nb": 3.0}),
    c("B-06", num={"phi": 40.0, "act_ext": 90000.0, "as_surf_provided": 1000.0}), c("B-06", num={"phi": 25.0, "act_ext": 90000.0}),
    c("B-08", num={"phi": 10.0}, text={"angle": "135"}), c("B-08", num={"phi": 12.0}, text={"angle": "90"}),
    c("B-09", num={"phi": 25.0, "dg": 16.0, "clear": 30.0}), c("B-09", num={"phi": 12.0, "dg": 32.0, "clear": 50.0}),
    c("DT-02", num={"v_ed": 250e3, "z": 405.0, "as_span": 1200.0, "cot_theta": 2.0, "cot_alpha": 0.0, "n_ed": 10e3, "as_support": 400.0}), c("DT-02", num={"v_ed": 100e3, "z": 300.0, "as_span": 800.0}),
    c("DT-03", num={"fck": 30.0, "bw": 300.0, "d": 450.0, "alpha_deg": 90.0, "fyk": 500.0, "asw": 100.0, "s": 250.0}), c("DT-03", "En", num={"fck": 30.0, "bw": 300.0, "d": 450.0, "alpha_deg": 60.0, "asw": 100.0, "s": 250.0}), c("DT-03", num={"fck": 50.0, "bw": 300.0, "d": 900.0}),
    c("DT-04", num={"u": 1600.0, "b": 300.0, "d": 450.0, "s": 150.0}), c("DT-04", num={"u": 1600.0, "b": 300.0, "d": 450.0}),
    c("DT-05", num={"act_ext": 120000.0, "as_provided": 1500.0, "s": 120.0}), c("DT-05", num={"act_ext": 120000.0}),
    c("DT-06", num={"fck": 30.0, "h": 200.0, "d": 160.0, "as_main": 500.0, "s": 150.0, "as_secondary": 120.0}), c("DT-06", num={"fck": 30.0, "h": 300.0, "d": 260.0, "as_main": 800.0, "s": 200.0}), c("DT-06", "En", num={"fck": 25.0, "h": 300.0, "d": 260.0, "as_main": 800.0, "s": 200.0}),
    c("DT-07", num={"h": 220.0, "v_ed": 90e3, "v_rd_max": 300e3, "v_rd_c": 85e3}),
    c("DT-08", num={"v_ed": 400e3, "a_t": 1800.0, "fyk": 500.0, "as_through": 900.0, "m_edge": 60e6, "b_e": 800.0, "d": 220.0, "fck": 30.0}), c("DT-08", "En", num={"v_ed": 250e3, "a_t": 1000.0}),
    c("DT-10", num={"ac": 200000.0, "tw": 200.0, "asv": 500.0}), c("DT-10", num={"ac": 200000.0, "tw": 100.0}),
    c("DT-11", num={"ac_per_m": 300000.0, "h": 600.0, "as_provided": 400.0}), c("DT-11", num={"ac_per_m": 100000.0, "h": 200.0}),
    c("DT-12", num={"n_ed": 800e3, "d": 400.0, "b": 2000.0, "a": 400.0, "fyk": 500.0, "as_provided": 900.0}, text={"kind": "strip"}), c("DT-12", "En", num={"n_ed": 1500e3, "d": 700.0, "s": 1800.0, "c": 400.0}, text={"kind": "pile_cap_2"}),
    c("DT-13", num={"f_bt": 100e3, "a_b": 100.0, "phi": 16.0, "fck": 30.0, "phi_m": 150.0, "m_ed": 40e6, "z": 300.0}), c("DT-13", "En", num={"f_bt": 60e3, "a_b": 150.0, "phi": 12.0, "fck": 25.0}),
]
