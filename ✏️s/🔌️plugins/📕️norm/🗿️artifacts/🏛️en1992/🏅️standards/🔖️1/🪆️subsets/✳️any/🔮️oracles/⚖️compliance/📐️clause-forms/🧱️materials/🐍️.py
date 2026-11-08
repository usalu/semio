"""🧱 Independent oracle of EN 1992-1-1 chapters 3, 4 and 7.1 (M-01 … M-14, D-01 … D-07): `structuralcodes.codes.ec2_2004` for the Table 3.1 laws, Eq. 3.1-3.5 and the stress-strain diagrams, otherwise the clause text transcribed here (Table 3.1 columns, Table 4.3N, 4.4N, 4.4DE, E.1DE); SI output (Pa, m, N·m)."""

import math

import numpy as np
from clause_forms_shared import *

def m01(case):
    fck, annex = case["num"]["fck"], case["annex"]
    suffix = "accidental" if case["text"].get("situation", "persistent") == "accidental" else "persistent"
    fcd = ec2.fcd(fck, ndp(f"ndp.en1992.m-01.alpha-cc-{suffix}", annex), ndp(f"ndp.en1992.m-01.gamma-c-{suffix}", annex))
    fctd = ec2.fctd(row_of(fck, FCTK05), ndp("ndp.en1992.m-01.alpha-ct", annex), ndp("ndp.en1992.m-01.gamma-c-persistent", annex))
    return {"fcd": fcd * MPA, "fctd": fctd * MPA, "fctk05": row_of(fck, FCTK05) * MPA}


def m03(case):
    return {"fck_cube": row_of(case["num"]["fck"], CUBE) * MPA}


def m04(case):
    fck, t = case["num"]["fck"], case["num"]["t"]
    s = ec2.s_time_development(case["text"].get("cement", "N"))
    beta = float(ec2.beta_cc(t, s))
    fcm = ec2.fcm(fck)
    alpha = 1.0 if t < 28 else 2.0 / 3.0
    fcm_t = float(ec2.fcm_time(fcm, beta))
    return {"fcm_t": fcm_t * MPA, "fctm_t": float(ec2.fctm_time(row_of(fck, FCTM), beta, alpha)) * MPA, "fck_t": (fcm_t - 8.0 if t < 28 else fck) * MPA}


def m06(case):
    fck, eps, annex = case["num"]["fck"], case["num"]["eps"], case["annex"]
    fcd = ec2.fcd(fck, ndp("ndp.en1992.m-01.alpha-cc-persistent", annex), ndp("ndp.en1992.m-01.gamma-c-persistent", annex))
    ec3, ecu3 = ec2.eps_c3(fck) * 1e3, ec2.eps_cu3(fck) * 1e3
    stress = 0.0 if eps <= 0 or eps > ecu3 + 1e-12 else float(np.interp(eps, [0.0, ec3, ecu3], [0.0, fcd, fcd]))
    return {"sigma_c": stress * MPA}


def m07(case):
    n, annex = case["num"], case["annex"]
    fck, b, x, d = n["fck"], n["b"], n["x"], n["d"]
    lam = 0.8 if fck <= 50 else 0.8 - (fck - 50) / 400
    eta = 1.0 if fck <= 50 else 1.0 - (fck - 50) / 200
    fcd = ec2.fcd(fck, ndp("ndp.en1992.m-01.alpha-cc-persistent", annex), ndp("ndp.en1992.m-01.gamma-c-persistent", annex))
    reduction = 0.9 if n.get("narrowing", 0.0) > 0 and annex == "De" else 1.0
    return {"lambda": lam, "eta": eta, "fc": reduction * eta * fcd * lam * x * b, "z": (d - lam * x / 2) * MM}


def m09(case):
    n, annex = case["num"], case["annex"]
    gamma = ndp("ndp.en1992.gamma-s", annex)
    ep, eps_ud = n.get("ep", 195000.0), n.get("eps_ud", 20.0)
    fpd, fpu = n["fp01k"] / gamma, n["fpk"] / gamma
    eps, eps_y = abs(n["eps"]) / 1000, fpd / ep
    sigma = ep * eps if eps <= eps_y else fpd + (fpu - fpd) * (min(eps, eps_ud / 1000) - eps_y) / (eps_ud / 1000 - eps_y)
    return {"sigma_p": math.copysign(sigma, n["eps"]) * MPA, "fpd": fpd * MPA}


def m12(case):
    n = case["num"]
    ecm = float(np.interp(n["fck"], GRADES, [27, 29, 30, 31, 33, 34, 35, 36, 37, 38, 39, 41, 42, 44])) * 1e3
    ec_eff = ecm / (1 + n["phi_inf"])
    return {"ec_eff": ec_eff * MPA, "alpha_e_eff": n.get("es", 200000.0) / ec_eff}


def m14(case):
    n = case["num"]
    fctm = row_of(n["fck"], FCTM)
    fl = max((1.6 - n["h"] / 1000) * fctm, fctm)
    w, area = n["b"] * n["h"] ** 2 / 6, n["b"] * n["h"]
    mcr = fl * w
    values = {"mcr": mcr / 1e3}
    if "p" in n and "e_p" in n:
        values["mcr_p"] = (mcr + n["p"] * (n["e_p"] + w / area)) / 1e3
    return values


EXPOSURE_ROW = {"X0": 0, "XC1": 1, "XC2": 2, "XC3": 2, "XC4": 3, "XD1": 4, "XS1": 4, "XD2": 5, "XS2": 5, "XD3": 6, "XS3": 6}
TABLE_4_4N = {"S1": [10, 10, 10, 15, 20, 25, 30], "S2": [10, 10, 15, 20, 25, 30, 35], "S3": [10, 10, 20, 25, 30, 35, 40], "S4": [10, 15, 25, 30, 35, 40, 45], "S5": [15, 20, 30, 35, 40, 45, 50], "S6": [20, 25, 35, 40, 45, 50, 55]}
TABLE_4_4DE = [10, 10, 20, 25, 40, 40, 40]
STRENGTH_FOR_CLASS_REDUCTION = [30, 30, 35, 40, 40, 40, 45]
TABLE_E1DE = {"X0": 12, "XC1": 16, "XC2": 16, "XC3": 20, "XC4": 25, "XD1": 30, "XS1": 30, "XD2": 35, "XS2": 35, "XD3": 35, "XS3": 35, "XF1": 25, "XF4": 30, "XA1": 25, "XA2": 35, "XA3": 35, "XM1": 30, "XM2": 35, "XM3": 35}


def c_min_dur(annex, row, structural_class):
    return float(TABLE_4_4DE[row] if annex == "De" else TABLE_4_4N[f"S{min(max(int(structural_class), 1), 6)}"][row])


def d01(case):
    return {"row": float(max(EXPOSURE_ROW[c.strip()] for c in case["text"]["classes"].split(",") if c.strip() in EXPOSURE_ROW))}


def d02(case):
    air = case["num"].get("air_entrained", 0.0) > 0
    need = 0.0
    for name in (c.strip() for c in case["text"]["classes"].split(",")):
        if name in ("XF2", "XF3"):
            need = max(need, 25 if air else 35)
        elif name in TABLE_E1DE:
            need = max(need, TABLE_E1DE[name])
    return {"fck_min@cap": need}


def d03(case):
    n, annex = case["num"], case["annex"]
    phi_n = min(n["phi"] * math.sqrt(max(n.get("nb", 1.0), 1.0)), 55.0)
    cmin_b = max(phi_n, n.get("dg", 16.0) + 5.0) if annex == "De" and n.get("dg", 16.0) > 16 else phi_n
    dur = c_min_dur(annex, EXPOSURE_ROW[case["text"]["exposure"]], n.get("structural_class", 4.0))
    cmin = max(cmin_b, dur + n.get("dc_gamma", 0.0) - n.get("dc_st", 0.0) - n.get("dc_add", 0.0), 10.0)
    return {"cmin_b": cmin_b * MM, "cmin_dur": dur * MM, "cmin": cmin * MM}


def d04(case):
    n, annex = case["num"], case["annex"]
    row = EXPOSURE_ROW[case["text"]["exposure"]]
    s = 4 + (2 if n.get("design_life", 50.0) >= 100 else 0) - (n["fck"] >= STRENGTH_FOR_CLASS_REDUCTION[row]) - (n.get("slab", 0.0) > 0) - (n.get("quality", 0.0) > 0) - (n.get("special", 0.0) > 0)
    s = min(max(s, 1), 6)
    cmin = max(n["phi"], c_min_dur(annex, row, s), 10.0)
    dev = n.get("dc_dev", ndp("ndp.en1992.delta-c-dev", annex))
    cnom = cmin + dev
    return {"structural_class": float(s), "cnom": cnom * MM, "c_provided@cap": cnom * MM}


def d05(case):
    need = {"soil": 75.0, "blinding": 40.0}.get(case["text"].get("ground", "none"), 0.0)
    return {"c_nom_ground@cap": need * MM}


def d07(case):
    life = case["num"]["design_life"]
    return {"category": float(1 if life <= 10 else 2 if life <= 30 else 4 if life <= 50 else 5)}


HANDLERS = {"M-01": m01, "M-03": m03, "M-04": m04, "M-06": m06, "M-07": m07, "M-09": m09, "M-12": m12, "S-12": m12, "M-14": m14, "D-01": d01, "D-02": d02, "D-03": d03, "D-04": d04, "D-05": d05, "D-07": d07}
