"""🔗 Independent oracle of EN 1992-1-1 chapters 8 and 9 (B-04 … B-06, B-08, B-09, DT-02 … DT-08, DT-10 … DT-13): Eq. 8.2 bond stress, Eq. 8.10 and Table 8.2 lap factors, 8.9 bundled bars, 8.5 hooks, 9.2 beams, 9.3 slabs, 9.4 flat slabs, 9.6 walls, 9.7 deep beams, 9.8 foundations and 9.9 regions with discontinuity written from the clause text."""

import math

from clause_forms_shared import *


def fbd_of(annex, fck, phi, good):
    eta2 = 1.0 if phi <= 32 else (132 - phi) / 100
    return ndp("ndp.en1992.fbd-eta", annex) * (1.0 if good else 0.7) * eta2 * fctd_of(annex, min(fck, 60.0))


def clamp(value, low, high):
    return min(max(value, low), high)


def b04(case):
    n, annex = case["num"], case["annex"]
    phi = n["phi"]
    fbd = n["fbd"] if "fbd" in n else fbd_of(annex, n["fck"], phi, case["text"].get("bond", "good") == "good")
    sigma = n["sigma_sd"] if "sigma_sd" in n else fyd_of(annex, n.get("fyk", 500.0))
    lb = phi / 4 * sigma / fbd
    compression = case["text"].get("loading", "tension") == "compression"
    a6 = 1.0 if compression else clamp(math.sqrt(n.get("rho1", 50.0) / 25), 1.0, 1.5)
    a2 = 1.0 if compression else clamp(1 - 0.15 * (n.get("cd", 3 * phi) - phi) / phi, 0.7, 1.0)
    a3 = 1.0 if compression else clamp(n.get("alpha3", 1.0), 0.7, 1.0)
    a5 = clamp(1 - 0.04 * n.get("p_transverse", 0.0), 0.7, 1.0)
    l0_min = max(0.3 * a6 * lb, 15 * phi, 200.0)
    l0 = max(n.get("alpha1", 1.0) * a2 * a3 * a5 * a6 * lb, l0_min)
    values = {"l0": l0 * MM, "l0_min": l0_min * MM}
    if "l_available" in n:
        values["util"] = l0 / n["l_available"]
    if "clear_distance" in n:
        values["clear@cap"] = max(2 * phi, 20.0) * MM
    if "sum_ast" in n and "as" in n:
        values["transverse@cap"] = n["as"] / 1e6
    return values


def b05(case):
    n = case["num"]
    return {"phi_n": min(n["phi"] * math.sqrt(n["nb"]), 55.0) * MM, "nb@util": n["nb"] / (4.0 if case["text"].get("situation", "other") == "compression" else 3.0)}


def b06(case):
    n = case["num"]
    if n["phi"] <= 32:
        return {"applicable": 0.0}
    values = {"as_surf": 0.01 * n["act_ext"] / 1e6, "eta2": (132 - n["phi"]) / 100}
    if "as_surf_provided" in n:
        values["util"] = 0.01 * n["act_ext"] / n["as_surf_provided"]
    return values


def b08(case):
    phi = case["num"]["phi"]
    return {"hook_length": (max(10 * phi, 70.0) if case["text"].get("angle", "135") == "90" else max(5 * phi, 50.0)) * MM, "mandrel": 4 * phi * MM}


def b09(case):
    n = case["num"]
    return {"clear@cap": max(n["phi"], n.get("dg", 16.0) + 5, 20.0) * MM}


def dt02(case):
    n = case["num"]
    a_l = n["z"] * (n.get("cot_theta", 2.5) - n.get("cot_alpha", 0.0)) / 2
    values = {"a_l": a_l * MM, "f_ed": n["v_ed"] * a_l / n["z"] + n.get("n_ed", 0.0)}
    if "as_support" in n:
        values["as_support@cap"] = 0.25 * n["as_span"] / 1e6
    return values


def dt03(case):
    n, annex = case["num"], case["annex"]
    fyk = n.get("fyk", 500.0)
    rho_min = 0.16 * row_of(n["fck"], FCTM) / fyk if annex == "De" else 0.08 * math.sqrt(n["fck"]) / fyk
    alpha = math.radians(n.get("alpha_deg", 90.0))
    s_l = 0.75 * n["d"] * (1 + 1 / math.tan(alpha))
    values = {"rho_w_min": rho_min, "sl_max": s_l * MM, "st_max": min(0.75 * n["d"], 600.0) * MM}
    if "asw" in n and "s" in n:
        values["rho_w@cap"] = rho_min
        values["s_util@cap"] = s_l * MM
    return values


def dt04(case):
    n = case["num"]
    limit = min(n["u"] / 8, 0.75 * n["d"], n["b"])
    values = {"s_max": limit * MM}
    if "s" in n:
        values["util"] = n["s"] / limit
    return values


def dt05(case):
    n = case["num"]
    required = 0.01 * n["act_ext"]
    values = {"as_surf": required / 1e6}
    if "as_provided" in n:
        values["util"] = required / n["as_provided"]
    if "s" in n:
        values["spacing@cap"] = 150.0 * MM
    return values


def dt06(case):
    n, annex = case["num"], case["annex"]
    h, d = n["h"], n["d"]
    a_min = max(0.26 * row_of(n["fck"], FCTM) / n.get("fyk", 500.0) * 1000 * d, 0.0013 * 1000 * d)
    s_max = min(3 * h, 400.0)
    if annex == "De" and h >= 250:
        s_max = min(ndp("ndp.en1992.slab-smax-cracked", annex), s_max)
    values = {"as_min": a_min / 1e6, "s_max": s_max * MM, "s_max_secondary": min(3.5 * h, 450.0) * MM, "s_max_concentrated": min(2 * h, 250.0) * MM}
    if "as_main" in n and "s" in n:
        values["as_main@cap"] = a_min / 1e6
        values["s_util@cap"] = s_max * MM
        if "as_secondary" in n:
            values["as_secondary@cap"] = 0.2 * n["as_main"] / 1e6
    return values


def dt07(case):
    n = case["num"]
    values = {"h_links@cap": 200.0 * MM}
    if "v_rd_max" in n:
        values["min_links@cap"] = n["v_rd_max"] / 3
    if "v_rd_c" in n:
        values["no_links@cap"] = n["v_rd_c"]
    return values


def dt08(case):
    n, annex = case["num"], case["annex"]
    integrity = n["v_ed"] / fyd_of(annex, n.get("fyk", 500.0), accidental=True)
    values = {"as_integrity": integrity / 1e6, "as_strip_min": 0.5 * n["a_t"] / 1e6}
    if "as_through" in n:
        values["integrity@cap"] = integrity / 1e6
    if "m_edge" in n and "b_e" in n and "d" in n:
        values["m_transfer@cap"] = 0.17 * n["b_e"] * n["d"] ** 2 * n["fck"] / 1e3
    return values


def dt10(case):
    n = case["num"]
    ac = n["ac"]
    values = {"asv_min": 0.002 * ac / 1e6, "asv_max": 0.04 * ac / 1e6, "sv_max": min(3 * n["tw"], 400.0) * MM}
    if "asv" in n:
        values["asv@cap"] = 0.002 * ac / 1e6
        values["ash_min"] = max(0.25 * n["asv"], 0.001 * ac) / 1e6
        values["links@util"] = n["asv"] / (0.02 * ac)
    return values


def dt11(case):
    n = case["num"]
    required = max(0.001 * n["ac_per_m"], 150.0)
    values = {"as_mesh": required / 1e6, "s_max": min(2 * n["h"], 300.0) * MM}
    if "as_provided" in n:
        values["util"] = required / n["as_provided"]
    return values


def dt12(case):
    n, annex = case["num"], case["annex"]
    if case["text"].get("kind", "strip") == "pile_cap_2":
        tie = n["n_ed"] / 2 * (n["s"] / 2 - n["c"] / 4) / (0.9 * n["d"])
    else:
        tie = n["n_ed"] * (n["b"] - n["a"]) / (8 * n["d"])
    required = tie / fyd_of(annex, n.get("fyk", 500.0))
    values = {"f_td": tie, "as_tie": required / 1e6}
    if "as_provided" in n:
        values["util"] = required / n["as_provided"]
    return values


def dt13(case):
    n, annex = case["num"], case["annex"]
    phi_m = n["f_bt"] * (1 / n["a_b"] + 1 / (2 * n["phi"])) / fcd_of(annex, n["fck"])
    values = {"phi_m_min": phi_m * MM}
    if "phi_m" in n:
        values["util"] = phi_m / n["phi_m"]
    if "m_ed" in n and "z" in n:
        values["corner_tension"] = n["m_ed"] / n["z"]
    return values


HANDLERS = {"B-04": b04, "B-05": b05, "B-06": b06, "B-08": b08, "B-09": b09, "DT-02": dt02, "DT-03": dt03, "DT-04": dt04, "DT-05": dt05, "DT-06": dt06, "DT-07": dt07, "DT-08": dt08, "DT-10": dt10, "DT-11": dt11, "DT-12": dt12, "DT-13": dt13}
