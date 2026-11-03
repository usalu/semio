import json
import sys

import numpy as np
from shapely.geometry import LineString, MultiPoint, Point, Polygon


def hull(points):
    h = MultiPoint(points).convex_hull
    ring = list(h.exterior.coords)[:-1]
    return Polygon(ring).area, len(ring)


def ray(points, origin, demand):
    poly = MultiPoint(points).convex_hull
    d = np.array(demand) - np.array(origin)
    far = np.array(origin) + d / np.linalg.norm(d) * 1e6
    hit = poly.exterior.intersection(LineString([origin, far.tolist()]))
    b = np.array(hit.coords[0]) if hit.geom_type == "Point" else np.array(list(hit.geoms)[0].coords[0])
    u = float(np.linalg.norm(d) / np.linalg.norm(b - np.array(origin)))
    return [float(b[0]), float(b[1]), u, bool(poly.covers(Point(demand)))]


def chi(lam, alpha):
    lam = np.asarray(lam, dtype=float)
    phi = 0.5 * (1 + alpha * (lam - 0.2) + lam**2)
    val = 1 / (phi + np.sqrt(phi**2 - lam**2))
    return np.where(lam <= 0.2, 1.0, np.minimum(val, 1.0)).tolist()


def spectrum(p, kind, ts):
    ts = np.asarray(ts, dtype=float)
    ag, s, tb, tc, td, eta, q, beta = (p[k] for k in ("ag", "s", "tb", "tc", "td", "eta", "q", "beta"))
    if kind == "elastic":
        pl = ag * s * eta * 2.5
        return np.select(
            [ts <= tb, ts <= tc, ts <= td],
            [ag * s * (1 + ts / tb * (eta * 2.5 - 1)), pl, pl * tc / np.maximum(ts, 1e-12)],
            pl * tc * td / np.maximum(ts, 1e-12) ** 2,
        ).tolist()
    pl = ag * s * 2.5 / q
    return np.select(
        [ts <= tb, ts <= tc, ts <= td],
        [ag * s * (2 / 3 + ts / tb * (2.5 / q - 2 / 3)), pl, np.maximum(pl * tc / np.maximum(ts, 1e-12), beta * ag)],
        np.maximum(pl * tc * td / np.maximum(ts, 1e-12) ** 2, beta * ag),
    ).tolist()


def sn(dc, ns):
    dd = dc * (2 / 5) ** (1 / 3)
    dl = dd * (5 / 100) ** (1 / 5)
    lx = np.log10([2e6, 5e6, 1e8])
    ly = np.log10([dc, dd, dl])
    ns = np.asarray(ns, dtype=float)
    lo = np.log10(ns)
    below = 10 ** np.interp(lo, lx, ly)
    return np.where(ns > 1e8, dl, below).tolist()


def block(b, d, x, fck, fcd):
    lam = 0.8 if fck <= 50 else 0.8 - (fck - 50) / 400
    eta = 1.0 if fck <= 50 else 1 - (fck - 50) / 200
    force = eta * fcd * b * lam * x
    return [force, d - lam * x / 2]


def fire(kind, ts):
    t = np.asarray(ts, dtype=float)
    if kind == "iso834":
        return (20 + 345 * np.log10(8 * t + 1)).tolist()
    if kind == "external":
        return (660 * (1 - 0.687 * np.exp(-0.32 * t) - 0.313 * np.exp(-3.8 * t)) + 20).tolist()
    return (1080 * (1 - 0.325 * np.exp(-0.167 * t) - 0.675 * np.exp(-2.5 * t)) + 20).tolist()


def main():
    c = json.load(sys.stdin)
    out = {
        "hull": hull(c["points"]),
        "ray": ray(c["points"], c["origin"], c["demand"]),
        "chi": chi(c["lambdas"], c["alpha"]),
        "elastic": spectrum(c["spectrum"], "elastic", c["periods"]),
        "design": spectrum(c["spectrum"], "design", c["periods"]),
        "sn": sn(c["dc"], c["cycles"]),
        "block": block(*c["block"]),
        "fire": {k: fire(k, c["minutes"]) for k in ("iso834", "external", "hydrocarbon")},
    }
    json.dump(out, sys.stdout)


main()
