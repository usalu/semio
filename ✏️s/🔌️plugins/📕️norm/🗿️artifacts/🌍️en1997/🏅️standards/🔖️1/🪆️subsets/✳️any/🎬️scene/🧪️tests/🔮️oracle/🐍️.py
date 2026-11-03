import json
import math
import sys

import numpy as np
from scipy import integrate
from shapely.geometry import Point, Polygon


def factors(phis):
    p = np.radians(np.asarray(phis, dtype=float))
    nq = np.exp(np.pi * np.tan(p)) * np.tan(np.pi / 4 + p / 2) ** 2
    nc = (nq - 1) / np.tan(p)
    ng = 2 * (nq - 1) * np.tan(p)
    return nq.tolist(), nc.tolist(), ng.tolist()


def mechanism(case):
    phi = math.radians(case["phi"])
    psi = math.pi / 4 + phi / 2
    r0 = case["b"] / (2 * math.cos(psi))
    return {
        "active": Polygon(case["active"]).area,
        "passive": [Polygon(p).area for p in case["passive"]],
        "r1": r0 * math.exp(math.pi / 2 * math.tan(phi)),
        "depth": case["b"] / 2 * math.tan(psi),
    }


def sigma(case):
    b, l, px, py, z, q = case["b"], case["l"], case["px"], case["py"], case["z"], case["q"]
    f = lambda y, x: 3 * q * z**3 / (2 * math.pi * (x * x + y * y + z * z) ** 2.5)
    value, _ = integrate.dblquad(f, -b / 2 - px, b / 2 - px, -l / 2 - py, l / 2 - py, epsabs=1e-10, epsrel=1e-10)
    return value


def slices(case):
    h, run = case["h"], case["run"]
    ground = Polygon([(-1000, -1000), (1000, -1000), (1000, 0), (run, 0), (0, h), (-1000, h)])
    disk = Point(case["xc"], case["yc"]).buffer(case["r"], 2048)
    exact = ground.intersection(disk).area
    mine = sum(Polygon([(s[0], s[2]), (s[1], s[3]), (s[1], s[5]), (s[0], s[4])]).area for s in case["slices"])
    return {"exact": exact, "slices": mine}


def resultant(c):
    x = np.asarray(c["x"], dtype=float)
    s = np.asarray(c["sigma"], dtype=float)
    return float(np.sum((x[1:] - x[:-1]) * (s[1:] + s[:-1]) / 2) * c["l"])


def main():
    case = json.load(sys.stdin)
    if "contact" in case:
        json.dump({"contact": [resultant(c) for c in case["contact"]]}, sys.stdout)
        return
    nq, nc, ng = factors(case["phis"])
    json.dump(
        {
            "nq": nq,
            "nc": nc,
            "ng": ng,
            "mechanism": mechanism(case["mechanism"]),
            "sigma": [sigma(c) for c in case["stress"]],
            "slope": slices(case["slope"]),
        },
        sys.stdout,
    )


main()
