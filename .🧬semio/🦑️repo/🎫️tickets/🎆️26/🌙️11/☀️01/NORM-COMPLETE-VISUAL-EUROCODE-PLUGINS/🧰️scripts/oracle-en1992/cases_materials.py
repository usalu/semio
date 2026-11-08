def c(id, annex="De", tol=1e-6, **groups):
    return {"id": id, "annex": annex, "tol": tol, **groups}

CASES = [
    c("M-01", num={"fck": 30.0}), c("M-01", "En", num={"fck": 30.0}), c("M-01", num={"fck": 50.0}, text={"situation": "accidental"}),
    c("M-03", num={"fck": 30.0}), c("M-03", num={"fck": 27.5}, tol=1e-9),
    c("M-04", num={"fck": 30.0, "t": 7.0}, text={"cement": "N"}), c("M-04", num={"fck": 30.0, "t": 91.0}, text={"cement": "S"}), c("M-04", num={"fck": 40.0, "t": 3.0}, text={"cement": "R"}),
    c("M-06", num={"fck": 30.0, "eps": 1.0}), c("M-06", num={"fck": 30.0, "eps": 3.0}), c("M-06", num={"fck": 60.0, "eps": 2.5}),
    c("M-07", num={"fck": 30.0, "b": 300.0, "x": 150.0, "d": 450.0}), c("M-07", num={"fck": 70.0, "b": 300.0, "x": 150.0, "d": 450.0, "narrowing": 1.0}),
    c("M-09", num={"fp01k": 1640.0, "fpk": 1860.0, "eps": 5.0}), c("M-09", "En", num={"fp01k": 1640.0, "fpk": 1860.0, "eps": 12.0}), c("M-09", num={"fp01k": 1640.0, "fpk": 1860.0, "eps": -3.0}),
    c("M-10", num={"fck": 30.0, "rh": 50.0, "h0": 200.0, "t0": 28.0, "t": 10000.0}, text={"cement": "N"}),
    c("M-12", num={"fck": 30.0, "phi_inf": 2.0}), c("S-12", num={"fck": 40.0, "phi_inf": 1.5, "es": 200000.0}),
    c("M-14", num={"fck": 30.0, "b": 300.0, "h": 500.0, "p": 800000.0, "e_p": 120.0}), c("M-14", num={"fck": 40.0, "b": 250.0, "h": 1800.0}),
    c("D-01", text={"classes": "XC1,XC4,XD2,XF1"}), c("D-01", text={"classes": "X0,XC3"}),
    c("D-02", num={"fck": 25.0}, text={"classes": "XC4,XD1"}), c("D-02", num={"fck": 40.0}, text={"classes": "XF3,XC3"}), c("D-02", num={"fck": 40.0, "air_entrained": 1.0}, text={"classes": "XF3,XA2"}),
    c("D-03", num={"phi": 20.0, "nb": 4.0, "structural_class": 4.0}, text={"exposure": "XC4"}), c("D-03", "En", num={"phi": 16.0, "structural_class": 4.0, "dc_gamma": 0.0, "dc_st": 5.0}, text={"exposure": "XD3"}), c("D-03", num={"phi": 12.0, "dg": 32.0}, text={"exposure": "XC1"}),
    c("D-04", num={"fck": 30.0, "phi": 20.0, "design_life": 50.0, "c_provided": 40.0}, text={"exposure": "XC3"}), c("D-04", "En", num={"fck": 25.0, "phi": 25.0, "design_life": 100.0, "slab": 1.0, "c_provided": 50.0}, text={"exposure": "XD2"}),
    c("D-05", num={"c_provided": 60.0}, text={"ground": "soil"}), c("D-05", num={"c_provided": 60.0}, text={"ground": "blinding"}),
    c("D-07", num={"design_life": 8.0}), c("D-07", num={"design_life": 20.0}), c("D-07", num={"design_life": 45.0}), c("D-07", num={"design_life": 100.0}),
]
