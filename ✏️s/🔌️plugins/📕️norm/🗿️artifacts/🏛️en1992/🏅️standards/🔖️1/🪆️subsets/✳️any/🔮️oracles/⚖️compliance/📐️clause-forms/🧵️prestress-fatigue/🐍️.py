"""🧵 Independent oracle of EN 1992-1-1 5.10, 6.8 and 8.10 (PR-01, PR-02, PR-06 … PR-10, F-01 … F-03, F-05 … F-07): stress limits k1/k2/k7/k8 from the NDP registry, Eq. 6.58 bursting force, Eq. 6.72/6.77/6.78 fatigue, Table 6.3N/6.4N S-N curves, the cracked-section steel range by the transformed-section neutral axis."""

import math

from clause_forms_shared import *

SN = {"straight": (1e6, 5.0, 9.0, "ndp.en1992.fatigue-straight"), "welded": (1e7, 3.0, 5.0, "ndp.en1992.fatigue-welded"), "coupler": (1e7, 3.0, 5.0, "ndp.en1992.fatigue-coupler")}
SN_PRESTRESS = {"pretensioned": (1e6, 5.0, 9.0, 185.0), "post_strand_plastic": (1e6, 5.0, 9.0, 185.0), "post_straight_plastic": (1e6, 5.0, 9.0, 150.0), "post_curved_steel": (1e6, 5.0, 7.0, 120.0), "prestress_coupler": (1e6, 5.0, 5.0, 80.0)}


def sn_curve(annex, kind):
    if kind in SN:
        n, k1, k2, row = SN[kind]
        return n, k1, k2, ndp(row, annex)
    return SN_PRESTRESS[kind]


def pr01(case):
    n, annex = case["num"], case["annex"]
    limit = min(ndp("ndp.en1992.p-k1", annex) * n["fpk"], ndp("ndp.en1992.p-k2", annex) * n["fp01k"])
    values = {"sigma_p_max": limit * MPA, "p_max": n["ap"] * limit}
    if "sigma_jack" in n:
        values["util"] = n["sigma_jack"] / limit
    return values


def pr02(case):
    n, annex = case["num"], case["annex"]
    limit = min(ndp("ndp.en1992.p-k7", annex) * n["fpk"], ndp("ndp.en1992.p-k8", annex) * n["fp01k"])
    values = {"sigma_pm0_max": limit * MPA, "pm0_max": n["ap"] * limit}
    if "pm0" in n:
        values["util"] = n["pm0"] / n["ap"] / limit
    return values


def pr06(case):
    n, annex = case["num"], case["annex"]
    sigma = n["sigma_pm_inf"] + n.get("delta_sigma", ndp("ndp.en1992.unbonded-dsigma", annex))
    return {"sigma_p_uls": sigma * MPA, "util": sigma / (n["fp01k"] / ndp("ndp.en1992.gamma-s", annex))}


def pr07(case):
    kind = "post-bonded" if case["text"].get("bond", "post_bonded") == "post_bonded" else "pre-or-unbonded"
    pm = case["num"]["pm_t"]
    return {"pk_sup": ndp(f"ndp.en1992.p-r-sup.{kind}", case["annex"]) * pm, "pk_inf": ndp(f"ndp.en1992.p-r-inf.{kind}", case["annex"]) * pm, "gamma_p": 1.0}


def pr08(case):
    n = case["num"]
    return {"q": 8 * n["p"] * n["sag"] / n["l"] ** 2 if "sag" in n else n["p"] / n["radius"]}


def pr09(case):
    n, annex = case["num"], case["annex"]
    t = 0.25 * (1 - n["a"] / n["h"]) * n["pmax"]
    required = t / fyd_of(annex, n.get("fyk", 500.0))
    values = {"t_burst": t, "as_burst": required / 1e6}
    if "as_provided" in n:
        values["util"] = required / n["as_provided"]
    return values


def pr10(case):
    n = case["num"]
    return {"util": n["sigma_c"] / ((0.7 if n.get("nonlinear_creep_proven", 0.0) > 0 else 0.6) * n["fck_t"])}


def f01(case):
    return {"required": 1.0 if case["text"].get("structure", "building") in ("bridge", "crane", "railway") else 0.0}


def f02(case):
    n = case["num"]
    b, d, a_s, ae = n["b"], n["d"], n["as"], n["alpha_e"]
    x = max(root.real for root in np.roots([b / 2, ae * a_s, -ae * a_s * d]))
    i2 = b * x**3 / 3 + ae * a_s * (d - x) ** 2
    sigma = lambda m: ae * m * (d - x) / i2
    return {"delta_sigma_s": (sigma(n["m_max"]) - (sigma(max(n["m_min"], 1.0)) if n["m_min"] > 0 else 0.0)) * MPA}


def f03(case):
    cycles, k1, k2, delta = sn_curve(case["annex"], case["text"]["kind"])
    return {"n_star": cycles, "k1": k1, "k2": k2, "dsigma_rsk": delta * MPA}


def f05(case):
    n, annex = case["num"], case["annex"]
    fck = n["fck"]
    fcd_fat = 0.85 * n.get("beta_cc_t0", 1.0) * fcd_of(annex, fck) * (1 - fck / 250)
    e_max, e_min = n["sigma_c_max"] / fcd_fat, n["sigma_c_min"] / fcd_fat
    r = e_min / e_max
    return {"util": e_max + 0.43 * math.sqrt(max(1 - r, 0.0)), "util_simple": e_max / min(0.5 + 0.45 * e_min, 0.9 if fck <= 50 else 0.8)}


def f06(case):
    n = case["num"]
    cap = 0.9 if n.get("fck", 30.0) <= 50 else 0.8
    values = {"util": (n["v_max"] / n["v_rd_c"]) / min(0.5 + 0.45 * n["v_min"] / n["v_rd_c"], cap)}
    if "cot_theta" in n:
        values["cot_fat"] = min(math.sqrt(n["cot_theta"]), 1.5)
    return values


def f07(case):
    n, annex = case["num"], case["annex"]
    lam = min(n.get("lambda1", 1.0) * n.get("lambda2", 1.0) * n.get("lambda3", 1.0) * n.get("lambda4", 1.0), n.get("lambda_max", 1e9))
    _, _, _, delta = sn_curve(annex, case["text"].get("kind", "straight"))
    return {"lambda_s": lam, "util": n.get("phi_fat", 1.0) * n["delta_sigma_71"] * lam / (delta / ndp("ndp.en1992.gamma-s-fat", annex))}


HANDLERS = {"PR-01": pr01, "PR-02": pr02, "PR-06": pr06, "PR-07": pr07, "PR-08": pr08, "PR-09": pr09, "PR-10": pr10, "F-01": f01, "F-02": f02, "F-03": f03, "F-05": f05, "F-06": f06, "F-07": f07}
