def c(id, annex="De", tol=1e-6, **groups):
    return {"id": id, "annex": annex, "tol": tol, **groups}

BASE = {"fck": 30.0, "d": 210.0, "c1": 400.0, "rho_ly": 0.01, "rho_lz": 0.01}
CASES = [
    c("P-01", num={"c1": 400.0, "c2": 300.0, "d": 200.0}), c("P-01", num={"c1": 500.0, "d": 250.0, "circular": 1.0}),
    c("P-01", "En", num={"c1": 400.0, "c2": 300.0, "d": 200.0, "e1": 150.0}, text={"position": "edge"}, tol=1e-4),
    c("P-01", num={"c1": 400.0, "c2": 300.0, "d": 200.0, "e1": 600.0}, text={"position": "edge"}, tol=1e-4),
    c("P-01", "En", num={"c1": 350.0, "c2": 350.0, "d": 220.0, "e1": 100.0, "e2": 250.0}, text={"position": "corner"}, tol=1e-4),
    c("P-05", num={**BASE, "v_ed": 900e3, "approval": 1.0}), c("P-05", "En", num={**BASE, "v_ed": 800e3, "approval": 1.0}), c("P-05", num={**BASE, "c2": 600.0, "v_ed": 600e3, "delta_v": 50e3}, text={"position": "edge"}, tol=1e-4),
    c("P-06", num={"fck": 25.0, "d": 450.0, "v_ed": 1500e3, "c1": 400.0, "c2": 400.0, "rho_ly": 0.005, "rho_lz": 0.005, "sigma_gd": 0.2, "beta": 1.1}, tol=2e-3),
    c("P-07", num={**BASE, "v_ed": 700e3}), c("P-07", "En", num={**BASE, "v_ed": 700e3, "p_force": 200e3, "tendon_angle": 0.1, "sigma_cp": 0.5}, tol=1e-4),
]
