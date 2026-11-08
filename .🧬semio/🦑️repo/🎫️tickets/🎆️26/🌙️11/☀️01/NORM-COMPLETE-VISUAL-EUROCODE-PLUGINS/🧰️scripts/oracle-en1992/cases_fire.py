def c(id, annex="De", tol=1e-6, **groups):
    return {"id": id, "annex": annex, "tol": tol, **groups}

CASES = [
    c("R-01", num={"r_class": 90.0, "dim": 250.0, "a": 50.0}, text={"kind": "beam"}), c("R-01", num={"r_class": 75.0, "dim": 400.0, "a": 30.0}, text={"kind": "beam_ss"}), c("R-01", num={"r_class": 120.0, "dim": 130.0, "a": 30.0, "fck": 30.0}, text={"kind": "beam"}),
    c("R-01", num={"r_class": 90.0, "dim": 120.0, "a": 35.0}, text={"kind": "slab", "sub": "one_way"}), c("R-01", num={"r_class": 60.0, "dim": 90.0, "a": 12.0}, text={"kind": "slab", "sub": "two_way"}), c("R-01", num={"r_class": 120.0, "dim": 200.0, "a": 36.0}, text={"kind": "flat_slab"}),
    c("R-01", num={"r_class": 90.0, "dim": 350.0, "a": 50.0, "mu_fi": 0.4}, text={"kind": "column"}), c("R-01", num={"r_class": 120.0, "dim": 300.0, "a": 50.0, "mu_fi": 0.7, "fck": 75.0}, text={"kind": "column"}), c("R-01", num={"r_class": 60.0, "dim": 150.0, "a": 12.0}, text={"kind": "wall"}), c("R-01", num={"r_class": 120.0, "dim": 150.0, "a": 60.0}, text={"kind": "tension"}), c("R-01", num={"r_class": 90.0, "dim": 200.0, "a": 30.0}, text={"kind": "beam_cont"}),
    c("D-06", num={"r_class": 60.0, "dim": 150.0, "a": 30.0}, text={"kind": "beam"}),
    c("R-02", num={"b": 300.0, "h": 500.0, "t_min": 90.0}), c("R-02", num={"b": 300.0, "h": 500.0, "t_min": 30.0}, text={"faces": "bt"}),
    c("R-06", num={"gk": 100.0, "qk": 50.0, "psi_fi": 0.3}), c("R-06", "En", num={"gk": 100.0, "qk": 0.0, "gamma_g": 1.35, "gamma_q": 1.5}),
    c("R-07", num={"theta": 20.0}), c("R-07", num={"theta": 450.0}), c("R-07", num={"theta": 750.0}), c("M-13", num={"theta": 1150.0}),
    c("R-08", num={"fck": 30.0}), c("R-08", num={"fck": 75.0}), c("R-08", num={"fck": 90.0}),
]
