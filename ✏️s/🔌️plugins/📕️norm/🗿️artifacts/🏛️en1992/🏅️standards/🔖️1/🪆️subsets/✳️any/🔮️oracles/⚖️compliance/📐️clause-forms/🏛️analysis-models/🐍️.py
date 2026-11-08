"""🏛 Independent oracle of EN 1992-1-1 chapter 5 (A-03 … A-09, A-11, A-12, A-14): Eq. 5.1-5.2 imperfections, Eq. 5.7 flange width, Eq. 5.12N-5.14N redistribution, Eq. 5.13N-5.14N slenderness limit, Eq. 5.15/5.16 effective length, Eq. 5.21 and 5.22 nominal stiffness, Eq. 5.18 global second order, cracked stiffness from the transformed-section neutral axis."""

import math

from clause_forms_shared import *


def ecm_of(fck):
    return float(np.interp(fck, GRADES, ECM_GPA)) * 1e3


def a03(case):
    n, annex = case["num"], case["annex"]
    l_m, m = n["l_m"], n.get("m", 1.0)
    theta = (1 / ndp("ndp.en1992.imperfection-theta0-divisor", annex)) * min(max(2 / math.sqrt(l_m), 2 / 3), 1.0) * math.sqrt(0.5 * (1 + 1 / m))
    values = {"theta_i": theta, "e_i": theta * n.get("l0", l_m * 1000.0) / 2 * MM}
    if "n_ed" in n:
        values["h_i"] = theta * n["n_ed"]
    return values


def a04(case):
    n = case["num"]
    flanges = case["list"]["b_i"]
    effective = sum(min(0.2 * b + 0.1 * n["l0"], 0.2 * n["l0"], b) for b in flanges) + n["bw"]
    return {"beff": min(effective, sum(flanges) + n["bw"]) * MM}


def a05(case):
    n = case["num"]
    b, h, d, a_s, fck = n["b"], n["h"], n["d"], n["as"], n["fck"]
    ecm = ecm_of(fck)
    ae = n.get("es", 200000.0) / ecm
    x = max(root.real for root in np.roots([b / 2, ae * a_s, -ae * a_s * d]))
    i2 = b * x**3 / 3 + ae * a_s * (d - x) ** 2
    i1 = b * h**3 / 12
    return {"ei_uncracked": ecm * i1, "ei_cracked": ecm * i2, "ratio": i2 / i1}


def a06(case):
    n, annex = case["num"], case["annex"]
    fck, xd = n["fck"], n["xu_d"]
    ecu2 = ec2.eps_cu2(fck)
    k2 = 1.25 * (0.6 + 0.0014 / ecu2)
    if fck <= 50:
        k1 = ndp("ndp.en1992.redistribution-k1", annex)
        k2 = ndp("ndp.en1992.redistribution-k2", annex) if annex == "De" else k2
    else:
        k1 = 0.54
    floor = 0.8 if case["text"].get("steel_class", "B") == "A" else 0.7
    return {"delta@cap": max(k1 + k2 * xd, floor), "xu_d@cap": 0.45 if fck <= 50 else 0.35}


def a08(case):
    n, annex = case["num"], case["annex"]
    return {"util": n["e_d"] / (n["r_m"] / ndp("ndp.en1992.gamma-o-nl", annex))}


def effective_length(k1, k2, l, braced):
    if braced:
        return 0.5 * l * math.sqrt((1 + k1 / (0.45 + k1)) * (1 + k2 / (0.45 + k2)))
    return l * max(math.sqrt(1 + 10 * k1 * k2 / (k1 + k2)), (1 + k1 / (1 + k1)) * (1 + k2 / (1 + k2)))


def a09(case):
    n, annex = case["num"], case["annex"]
    fck, b, h = n["fck"], n["b"], n["h"]
    ac, fcd, fyd = b * h, fcd_of(annex, fck), fyd_of(annex, n.get("fyk", 500.0))
    l0 = effective_length(n["k1"], n["k2"], n["l"], case["text"].get("bracing", "braced") == "braced") if "k1" in n and "k2" in n else n["l0"]
    i = n.get("i", min(h, b) / math.sqrt(12))
    nn = n["n_ed"] / (ac * fcd)
    a = 1 / (1 + 0.2 * n["phi_ef"]) if "phi_ef" in n else 0.7
    bb = math.sqrt(1 + 2 * n["as"] * fyd / (ac * fcd)) if "as" in n else 1.1
    cc = 1.7 - n["rm"] if "rm" in n else 0.7
    lim = 20 * a * bb * cc / math.sqrt(max(nn, 1e-9))
    if annex == "De" and nn >= ndp("ndp.en1992.lambda-lim-de-n", annex) and n.get("isolated", 1.0) > 0:
        lim = max(lim, 25.0)
    values = {"lambda": l0 / i, "lambda_lim": lim, "util": l0 / i / lim}
    if "lambda_y" in n and "lambda_z" in n:
        values["lambda_ratio@util"] = max(n["lambda_y"] / n["lambda_z"], n["lambda_z"] / n["lambda_y"]) / 2.0
    return values


def a11(case):
    n, annex = case["num"], case["annex"]
    fck, b, h, ned, l0, phi_ef = n["fck"], n["b"], n["h"], n["n_ed"], n["l0"], n.get("phi_ef", 0.0)
    ac, ic = b * h, b * h**3 / 12
    ecd = ecm_of(fck) / ndp("ndp.en1992.gamma-ce", annex)
    nn = ned / (ac * fcd_of(annex, fck))
    lam = l0 / (h / math.sqrt(12))
    k1, k2 = math.sqrt(fck / 20), min(nn * lam / 170, 0.2)
    kc, ks = (k1 * k2 / (1 + phi_ef), 1.0) if n["as"] / ac >= 0.002 else (0.3 / (1 + 0.5 * phi_ef), 0.0)
    ei = kc * ecd * ic + ks * n.get("es", 200000.0) * n["is_moment"]
    nb = math.pi**2 * ei / l0**2
    beta = math.pi**2 / n.get("c0", 8.0)
    return {"ei": ei, "n_b": nb, "m_ed": n["m_0ed"] * (1 + beta / (nb / ned - 1)) / 1e3}


def a12(case):
    n = case["num"]
    a = float(np.interp(n["n_ed"] / n["n_rd"], [0.1, 0.7, 1.0], [1.0, 1.5, 2.0]))
    return {"util": (abs(n["m_edz"]) * n.get("amp_z", 1.0) / n["m_rdz"]) ** a + (abs(n["m_edy"]) * n.get("amp_y", 1.0) / n["m_rdy"]) ** a}


def a14(case):
    n, annex = case["num"], case["annex"]
    return {"util": n["f_v_ed"] / (ndp("ndp.en1992.k1-global", annex) * n["sum_ecd_ic"] / n["l"] ** 2 * n["n_s"] / (n["n_s"] + 1.6))}


def a07(case):
    n = case["num"]
    return {"xu_d@cap": 0.25 if n["fck"] <= 50 else 0.15}


HANDLERS = {"A-07": a07, "A-03": a03, "A-04": a04, "A-05": a05, "A-06": a06, "A-08": a08, "A-09": a09, "A-11": a11, "A-12": a12, "A-14": a14}
