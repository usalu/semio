"""✂️ Independent oracle of EN 1992-1-1 6.1, 6.2, 6.3 and 12.6 (S-05, S-08, S-09, V-01, V-05, V-06, T-03): Eq. 5.39 exponent table, a_v/2d reduction, Eq. 6.25 interface shear with the c/μ values of 6.2.5(2) (DIN NA row for `De`), Eq. 6.20-6.22 flange shear and Eq. 12.10 plain concrete."""

import math

import numpy as np
from clause_forms_shared import *

INTERFACE_NA_DE = {"very_smooth": (0.025, 0.5), "smooth": (0.20, 0.6), "rough": (0.40, 0.7), "indented": (0.50, 0.9)}
INTERFACE_EN = {"very_smooth": (0.25, 0.5), "smooth": (0.35, 0.6), "rough": (0.45, 0.7), "indented": (0.50, 0.9)}


def s05(case):
    n = case["num"]
    ratio = n["n_ed"] / n["n_rd"]
    a = float(np.interp(ratio, [0.1, 0.7, 1.0], [1.0, 1.5, 2.0]))
    return {"exponent": a, "util": (abs(n["m_edz"]) / n["m_rdz"]) ** a + (abs(n["m_edy"]) / n["m_rdy"]) ** a}


def s08(case):
    n = case["num"]
    e0 = max(n["h"] / 30, 20.0)
    values = {"e0": e0 * MM, "m_min": abs(n["n_ed"]) * e0 / 1e3}
    if "m_ed" in n:
        values["m_design@cap"] = abs(n["n_ed"]) * e0
    return values


def s09(case):
    n, annex = case["num"], case["annex"]
    hw, e_tot, l0 = n["hw"], n.get("e_tot", 0.0), n.get("l0", 0.0)
    fcd_pl = ndp("ndp.en1992.alpha-cc-pl", annex) * n["fck"] / ndp("ndp.en1992.m-01.gamma-c-persistent", annex)
    phi = min(max(1.14 * (1 - 2 * e_tot / hw) - 0.02 * l0 / hw, 0.0), 1.0)
    n_rd = n["b"] * hw * fcd_pl * phi
    values = {"n_rd": n_rd}
    if "n_ed" in n:
        values["util"] = n["n_ed"] / n_rd
    if annex == "De":
        values["hw_min@cap"] = 120.0 * MM
    return values


def v01(case):
    n, annex = case["num"], case["annex"]
    av, d = n["av"], n["d"]
    direct = n.get("direct_support", 1.0) > 0
    beta = max(max(av, 0.5 * d) / (2 * d), 0.25) if direct and av <= 2 * d else 1.0
    return {"beta": beta, "v_ed_red": beta * n["v_ed"]}


def v05(case):
    n, annex = case["num"], case["annex"]
    table = INTERFACE_NA_DE if annex == "De" else INTERFACE_EN
    c, mu = table[case["text"].get("roughness", "rough")]
    if n.get("fatigue", 0.0) > 0:
        c = 0.0
    fck, sn = n["fck"], n.get("sigma_n", 0.0)
    fcd, fctd = fcd_of(annex, fck), fctd_of(annex, fck)
    alpha = math.radians(n.get("alpha_deg", 90.0))
    rho, fyd = n.get("rho", 0.0), fyd_of(annex, n.get("fyk", 500.0))
    v_rdi = min((0.0 if sn < 0 else c * fctd) + mu * min(sn, 0.6 * fcd) + rho * fyd * (mu * math.sin(alpha) + math.cos(alpha)), 0.5 * nu_of(fck) * fcd)
    v_edi = n.get("beta", 1.0) * n["v_ed"] / (n["z"] * n["bi"])
    return {"util": v_edi / v_rdi, "v_rdi": v_rdi * MPA}


def v06(case):
    n, annex = case["num"], case["annex"]
    limit = 1.25 if n.get("tension_flange", 0.0) > 0 else 2.0
    cot = min(max(n.get("cot_theta_f", 1.2), 1.0), limit)
    v_ed = n["delta_fd"] / (n["hf"] * n["delta_x"])
    fyd, fcd = fyd_of(annex, n.get("fyk", 500.0)), fcd_of(annex, n["fck"])
    theta = math.atan(1 / cot)
    return {"v_ed": v_ed * MPA, "asf_sf": v_ed * n["hf"] / (fyd * cot), "util_crush": v_ed / (nu_of(n["fck"]) * fcd * math.sin(theta) * math.cos(theta))}


def t03(case):
    stiffness = np.array(case["list"]["stiffness"])
    return {f"t_{i}": float(case["num"]["t_ed"] * k / stiffness.sum()) for i, k in enumerate(stiffness)}


HANDLERS = {"S-05": s05, "S-08": s08, "S-09": s09, "V-01": v01, "V-05": v05, "V-06": v06, "T-03": t03}
