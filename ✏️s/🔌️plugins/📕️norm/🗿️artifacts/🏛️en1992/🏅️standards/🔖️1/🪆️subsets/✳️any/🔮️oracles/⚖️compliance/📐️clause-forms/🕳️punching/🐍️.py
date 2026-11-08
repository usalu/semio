"""🕳 Independent oracle of EN 1992-1-1 6.4 (P-01, P-05, P-06, P-07): control perimeters as shapely buffers of the loaded area clipped by the free slab edges, the punching resistance of Eq. 6.47 from the clause, the foundation perimeter scan by `scipy.optimize.minimize_scalar`."""

import math

from scipy import optimize
from shapely.geometry import LineString, box
from clause_forms_shared import *

POSITION_BETA = {"interior": "ndp.en1992.beta-interior", "edge": "ndp.en1992.beta-edge", "corner": "ndp.en1992.beta-corner", "wall_end": "ndp.en1992.beta-wall-end"}


def clipped_perimeter(c1, c2, offset, e1, e2, edges):
    column = box(0.0, 0.0, c1, c2)
    zone = column.buffer(offset, quad_segs=256)
    window = box(-e1 if edges >= 1 else -1e9, -e2 if edges >= 2 else -1e9, 1e9, 1e9)
    clipped = zone.intersection(window)
    total = clipped.exterior.length
    if edges >= 1:
        total -= clipped.exterior.intersection(LineString([(-e1, -1e9), (-e1, 1e9)])).length
    if edges >= 2:
        total -= clipped.exterior.intersection(LineString([(-1e9, -e2), (1e9, -e2)])).length
    return total


def u1_of(position, c1, c2, d, e1, e2):
    if position == "interior" or (position in ("edge", "wall_end") and e1 >= 2 * d):
        return 2 * (c1 + c2) + 4 * math.pi * d
    if position in ("edge", "wall_end"):
        return clipped_perimeter(c1, c2, 2 * d, e1, 0.0, 1)
    return clipped_perimeter(c1, c2, 2 * d, min(e1, 2 * d), min(e2, 2 * d), 2)


def u0_of(position, c1, c2, d):
    return {"interior": 2 * (c1 + c2), "edge": min(c2 + 3 * d, c2 + 2 * c1), "wall_end": min(c2 + 3 * d, c2 + 2 * c1), "corner": min(3 * d, c1 + c2)}[position]


def p01(case):
    n, position = case["num"], case["text"].get("position", "interior")
    c1, d = n["c1"], n["d"]
    c2 = n.get("c2", c1)
    if n.get("circular", 0.0) > 0:
        u0, u1 = math.pi * c1, math.pi * (c1 + 4 * d)
    else:
        u0, u1 = u0_of(position, c1, c2, d), u1_of(position, c1, c2, d, n.get("e1", 0.0), n.get("e2", 0.0))
    return {"u0": u0 * MM, "u1": u1 * MM, "uout_dist": ndp("ndp.en1992.punch-k-out", case["annex"]) * d * MM}


def v_rd_c(annex, fck, d, rho_y, rho_z, sigma_cp, position, u0_over_d):
    gamma = ndp("ndp.en1992.m-01.gamma-c-persistent", annex)
    k = min(1 + math.sqrt(200 / d), 2.0)
    rho = min(math.sqrt(rho_y * rho_z), 0.02)
    if annex == "De":
        rho = min(rho, 0.5 * fcd_of(annex, fck) / fyd_of(annex, 500.0))
    numerator = ndp("ndp.en1992.crd-c-punch", annex)
    if position == "interior" and u0_over_d < ndp("ndp.en1992.punch-small-u0.limit", annex):
        numerator = max(numerator * (ndp("ndp.en1992.punch-small-u0.slope", annex) * u0_over_d + ndp("ndp.en1992.punch-small-u0.offset", annex)), ndp("ndp.en1992.punch-small-u0.floor", annex))
    coefficient = ndp("ndp.en1992.vmin-shear", annex) if annex == "En" else float(np.interp(d, [600, 800], [ndp("ndp.en1992.vmin-shear", annex) / gamma, ndp("ndp.en1992.vmin-shear-d800", annex) / gamma]))
    k1 = ndp("ndp.en1992.k1-punch", annex)
    return max(numerator / gamma * k * (100 * rho * fck) ** (1 / 3) + k1 * sigma_cp, coefficient * k**1.5 * math.sqrt(fck) + k1 * sigma_cp)


def punching_state(case):
    n, annex = case["num"], case["annex"]
    position = case["text"].get("position", "interior")
    c1, d = n["c1"], n["d"]
    c2 = n.get("c2", c1)
    beta = max(ndp(POSITION_BETA[position], annex), 1.0)
    u1 = u1_of(position, c1, c2, d, n.get("e1", 0.0), n.get("e2", 0.0))
    u0 = u0_of(position, c1, c2, d)
    vrdc = v_rd_c(annex, n["fck"], d, n["rho_ly"], n["rho_lz"], n.get("sigma_cp", 0.0), position, u0 / d)
    return beta, u0, u1, vrdc


def p05(case):
    n, annex = case["num"], case["annex"]
    beta, u0, u1, vrdc = punching_state(case)
    d = n["d"]
    approval = n.get("approval", 0.0) > 0
    if annex == "De":
        limit = ndp("ndp.en1992.punch-vrdmax-factor", annex) * (1.4 if approval else 1.0) * vrdc
        shown = beta * (n["v_ed"] - n.get("delta_v", 0.0)) / (u1 * d)
    else:
        limit = 0.5 * nu_of(n["fck"]) * fcd_of(annex, n["fck"])
        shown = beta * (n["v_ed"] - n.get("delta_v", 0.0)) / (u0 * d)
    return {"util_max": shown / limit}


def p06(case):
    n, annex = case["num"], case["annex"]
    fck, d, v_ed, sigma = n["fck"], n["d"], n["v_ed"], n.get("sigma_gd", 0.0)
    c1 = n["c1"]
    c2 = n.get("c2", c1)
    vrdc = v_rd_c(annex, fck, d, n["rho_ly"], n["rho_lz"], 0.0, "interior", math.inf)
    column = box(0, 0, c1, c2)

    def util(a):
        zone = column.buffer(a, quad_segs=256)
        return -n.get("beta", 1.0) * (v_ed - sigma * zone.area) / (zone.exterior.length * d) / (vrdc * 2 * d / a)

    grid = np.linspace(0.5 * d, 2 * d, 61)
    best = max(-util(a) for a in grid)
    refined = optimize.minimize_scalar(util, bounds=(0.5 * d, 2 * d), method="bounded", options={"xatol": 1e-9})
    return {"util": max(best, -refined.fun)}


def p07(case):
    n = case["num"]
    beta, u0, u1, vrdc = punching_state(case)
    d = n["d"]
    vp = n.get("p_force", 0.0) * math.sin(n.get("tendon_angle", 0.0))
    return {"util": max(beta * (n["v_ed"] - vp) / (u1 * d), 0.0) / vrdc}


def p03_clause(case):
    n = case["num"]
    beta, u0, u1, vrdc = punching_state(case)
    v_ed = beta * (n["v_ed"] - n.get("delta_v", 0.0)) / (u1 * n["d"])
    return {"v_ed": v_ed * MPA, "util": v_ed / vrdc}


HANDLERS = {"P-03@clause": p03_clause, "P-01": p01, "P-05": p05, "P-06": p06, "P-07": p07}
