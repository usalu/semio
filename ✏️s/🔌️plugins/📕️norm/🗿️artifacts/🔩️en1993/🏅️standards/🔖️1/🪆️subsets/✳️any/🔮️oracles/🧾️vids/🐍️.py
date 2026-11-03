"""🧾️ Independent oracle of the EN 1993 catalogue-gap checks (torsional buckling N_cr,T, block tearing, flange curling, constant-amplitude fatigue limit, LTB in fire): the closed forms re-derived with numpy and the HEB 240 section constants re-computed with the third-party library `sectionproperties` (finite element torsion constant J and warping constant Gamma of the rolled I-section outline). Run with `python3 -I` from the project virtual environment; prints one JSON object."""
import json
import math

import numpy as np

E = 210.0e9
G = 81.0e9
FY = 355.0e6
HEB240 = dict(h=0.240, b=0.240, tw=0.010, tf=0.017, r=0.021, area=0.0106, iy=1.126e-4, iz=3.923e-5, it=1.037e-6, iw=4.762e-7, wpl=1.053e-3)
KY = [(20, 1.0), (100, 1.0), (200, 1.0), (300, 1.0), (400, 1.0), (500, 0.78), (600, 0.47), (700, 0.23), (800, 0.11), (900, 0.06), (1000, 0.04), (1100, 0.02), (1200, 0.0)]
KE = [(20, 1.0), (100, 1.0), (200, 0.9), (300, 0.8), (400, 0.7), (500, 0.6), (600, 0.31), (700, 0.13), (800, 0.09), (900, 0.0675), (1000, 0.045), (1100, 0.0225), (1200, 0.0)]


def table(points, theta):
    xs, ys = zip(*points)
    return float(np.interp(theta, xs, ys))


def ncr_torsional(section, length):
    i0_sq = (section["iy"] + section["iz"]) / section["area"]
    return (G * section["it"] + math.pi**2 * E * section["iw"] / length**2) / i0_sq


def sectionproperties_ncr(length):
    from sectionproperties.analysis import Section
    from sectionproperties.pre.library import i_section

    s = HEB240
    geometry = i_section(d=s["h"] * 1e3, b=s["b"] * 1e3, t_f=s["tf"] * 1e3, t_w=s["tw"] * 1e3, r=s["r"] * 1e3, n_r=16)
    section = Section(geometry.create_mesh(mesh_sizes=[4.0]))
    section.calculate_geometric_properties()
    section.calculate_warping_properties()
    area = section.get_area() * 1e-6
    ix, iy, _ = section.get_ic()
    j = section.get_j() * 1e-12
    gamma = section.get_gamma() * 1e-18
    return (G * j + math.pi**2 * E * gamma / length**2) / ((ix + iy) * 1e-12 / area), j, gamma


def block_tearing(n1, n2, p1, p2, e1, e2, d0, t, fu, fy, gm0, gm2):
    lv = e1 + (n1 - 1) * p1 - (n1 - 0.5) * d0
    a_nt, a_nv = ((n2 - 1) * (p2 - d0) * t, 2 * lv * t) if n2 > 1 else ((e2 - 0.5 * d0) * t, lv * t)
    return fu * a_nt / gm2 + fy * a_nv / (math.sqrt(3) * gm0)


def flange_curling(sigma, bs, t, z):
    return 2 * sigma**2 * bs**4 / (E**2 * t**2 * z)


def ltb_fire(theta, lam, wpl, fy):
    ky, ke = table(KY, theta), table(KE, theta)
    lam_t = lam * math.sqrt(ky / ke)
    alpha = 0.65 * math.sqrt(235.0e6 / fy)
    phi = 0.5 * (1 + alpha * lam_t + lam_t**2)
    chi = 1.0 / (phi + math.sqrt(phi**2 - lam_t**2))
    return chi * wpl * ky * fy


def mcr_linear(length, psi):
    c1 = float(np.interp(psi, [-1, -0.75, -0.5, -0.25, 0, 0.25, 0.5, 0.75, 1], [2.752, 2.927, 2.704, 2.281, 1.879, 1.563, 1.323, 1.141, 1.0]))
    s = HEB240
    a = math.pi**2 * E * s["iz"] / length**2
    return c1 * a * math.sqrt(s["iw"] / s["iz"] + length**2 * G * s["it"] / (math.pi**2 * E * s["iz"]))


def main():
    s = HEB240
    ncr_sp, j, gamma = sectionproperties_ncr(4.0)
    lam_lt = math.sqrt(s["wpl"] * FY / mcr_linear(4.0, -1.0))
    out = {
        "ncrTCatalogue": ncr_torsional(s, 4.0),
        "ncrTSectionproperties": ncr_sp,
        "torsionConstantSectionproperties": j,
        "warpingConstantSectionproperties": gamma,
        "blockTearing": block_tearing(2, 2, 0.060, 0.060, 0.040, 0.040, 0.022, 0.010, 510.0e6, FY, 1.0, 1.25),
        "flangeCurling": flange_curling(150.0e6, 0.06, 0.003, 0.075),
        "fatigueLimitCategory71": 71.0e6 * (2.0 / 5.0) ** (1.0 / 3.0),
        "ltbFire560": ltb_fire(560.0, lam_lt, s["wpl"], FY),
        "lambdaLt": lam_lt,
    }
    print(json.dumps(out, indent=2))


main()
