"""🔮️ Third-party oracle for the geotechnical kernel: numpy solves the force polygon of the sliding wedge of Mononobe–Okabe and scipy maximises (active) or minimises (passive) it over the plane angle, scipy brentq solves the factors of safety of the sliced circle with exact slice geometry.

Prints `key value` lines. Every case is defined here independently of the Rust sources.
"""
import math

import numpy as np
from scipy import optimize

OUT = []
r = math.radians
G_W = 9810.0


def emit(key, value):
    OUT.append(f"{key} {float(value)!r}")


def wedge_polygon(gamma, h, phi, delta, kh, f, active):
    tp, cd, sd = math.tan(r(phi)), math.cos(r(delta)), math.sin(r(delta))

    def force(rho):
        w = 0.5 * gamma * h * h / math.tan(rho)
        if active:
            a = np.array([[cd, -math.sin(rho) + tp * math.cos(rho)], [sd, math.cos(rho) + tp * math.sin(rho)]])
        else:
            a = np.array([[cd, -math.sin(rho) - tp * math.cos(rho)], [-sd, math.cos(rho) - tp * math.sin(rho)]])
        return np.linalg.solve(a, [(kh if active else -kh) * w, f * w])[0]

    if active:
        best = optimize.minimize_scalar(lambda x: -force(x), bounds=(r(1), r(89)), method="bounded", options={"xatol": 1e-12})
        return force(best.x), math.degrees(best.x)
    grid = [r(d) for d in np.arange(2.0, 80.0, 0.01)]
    valid = [x for x in grid if force(x) > 0 and np.linalg.solve(np.array([[cd, -math.sin(x) - tp * math.cos(x)], [-sd, math.cos(x) - tp * math.sin(x)]]), [-kh * 0.5 * gamma * h * h / math.tan(x), f * 0.5 * gamma * h * h / math.tan(x)])[1] > 0]
    start = min(valid, key=force)
    best = optimize.minimize_scalar(force, bounds=(start - r(0.01), start + r(0.01)), method="bounded", options={"xatol": 1e-13})
    return force(best.x), math.degrees(best.x)


def earth_pressure():
    gamma, h = 18000.0, 4.0
    cases = {"a": (30.0, 0.0, 0.0, 1.0), "b": (30.0, 0.0, 0.2, 1.0), "c": (33.0, 11.0, 0.15, 1.075), "d": (33.0, 11.0, 0.15, 0.925), "e": (35.0, 17.0, 0.25, 0.9)}
    for name, (phi, delta, kh, f) in cases.items():
        p, rho = wedge_polygon(gamma, h, phi, delta, kh, f, True)
        emit(f"mo.active.{name}.k", 2 * p / (gamma * f * h * h))
        emit(f"mo.active.{name}.rho", rho)
    for name, (phi, delta, kh, f) in {"a": (30.0, 0.0, 0.0, 1.0), "b": (30.0, 0.0, 0.1, 1.0), "c": (33.0, 5.0, 0.1, 1.05)}.items():
        p, _ = wedge_polygon(gamma, h, phi, delta, kh, f, False)
        emit(f"mo.passive.{name}.k", 2 * p / (gamma * f * h * h))


def slices(h, beta, xc, yc, rad, gamma, gamma_sat, gwl, n):
    tb = math.tan(r(beta))
    run = h / tb
    ground = lambda x: h if x <= 0 else (0.0 if x >= run else h - x * tb)
    lower = lambda x: yc - math.sqrt(max(rad * rad - (x - xc) ** 2, 0.0))
    xs = np.linspace(xc - rad, xc + rad, 400001)
    below = np.array([lower(x) < ground(x) for x in xs[::100]])
    inside = xs[::100][below]
    xa, xb = inside[0], inside[-1]
    width = (xb - xa) / n
    out = []
    for i in range(n):
        x = xa + (i + 0.5) * width
        yb, yg = lower(x), ground(x)
        height = max(yg - yb, 0.0)
        wet = min(max(min(yg, gwl) - yb, 0.0), height)
        ponded = max(gwl - yg, 0.0)
        out.append(dict(b=width, w=gamma * (height - wet) * width + gamma_sat * wet * width + G_W * ponded * width, alpha=math.asin((xc - x) / rad), u=G_W * max(gwl - yb, 0.0) * width, ycg=yb + 0.5 * height))
    return out


def factor(method, sl, yc, rad, tan_phi, c, kh, vshare, sign):
    for k in sl:
        k["fh"] = kh * k["w"]
        k["wv"] = k["w"] + sign * vshare * k["fh"]
    if method == "janbu":
        drive = sum(k["wv"] * math.tan(k["alpha"]) + k["fh"] for k in sl)
    else:
        drive = sum(k["wv"] * math.sin(k["alpha"]) + k["fh"] * (yc - k["ycg"]) / rad for k in sl)
    if method == "fellenius":
        return sum(c * k["b"] / math.cos(k["alpha"]) + max(k["wv"] * math.cos(k["alpha"]) - k["fh"] * math.sin(k["alpha"]) - k["u"] / math.cos(k["alpha"]), 0.0) * tan_phi for k in sl) / drive
    power = 2 if method == "janbu" else 1

    def residual(f):
        return sum((c * k["b"] + max(k["wv"] - k["u"], 0.0) * tan_phi) / (math.cos(k["alpha"]) ** power * (1 + math.tan(k["alpha"]) * tan_phi / f)) for k in sl) / drive - f

    return optimize.brentq(residual, 0.6, 20.0, xtol=1e-13)


def slope():
    h, beta, xc, yc, rad, gamma, gamma_sat, gwl, n = 10.0, 30.0, 4.0, 13.0, 16.0, 19000.0, 20000.0, 3.0, 120
    sl = slices(h, beta, xc, yc, rad, gamma, gamma_sat, gwl, n)
    tan_phi, c = math.tan(r(28.0)) / 1.25, 6000.0 / 1.25
    for method in ("bishop", "fellenius"):
        emit(f"slope.{method}.static", factor(method, sl, yc, rad, tan_phi, c, 0.0, 0.0, 1.0))
        for sign in (1.0, -1.0):
            emit(f"slope.{method}.seismic.{'down' if sign > 0 else 'up'}", factor(method, sl, yc, rad, tan_phi, c, 0.08, 0.5, sign))


def janbu_ponded():
    h, beta, xc, yc, rad, n = 6.0, 30.0, 6.0, 10.0, 11.0, 120
    sl = slices(h, beta, xc, yc, rad, 18000.0, 20000.0, 2.0, n)
    tan_phi, c = math.tan(r(30.0)), 5000.0
    width = sl[0]["b"]
    xs = [xc - rad * math.sin(k["alpha"]) for k in sl]
    tb = math.tan(r(beta))
    ground = lambda x: h if x <= 0 else (0.0 if x >= h / tb else h - x * tb)
    depth = max(ground(x) - (yc - math.sqrt(max(rad * rad - (x - xc) ** 2, 0.0))) for x in xs)
    ratio = depth / (xs[-1] - xs[0] + width)
    f0 = 1 + 0.69 * (ratio - 1.4 * ratio * ratio)
    drive = sum(k["w"] * math.tan(k["alpha"]) for k in sl)
    resid = lambda f: f0 * sum((c * k["b"] + max(k["w"] - k["u"], 0.0) * tan_phi) / (math.cos(k["alpha"]) ** 2 * (1 + math.tan(k["alpha"]) * tan_phi / f)) for k in sl) / drive - f
    emit("slope.janbu.ponded", optimize.brentq(resid, 0.3, 20.0, xtol=1e-13))


if __name__ == "__main__":
    earth_pressure()
    slope()
    janbu_ponded()
    print("\n".join(OUT))
