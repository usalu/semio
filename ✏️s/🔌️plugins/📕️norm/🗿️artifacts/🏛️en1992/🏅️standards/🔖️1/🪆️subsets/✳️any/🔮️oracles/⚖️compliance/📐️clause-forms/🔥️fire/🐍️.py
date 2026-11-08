"""🔥 Independent oracle of EN 1992-1-2 (R-01, D-06, R-02 gas curve, R-06 … R-08, M-13): the tabulated Tables 5.2a, 5.4, 5.5, 5.6, 5.8-5.10 and 5.? transcribed as columns of (R, b_min, a), Eq. 3.1-3.? reduction factors of Tables 3.1 and 3.2a and the thermal strain of 3.3.1, ISO 834 gas curve of EN 1991-1-2 Eq. 3.4."""

import math

from clause_forms_shared import *

R_CLASSES = [30, 60, 90, 120, 180, 240]
BEAM_SS = [([80, 120, 150, 200, 240, 280], [25, 40, 55, 65, 80, 90]), ([120, 160, 200, 240, 300, 350], [20, 35, 45, 60, 70, 80]), ([160, 200, 300, 300, 400, 500], [15, 30, 40, 55, 65, 75]), ([200, 300, 400, 500, 600, 700], [15, 25, 35, 50, 60, 70])]
TABLES = {
    "beam_cont": [([80, 120, 150, 200, 240, 280], [15, 25, 35, 45, 60, 75])],
    "slab_one_way": [([60, 80, 100, 120, 150, 175], [10, 20, 30, 40, 55, 65])],
    "slab_two_way": [([60, 80, 100, 120, 150, 175], [10, 10, 15, 20, 30, 40])],
    "flat_slab": [([150, 180, 200, 200, 200, 200], [10, 15, 25, 35, 45, 50])],
    "column_0": [([200, 200, 200, 250, 350, 350], [25, 25, 31, 40, 45, 61])],
    "column_1": [([200, 200, 300, 350, 350, 450], [25, 36, 45, 45, 63, 75])],
    "column_2": [([200, 250, 350, 350, 450, 450], [32, 46, 53, 57, 70, 80])],
    "wall": [([120, 130, 140, 160, 210, 270], [10, 10, 25, 35, 50, 60])],
    "tension": [([80, 120, 150, 200, 240, 280], [25, 40, 55, 65, 80, 90])],
}
KC = ([20, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200], [1.0, 1.0, 0.95, 0.85, 0.75, 0.60, 0.45, 0.30, 0.15, 0.08, 0.04, 0.01, 0.0])
KS = ([20, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200], [1.0, 1.0, 0.78, 0.47, 0.23, 0.11, 0.06, 0.04, 0.02, 0.0])


def columns_of(kind, sub, key):
    if kind in ("beam", "beam_ss"):
        return BEAM_SS
    if kind == "slab":
        return TABLES["slab_two_way" if sub == "two_way" else "slab_one_way"]
    if kind == "column":
        return TABLES["column_0" if key <= 0.2 else "column_1" if key <= 0.5 else "column_2"]
    return TABLES[kind]


def r01(case):
    n, text, annex = case["num"], case["text"], case["annex"]
    columns = columns_of(text["kind"], text.get("sub", "one_way"), n.get("mu_fi", n.get("width", 0.0)))
    r, dim, axis = n["r_class"], n["dim"], n["a"]
    at = [(float(np.interp(r, R_CLASSES, sizes)), float(np.interp(r, R_CLASSES, axes))) for sizes, axes in columns]
    feasible = [(d, a) for d, a in at if dim >= d]
    d_min, a_min = min(feasible, key=lambda pair: pair[1]) if text["kind"] in ("beam", "beam_ss") and feasible else (at[-1] if text["kind"] in ("beam", "beam_ss") else at[0])
    if n.get("fck", 30.0) >= 70:
        a_min += ndp("ndp.en1992.spalling-add-mm", annex)
    achieved = max([c for sizes, axes in columns for c, s, a in zip(R_CLASSES, sizes, axes) if dim >= s and axis >= a] or [0])
    return {"dim@cap": d_min * MM, "axis@cap": a_min * MM, "r_achieved": float(achieved)}


def r02(case):
    return {"gas": 20 + 345 * math.log10(8 * case["num"]["t_min"] + 1)}


def r06(case):
    n, annex = case["num"], case["annex"]
    gk, qk = n["gk"], n.get("qk", 0.0)
    gamma_g = n.get("gamma_g", ndp("ndp.en1990.gG_sup.str.A1", annex))
    gamma_q = n.get("gamma_q", ndp("ndp.en1990.gQ.str.A1", annex))
    return {"eta_fi": (gk + n.get("psi_fi", 0.5) * qk) / (gamma_g * gk + gamma_q * qk), "e_d_fi": gk + n.get("psi_fi", 0.5) * qk}


def r07(case):
    theta = case["num"]["theta"]
    strain = -1.8e-4 + 9e-6 * theta + 2.3e-11 * theta**3 if theta <= 700 else 14e-3
    return {"kc": float(np.interp(theta, *KC)), "ks": float(np.interp(theta, *KS)), "eps_th": strain * 1000}


def r08(case):
    fck = case["num"]["fck"]
    return {"spalling_risk": 1.0 if fck > 80 else 0.0, "axis_add": (ndp("ndp.en1992.spalling-add-mm", case["annex"]) if fck >= 70 else 0.0) * MM}


HANDLERS = {"R-01": r01, "D-06": r01, "R-02": r02, "R-06": r06, "R-07": r07, "M-13": r07, "R-08": r08}
