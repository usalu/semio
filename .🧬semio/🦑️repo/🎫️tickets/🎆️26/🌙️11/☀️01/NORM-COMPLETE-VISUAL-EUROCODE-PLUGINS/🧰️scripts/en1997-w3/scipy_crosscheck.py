"""🔬 Third-party (scipy) cross-check of the EN 1997 catalogue entries SLS-05, PILE-12 and SLOPE-06 against the stdlib oracle `📇️catalogue.py`.

Usage: /home/deck/semio/.venv/bin/python -I scipy_crosscheck.py   (exit 1 on any miss)
  SLS-05  : brentq on the Terzaghi series (scipy.optimize), solve_ivp BDF method-of-lines PDE (scipy.integrate).
  PILE-12 : Boussinesq centre stress by dblquad of the point-load kernel (scipy.integrate).
  SLOPE-06: Spencer (F, theta) by scipy.optimize.root on its own 2000-slice circle, Bishop by brentq, theta = 0 closure.
"""
import importlib.util, json, math, pathlib, sys
import numpy as np
from scipy import integrate, optimize

SUB = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any")
spec = importlib.util.spec_from_file_location("cat", SUB / "🔮️oracles/⚖️compliance/📇️catalogue.py")
C = importlib.util.module_from_spec(spec); spec.loader.exec_module(C)
fixture = json.loads((SUB / "🧪️tests/🧫️fixtures/📇️catalogue/🔣️.json").read_text())
misses = []


def check(name, ours, theirs, tol):
    ok = abs(ours - theirs) <= tol * max(1.0, abs(theirs))
    print(f"{'ok  ' if ok else 'MISS'} {name}: oracle {ours:.8g} scipy {theirs:.8g}")
    if not ok:
        misses.append(name)


def series_u(tv):
    m = (2 * np.arange(0, 400) + 1) * np.pi / 2
    return 1 - np.sum(2 / m**2 * np.exp(-m**2 * tv))


def pde_u(tv, n=300):
    h = 1 / n
    main = np.full(n, -2.0); main[0] = -3.0; main[-1] = -1.0
    a = (np.diag(main) + np.diag(np.ones(n - 1), 1) + np.diag(np.ones(n - 1), -1)) / h**2
    sol = integrate.solve_ivp(lambda t, u: a @ u, (0, tv), np.ones(n), method="BDF", rtol=1e-10, atol=1e-12)
    return 1 - h * sol.y[:, -1].sum()


def sls_05(case):
    num = case["num"]
    t90_theory = optimize.brentq(lambda t: series_u(t) - 0.9, 1e-6, 5, xtol=1e-14)
    t50_theory = optimize.brentq(lambda t: series_u(t) - 0.5, 1e-6, 5, xtol=1e-14)
    cv = num["cv"] if "cv" in num else t90_theory * num["h_dr_lab"] ** 2 / num["t90_lab"]
    got = C.evaluate("SLS-05", case["annex"], num, case["text"])
    check("SLS-05 T50", got["t50"] * cv / num["h_dr"] ** 2, t50_theory, 5e-4)
    check("SLS-05 T90", got["t90"] * cv / num["h_dr"] ** 2, t90_theory, 5e-4)
    check("SLS-05 cv", got["cv"], cv, 5e-4)
    if "t" in num:
        tv = cv * num["t"] / num["h_dr"] ** 2
        check("SLS-05 U(t) series", got["u_avg"], series_u(tv), 1e-4)
        check("SLS-05 U(t) BDF PDE", got["u_avg"], pde_u(tv), 1e-4)
    return t90_theory


def boussinesq_rect_centre(p, b, l, z):
    f = lambda y, x: 3 * z**3 / (2 * np.pi * (x * x + y * y + z * z) ** 2.5)
    val, _ = integrate.dblquad(f, -b / 2, b / 2, -l / 2, l / 2, epsabs=1e-11, epsrel=1e-11)
    return p * val


def pile_12(case):
    num = case["num"]
    m, n, s, d, l, v = (num[k] for k in ("m", "n", "s", "d", "l", "v"))
    grow = 2 * num["spread"] * num["raft_depth_factor"] * l
    b, ln = (n - 1) * num.get("sx", s) + d + grow, (m - 1) * num.get("sy", s) + d + grow
    p, z, total, i = v / (b * ln), 0.0, 0.0, 1
    while f"h{i}" in num:
        h, e = num[f"h{i}"], num[f"e{i}"]
        total += boussinesq_rect_centre(p, b, ln, z + h / 2) * h / e
        z += h; i += 1
    got = C.evaluate("PILE-12", case["annex"], num, case["text"])
    check("PILE-12 b_raft", got["b_raft"], b, 1e-12)
    check("PILE-12 s_group", got["s_group"], total, 2e-5)


def slices(num, n=2000, factor=1.0):
    h, beta, xc, yc, r = (num[k] for k in ("h", "beta_deg", "xc", "yc", "r"))
    gamma, gsat = num["gamma"], num.get("gamma_sat", num["gamma"])
    gwl, ru, q = num.get("gwl_level", -1000.0), num.get("ru", 0.0), num.get("q", 0.0) * factor
    run, tb = h / math.tan(math.radians(beta)), math.tan(math.radians(beta))
    ground = lambda x: np.where(x <= 0, h, np.where(x >= run, 0.0, h - x * tb))
    xs = np.linspace(xc - r, xc + r, 200001)
    yb = yc - np.sqrt(np.maximum(r * r - (xs - xc) ** 2, 0.0))
    inside = xs[(np.abs(xs - xc) <= r) & (yb < ground(xs))]
    xa, xb = inside[0], inside[-1]
    dx = (xb - xa) / n
    x = xa + (np.arange(n) + 0.5) * dx
    yb = yc - np.sqrt(r * r - (x - xc) ** 2); yg = ground(x)
    ht = np.maximum(yg - yb, 0.0); wet = np.clip(np.minimum(yg, gwl) - yb, 0.0, ht)
    w = gamma * (ht - wet) * dx + gsat * wet * dx + q * dx
    pw = ru * gamma * ht if ru > 0 else 9810.0 * wet
    return w, np.arcsin((xc - x) / r), pw * dx / np.cos(np.arcsin((xc - x) / r)), dx


def slope_06(case):
    num = case["num"]
    t, cd = C._drive_resist(C.Inputs(case["annex"], num, case["text"]))
    w, al, u, dx = slices(num, factor=C._slope_f(C.Inputs(case["annex"], num, case["text"]))["gamma_q"])
    lb = dx / np.cos(al)

    def q_of(fs, th):
        psi = al - th
        den = np.cos(psi) + t * np.sin(psi) / fs
        n_eff = np.maximum(w * np.cos(th) - u * np.cos(psi) - cd * lb / fs * np.sin(psi), 0.0) / den
        return (w * np.sin(al) - (cd * lb + n_eff * t) / fs) / np.cos(psi)

    def eqs(x):
        q = q_of(*x)
        return [q.sum(), (q * np.cos(al - x[1])).sum()]

    sol = optimize.root(eqs, [1.5, 0.1], method="hybr", tol=1e-13)
    fs, th = sol.x
    bishop = optimize.brentq(lambda f: f - np.sum((cd * dx + np.maximum(w - u * np.cos(al), 0) * t) / (np.cos(al) * (1 + np.tan(al) * t / f))) / np.sum(w * np.sin(al)), 0.6, 6, xtol=1e-14)
    moment0 = optimize.brentq(lambda f: (q_of(f, 0.0) * np.cos(al)).sum(), 0.6, 6, xtol=1e-14)
    got = C.evaluate("SLOPE-06", case["annex"], num, case["text"])
    psi = al - th
    raw = w * np.cos(th) - u * np.cos(psi) - cd * lb / fs * np.sin(psi)
    clamped = int((raw < 0).sum())
    print(f"     slices with a no-tension clamp at the root: {clamped}")
    check("SLOPE-06 F_spencer (1/F in oracle)", got["f"], fs, 3e-4 if not clamped else 2e-3)
    check("SLOPE-06 Bishop", got["f_bishop"], bishop, 3e-4)
    if not clamped:
        check("SLOPE-06 theta_deg", got["theta_deg"], math.degrees(th), 2e-3)
        check("SLOPE-06 Spencer(theta=0) == Bishop", moment0, bishop, 5e-4)
    print(f"     spencer_equilibrium residual {abs(np.array(eqs(sol.x))).max():.2e}, Spencer {fs:.5f} vs Bishop {bishop:.5f} (differences {100*(fs/bishop-1):+.2f} %)")


import os
ONLY = os.environ.get("ONLY")
for row in fixture["cases"]:
    if ONLY and row["entry"] != ONLY:
        continue
    if row["entry"] in ("SLS-05", "PILE-12", "SLOPE-06"):
        print("--", row["entry"], row["label"])
        {"SLS-05": sls_05, "PILE-12": pile_12, "SLOPE-06": slope_06}[row["entry"]](row["case"])
t90 = optimize.brentq(lambda t: series_u(t) - 0.9, 1e-6, 5, xtol=1e-14)
print(f"T90 (series root) = {t90:.6f}")
sys.exit(1 if misses else 0)
