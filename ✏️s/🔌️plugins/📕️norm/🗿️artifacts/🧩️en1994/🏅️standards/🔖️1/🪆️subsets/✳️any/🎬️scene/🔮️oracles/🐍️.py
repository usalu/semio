"""📐️ Independent oracle for the EN 1994 scene numbers.

Reads {"cases": [...]} on stdin and prints {"failures": [...]}. Plastic neutral axis: scipy.optimize.brentq on the fibre-integrated
force equilibrium of a shapely I section (independent of the closed-form depth inversion in Rust). Rib profile: polyline length
from the trapezoid dimensions. Stud slip: numpy evaluation of the Ollgaard-Slutter-Fisher law. Deflection: the textbook elastic line
q x (L^3 - 2 L x^2 + x^3) / (24 E I) normalised by its midspan value. Buckling: EN 1993-1-1 6.3.1.2 chi. Polygon: shapely area of the
interaction polygon against the shoelace sum of the table twin.
"""
import json
import math
import sys

import numpy as np
import shapely
from scipy.optimize import brentq
from shapely.geometry import Polygon, box


def i_section(h, b, tw, tf):
    return shapely.union_all([box(0, 0, b, tf), box((b - tw) / 2, tf, (b + tw) / 2, h - tf), box(0, h - tf, b, h)])


def area_above(section, h, b, z):
    return section.intersection(box(-1, 0, b + 1, z)).area if z > 0 else 0.0


def check_pna(case):
    h, b, tw, tf = case["h"], case["b"], case["tw"], case["tf"]
    section = i_section(h, b, tw, tf)
    area = section.area
    fyd, fcd85 = case["fyd"], case["fcd85"]
    n_a = area * fyd
    n_c = case["eta"] * min(n_a, fcd85 * case["bEff"] * case["hc"])
    x_c = n_c / (fcd85 * case["bEff"])
    def residual(z):
        above = area_above(section, h, b, z)
        return n_c + fyd * (above - (area - above))
    z_s = 0.0 if residual(0.0) >= 0 else brentq(residual, 0.0, h, xtol=1e-14)
    errors = []
    if abs(x_c - case["xC"]) > 1e-9:
        errors.append(f"x_c {x_c} vs {case['xC']}")
    if abs(z_s - case["zS"]) > 1e-7:
        errors.append(f"z_s {z_s} vs {case['zS']}")
    if abs(n_c - case["nC"]) > 1e-6 * max(n_c, 1.0):
        errors.append(f"n_c {n_c} vs {case['nC']}")
    return errors


def check_rib(case):
    hp, e, crest, bottom, pitch = case["hp"], case["web"], case["crest"], case["bottom"], case["pitch"]
    count = max(1, round(case["length"] / pitch))
    scale = case["length"] / (count * pitch)
    web = math.hypot(e * scale, hp)
    expected = count * (crest * scale + 2 * web + bottom * scale)
    errors = []
    if abs(expected - case["polyline"]) > 1e-9:
        errors.append(f"rib length {expected} vs {case['polyline']}")
    if case["points"] != 4 * count + 1:
        errors.append(f"rib points {case['points']} vs {4 * count + 1}")
    if abs(crest + 2 * e + bottom - pitch) > 1e-12:
        errors.append("rib pitch does not close")
    return errors


def check_slip(case):
    load = case["pRk"] * (1 - np.exp(-0.709 * case["delta"])) ** 0.4
    errors = []
    if abs(load - case["load"]) > 1e-9:
        errors.append(f"slip load {load} vs {case['load']}")
    if case["inverse"] is None or abs(case["inverse"] - case["delta"]) > 1e-9:
        errors.append(f"slip inverse {case['inverse']} vs {case['delta']}")
    return errors


def check_deflection(case):
    u = case["u"]
    line = lambda x: x * (1 - 2 * x * x + x ** 3)
    expected = line(u) / line(0.5)
    return [] if abs(expected - case["shape"]) < 1e-12 else [f"deflection {expected} vs {case['shape']}"]


def check_buckling(case):
    lam, alpha = case["lambda"], case["alpha"]
    phi = 0.5 * (1 + alpha * (lam - 0.2) + lam * lam)
    chi = 1.0 if lam <= 0.2 else min(1.0, 1.0 / (phi + math.sqrt(phi * phi - lam * lam)))
    return [] if abs(chi - case["chi"]) < 1e-12 else [f"chi {chi} vs {case['chi']}"]


def check_polygon(case):
    ring = list(zip(case["n"], case["m"]))
    shoelace = abs(sum(ring[i][0] * ring[(i + 1) % len(ring)][1] - ring[(i + 1) % len(ring)][0] * ring[i][1] for i in range(len(ring)))) / 2
    area = Polygon(ring).area
    return [] if abs(area - shoelace) <= 1e-9 * max(area, 1.0) else [f"polygon area {area} vs {shoelace}"]


CHECKS = {"pna": check_pna, "rib": check_rib, "slip": check_slip, "deflection": check_deflection, "buckling": check_buckling, "polygon": check_polygon}


def main():
    cases = json.load(sys.stdin)["cases"]
    failures = [f"{case['kind']}: {error}" for case in cases for error in CHECKS[case["kind"]](case)]
    print(json.dumps({"failures": failures}))


main()
