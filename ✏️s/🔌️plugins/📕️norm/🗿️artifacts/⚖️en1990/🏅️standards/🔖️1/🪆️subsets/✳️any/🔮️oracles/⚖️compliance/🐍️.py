#!/usr/bin/env python3
"""🐍️ Independent EN 1990 compliance oracle (Python, scipy).

Nothing is imported from or transliterated out of the Rust kernel. Combinations are enumerated by brute force with itertools
(every choice of active exclusive group and of leading unit, with the equation written out term by term from EN 1990 6.4.3);
Phi, Phi^-1, Gumbel, lognormal and Weibull come from scipy.stats; FORM is an optimisation of the design point in standard
normal space (scipy.optimize) cross-checked against exact failure-probability integration (scipy.integrate); Palmgren-Miner
uses the S-N curve of EN 1993-1-9 evaluated point by point. Usage: `python -I 🐍️.py catalogue <fixture.json>` and
`python -I 🐍️.py snapshot` (snapshot JSON on stdin); both print JSON.
"""
import importlib.util
import itertools
import json
import math
import sys
from pathlib import Path

import numpy as np
from scipy import integrate, optimize, stats

N = stats.norm

_FACTORS_PATH = next(parent / "⚖️compliance" / "📜️annex-and-tables" / "⚖️action-factors" / "🐍️.py" for parent in Path(__file__).resolve().parents if (parent / "⚖️compliance" / "📜️annex-and-tables" / "⚖️action-factors" / "🐍️.py").is_file())
_SPEC = importlib.util.spec_from_file_location("action_factors", _FACTORS_PATH)
FACTORS = importlib.util.module_from_spec(_SPEC)
_SPEC.loader.exec_module(FACTORS)
ANNEXES = ("En", "De")
PSI_KEYS = ['residential', 'office', 'congregation', 'retail', 'storage', 'traffic_light', 'traffic_heavy', 'roof', 'snow', 'snow_high', 'wind', 'temperature', 'road_traffic', 'road_udl', 'road_pedestrian', 'road_single_axle', 'road_horizontal', 'road_gr3_pedestrian', 'road_gr4_crowd', 'road_special', 'wind_bridge', 'wind_bridge_execution', 'temperature_bridge', 'snow_construction', 'construction_load', 'footbridge_crowd', 'footbridge_service', 'rail_traffic', 'rail_sw2', 'rail_unloaded', 'rail_hslm', 'wind_rail', 'settlement', 'variable_other', 'temperature_bridge']
PSI = {a: {name: tuple(FACTORS.ndp(f"ndp.en1990.psi.{name}.{i}", a) for i in range(3)) for name in PSI_KEYS} for a in ANNEXES}
LETTERS = dict(zip("ABCDEFGH", ["residential", "office", "congregation", "retail", "storage", "traffic_light", "traffic_heavy", "roof"]))

# (gamma_G_sup, gamma_G_inf) per limit-state family and annex group; gamma_Q per family and traffic kind.
FAMILY = {"wind_bridge": "wind", "wind_rail": "wind", "temperature_bridge": "thermal"}
FAMILIES = ("equ", "str", "geo")
GAMMA_G = {a: {(fam, group): (FACTORS.ndp(f"ndp.en1990.gG_sup.{fam}.A{group}", a), FACTORS.ndp(f"ndp.en1990.gG_inf.{fam}.A{group}", a)) for fam in FAMILIES for group in (1, 2)} for a in ANNEXES}
GAMMA_Q_BUILDING = {a: {fam: FACTORS.ndp(f"ndp.en1990.gQ.{fam}.A1", a) for fam in FAMILIES} for a in ANNEXES}
GAMMA_Q_BRIDGE = {a: {(fam, kind): FACTORS.ndp(f"ndp.en1990.gQ.{kind}.{fam}.A2", a) for fam in FAMILIES for kind in ("road", "rail", "other")} for a in ANNEXES}
XI = {a: {group: FACTORS.ndp(f"ndp.en1990.xi.str.A{group}", a) for group in (1, 2)} for a in ANNEXES}
KFI = {rc: FACTORS.ndp(f"ndp.en1990.kfi.{rc}", "En") for rc in (1, 2, 3)}
BETA = {1: (4.2, 3.3), 2: (4.7, 3.8), 3: (5.2, 4.3)}


def beta_target(b1, b50, t):
    """⚖️ Table B2 anchors at 1 and 50 years, linear in ln T between them, p_T = 1 - (1 - p_a)^(T/a) outside."""
    if 1.0 <= t <= 50.0:
        return b1 + (b50 - b1) * math.log(t) / math.log(50.0)
    a, b = (1.0, b1) if t < 1.0 else (50.0, b50)
    return float(-N.ppf(-math.expm1((t / a) * math.log1p(-N.sf(b)))))

DWL = {1: (10, 10), 2: (10, 30), 3: (15, 30), 4: (50, 50), 5: (100, 100)}
KN_NODES = [1, 2, 3, 4, 5, 6, 8, 10, 20, 30]
KN_KNOWN = [2.31, 2.01, 1.89, 1.83, 1.80, 1.77, 1.74, 1.72, 1.68, 1.67]
KN_UNKNOWN = {3: 3.37, 4: 2.63, 5: 2.33, 6: 2.18, 8: 2.00, 10: 1.92, 20: 1.76, 30: 1.73}
KN_INF = 1.64
KD_KNOWN = [4.36, 3.77, 3.56, 3.44, 3.37, 3.33, 3.27, 3.23, 3.16, 3.13]
KD_INF = 3.04


def category(name, altitude=0.0):
    name = LETTERS.get(name, name)
    if name in ("snow", "snow_high"):
        return "snow_high" if altitude > 1000 else "snow"
    return name


def psi_of(annex, name):
    return PSI[annex][name]


def kind_of(name):
    return "road" if name.startswith(("road_", "footbridge_")) else "rail" if name.startswith("rail_") else "other"


def parse_actions(text, altitude, annex):
    out = []
    for row in filter(None, (r.strip() for r in text.split(";"))):
        p = [x.strip() for x in row.split("|")] + [""] * 6
        out.append(dict(id=p[0], cls=p[1], value=float(p[2]), cat=category(p[3], altitude) if p[1] == "Q" else p[3], group="" if p[4] == "-" else p[4]))
    return out


EXECUTION = ("wind_bridge_execution", "snow_construction", "construction_load")


def combos(actions, eq, annex, group, family, kfi, lead_psi1=True, d=1.0, transient=False):
    """🔀️ Brute force: every pick of one exclusive group (or none) and one leading unit; returns (unfavourable, favourable, label) triples."""
    perm = [a for a in actions if a["cls"] == "G"]
    pre = [a for a in actions if a["cls"] == "P"]
    var = [a for a in actions if a["cls"] == "Q" and a["value"] * d > 0.0 and (a["cat"] in EXECUTION) == transient]
    design = [a for a in actions if a["cls"] == ("A" if eq == "6.11b" else "E")] if eq in ("6.11b", "6.12b") else [None]
    if design == []:
        return []
    groups = sorted({a["group"] for a in var if a["group"]})
    picks = groups if groups else [""]
    g_sup, g_inf = GAMMA_G[annex][(family, group)]
    xi = XI[annex][group]
    result = []
    for dsg, pick in itertools.product(design, picks):
        everything = [a for a in var if a["group"] in ("", pick)]
        families = {FAMILY.get(a["cat"]) for a in everything}
        variants = [(everything, "")]
        if group != 1 and "wind" in families and "thermal" in families:
            variants = [([a for a in everything if FAMILY.get(a["cat"]) != excluded], excluded) for excluded in ("thermal", "wind")]
        for active, excluded in variants:
            units = {}
            for a in active:
                units.setdefault(a["group"] or a["id"], []).append(a)
            leads = list(units) if units and eq not in ("6.12b", "6.16b") else [None]
            for lead in leads:
                unfav = fav = 0.0
                for a in perm:
                    if eq in ("6.10", "6.10a", "6.10b"):
                        if a["value"] * d < 0:
                            fav += g_inf * a["value"]
                        else:
                            unfav += kfi * g_sup * (xi if eq == "6.10b" else 1.0) * a["value"]
                    else:
                        unfav += a["value"]
                for a in pre:
                    unfav += a["value"]
                if dsg is not None:
                    unfav += dsg["value"]
                for unit, members in units.items():
                    for a in members:
                        p0, p1, p2 = psi_of(annex, a["cat"])
                        gq = kfi * (GAMMA_Q_BUILDING[annex][family] if group == 1 else GAMMA_Q_BRIDGE[annex][(family, kind_of(a["cat"]))])
                        is_lead = unit == lead
                        if eq == "6.10":
                            f = gq if is_lead else gq * p0
                        elif eq == "6.10a":
                            f = gq * p0
                        elif eq == "6.10b":
                            f = gq if is_lead else gq * p0
                        elif eq == "6.11b":
                            f = (p1 if lead_psi1 else p2) if is_lead else p2
                        elif eq in ("6.12b", "6.16b"):
                            f = p2
                        elif eq == "6.14b":
                            f = 1.0 if is_lead else p0
                        elif eq == "6.15b":
                            f = p1 if is_lead else p2
                        unfav += f * a["value"]
                result.append((unfav, fav, f"{lead}"))
    return result


def total(c):
    return c[0] + c[1]


def net(c):
    return c[0] + c[1]


def worst(cs, use_net=False):
    return max(cs, key=net if use_net else total)


def kfi_of(case):
    return case["num"]["k_fi"] if "k_fi" in case["num"] else KFI[int(case["num"].get("rc", 2))]


def group_of(case):
    return 2 if case["text"].get("structure", "building") in ("bridge", "road_bridge", "footbridge", "rail_bridge") else 1


def info(v):
    return [0.0, float(v)]


def limit(d, c):
    return [float(d), float(c)]


def minimum(provided, required):
    return [float(required), float(provided)]


def lim_or_info(v, r):
    return limit(v, r) if r is not None else info(v)


def phi_pdf_fractile_gumbel(mean, cov):
    sd = mean * cov
    return float(stats.gumbel_r(loc=mean - 0.5772156649015329 * sd * math.sqrt(6) / math.pi, scale=sd * math.sqrt(6) / math.pi).ppf(0.98))


def scipy_dist(kind, mean, sd):
    if kind == "normal":
        return stats.norm(mean, sd)
    if kind == "lognormal":
        z = math.sqrt(math.log(1 + (sd / mean) ** 2))
        return stats.lognorm(s=z, scale=mean * math.exp(-0.5 * z * z))
    if kind == "gumbel":
        scale = sd * math.sqrt(6) / math.pi
        return stats.gumbel_r(loc=mean - 0.5772156649015329 * scale, scale=scale)
    if kind == "weibull":
        v = sd / mean

        def spread(k):
            return math.sqrt(math.gamma(1 + 2 / k) / math.gamma(1 + 1 / k) ** 2 - 1) - v

        k = optimize.brentq(spread, 0.3, 100.0, xtol=1e-14)
        return stats.weibull_min(c=k, scale=mean / math.gamma(1 + 1 / k))
    raise ValueError(kind)


def form(r, e):
    """🎯️ Design point of g = R - E in standard normal space (Rosenblatt map by exact cdf/ppf), minimising the distance to the origin."""
    to_x = lambda dist, u: dist.ppf(N.cdf(u))
    g = lambda u: to_x(r, u[0]) - to_x(e, u[1])
    best = optimize.minimize(lambda u: u @ u, x0=np.array([-0.5, 0.5]), constraints=[{"type": "eq", "fun": g}], method="SLSQP", options={"ftol": 1e-14, "maxiter": 500})
    u = best.x
    beta = float(np.linalg.norm(u))
    alpha = u / beta
    return beta, float(-alpha[0]), float(-alpha[1]), float(to_x(r, u[0])), float(to_x(e, u[1]))


def form_multi(r, actions):
    """🎯️ Design point of g = R - sum(E_i) in standard normal space (exact Rosenblatt map), minimising the distance to the origin; returns beta, alpha_R, alpha_E_i, x_R, x_E_i."""
    to_x = lambda dist, u: dist.ppf(N.cdf(u))
    dists = [r] + list(actions)
    g = lambda u: to_x(dists[0], u[0]) - sum(to_x(d, ui) for d, ui in zip(dists[1:], u[1:]))
    x0 = np.array([-0.5] + [0.5] * len(actions))
    best = optimize.minimize(lambda u: u @ u, x0=x0, constraints=[{"type": "eq", "fun": g}], method="SLSQP", options={"ftol": 1e-14, "maxiter": 500})
    u = best.x
    beta = float(np.linalg.norm(u))
    alpha = u / beta
    return beta, [float(-a) for a in alpha], [float(to_x(d, ui)) for d, ui in zip(dists, u)]


def standard_evaluation(pairs, v_rt, g_mean):
    """🧪️ Annex D.7.2 standard evaluation with numpy: least-squares slope b, log errors, Table D1 k_n (V unknown, the printed nodes as in `kn`, whose deviation from the Student t formula is asserted by `table-d1`) and k_infinity the last node."""
    r_e = np.array([p[0] for p in pairs]); r_t = np.array([p[1] for p in pairs]); n = len(pairs)
    b = float(r_e @ r_t / (r_t @ r_t))
    delta = np.log(r_e / (b * r_t))
    q_d = float(np.std(delta, ddof=1)); q_rt = math.sqrt(math.log1p(v_rt ** 2)); q = math.hypot(q_d, q_rt)
    k_n = kn(n, False); k_inf = KN_INF
    r_k = b * g_mean * math.exp(-k_inf * q_rt ** 2 / q - k_n * q_d ** 2 / q - 0.5 * q * q) if q > 0 else b * g_mean
    return {"b": b, "v-delta": math.sqrt(math.expm1(q_d ** 2)), "v-r": math.sqrt(math.expm1(q ** 2)), "k-n": k_n, "r-k": r_k}


def exact_pf(r, e):
    return float(integrate.quad(lambda x: r.cdf(x) * e.pdf(x), e.ppf(1e-12), e.ppf(1 - 1e-12), limit=400, epsabs=0, epsrel=1e-10)[0])


def endurance(range_, dc):
    dd = (2 / 5) ** (1 / 3) * dc
    dl = (5 / 100) ** (1 / 5) * dd
    if range_ >= dd:
        return 2e6 * (dc / range_) ** 3
    if range_ > dl:
        return 5e6 * (dd / range_) ** 5
    return None


def kn(n, known):
    if known:
        pts = list(zip(KN_NODES, KN_KNOWN)) + [(float("inf"), KN_INF)]
    else:
        pts = sorted(KN_UNKNOWN.items()) + [(float("inf"), KN_INF)]
        if n < 3:
            return None
    for (n0, k0), (n1, k1) in zip(pts, pts[1:]):
        if n0 <= n <= n1:
            x = lambda m: 0.0 if m == float("inf") else 1.0 / m
            if n == n0:
                return k0
            if n == n1:
                return k1
            return float(np.interp(1.0 / n, [x(n1), x(n0)], [k1, k0]))
    return None


def kd(n, known):
    if not known:
        return None
    pts = list(zip(KN_NODES, KD_KNOWN)) + [(float("inf"), KD_INF)]
    for (n0, k0), (n1, k1) in zip(pts, pts[1:]):
        if n0 <= n <= n1:
            x = lambda m: 0.0 if m == float("inf") else 1.0 / m
            return k0 if n == n0 else k1 if n == n1 else float(np.interp(1.0 / n, [x(n1), x(n0)], [k1, k0]))
    return None


def catalogue(case):
    e, num, text, annex = case["entry"], case["num"], case["text"], case["annex"]
    out = {}
    if e == "E90.01":
        out["annex-group"] = info(1 if text["use"] in ("building", "crane_runway", "silo", "tower") else 2)
    elif e == "E90.02":
        cc = int(num["cc"]); rc = int(num.get("rc", cc)); k = KFI[rc]
        gf = num.get("gamma_f", FACTORS.ndp("ndp.en1990.gG_sup.str.A1", annex))
        out["k-fi"] = info(k); out["gamma-f-eff"] = info(k * gf)
        out["rc-vs-cc"] = minimum(1.0 if rc == cc else 0.5, 1.0)
        if "k_fi_declared" in num:
            out["k-fi-declared"] = minimum(num["k_fi_declared"], k)
    elif e == "E90.03":
        lo, hi = DWL[int(num["category"])]
        out["range-min"] = info(lo); out["range-max"] = info(hi)
        if "years" in num:
            out["years-min"] = minimum(num["years"], lo); out["years-max"] = limit(num["years"], hi)
    elif e == "E90.04":
        out["durability-period"] = info(num["years"])
    elif e == "E90.05":
        rc = int(num["rc"])
        out["dsl"] = minimum(num["dsl"], rc); out["il"] = minimum(num["il"], rc)
    elif e == "E90.06":
        for i, s in enumerate([x.strip() for x in text["situations"].split(",") if x.strip()], 1):
            out[f"situation-{s}"] = info(i)
    elif e == "E90.07":
        bits = 0
        for key, left, right, shift in [("variation", "permanent", "variable", 0), ("origin", "direct", "indirect", 2), ("spatial", "fixed", "free", 3), ("nature", "static", "dynamic", 4), ("effect", "favourable", "unfavourable", 5)]:
            v = text[key]
            idx = 2 if key == "variation" and v == "accidental" else 0 if v == left else 1
            bits += idx << shift
        out["class-bits"] = info(bits)
    elif e == "E90.08":
        if "g_mean" in num:
            m, v = num["g_mean"], num["v_g"]
            sup, inf = (m, m) if v <= 0.05 else (m * (1 + 1.64 * v), m * (1 - 1.64 * v))
            out["g-k-sup"] = info(sup); out["g-k-inf"] = info(inf); out["v-g-bound"] = minimum(0.1, max(v, 1e-9))
        if "q_mean" in num:
            out["q-k"] = info(phi_pdf_fractile_gumbel(num["q_mean"], num["q_cov"]))
    elif e == "E90.09":
        row = psi_of(annex, category(text["category"], num.get("altitude", 0.0))) if "category" in text else (num["psi0"], num["psi1"], num["psi2"])
        out["psi0"], out["psi1"], out["psi2"] = info(row[0]), info(row[1]), info(row[2])
        out["ordered"] = minimum(1.0 + min(row[0] - row[1], row[1] - row[2], row[2]), 1.0)
    elif e == "E90.10":
        gm = num.get("gamma_m") or {"concrete": 1.5, "steel": 1.0, "timber": 1.3, "other": 1.0}[text.get("material", "other")]
        xd = num.get("eta", 1.0) * num["x_k"] / gm
        out["x-d"] = info(xd)
        if "e_d" in num:
            out["e-d-vs-x-d"] = limit(num["e_d"], xd)
    elif e == "E90.11":
        out["a-d-plus"] = info(num["a_nom"] + abs(num["delta_a"])); out["a-d-minus"] = info(num["a_nom"] - abs(num["delta_a"]))
    elif e == "E90.12":
        out["analysis"] = info({"first-order": 1, "second-order": 2, "plastic": 3, "non-linear": 4}[text["analysis"]])
        if "alpha_cr" in num:
            out["alpha-cr"] = minimum(num["alpha_cr"], 10.0)
    elif e in ("E90.13", "E90.30") and "pairs" in text:
        pairs = [tuple(map(float, row.split(":"))) for row in text["pairs"].split(";") if row.strip()]
        fit = standard_evaluation(pairs, num.get("v_rt", 0.0), num["g_rt_mean"])
        out["b"] = info(fit["b"]); out["v-delta"] = info(fit["v-delta"]); out["v-r"] = info(fit["v-r"]); out["k-n"] = info(fit["k-n"]); out["r-k"] = info(fit["r-k"])
        out["r-d"] = info(num.get("eta_d", 1.0) * fit["r-k"] / num.get("gamma_m", 1.0)); out["sample-size"] = minimum(len(pairs), 6.0)
    elif e in ("E90.13", "E90.30"):
        n, known = num["n"], num.get("v_known", 0.0) > 0.5
        k = kn(n, known); v = num["v"]; mean = num["mean"]
        if text.get("dist", "normal") == "lognormal":
            sy = math.sqrt(math.log(1 + v * v)); xk = math.exp(math.log(mean) - 0.5 * sy * sy - k * sy)
        else:
            xk = mean * (1 - k * v)
        xd = num.get("eta_d", 1.0) * xk / num.get("gamma_m", 1.0)
        out["k-n"] = info(k); out["x-k"] = info(xk); out["x-d"] = info(xd)
        if known:
            kdn = kd(n, True); out["k-d-n"] = info(kdn)
            if text.get("dist", "normal") == "lognormal":
                out["x-d-direct"] = info(num.get("eta_d", 1.0) * math.exp(math.log(mean) - 0.5 * sy * sy - kdn * sy))
            else:
                out["x-d-direct"] = info(num.get("eta_d", 1.0) * mean * (1 - kdn * v))
        if n < 3:
            out["few-tests"] = minimum(n, 3.0)
        if not known:
            out["sample-size"] = minimum(n, 6.0)
    elif e == "E90.14":
        out["limit-states"] = info(4)
    elif e in ("E90.15", "E90.16", "E90.17", "E90.18", "E90.19", "E90.23", "E90.26"):
        out.update(combination_entry(case))
    elif e == "E90.20":
        gt = num["gamma_f"] * num.get("gamma_sd", 1.0); ed = gt * num["f_rep"]
        xd = num.get("eta", 1.0) * num["x_k"] / num["gamma_m"]; rd = num.get("section", 1.0) * xd / num.get("gamma_rd", 1.0)
        out["gamma-F"] = info(gt); out["e-d"] = info(ed); out["x-d"] = info(xd); out["verification"] = limit(ed, rd)
    elif e == "E90.21":
        rows = [tuple(map(float, r.split(":"))) for r in text["spectrum"].split(";") if r.strip()]
        dc = num["delta_sigma_c"] / num.get("gamma_mf", 1.0)
        damage = sum(n / endurance(r * num.get("gamma_ff", 1.0), dc) for r, n in rows if endurance(r * num.get("gamma_ff", 1.0), dc))
        out["miner"] = limit(damage, 1.0)
    elif e in ("E90.22", "E90.25"):
        row = psi_of(annex, category(text["category"], num.get("altitude", 0.0)))
        out["psi0"], out["psi1"], out["psi2"] = info(row[0]), info(row[1]), info(row[2])
    elif e == "E90.24":
        span = num["span"] * (2.0 if num.get("cantilever", 0) > 0.5 else 1.0)
        tot = num.get("w1", 0) + num.get("w2", 0) + num.get("w3", 0); wn = tot - num.get("w_c", 0)
        out["w-tot"] = info(tot)
        out["w-net-fin"] = limit(wn, span * 1000 / num.get("ratio_net", 250.0)); out["w-3"] = limit(num.get("w3", 0), span * 1000 / num.get("ratio_w3", 300.0))
        if "f1" in num:
            out["vibration"] = minimum(num["f1"], num.get("f_min", 3.0))
        if "u" in num and "height" in num:
            out["drift"] = limit(num["u"], num["height"] * 1000 / num.get("drift_ratio", 500.0))
        if "u_storey" in num and "h_storey" in num:
            out["storey-drift"] = limit(num["u_storey"], num["h_storey"] * 1000 / num.get("storey_ratio", 300.0))
    elif e == "E90.27":
        if "acc_v" in num:
            out["acc-v"] = limit(num["acc_v"], num.get("acc_v_limit", 0.7))
        if "acc_h" in num:
            out["acc-h"] = limit(num["acc_h"], num.get("acc_h_limit", 0.4 if num.get("crowd", 0) > 0.5 else 0.2)); out["lock-in-trigger"] = info(0.1)
        if "deck_acc" in num:
            out["deck-acc"] = limit(num["deck_acc"], 5.0 if text.get("track") == "slab" else 3.5)
        if "twist" in num:
            v = num["v_kmh"]; out["twist"] = limit(num["twist"], 4.5 if v <= 120 else 3.0 if v <= 200 else 1.5)
        if "deflection" in num:
            out["deflection"] = limit(num["deflection"], num["span"] * 1000 / num.get("deflection_ratio", 600.0))
    elif e == "E90.28":
        rc = int(num["rc"]); t = num.get("t_years", 50.0); b1, b50 = BETA[rc]
        target = beta_target(b1, b50, t)
        out["beta-target"] = info(target); out["p-f"] = info(N.sf(target))
        out["beta-50-from-1"] = info(float(-N.ppf(-math.expm1(50 * math.log1p(-N.sf(b1))))))
        if "beta" in num:
            out["beta"] = minimum(num["beta"], target)
    elif e == "E90.29" and "e2_dist" in text:
        r = scipy_dist(text["r_dist"], num["r_mean"], num["r_sd"]); e1 = scipy_dist(text["e_dist"], num["e_mean"], num["e_sd"]); e2 = scipy_dist(text["e2_dist"], num["e2_mean"], num["e2_sd"])
        beta, alpha, xs = form_multi(r, [e1, e2]); bt = num.get("beta_t", 3.8)
        out["beta"] = info(beta); out["alpha-r"] = info(alpha[0]); out["alpha-e"] = info(alpha[1]); out["alpha-e2"] = info(alpha[2])
        out["r-d"] = info(xs[0]); out["e-d"] = info(xs[1]); out["e2-d"] = info(xs[2]); out["p-f"] = info(float(N.sf(beta))); out["beta-target"] = minimum(beta, bt)
        sds = [num["e_sd"], num["e2_sd"]]; mus = [num["e_mean"], num["e2_mean"]]
        lead = 0 if abs(alpha[1]) >= abs(alpha[2]) else 1
        ratio = sds[lead] / num["r_sd"]
        a_r, a_e = (0.8, -0.7) if 0.16 < ratio < 7.6 else (0.4, -1.0) if ratio >= 7.6 else (1.0, -0.4)
        out["r-d-fixed"] = info(num["r_mean"] - a_r * bt * num["r_sd"]); out["e-lead-fixed"] = info(mus[lead] - a_e * bt * sds[lead])
        out["e-second-fixed"] = info(mus[1 - lead] - 0.4 * a_e * bt * sds[1 - lead])
        if "r_k" in num:
            out["gamma-r"] = info(num["r_k"] / xs[0])
        if "e_k" in num:
            out["gamma-e"] = info(xs[1] / num["e_k"])
        if "e2_k" in num:
            out["gamma-e2"] = info(xs[2] / num["e2_k"])
    elif e == "E90.29":
        r = scipy_dist(text["r_dist"], num["r_mean"], num["r_sd"]); ev = scipy_dist(text["e_dist"], num["e_mean"], num["e_sd"])
        beta, ar, ae, rd, ed = form(r, ev)
        bt = num.get("beta_t", 3.8)
        ratio = num["e_sd"] / num["r_sd"]
        a_r, a_e = (0.8, -0.7) if 0.16 < ratio < 7.6 else (0.4, -1.0) if ratio >= 7.6 else (1.0, -0.4)
        out["beta"] = info(beta); out["alpha-r"] = info(ar); out["alpha-e"] = info(ae); out["r-d"] = info(rd); out["e-d"] = info(ed)
        out["p-f"] = info(float(N.sf(beta)))
        out["r-d-fixed"] = info(num["r_mean"] - a_r * bt * num["r_sd"]); out["e-d-fixed"] = info(num["e_mean"] - a_e * bt * num["e_sd"])
        out["beta-target"] = minimum(beta, bt)
        if "r_k" in num:
            out["gamma-r"] = info(num["r_k"] / rd)
        if "e_k" in num:
            out["gamma-e"] = info(ed / num["e_k"])
        out["_exact-beta"] = info(float(-N.ppf(exact_pf(r, ev))))
    return out


def combination_entry(case):
    e, num, text, annex = case["entry"], case["num"], case["text"], case["annex"]
    group = group_of(case)
    kfi = kfi_of(case)
    actions = parse_actions(text.get("actions", ""), num.get("altitude", 0.0), annex)
    out = {}
    if e == "E90.15":
        w = worst(combos(actions, "6.10", annex, group, "equ", kfi), True)
        out["e-dst"] = info(w[0]); out["e-stb"] = info(-w[1]); out["equ"] = limit(w[0], -w[1] + num.get("t_d", 0.0))
        if num.get("t_d", 0.0) > 0.0:
            w2 = worst(combos(actions, "6.10", annex, group, "str", kfi), True)
            out["equ-set-b"] = limit(w2[0], -w2[1] + num["t_d"])
    elif e == "E90.16":
        fam = "geo" if text.get("set", "B") == "C" else "str"
        r = num.get("r_d")
        w = worst(combos(actions, "6.10", annex, group, fam, kfi)); out["e610"] = lim_or_info(total(w), r)
        if annex != "De" and group == 1 and fam == "str":
            a = total(worst(combos(actions, "6.10a", annex, group, fam, kfi))); b = total(worst(combos(actions, "6.10b", annex, group, fam, kfi)))
            out["e610a"] = lim_or_info(a, r); out["e610b"] = lim_or_info(b, r); out["e610ab"] = lim_or_info(max(a, b), r)
        qs = [a for a in actions if a["cls"] == "Q" and a["value"] > 0]
        if annex == "De" and fam == "str" and len(qs) > 1:
            g = sum(a["value"] for a in actions if a["cls"] in ("G", "P") and a["value"] > 0); q = sum(a["value"] for a in qs)
            out["simplified"] = lim_or_info(kfi * (GAMMA_G[annex][("str", 1)][0] * g + FACTORS.ndp("ndp.en1990.simplified.gq", annex) * q), r)
    elif e == "E90.17":
        cs = combos(actions, "6.11b", annex, group, "str", kfi, text.get("lead_psi", "psi1") != "psi2")
        out["6.11b"] = lim_or_info(total(worst(cs)), num.get("r_d")); out["branches"] = info(len(cs))
    elif e == "E90.18":
        out["6.12b"] = lim_or_info(total(worst(combos(actions, "6.12b", annex, group, "str", kfi))), num.get("r_d"))
    elif e == "E90.19":
        for eq, key, lim in [("6.14b", "6.14b", "cd_char"), ("6.15b", "6.15b", "cd_freq"), ("6.16b", "6.16b", "cd_qp")]:
            out[key] = lim_or_info(total(worst(combos(actions, eq, annex, group, "str", kfi))), num.get(lim))
    elif e == "E90.23":
        g = 2 if text["structure"] in ("bridge", "road_bridge", "footbridge", "rail_bridge") else 1
        cat = text.get("category", "office")
        for fam in ("equ", "str", "geo"):
            sup, inf = GAMMA_G[annex][(fam, g)]
            q = GAMMA_Q_BUILDING[annex][fam] if g == 1 else GAMMA_Q_BRIDGE[annex][(fam, kind_of(cat))]
            out[f"{fam}-g-sup"], out[f"{fam}-g-inf"], out[f"{fam}-q"] = info(sup), info(inf), info(q)
        out["xi"] = info(XI[annex][g])
    elif e == "E90.26":
        r = num.get("r_d")
        for fam, key in (("str", "str-e610"), ("geo", "geo-e610")):
            out[key] = lim_or_info(total(worst(combos(actions, "6.10", annex, 2, fam, kfi))), r)
        out["equ-net"] = info(net(worst(combos(actions, "6.10", annex, 2, "equ", kfi), True)))
        out["exclusive-groups"] = info(len({a["group"] for a in actions if a["group"]}))
    return out


# ---- snapshot-level oracle -------------------------------------------------------------
def snapshot(doc):
    annex = "De" if doc["annex"] == "De" else "En"
    domain = 2 if doc["structureKind"] != "building" else 1
    rc = int(doc["reliabilityClass"][2:])
    kfi = KFI[rc]
    out = {}
    eff = {(x["memberId"], x["actionId"]): x["influence"] for x in doc["effects"]}
    for m in doc["members"]:
        acts = []
        for p in doc["permanents"]:
            v = eff.get((m["id"], p["id"]), 0.0) * p["gk"]
            acts.append(dict(id=p["id"], cls="P" if p["kind"] == "prestress" else "G", value=v, cat="", group=""))
        for q in doc["variables"]:
            row = next((r for r in doc.get("trafficGroups", []) if r["variableId"] == q["id"]), None)
            v = eff.get((m["id"], q["id"]), 0.0) * q["qk"]
            acts.append(dict(id=q["id"], cls="Q", value=v, cat=category(q["category"], doc["altitudeM"]), group=row["group"].lower() if row else q.get("group", "")))
        for a in doc["accidentals"]:
            acts.append(dict(id=a["id"], cls="A", value=eff.get((m["id"], a["id"]), 0.0) * a["ad"], cat="", group=""))
        gi = {"I": 0.8, "II": 1.0, "III": 1.2, "IV": 1.4}
        for s in doc["seismics"]:
            acts.append(dict(id=s["id"], cls="E", value=eff.get((m["id"], s["id"]), 0.0) * gi[s["importanceClass"]] * s["aEk"], cat="", group=""))
        mid = m["id"]
        eqs = ["6.10"] if annex == "De" or domain == 2 else ["6.10a", "6.10b"]
        for eq in eqs:
            out[f"en1990.str.{mid}.{eq}"] = total(worst(combos(acts, eq, annex, domain, "str", kfi)))
            out[f"en1990.geo.{mid}.{eq}"] = total(worst(combos(acts, eq, annex, domain, "str", kfi)))
            lows = combos(acts, eq, annex, domain, "str", kfi, d=-1.0)
            lo = min(lows, key=total) if lows else None
            if lo is not None and total(lo) < 0:
                out[f"en1990.str.{mid}.{eq}.min"] = -total(lo)
                out[f"en1990.geo.{mid}.{eq}.min"] = -total(lo)
        out[f"en1990.geo.{mid}.set-c"] = total(worst(combos(acts, "6.10", annex, domain, "geo", kfi)))
        w = worst(combos(acts, "6.10", annex, domain, "equ", kfi), True)
        out[f"en1990.equ.destab.{mid}"] = max(net(w), 0.0)
        out[f"en1990.equ.stab.{mid}"] = w[0]
        for eq, key in (("6.11b", "6.11b"), ("6.12b", "6.12b")):
            cs = combos(acts, eq, annex, domain, "str", kfi)
            if cs:
                out[f"en1990.{key}.{mid}"] = total(worst(cs))
        for eq in ("6.14b", "6.15b", "6.16b"):
            out[f"en1990.sls.{mid}.{eq}"] = total(worst(combos(acts, eq, annex, domain, "str", kfi)))
        if any(a["cls"] == "Q" and a["cat"] in EXECUTION and a["value"] != 0 for a in acts):
            transient = combos(acts, "6.10", annex, domain, "str", kfi, transient=True)
            if transient:
                out[f"en1990.a2.transient.{mid}"] = total(worst(transient))
        out[f"en1990.a14.deflection.{mid}"] = m["deflectionW1"] + m["deflectionW2"] + m["deflectionW3"] - m["precamber"]
        out[f"en1990.a14.deflection-w3.{mid}"] = m["deflectionW3"]
        if m["rdFat"] > 0:
            perm = sum(a["value"] for a in acts if a["cls"] in ("G", "P"))
            out[f"en1990.fat.{mid}"] = total(worst(combos(acts, "6.15b", annex, domain, "str", kfi))) - perm
    for m in doc["members"]:
        bins = [b for b in doc.get("fatigueSpectra", []) if b["memberId"] == m["id"]]
        if bins:
            out[f"en1990.fat.damage.{m['id']}"] = sum(b["cycles"] / endurance(b["stressRange"], b["detailCategory"]) for b in bins if endurance(b["stressRange"], b["detailCategory"]))
    for s in doc.get("testSeries", []):
        known = s["variationKnown"] > 0
        k = kn(s["sampleSize"], known)
        v = s["variationKnown"] if known else s["stdDev"] / s["mean"]
        if s["distribution"] == "normal":
            xd = s["conversionFactor"] * s["mean"] * (1 - k * v)
        else:
            sy = math.sqrt(math.log1p(v * v))
            xd = s["conversionFactor"] * math.exp(math.log(s["mean"]) - 0.5 * sy * sy - k * sy)
        out[f"en1990.annex-d.{s['id']}"] = xd
    for a in doc.get("reliabilityAnalyses", []):
        r = scipy_dist(a["resistanceDistribution"], a["resistanceMean"], a["resistanceSd"]); e = scipy_dist(a["actionDistribution"], a["actionMean"], a["actionSd"])
        beta, _, _, design_r, design_e = form(r, e)
        out[f"_form.en1990.annex-c.form.{a['id']}"] = beta
        out[f"_form.en1990.annex-c.partial-factors.{a['id']}"] = (r.ppf(0.05) / design_r) * (design_e / e.ppf(0.98))
    out["en1990.annex-b.kfi-applied"] = kfi * GAMMA_G[annex][("str", domain)][0]
    t = doc["referencePeriodYears"]
    b1, b50 = BETA[rc]
    target = beta_target(b1, b50, t)
    out["en1990.annex-c.beta"] = doc["betaComputed"]
    out["_limit.en1990.annex-c.beta"] = target
    return out


def table_d1():
    """📚️ Table D1 from first principles: the 5 % prediction fractile k_n = t_{0.95,n-1} sqrt(1 + 1/n) for V_X unknown and u_{0.95} sqrt(1 + 1/n) for V_X known (scipy.stats)."""
    nodes = [1, 2, 3, 4, 5, 6, 8, 10, 20, 30]
    known = [float(stats.norm.ppf(0.95)) * math.sqrt(1 + 1 / n) for n in nodes]
    unknown = [None if n < 3 else float(stats.t.ppf(0.95, n - 1)) * math.sqrt(1 + 1 / n) for n in nodes]
    return {"nodes": nodes, "known": known, "unknown": unknown, "infinity": float(stats.norm.ppf(0.95))}


def table_d2():
    """📚️ Table D2 closed form (V_X known): k_d,n = u(alpha_R beta) sqrt(1 + 1/n) with alpha_R beta = 0.8 * 3.8 (scipy.stats); the printed row lies above it by at most 0.07."""
    nodes = [1, 2, 3, 4, 5, 6, 8, 10, 20, 30]
    u = 0.8 * 3.8
    return {"nodes": nodes, "known": [u * math.sqrt(1 + 1 / n) for n in nodes], "printed": KD_KNOWN, "infinity": u, "printed-infinity": KD_INF}


def main(argv):
    if argv[1] == "catalogue":
        cases = json.load(open(argv[2]))
        print(json.dumps({c["id"]: catalogue(c) for c in cases}))
    elif argv[1] == "table-d1":
        print(json.dumps(table_d1()))
    elif argv[1] == "table-d2":
        print(json.dumps(table_d2()))
    elif argv[1] == "snapshot":
        docs = json.load(sys.stdin)
        print(json.dumps({name: snapshot(doc) for name, doc in docs.items()}))
    else:
        raise SystemExit("usage: catalogue <fixture> | snapshot | table-d1 | table-d2")


if __name__ == "__main__":
    main(sys.argv)
