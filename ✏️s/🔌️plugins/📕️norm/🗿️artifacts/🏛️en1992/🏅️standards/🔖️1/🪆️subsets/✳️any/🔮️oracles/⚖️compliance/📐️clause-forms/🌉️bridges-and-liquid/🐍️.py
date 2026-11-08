"""🌉 Independent oracle of EN 1992-2 and EN 1992-3 (BR-01 … BR-08, LQ-01 … LQ-06): each rule composes the independent part 1-1 handlers of the oracle registry with the part 2/3 modification written from the clause text (long-term factor, chloride cover addition, bridge f_ct,eff floor, tightness-class limits, early-age restraint, self-healing)."""

import math

import clause_forms_shared as shared
from clause_forms_shared import *


def derived(case, num=None, text=None, drop=()):
    return {**case, "num": {**{k: v for k, v in case["num"].items() if k not in drop}, **(num or {})}, "text": {**case["text"], **(text or {})}}


def wk_limit(annex, exposure, bonded):
    if bonded:
        return ndp("ndp.en1992.wk-lim-prestress", annex)
    return ndp("ndp.en1992.wk-lim-xc1" if exposure in ("X0", "XC1") else "ndp.en1992.wk-lim", annex)


def br01(case):
    n, annex = case["num"], case["annex"]
    lt = ndp("ndp.en1992.bridge-alpha-cc-lt", annex) if n.get("prestressed", 0.0) > 0 else 1.0
    return {"fcd": fcd_of(annex, n["fck"]) * lt * MPA, "fck_limit@cap": 70.0}


def br02(case):
    n, annex = case["num"], case["annex"]
    inner = derived(case, num={"dc_dev": 5.0 if n.get("precast", 0.0) > 0 else 10.0, "design_life": n.get("design_life", 100.0)})
    values = shared.REGISTRY["D-04"](inner)
    if case["text"]["exposure"] in ("XD3", "XS3"):
        add = ndp("ndp.en1992.bridge-cover-chloride", annex) * MM
        values["cnom"] += add
        values["c_provided@cap"] += add
    return values


def br03(case):
    values = dict(shared.REGISTRY["V-03"](case))
    if "bi" in case["num"]:
        for key, value in shared.REGISTRY["V-05"](derived(case, num={"fatigue": 1.0})).items():
            values[f"fatigue_joint_{key}"] = value
    return values


def br04(case):
    n, annex = case["num"], case["annex"]
    values = dict(shared.REGISTRY["F-07"](case))
    if "delta_sigma_tendon" in n:
        delta = shared.REGISTRY["F-03"](derived(case, text={"kind": case["text"].get("tendon_kind", "post_curved_steel")}))["dsigma_rsk"] / MPA
        values["util_tendon"] = n["delta_sigma_tendon"] / (delta / ndp("ndp.en1992.gamma-s-fat", annex))
    return values


def br05(case):
    n, annex = case["num"], case["annex"]
    fct = max(n.get("fct_eff", row_of(n["fck"], FCTM)), ndp("ndp.en1992.bridge-fct-min", annex))
    values = dict(shared.REGISTRY["SL-04"](derived(case, num={"fct_eff": fct})))
    limit = wk_limit(annex, case["text"].get("exposure", "XC2"), n.get("prestressed_bonded", 0.0) > 0)
    values["wk_lim"] = limit * MM
    if "wk" in n:
        values["util_wk"] = n["wk"] / limit
    return values


def br06(case):
    n = case["num"]
    values = {"m_sec": (n["m_total"] - n["p"] * n["e_p"]) / 1e3}
    if "sag" in n and "l" in n:
        values["q_dev"] = 8 * n["p"] * n["sag"] / n["l"] ** 2
    return values


def br07(case):
    n = case["num"]
    size = n.get("wheel_print", 400.0) + n["h"]
    return shared.REGISTRY["P-03@clause"](derived(case, num={"c1": size, "c2": size, "v_ed": n["q_wheel"]}, text={"position": "interior"}))


def br08(case):
    return shared.REGISTRY["A-09"](case)


def lq01_limit(case):
    n, annex = case["num"], case["annex"]
    if int(n["class"]) == 0:
        return wk_limit(annex, case["text"].get("exposure", "XC2"), False)
    return float(np.interp(n["hd_over_h"], [5.0, 35.0], [ndp("ndp.en1992.tightness-wk-high", annex), ndp("ndp.en1992.tightness-wk-low", annex)]))


def lq01(case):
    n = case["num"]
    limit = lq01_limit(case)
    values = {"wk_lim": limit * MM}
    if n["class"] >= 2:
        values["compression_zone@cap"] = min(50.0, 0.2 * n["h"]) * MM
    if "wk" in n:
        values["util"] = n["wk"] * 1.0 / limit
    return values


def lq02(case):
    n = case["num"]
    free = n["eps_free"] if "eps_free" in n else n.get("eps_ca", 0.0) + n.get("eps_cd", 0.0) + n.get("alpha_c", 1.0e-5) * n.get("delta_t", 0.0)
    eps = n["rax"] * free
    values = {"eps_sm_cm": eps, "wk": n["sr_max"] * eps * MM}
    if "wk_lim" in n:
        values["util"] = n["sr_max"] * eps / n["wk_lim"]
    return values


def lq03(case):
    n = case["num"]
    limit = n["wk_lim"] if "wk_lim" in n else lq01_limit(case)
    values = dict(shared.REGISTRY["SL-06"](case))
    values["util"] = values["sr_max"] / MM * values["eps_diff"] / limit
    return {key: value for key, value in values.items() if key in ("sr_max", "eps_diff", "util")}


def lq04(case):
    n, annex = case["num"], case["annex"]
    head = n["hd_over_h"] <= ndp("ndp.en1992.lq-04-head-ratio", annex)
    limit = ndp("ndp.en1992.lq-04-wk-selfhealing" if head else "ndp.en1992.lq-04-wk-above", annex)
    return {"util": n["wk"] / limit, "self_healing": 1.0 if head else 0.0}


def lq05(case):
    values = dict(shared.REGISTRY["D-04"](case))
    add = ndp("ndp.en1992.liquid-cover-add", case["annex"]) * MM
    values["cnom"] += add
    values["c_provided@cap"] += add
    return values


def lq06(case):
    n, annex = case["num"], case["annex"]
    ring = n["pressure"] * n["radius"]
    sigma = (n["pm_t_per_m"] - ring) / n["t"]
    return {"ring_force": ring, "util_decompression": ndp("ndp.en1992.tank-sigma-min", annex) / max(sigma, 1e-9)}


HANDLERS = {"BR-01": br01, "BR-02": br02, "BR-03": br03, "BR-04": br04, "BR-05": br05, "BR-06": br06, "BR-07": br07, "BR-08": br08, "LQ-01": lq01, "LQ-02": lq02, "LQ-03": lq03, "LQ-04": lq04, "LQ-05": lq05, "LQ-06": lq06}
