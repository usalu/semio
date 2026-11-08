"""🪝 Independent oracle of EN 1992-4 (FA-01, FA-06, FA-11 … FA-14): linear-elastic anchor load distribution by `numpy`, Eq. 7.2.1.8 blow-out, cone footprints as shapely unions (EN 1992-4 7.2.1.4, c_cr,N = 1.5 h_ef), pry-out V_Rk,cp = k_8 N_Rk,c and the NDP-based partial factors."""

import math

from shapely.geometry import box
from shapely.ops import unary_union
from clause_forms_shared import *

K1 = {("cast_in", True): 8.9, ("cast_in", False): 12.7, ("post", True): 7.7, ("post", False): 11.0}


def gamma_mc(case):
    annex = case["annex"]
    return ndp("ndp.en1992.m-01.gamma-c-persistent", annex) * case["num"].get("gamma_inst", ndp("ndp.en1992.gamma-inst", annex))


def fa01(case):
    n, lists = case["num"], case["list"]
    x, y = np.array(lists.get("ax", [0.0])), np.array(lists.get("ay", [0.0]))
    count = len(x)
    sx, sy = float(((x - x.mean()) ** 2).sum()), float(((y - y.mean()) ** 2).sum())
    loads = n.get("n_ed", 0.0) / count + (n.get("m_x", 0.0) * (y - y.mean()) / sy if sy > 0 else 0.0) + (n.get("m_y", 0.0) * (x - x.mean()) / sx if sx > 0 else 0.0)
    loads = np.maximum(loads, 0.0)
    values = {f"n_{i}": float(v) for i, v in enumerate(loads)}
    values["v_each"] = n.get("v_ed", 0.0) / count
    values["n_max"] = float(loads.max())
    return values


def cone_area_ratio(case, indices):
    n, lists = case["num"], case["list"]
    hef = n["hef"]
    ccr = 1.5 * hef
    xs, ys = lists.get("ax", [0.0]), lists.get("ay", [0.0])
    region = box(n.get("x_min", -1e9), n.get("y_min", -1e9), n.get("x_max", 1e9), n.get("y_max", 1e9))
    footprint = unary_union([box(xs[i] - ccr, ys[i] - ccr, xs[i] + ccr, ys[i] + ccr) for i in indices]).intersection(region)
    return footprint.area / (2 * ccr) ** 2, ccr


def cone_resistance(case, indices):
    n, lists = case["num"], case["list"]
    ratio, ccr = cone_area_ratio(case, indices)
    xs, ys = lists.get("ax", [0.0]), lists.get("ay", [0.0])
    n_rk0 = K1[(case["text"].get("type", "cast_in"), n.get("cracked", 1.0) > 0)] * math.sqrt(n["fck"]) * n["hef"] ** 1.5
    edge = min(min(xs[i] - n.get("x_min", -1e9), n.get("x_max", 1e9) - xs[i], ys[i] - n.get("y_min", -1e9), n.get("y_max", 1e9) - ys[i]) for i in indices)
    psi_s = min(1.0, 0.7 + 0.3 * edge / ccr)
    psi_re = min(1.0, 0.5 + n["hef"] / 200) if n.get("dense_reinforcement", 0.0) > 0 else 1.0
    return n_rk0 * ratio * psi_s * psi_re


def fa06(case):
    n = case["num"]
    if n["c1"] > 0.5 * n["hef"]:
        return {"applicable": 0.0}
    a_h = math.pi / 4 * (n["dh"] ** 2 - n["d"] ** 2)
    n_rk = 8.7 * n["c1"] * math.sqrt(a_h) * math.sqrt(n["fck"]) * max(1 + n.get("s2", 0.0) / (4 * n["c1"]), 1.0) * n.get("psi_h_nb", 1.0)
    values = {"n_rk_cb": n_rk}
    if "n_ed" in n:
        values["util"] = n["n_ed"] / (n_rk / gamma_mc(case))
    return values


def fa11(case):
    n, lists = case["num"], case["list"]
    xs, ys, ni = lists["ax"], lists["ay"], lists["ni"]
    tension = [i for i, v in enumerate(ni) if v > 0]
    total = sum(ni[i] for i in tension)
    ex = abs(sum(ni[i] * xs[i] for i in tension) / total - sum(xs[i] for i in tension) / len(tension))
    ey = abs(sum(ni[i] * ys[i] for i in tension) / total - sum(ys[i] for i in tension) / len(tension))
    ratio, _ = cone_area_ratio(case, tension)
    return {"e_nx": ex * MM, "e_ny": ey * MM, "ac_ratio": ratio}


def fa12(case):
    n = case["num"]
    rd = n["delta_n_rk_s_0"] * n.get("alpha_seis", 1.0) / ndp("ndp.en1992.gamma-ms-fat", case["annex"])
    return {"delta_n_rd": rd, "util": n["delta_n_ed"] / rd}


def fa13(case):
    n = case["num"]
    indices = list(range(len(case["list"].get("ax", [0.0]))))
    v_rk = (1.0 if n["hef"] < 60 else 2.0) * cone_resistance(case, indices)
    values = {"v_rk_cp": v_rk, "v_rd_cp": v_rk / gamma_mc(case)}
    if "v_ed" in n:
        values["util"] = n["v_ed"] / (v_rk / gamma_mc(case))
    return values


def fa14(case):
    n, annex = case["num"], case["annex"]
    as_re = n["n_ed_re"] / (n.get("fyk", 500.0) / ndp("ndp.en1992.gamma-ms-re", annex))
    values = {"as_re": as_re / 1e6}
    if "as_provided" in n:
        values["util"] = as_re / n["as_provided"]
    if "l_bd" in n and "l_s" in n:
        values["anchorage@util"] = n["l_bd"] / n["l_s"]
    return values


HANDLERS = {"FA-01": fa01, "FA-06": fa06, "FA-11": fa11, "FA-12": fa12, "FA-13": fa13, "FA-14": fa14}
