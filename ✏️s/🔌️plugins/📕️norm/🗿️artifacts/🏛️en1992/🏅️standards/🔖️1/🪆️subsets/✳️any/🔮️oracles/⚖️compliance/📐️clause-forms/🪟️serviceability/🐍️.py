"""🪟 Independent oracle of EN 1992-1-1 chapter 7 (SL-03 cracked section, SL-04 minimum reinforcement, SL-05 tables 7.2N/7.3N): the neutral axis is the positive root of the transformed-section equilibrium, A_s,min and k, k_c from `structuralcodes.codes.ec2_2004` Eq. 7.1-7.3."""

import math

import numpy as np
from clause_forms_shared import *

T72N = [(160, (40, 32, 25)), (200, (32, 25, 16)), (240, (20, 16, 12)), (280, (16, 12, 8)), (320, (12, 10, 6)), (360, (10, 8, 5)), (400, (8, 6, 4)), (450, (6, 5, 0))]
T73N = [(160, (300, 300, 200)), (200, (300, 250, 150)), (240, (250, 200, 100)), (280, (200, 150, 50)), (320, (150, 100, 0)), (360, (100, 50, 0))]


def ecm_of(fck):
    return float(np.interp(fck, GRADES, ECM_GPA)) * 1e3


def sl03(case):
    n = case["num"]
    b, h, d, a_s, fck, m = n["b"], n["h"], n["d"], n["as"], n["fck"], abs(n["m_ed"])
    ec = ecm_of(fck) / (1 + n["phi_inf"]) if "phi_inf" in n else ecm_of(fck)
    ae = n.get("es", 200000.0) / ec
    cracking = m / (b * h * h / 6) >= row_of(fck, FCTM)
    x = float(max(root.real for root in np.roots([b / 2, ae * a_s, -ae * a_s * d])))
    if cracking and x < h:
        i2 = b * x**3 / 3 + ae * a_s * (d - x) ** 2
        return {"x": x * MM, "i_ii": i2, "sigma_c": m * x / i2 * MPA, "sigma_s": ae * m * (d - x) / i2 * MPA}
    area = b * h + ae * a_s
    zc = (b * h * h / 2 + ae * a_s * d) / area
    i1 = b * h**3 / 12 + b * h * (zc - h / 2) ** 2 + ae * a_s * (d - zc) ** 2
    return {"x": zc * MM, "sigma_c": m * zc / i1 * MPA, "sigma_s": ae * m * (d - zc) / i1 * MPA}


def sl04(case):
    n, annex = case["num"], case["annex"]
    fck, b, h = n["fck"], n["b"], n["h"]
    fct = n.get("fct_eff", row_of(fck, FCTM))
    k = float(ec2.k(n.get("h_member", h)))
    if case["text"].get("part", "web") == "flange":
        kc = float(ec2.kc_flanges_area(n["f_cr"] / 1e3, n["act"], fct))
    elif n.get("pure_tension", 0.0) > 0:
        kc = 1.0
    else:
        kc = float(ec2.kc_rect_area(h, b, fct, n.get("n_ed", 0.0) / 1e3))
    act = n.get("act", b * h if n.get("pure_tension", 0.0) > 0 else b * h / 2)
    sigma_s = n.get("sigma_s_allowed", n.get("fyk", 500.0))
    as_min = float(ec2.As_min(act, sigma_s, fct, k, kc))
    values = {"as_min": as_min / 1e6, "k_c": kc, "k": k}
    if "as_provided" in n:
        values["util"] = as_min / n["as_provided"]
    return values


def table_lookup(table, sigma_s, wk):
    column = 0 if wk >= 0.35 else 1 if wk >= 0.25 else 2
    for limit, row in table:
        if sigma_s <= limit:
            return float(row[column])
    return 0.0


def sl05(case):
    n = case["num"]
    phi_max = table_lookup(T72N, n["sigma_s"], n["wk_lim"])
    s_max = table_lookup(T73N, n["sigma_s"], n["wk_lim"])
    values = {"phi_max": phi_max * MM, "s_max": s_max * MM}
    if "phi" in n:
        values["util_phi"] = n["phi"] / phi_max
    if "s" in n and s_max > 0:
        values["util_s"] = n["s"] / s_max
    return values


HANDLERS = {"SL-03": sl03, "SL-04": sl04, "SL-05": sl05}
