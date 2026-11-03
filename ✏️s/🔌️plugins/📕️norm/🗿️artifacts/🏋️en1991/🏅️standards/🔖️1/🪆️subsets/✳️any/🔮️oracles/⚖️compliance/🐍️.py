#!/usr/bin/env python3
"""🏋️ Independent EN 1991 oracle.

Two modes, both stdin/stdout only:

* ``🐍️.py [SNAPSHOT.json]``: required values of the document-level checks of one snapshot (stdin when no path).
* ``🐍️.py --catalogue``: reads lines ``ENTRY|annex=de|n:key=1.5|s:key=text|v:key=1,2`` on stdin and prints
  ``ENTRY|case|key|value`` lines for every output the oracle can derive independently.

The oracle does not transliterate the Rust. It uses third-party numerics where a library covers the physics
(scipy.stats Gumbel quantiles for the return-period factors, scipy.integrate for the view factor, the admittance
functions and the Janssen force balance as an ODE, scipy.constants for sigma) and otherwise a different formulation
(piecewise tables with numpy.interp, closed forms against dense sampling).
"""
from __future__ import annotations

import json
import math
import sys
from pathlib import Path

import numpy as np
from scipy import constants, integrate, optimize, stats

KN = 1000.0
REGISTRY = next(parent / "⚖️compliance" / "📜️annex-and-tables" / "🔖️ndp" / "🔣️.json" for parent in Path(__file__).resolve().parents if (parent / "⚖️compliance" / "📜️annex-and-tables" / "🔖️ndp" / "🔣️.json").is_file())
_NDP: dict = {}


def ndp(row: str, annex: str) -> float:
    """📖 Value of a registry row `ndp.en1991.<row>` for the annex: the very row the Rust reads, so rows resting on a recollection are decided in one place and stay `unverified` there."""
    if not _NDP:
        _NDP.update({e["id"]: e for e in json.loads(REGISTRY.read_text(encoding="utf-8"))["entries"]})
    entry = _NDP[f"ndp.en1991.{row}"]
    return float(entry["naDe"] if annex == "de" else entry["recommended"])


def interp(x: float, xs, ys) -> float:
    return float(np.interp(x, xs, ys))


def gumbel_ratio(cov_or_k: float, p: float, mode: str) -> float:
    if mode == "wind":
        d = stats.gumbel_r(loc=1.0, scale=cov_or_k)
        return float(d.ppf(1 - p) / d.ppf(0.98))
    v = cov_or_k
    scale = v * math.sqrt(6) / math.pi
    d = stats.gumbel_r(loc=1.0 - np.euler_gamma * scale, scale=scale)
    return float(d.ppf(1 - p) / d.ppf(0.98))


def snow_ground(zone: str, altitude: float) -> float:
    t = ((max(altitude, 0.0) + 140.0) / 760.0) ** 2
    base = {"1": (0.19, 0.91, 0.65), "2": (0.25, 1.91, 0.85), "3": (0.31, 2.91, 1.10)}[zone[0]]
    value = max(base[0] + base[1] * t, base[2])
    return KN * value * (1.25 if zone.endswith("a") else 1.0)


def terrain_params(cat: int):
    return {0: (0.003, 1.0), 1: (0.01, 1.0), 2: (0.05, 2.0), 3: (0.3, 5.0), 4: (1.0, 10.0)}[cat]


def en_profile(z: float, cat: int, vb: float, rho: float, co: float):
    z0, zmin = terrain_params(cat)
    kr = 0.19 * (z0 / 0.05) ** 0.07
    ln = math.log(max(z, zmin) / z0)
    cr = kr * ln
    vm = cr * co * vb
    iv = 1.0 / (co * ln)
    return kr, cr, vm, iv, (1 + 7 * iv) * 0.5 * rho * vm * vm


def na_profile(qb0: float, z: float, coast: bool) -> float:
    z = max(z, 1.0)
    if coast:
        return qb0 * (1.8 if z <= 4 else 2.3 * (z / 10) ** 0.27 if z <= 50 else 2.6 * (z / 10) ** 0.19)
    return qb0 * (1.5 if z <= 7 else 1.7 * (z / 10) ** 0.37 if z <= 50 else 2.1 * (z / 10) ** 0.24)


def wall_cpe(zone: str, hd: float):
    xs = [0.25, 1.0, 5.0]
    rows = {"A": ([-1.2] * 3, [-1.4] * 3), "B": ([-0.8] * 3, [-1.1] * 3), "C": ([-0.5] * 3, [-0.5] * 3), "D": ([0.7, 0.8, 0.8], [1.0] * 3), "E": ([-0.3, -0.5, -0.7], [-0.3, -0.5, -0.7])}[zone]
    return interp(hd, xs, rows[0]), interp(hd, xs, rows[1])


def view_factor(a: float, b: float) -> float:
    value, _ = integrate.dblquad(lambda y, x: 1.0 / (math.pi * (x * x + y * y + 1.0) ** 2), 0.0, b, 0.0, a)
    return value


def admittance(eta: float) -> float:
    if eta <= 0:
        return 1.0
    value, _ = integrate.quad(lambda x: 2.0 * (1.0 - x) * math.exp(-2.0 * eta * x), 0.0, 1.0)
    return value


def structural_factor(h, b, cat, vb, delta, n1):
    z0, zmin = terrain_params(cat)
    zs = max(0.6 * h, zmin)
    alpha = 0.67 + 0.05 * math.log(z0)
    length = 300.0 * (zs / 200.0) ** alpha
    b2 = 1.0 / (1.0 + 0.9 * ((b + h) / length) ** 0.63)
    vm = 0.19 * (z0 / 0.05) ** 0.07 * math.log(zs / z0) * vb
    fl = n1 * length / vm
    sl = 6.8 * fl / (1 + 10.2 * fl) ** (5.0 / 3.0)
    r2 = math.pi ** 2 / (2 * delta) * sl * admittance(4.6 * h / length * fl) * admittance(4.6 * b / length * fl)
    nu = max(n1 * math.sqrt(r2 / (b2 + r2)), 0.08)
    root = math.sqrt(2 * math.log(nu * 600.0))
    kp = max(root + 0.6 / root, 3.0)
    iv = 1.0 / math.log(zs / z0)
    return {"z_s": zs, "L": length, "B2": b2, "n_1": n1, "S_L": sl, "R2": r2, "nu": nu, "k_p": kp, "cscd": (1 + 2 * kp * iv * math.sqrt(b2 + r2)) / (1 + 7 * iv)}


def parametric(af, at, av, heq, b, qfd, tlim):
    o = av * math.sqrt(heq) / at
    g = lambda oo: ((oo / b) / (0.04 / 1160.0)) ** 2
    gamma = g(o)
    qtd = qfd * af / at
    tmax_v = 0.2e-3 * qtd / o
    fuel = tmax_v < tlim
    gh, tmax = gamma, tmax_v
    if fuel:
        gh = g(0.1e-3 * qtd / tlim)
        if o > 0.04 and qtd < 75 and b < 1160:
            gh *= 1 + ((o - 0.04) / 0.04) * ((qtd - 75) / 75) * ((1160 - b) / 1160)
        tmax = tlim
    heat = lambda ts: 20 + 1325 * (1 - 0.324 * math.exp(-0.2 * ts) - 0.204 * math.exp(-1.7 * ts) - 0.472 * math.exp(-19 * ts))
    ts = np.linspace(0.0, tmax * gh, 20001)
    theta_max = float(max(heat(t) for t in ts))
    return {"O": o, "gamma": gh, "q_td": qtd, "t_max": tmax * 3600.0, "theta_max": theta_max + 273.15}


def janssen(gamma, mu, k, a, z):
    ode = lambda zz, y: [gamma - mu * k * y[0] / a, k * y[0] * mu]
    sol = integrate.solve_ivp(ode, [0.0, z], [0.0, 0.0], rtol=1e-10, atol=1e-10)
    sv, friction = sol.y[0][-1], sol.y[1][-1]
    ph = k * sv
    return {"K": k, "z_o": a / (k * mu), "p_ho": gamma * a / mu, "Y_J": ph / (gamma * a / mu), "p_hf": ph, "p_wf": mu * ph, "p_vf": sv, "N_wf": friction}


def lanes(w: float):
    if w < 5.4:
        return 1, 3.0, w - 3.0
    if w < 6.0:
        return 2, w / 2.0, 0.0
    n = int(w // 3)
    return n, 3.0, w - 3.0 * n


DENSITY_KN_M3 = {"concrete": 24.0, "reinforced_concrete": 25.0, "cement_screed": 22.0, "aluminium": 27.0}
PARTITION_BANDS = {"en": ((1.0, 0.5), (2.0, 0.8), (3.0, 1.2)), "de": ((3.0, 0.8), (5.0, 1.2), (6.0, 1.6))}
ACTIVE_MEASURE_INPUTS = ("sprinkler", "water_supplies", "heat_detection", "smoke_detection", "alarm", "onsite_brigade", "offsite_brigade", "safe_access")


def density(material: str, annex: str, n: dict, suffix: str) -> float:
    if "gamma" + suffix in n:
        return n["gamma" + suffix]
    if material == "steel":
        return (78.5 if annex == "de" else 77.0) * KN
    return DENSITY_KN_M3[material] * KN


def ev(entry: str, annex: str, n: dict, s: dict, v: dict):
    de = annex == "de"
    out: dict[str, float] = {}
    if entry == "E11.01":
        layers = [(sfx, s["material" + sfx]) for sfx in ("", "2", "3") if "material" + sfx in s]
        loads = {"g" + sfx: density(name, annex, n, sfx) * n["thickness" + sfx] for sfx, name in layers}
        out.update(loads)
        out["total"] = math.fsum(loads.values())
    elif entry == "E11.02":
        line = n["wall_load"] if "wall_load" in n else n["wall_gamma"] * n["wall_thickness"] * n["wall_height"] + n.get("finish", 0.0)
        out["line"] = line
        out["delta_q"] = KN * next(q for limit, q in PARTITION_BANDS[annex] if line <= limit * KN)
    elif entry == "E11.09":
        out["pressure"] = n["load"] / (n["a"] * n["b"])
    elif entry == "E12.03":
        if any(n.get(key, 0.0) or s.get(key) for key in ACTIVE_MEASURE_INPUTS):
            raise ValueError("active fire-fighting measures are not modelled by this oracle")
        delta_q1 = float(np.interp(np.log(max(n["area"], 1.0)), np.log([25.0, 250.0, 2500.0, 5000.0, 10000.0]), [1.10, 1.50, 1.90, 2.00, 2.13]))
        out["delta_q1"] = delta_q1
        out["delta_n"] = 1.0
        out["q_fd"] = n["q_fk_mj"] * 1e6 * 0.8 * delta_q1 * n["delta_q2"]
    elif entry == "E11.04":
        cat, area, st = s["category"], n["area"], n["storeys"]
        psi0 = 1.0 if cat.startswith("E") else 0.7
        if de:
            out["alpha_a"] = min(1.0, max(0.7, 0.5 + 10 / math.sqrt(area))) if cat[0] in "CDE" else 1.0
        elif cat[0] in "AB":
            out["alpha_a"] = min(1.0, 5 / 7 * psi0 + 10 / area)
        elif cat[0] in "CD":
            out["alpha_a"] = min(1.0, max(0.6, 5 / 7 * psi0 + 10 / area))
        else:
            out["alpha_a"] = 1.0
        out["alpha_n"] = min(1.0, (2 + (st - 2) * psi0) / st) if cat[0] in "ABCD" and st > 2 else 1.0
    elif entry == "E12.01":
        t, c = n["t_min"], s["curve"]
        th = {"standard": lambda t: 20 + 345 * np.log10(8 * t + 1), "external": lambda t: 20 + 660 * (1 - 0.687 * np.exp(-0.32 * t) - 0.313 * np.exp(-3.8 * t)), "hydrocarbon": lambda t: 20 + 1080 * (1 - 0.325 * np.exp(-0.167 * t) - 0.675 * np.exp(-2.5 * t))}[c](t)
        out["theta_g"] = float(th) + 273.15
    elif entry == "E12.02":
        out.update(parametric(n["a_f"], n["a_t"], n["a_v"], n["h_eq"], n["b"], n["q_fd_mj"], n.get("t_lim_min", 20.0) / 60.0))
    elif entry == "E12.04":
        av, ah, h = n["alpha_v"], n.get("alpha_h", 0.0), n["height"]
        bv = max(10.0, 12.5 * (1 + 10 * av - av * av))
        wf = max(0.5, (6 / h) ** 0.3 * (0.62 + 90 * (0.4 - av) ** 4 / (1 + bv * ah)))
        out["t_ed"] = n["q_fd_mj"] * n["k_b"] * wf * 60.0
    elif entry == "E12.05":
        d, q, z = n["d"], n["q_w"], n["z"]
        lf = -1.02 * d + 0.0148 * q ** 0.4
        z0 = -1.02 * d + 0.00524 * q ** 0.4
        out.update({"L_f": lf, "z_0": z0, "theta": min(900.0, 20 + 0.25 * (0.8 * q) ** (2 / 3) * (z - z0) ** (-5 / 3)) + 273.15})
    elif entry == "E12.06":
        tg, tm = n["theta_g_c"] + 273.15, n["theta_m_c"] + 273.15
        out["h_net"] = n.get("alpha_c", 25.0) * (tg - tm) + 1.0 * n.get("eps_m", 0.8) * 1.0 * constants.Stefan_Boltzmann * (tg ** 4 - tm ** 4)
    elif entry == "E12.07":
        out["eta_fi"] = (n["g_k"] + n["psi"] * n["q_k"]) / (n.get("gamma_g", 1.35) * n["g_k"] + n.get("gamma_q", 1.5) * n["q_k"])
    elif entry == "E12.09":
        out["phi"] = view_factor(n["height"] / n["distance"], n["width"] / n["distance"])
    elif entry == "E13.02":
        sk = snow_ground(s["zone"], n["altitude"])
        out["s_k"] = sk
        if n.get("north_german_lowland", 0.0) > 0.5:
            out["s_Ad"] = 2.3 * sk
    elif entry == "E13.03":
        a = n["alpha"]
        out["mu1"] = 0.8 if a <= 30 else (0.8 * (60 - a) / 30 if a < 60 else 0.0)
    elif entry == "E13.05":
        out["mu3"] = min(2.0, 0.2 + 10 * n["h"] / n["b"])
    elif entry == "E13.06":
        raw = 2000.0 * n["h"] / n["s_k"]
        cap = ndp("snow.drift-mu2-cap", annex)
        out.update({"mu2_raw": raw, "mu2": min(cap, max(0.8, raw)), "l_s": max(5.0, min(15.0, 2 * n["h"]))})
        if "b1" in n and "b2" in n:
            geometric = (n["b1"] + n["b2"]) / (2 * n["h"])
            out["mu_w"] = max(0.8, min(cap, geometric, raw)) if ndp("1-3.drift-mu-w-reading", annex) < 0.5 else min(cap, raw, max(0.8, geometric))
    elif entry == "E13.07":
        sk = n["s"] / KN
        d = sk / 3.0
        out["s_e"] = min(3 / d, d * 3.0) * sk * sk / 3.0 * KN
    elif entry == "E13.08":
        out["F_s"] = n["s"] * n["b"] * math.sin(math.radians(n["alpha"]))
    elif entry == "E13.13":
        sk = n["s"] / KN
        d = sk / 3.0
        out["s_e"] = min(3 / d, d * 3.0) * sk * sk / 3.0 * KN
        if "b" in n and "alpha" in n:
            out["F_s"] = n["s"] * n["b"] * math.sin(math.radians(n["alpha"]))
    elif entry == "E13.10":
        out["factor"] = gumbel_ratio(n["v"], 1.0 / n["n_years"], "snow")
    elif entry == "E14.01":
        vb0 = {1: 22.5, 2: 25.0, 3: 27.5, 4: 30.0}[int(n["zone"])]
        out.update({"v_b": vb0, "q_b": 0.5 * n.get("rho", 1.25) * vb0 ** 2})
    elif entry == "E14.02":
        qb0 = 0.5 * 1.25 * {1: 22.5, 2: 25.0, 3: 27.5, 4: 30.0}[int(n["zone"])] ** 2
        out["q_p"] = na_profile(qb0, n["z"], n.get("coast", 0.0) > 0.5)
    elif entry == "E14.03":
        phi, sf = n["phi"], n["s"]
        out["c_o"] = 1.0 if phi < 0.05 else (1 + 2 * sf * phi if phi <= 0.3 else 1 + 0.6 * sf)
    elif entry == "E14.05":
        kr, cr, vm, iv, qp = en_profile(n["z"], int(n["terrain"]), n["v_b"], n.get("rho", 1.25), n.get("c_o", 1.0))
        out.update({"k_r": kr, "c_r": cr, "v_m": vm, "I_v": iv, "q_p": qp})
    elif entry == "E14.06":
        p = n["p"] if "p" in n else 1.0 / n["n_years"]
        out["c_prob"] = gumbel_ratio(0.2, p, "wind") ** 0.5
    elif entry == "E14.07":
        e = min(n["b"], 2 * n["h"])
        d = n["d"]
        a, bz = (d, 0.0) if e >= 5 * d else ((e / 5, d - e / 5) if e >= d else (e / 5, 4 * e / 5))
        out.update({"e": e, "A": a, "B": bz, "C": 0.0 if e >= d else d - e})
    elif entry == "E14.08" and s["zone"] in "ABCDE":
        c10, c1 = wall_cpe(s["zone"], n.get("h_over_d", 1.0))
        out.update({"c_pe10_0": c10, "c_pe1_0": c1, "c_pe_area_0": float(np.interp(math.log10(max(min(n.get("area", 10.0), 10.0), 1.0)), [0.0, 1.0], [c1, c10]))})
    elif entry == "E14.08" and s["zone"] == "F" and s.get("roof_type") == "flat":
        e, b, d = n["e"], n["b"], n["d"]
        f = e / 4 * e / 10
        g = (b - e / 2) * e / 10
        h = (e / 2 - e / 10) * b
        out.update({"area_f": f, "area_g": g, "area_h": h, "area_i": b * d - 2 * f - g - h, "c_pe_area_0": float(np.interp(math.log10(n["area"]), [0.0, 1.0], [-2.5, -1.8]))})
    elif entry == "E14.12":
        out.update(structural_factor(n["h"], n["b"], int(n["terrain"]), n["v_b"], n["delta"], n.get("n_1", 46.0 / n["h"])))
    elif entry == "E14.13":
        out["F_w"] = 0.5 * n.get("rho", 1.25) * n["v_b"] ** 2 * n["c_e"] * n["c_fx"] * n["a_ref"]
    elif entry == "E14.14":
        out["v_crit"] = n["b"] * n["n"] / n.get("st", 0.18)
        if "delta_s" in n:
            out["Sc"] = 2 * n["delta_s"] * n["m_e"] / (n.get("rho", 1.25) * n["b"] ** 2)
    elif entry == "E14.16" and "b" in n:
        out["Re"] = n["b"] * math.sqrt(2 * n["q_p"] / n.get("rho", 1.25)) / 15e-6
    elif entry == "E15.01":
        shifts = {1: (-3, 16, 12e-6), 2: (4, 4, 12e-6), 3: (8, 2, 10e-6)}[int(n["deck_type"])]
        temin, temax = n["t_min_c"] + shifts[0], n["t_max_c"] + shifts[1]
        t0 = n.get("t_0_c", 10.0)
        l = n.get("length", 100.0)
        out.update({"T_e_min": temin + 273.15, "T_e_max": temax + 273.15, "dT_con": t0 - temin, "dT_exp": temax - t0, "dL_con": shifts[2] * (t0 - temin) * l, "dL_exp": shifts[2] * (temax - t0) * l})
    elif entry == "E15.03":
        out["sigma"] = n["e"] * n["alpha"] * n["dt"]
    elif entry == "E15.04":
        out.update({"m_leading": n["dt_m"] + 0.35 * n["dt_n"], "n_leading": n["dt_n"] + 0.75 * n["dt_m"]})
    elif entry == "E15.05":
        out["M"] = n["e"] * n["i"] * n["alpha"] * n["dt_m"] / n["h"]
    elif entry == "E16.01":
        days = n["duration_days"]
        years = 2.0 if days <= 3 else 5.0 if days <= 90 else 10.0 if days <= 365 else 50.0
        out.update({"c_prob": gumbel_ratio(0.2, 1 / years, "wind") ** 0.5, "snow_factor": gumbel_ratio(0.4, 1 / years, "snow")})
    elif entry == "E16.03":
        cp = gumbel_ratio(0.2, n["p"], "wind") ** 0.5
        out.update({"c_prob": cp, "v_b": n["v_b"] * cp, "q_b": 0.5 * n.get("rho", 1.25) * (n["v_b"] * cp) ** 2})
    elif entry == "E17.06":
        g, q, psi, sp, l = n["g_k"], n["q_k"], n["psi"], n["s"], n["l"]
        tp, ti = 0.4 * (g + psi * q) * sp * l, 0.8 * (g + psi * q) * sp * l
        out.update({"T_p": tp, "T_i": ti, "T_p_cap": min(tp, 75e3), "T_i_cap": min(ti, 75e3)})
    elif entry == "E17.08":
        out.update({"energy": 0.5 * n["mass"] * n["speed"] ** 2, "F_el": n["speed"] * math.sqrt(n.get("k", 3e5) * n["mass"])})
    elif entry == "E17.09":
        out.update({"p_d1": (3 + n["p_v"]) * KN, "p_d2": (3 + n["p_v"] / 2 + 0.04 / n["av_over_v"] ** 2) * KN})
    elif entry == "E2.01":
        nl, wl, rem = lanes(n["width"])
        out.update({"n_lanes": nl, "w_lane": wl, "w_rem": max(rem, 0.0) if nl > 1 or rem > 0 else 0.0})
    elif entry == "E2.07":
        out["Q_lk"] = min(900e3, max(180e3, 0.6 * 2 * 300e3 + 0.1 * 9e3 * 3.0 * n["loaded_length"]))
    elif entry == "E2.08":
        qv, r = n["q_v"], n["radius"]
        out["Q_tk"] = 0.2 * qv if r < 200 else (40 * qv / r if r <= 1500 else 0.0)
    elif entry == "E2.12":
        out["q_fk"] = min(5e3, max(2.5e3, 2e3 + 120e3 / (n["span"] + 30)))
    elif entry == "E2.16":
        r = math.sqrt(n["l_phi"]) - 0.2
        out.update({"phi_2": min(1.67, max(1.0, 1.44 / r + 0.82)), "phi_3": min(2.0, max(1.0, 2.16 / r + 0.73))})
    elif entry == "E2.17":
        vk, rad, lf = n["v_kmh"], n["radius"], n["l_f"]
        f = 1.0 if vk <= 120 else 1 - (vk - 120) / 1000 * (814 / vk + 1.75) * (1 - math.sqrt(2.88 / lf))
        out.update({"f": f, "Q_tk": vk * vk / (127 * rad) * f * n["q_vk"]})
        if "loaded_length" in n:
            out["traction"] = min(33e3 * n["loaded_length"], 1000e3)
            out["braking"] = min(20e3 * n["loaded_length"], 6000e3)
    elif entry == "E3.01":
        mn, beta = {"HC1": (1.05, 0.17), "HC2": (1.10, 0.34), "HC3": (1.15, 0.51), "HC4": (1.20, 0.68)}[s["hoist_class"]]
        out["phi_2"] = mn + beta * n.get("v_h", 0.0)
        if "dm_over_m" in n:
            out["phi_3"] = 1 - n["dm_over_m"] * (1 + n["beta_3"])
    elif entry == "E3.02":
        mn, beta = {"HC1": (1.05, 0.17), "HC2": (1.10, 0.34), "HC3": (1.15, 0.51), "HC4": (1.20, 0.68)}[s["hoist_class"]]
        out["Q_r_max"] = (n.get("phi_1", 1.1) * n["q_c"] + (mn + beta * n.get("v_h", 0.0)) * n["q_h"]) / n["wheels"]
    elif entry == "E3.03":
        out.update({"K": 0.2 * n["sum_q_r"], "H_L": 1.5 * 0.2 * n["sum_q_r"] / n["runway_beams"], "f_skew": 0.3 * (1 - math.exp(-250 * n["skew_rad"])), "H_B": n["phi_7"] * n["v_1"] * math.sqrt(n["m_c"] * n["s_b"]), "H_T": n["xi"] * n["m_drive"] / n["a"]})
    elif entry == "E4.02":
        phi = math.radians(n["phi"])
        out.update({"K_m": 1.1 * (1 - math.sin(phi)), "K_a": math.tan(math.pi / 4 - phi / 2) ** 2, "mu": math.tan(0.75 * phi)})
    elif entry == "E4.03":
        phi = n.get("phi")
        k = n["k"] if "k" in n else 1.1 * (1 - math.sin(math.radians(phi)))
        mu = n["mu"] if "mu" in n else math.tan(0.75 * math.radians(phi))
        a = n["a_over_u"] if "a_over_u" in n else n["dc"] / 4.0
        out.update(janssen(n["gamma"], mu, k, a, n["z"]))
        out["p_hf_max"] = janssen(n["gamma"], mu / 1.15, 1.2 * k, a, n["z"])["p_hf"]
        out["p_wf_max"] = 1.15 * mu * janssen(n["gamma"], 1.15 * mu, k / 1.2, a, n["z"])["p_hf"]
    elif entry in ("E13.01", "E13.12"):
        out["s"] = n["mu"] * n.get("c_e", 1.0) * n.get("c_t", 1.0) * n["s_k"]
    elif entry == "E14.10" and "c_f" in n:
        out["F_w"] = n.get("c_s", 1.0) * n.get("c_d", 1.0) * n["c_f"] * n["q_p"] * n["a_ref"]
        out["z_e"] = n.get("z_g", 0.0) + n["h"] / 2.0
    elif entry == "E14.11" and "c_f" in n:
        out["F_w"] = n.get("c_s", 1.0) * n.get("c_d", 1.0) * n["c_f"] * n["q_p"] * n["a_ref"]
    elif entry == "E15.06":
        out["movement"] = n["alpha"] * (n["dt"] + n.get("margin_k", 0.0)) * n["length"]
    if not out:
        out = ev_more(entry, annex, n, s, v)
    return out


IMPOSED_DE = {"A1": (1.0, 1.0), "A2": (1.5, 1.0), "A3": (2.0, 1.0), "B1": (2.0, 1.0), "B2": (3.0, 1.5), "B3": (5.0, 4.0), "C1": (3.0, 4.0), "C2": (4.0, 4.0), "C3": (5.0, 4.0), "C4": (5.0, 4.0), "C5": (5.0, 4.0), "C6": (7.5, 10.0), "D1": (5.0, 4.0), "D2": (5.0, 4.0), "D3": (5.0, 4.0), "E1": (6.0, 7.0), "T1": (2.0, 2.0), "T2": (3.0, 3.0), "Z": (4.0, 2.0), "F": (2.5, 20.0), "G": (5.0, 90.0), "H": (0.0, 1.0)}
IMPOSED_EN = {"A": (2.0, 2.0), "B": (3.0, 4.5), "C1": (3.0, 4.0), "C2": (4.0, 4.0), "C3": (5.0, 4.0), "C4": (5.0, 4.0), "C5": (5.0, 4.5), "D1": (4.0, 4.0), "D2": (5.0, 7.0), "E1": (7.5, 7.0), "F": (2.5, 20.0), "G": (5.0, 90.0), "H": (0.4, 1.0)}
FORKLIFT = {1: (21.0, 10.0), 2: (31.0, 15.0), 3: (44.0, 25.0), 4: (60.0, 40.0), 5: (90.0, 60.0), 6: (110.0, 60.0)}
DECK_GRADIENT = {"steel": (18.0, 13.0), "composite": (15.0, 18.0), "slab": (15.0, 8.0), "box": (10.0, 5.0), "beam": (15.0, 8.0)}
SURFACING = {"unsurfaced": (0.7, 0.9), "waterproofed": (1.6, 1.1), "50mm": (1.0, 1.0), "100mm": (0.7, 1.2), "150mm": (0.7, 1.2), "ballast": (0.6, 1.4)}
SNOW_WEIGHT = {"fresh": 1.0, "settled_days": 2.0, "settled_months": 3.0, "wet": 4.0}
IMPACT = {"motorway": (1000.0, 500.0), "urban": (750.0, 375.0), "courtyard_cars": (50.0, 25.0), "courtyard_lorries": (150.0, 75.0)}
MODE_EXPONENT = {"framed": 0.6, "core": 1.0, "cantilever": 1.5, "lattice": 2.0}
DELTA_S = {"concrete": 0.10, "steel": 0.05, "mixed": 0.08}
RAIL_SW = {"sw0": (133e3, 15.0), "sw2": (150e3, 25.0)}
LM1_DE = {"tandem": (300e3, 200e3, 100e3), "udl": (9e3, 2.5e3, 2.5e3), "rest": 2.5e3}
GROUPS = {"gr1a": (1, 0, 0, 0, 3e3), "gr1b": (0, 1, 0, 0, None), "gr2": (1, 0, 0, 0, None), "gr3": (0, 0, 0, 0, 5e3), "gr4": (0, 0, 0, 1, None), "gr5": (0, 0, 1, 0, None)}


def mills_overpressure(mass_kg: float, standoff: float) -> float:
    """💥 Incident overpressure of a hemispherical TNT-equivalent charge, Mills (1987): p = 1772/Z^3 - 114/Z^2 + 108/Z kPa with Z = R/W^(1/3). The Kinney-Graham fit is 8.4 percent lower at Z = 6.5."""
    z = standoff / mass_kg ** (1.0 / 3.0)
    return max(1772.0 / z ** 3 - 114.0 / z ** 2 + 108.0 / z, 0.0) * 1000.0


def kinney_graham(mass_kg: float, standoff: float) -> float:
    """💥 Kinney-Graham incident overpressure (Pa) used as the cross-check of `mills_overpressure`."""
    z = standoff / mass_kg ** (1.0 / 3.0)
    ratio = 808.0 * (1 + (z / 4.5) ** 2) / (math.sqrt(1 + (z / 0.048) ** 2) * math.sqrt(1 + (z / 0.32) ** 2) * math.sqrt(1 + (z / 1.35) ** 2))
    return ratio * constants.atm


def spread_footprint(a: float, b: float, depth: float):
    return a + 2 * depth, b + 2 * depth


def ev_more(entry: str, annex: str, n: dict, s: dict, v: dict) -> dict:
    """📚️ EN 1991-1-1 6.3/6.4, -1-3 5, -1-4 4.4/7.2, -1-5 6/A, -1-6 4, -1-7 4, -2 4/6, -3 2, -4 2/5 restated from the standard text."""
    de = annex == "de"
    out: dict[str, float] = {}
    if entry == "E11.03":
        q, big = (IMPOSED_DE if de else IMPOSED_EN)[s["category"]]
        out.update({"q_k": q * KN, "Q_k": big * KN})
        if "depth" in n:
            out["line"] = q * KN * n["depth"]
    elif entry == "E11.05":
        c = s["category"].upper()
        line = 0.5 if c[0] in "AB" else 3.0 if c == "C5" else 1.0 if c[0] in "CD" else 2.0
        out["q_k"] = line * KN
        if "mass" in n and "speed" in n:
            out["F_k"] = 0.5 * n["mass"] * n["speed"] ** 2 / (n["delta_c"] + n["delta_b"])
    elif entry == "E11.06":
        q, big = (IMPOSED_DE if de else IMPOSED_EN)["H"]
        out.update({"q_k": q * KN, "Q_k": big * KN})
    elif entry == "E11.07":
        net, hoist = FORKLIFT[int(n["class"])]
        out["axle"] = {"pneumatic": 1.4, "solid": 2.0}[s["tyres"]] * (net + hoist) * KN
    elif entry in ("E11.08", "E2.06"):
        depth = n.get("cover", n.get("pavement", 0.0)) + n.get("slab_thickness", 0.0) / 2.0
        sa, sb = spread_footprint(n["a"], n["b"], depth)
        out["footprint"] = sa * sb
        out["pressure"] = n["load"] / (sa * sb)
    elif entry == "E13.04":
        a1, a2 = n["alpha_1"], n["alpha_2"]
        mu1 = lambda a: float(np.interp(a, [0.0, 30.0, 60.0], [0.8, 0.8, 0.0]))
        out["mu1"] = max(mu1(a1), mu1(a2))
        out["mu2"] = min(1.6, 0.8 + 0.8 * (a1 + a2) / 2.0 / 30.0)
    elif entry == "E13.09":
        sk = snow_ground(s["zone"], n["altitude"]) if de else n["s_k"]
        if de and n.get("north_german_lowland", 0.0) < 0.5:
            raise ValueError("exceptional snow load applies to the North German lowland only")
        out["s_Ad"] = (2.3 if de else n.get("c_esl", 2.0)) * sk
    elif entry == "E13.11":
        out["gamma"] = SNOW_WEIGHT[s["state"]] * KN
    elif entry == "E12.08":
        out["flux"] = constants.sigma * (n["t_f_c"] + 273.15) ** 4
    elif entry == "E12.10":
        out["theta_g"] = float(np.interp(n["t_min"], v["times_min"], v["temps_c"])) + 273.15
    elif entry == "E14.04":
        z0, zmin = terrain_params(int(n["terrain"]))
        out["I_v"] = 1.0 / (n.get("c_o", 1.0) * math.log(max(n["z"], zmin) / z0))
    elif entry == "E14.09":
        qp, cpe, cs, cd = n["q_p"], n["c_pe"], n.get("c_s", 1.0), n.get("c_d", 1.0)
        pressure = lambda cpi: qp * (cpe - cpi) * cs * cd
        out["w_0"], out["w_1"] = pressure(0.2), pressure(-0.3)
        ratio = n.get("dominant_ratio", 0.0)
        if ratio >= 2.0:
            out["c_pi_dominant"] = interp(ratio, (2.0, 3.0), (0.75, 0.90)) * cpe
        out["w_governing"] = max(abs(pressure(c)) for c in (0.2, -0.3, n.get("c_pi", 0.2)))
    elif entry == "E14.15":
        h = n["h"]
        out.update({"n_1": 46.0 / h, "phi": (n.get("z", h) / h) ** MODE_EXPONENT[s["mode"]], "delta_s": DELTA_S[s["structure"]]})
    elif entry == "E15.02":
        heat, cool = DECK_GRADIENT[s["deck"]]
        kh, kc = (n["k_sur_heat"], n["k_sur_cool"]) if "k_sur_heat" in n and "k_sur_cool" in n else SURFACING[s["surfacing"]]
        out.update({"dT_M_heat": heat * kh, "dT_M_cool": cool * kc})
    elif entry == "E15.07":
        shift = 0.5 * n["altitude"] / 100.0
        out.update({"t_min": n["t_min_c"] - shift + 273.15, "t_max": n["t_max_c"] - shift + 273.15})
    elif entry == "E15.08":
        t0 = n.get("t_0_c", 10.0)
        out.update({"t_0": t0 + 273.15, "dT_con": t0 - n["t_min_c"], "dT_exp": n["t_max_c"] - t0})
    elif entry == "E16.02":
        out["Q_ca"] = n.get("q_ca", 1000.0) * n["area"]
        if "gamma" in n and "h" in n:
            out["p_fresh"] = n["gamma"] * n["h"]
            out["p_total"] = n["gamma"] * n["h"] + n.get("q_ca_concreting", 750.0)
    elif entry == "E16.06":
        out["Q_c_sum"] = math.fsum(v["q_ci"])
    elif entry == "E17.01":
        fx, fy = IMPACT[s["road"]]
        out.update({"F_dx": fx * KN, "F_dy": fy * KN})
    elif entry == "E17.02":
        out["F_d"] = 5.0 * n["weight"]
    elif entry == "E17.05":
        out["F_d"] = 3.0 * KN * math.sqrt(n["mass"])
    elif entry == "E17.07":
        out["risk"] = math.fsum(p * d * c for p, d, c in zip(v["p_h"], v["p_d_h"], v["consequence"]))
    elif entry == "E17.10":
        out["p_so"] = mills_overpressure(n["charge_kg"], n["standoff"])
    elif entry == "E2.02":
        nl, wl, rem = lanes(n["width"])
        tandem = lambda i: LM1_DE["tandem"][i - 1] if i <= 3 else 0.0
        udl = lambda i: LM1_DE["udl"][i - 1] if i <= 3 else LM1_DE["rest"]
        out["tandems"] = 2.0 * math.fsum(tandem(i) for i in range(1, nl + 1))
        for i in range(1, nl + 1):
            out[f"udl_{i}"] = udl(i) * wl
        out["udl_rest"] = LM1_DE["rest"] * max(rem, 0.0)
    elif entry == "E2.03":
        axle = n.get("axle", 400e3)
        out.update({"axle": axle, "wheel_pressure": axle / 2.0 / (n.get("a", 0.35) * n.get("b", 0.60))})
    elif entry == "E2.04":
        out["total"] = n["axles"] * n["axle_load"]
    elif entry == "E2.05":
        out["q_fk"] = 5.0 * KN
    elif entry == "E2.09":
        lm1, lm2, lm3, lm4, footway = GROUPS[s["group"]]
        out.update({"lm1": float(lm1), "lm2": float(lm2), "lm3": float(lm3), "lm4": float(lm4)})
        if footway is not None:
            out["footway"] = footway
    elif entry == "E2.10":
        lane = int(n["lane"])
        out.update({"flm1_tandem": 0.7 * LM1_DE["tandem"][lane - 1], "flm1_udl": 0.3 * LM1_DE["udl"][lane - 1], "flm3_total": 4 * 120e3})
    elif entry == "E2.11":
        fx, fy = {"deck": (500.0, 250.0), "pier": (1000.0, 500.0)}[s["member"]]
        out.update({"F_dx": fx * KN, "F_dy": fy * KN})
    elif entry == "E2.13":
        band = lambda key: ndp(f"2.footbridge-band-{key}", annex)
        lo, hi = {"vertical": (0.0, band("vertical-max")), "horizontal": (0.0, band("horizontal-max")), "jogging": (band("jogging-min"), band("jogging-max"))}[s["direction"]]
        out["resonance"] = 2.0 if lo <= n["frequency"] <= hi else 0.0
    elif entry == "E2.14":
        out.update({"axle": n.get("alpha", 1.0) * 250e3, "udl": n.get("alpha", 1.0) * 80e3})
    elif entry == "E2.15":
        udl, length = RAIL_SW[s["model"]]
        out.update({"udl": n.get("alpha", 1.0) * udl, "total": n.get("alpha", 1.0) * udl * length})
    elif entry == "E3.04":
        out.update({"static": 1.25 * n["q_h"], "dynamic": 1.1 * n["q_h"]})
    elif entry == "E3.05":
        out["Q_e"] = n["lambda"] * n["q_max"]
    elif entry == "E3.07":
        out["F_dyn"] = n["mass"] * n["eccentricity"] * (n["rpm"] * 2.0 * math.pi / 60.0) ** 2
    elif entry == "E4.01":
        if "capacity_t" in n:
            out["aac"] = 3.0 if n["capacity_t"] > ndp("4.aac-class3-above", annex) else 1.0 if n["capacity_t"] < ndp("4.aac-class1-below", annex) else 2.0
        if "hc" in n and "dc" in n:
            out["slenderness"] = n["hc"] / n["dc"]
            if "gamma" in n:
                out["mass"] = math.pi / 4.0 * n["dc"] ** 2 * n["hc"] * n["gamma"] / constants.g
    elif entry in ("E4.04", "E4.05"):
        mu = n["mu"] if "mu" in n else math.tan(0.75 * math.radians(n["phi"]))
        k = n["k"] if "k" in n else 1.1 * (1 - math.sin(math.radians(n["phi"])))
        a = n["a_over_u"] if "a_over_u" in n else n["dc"] / 4.0
        p_hf = janssen(n["gamma"], mu, k, a, n["z"])["p_hf"]
        if entry == "E4.04":
            out.update({"p_he": n.get("c_h", 1.15) * p_hf, "p_we": n.get("c_w", 1.10) * mu * p_hf, "p_pf": 0.2 * p_hf})
        else:
            out["p_pf"] = 0.2 * janssen(n["gamma"], mu / 1.15, 1.2 * k, a, n["z"])["p_hf"]
        if "dc" in n:
            out["patch_width"] = math.pi * n["dc"] / 16.0
    elif entry == "E4.08":
        out["p"] = n["gamma"] * n["z"]
    return out


def parse_line(line: str):
    parts = line.strip().split("|")
    entry, case = parts[0], parts[1]
    annex, n, s, v = "de", {}, {}, {}
    for token in parts[2:]:
        key, value = token.split("=", 1)
        if key == "annex":
            annex = value
        elif key.startswith("n:"):
            n[key[2:]] = float(value)
        elif key.startswith("s:"):
            s[key[2:]] = value
        elif key.startswith("v:"):
            v[key[2:]] = [float(x) for x in value.split(",")]
    return entry, case, annex, n, s, v


def catalogue_main() -> None:
    for line in sys.stdin:
        if not line.strip():
            continue
        entry, case, annex, n, s, v = parse_line(line)
        for key, value in ev(entry, annex, n, s, v).items():
            print(f"{entry}|{case}|{key}|{float(value)!r}")


def snapshot_checks(doc: dict) -> list[dict]:
    annex = str(doc.get("annex", "de")).lower()
    de = annex == "de"
    rho = doc.get("airDensity", 1.25)
    out = []
    de_table = {"A1": 1.0, "A2": 1.5, "A3": 2.0, "B1": 2.0, "B2": 3.0, "B3": 5.0, "C1": 3.0, "C2": 4.0, "C3": 5.0, "C4": 5.0, "C5": 5.0, "C6": 7.5, "D1": 5.0, "D2": 5.0, "D3": 5.0, "E1": 6.0, "T1": 2.0, "T2": 3.0, "Z": 4.0, "F": 2.5, "G": 5.0, "H": 0.0}
    en_table = {"A": 2.0, "B": 3.0, "C1": 3.0, "C2": 4.0, "C3": 4.0, "C4": 5.0, "C5": 5.0, "D1": 4.0, "D2": 5.0, "E1": 7.5, "F": 2.5, "G": 5.0, "H": 0.4}
    storeys = int(doc.get("storeyCount", 1) or 1)
    for floor in doc.get("floors", []):
        cat = floor["category"]
        table = de_table if de else en_table
        if cat not in table:
            continue
        r = ev("E11.04", annex, {"area": floor.get("area", 1.0), "storeys": storeys}, {"category": cat}, {})
        out.append({"id": f"en1991.1-1.imposed.{floor['id']}", "required": KN * table[cat] * r["alpha_a"] * r["alpha_n"], "assumed": floor["assumedQk"]})
    density = {"reinforced_concrete": 25.0, "concrete": 24.0, "cement_screed": 22.0, "steel": 78.5 if de else 77.0, "timber": 5.0, "glass": 25.0}
    for el in doc.get("selfWeightElements", []):
        if el["material"] in density:
            out.append({"id": f"en1991.1-1.self-weight.{el['id']}", "required": KN * density[el["material"]] * el["thickness"], "assumed": el["assumedGk"]})
    sk = snow_ground(str(doc.get("snowZone", "2")), doc.get("altitude", 150.0)) if de else doc.get("enSk", 850.0)
    for roof in doc.get("roofs", []):
        mu = ev("E13.03", annex, {"alpha": roof.get("pitchDeg", 0.0)}, {}, {})["mu1"]
        if roof.get("multiSpan"):
            mu = max(mu, min(1.6, 0.8 + 0.8 * roof.get("pitchDeg", 0.0) / 30))
        hobs = max(roof.get("parapetHeight", 0.0) if roof.get("hasParapet") else 0.0, roof.get("driftObstructionHeight", 0.0))
        if hobs > 0:
            mu = max(mu, min(2.0, max(0.8, 2000.0 * hobs / sk)))
        out.append({"id": f"en1991.1-3.snow.{roof['id']}", "required": mu * roof.get("cE", 1.0) * roof.get("cT", 1.0) * sk, "assumed": roof["assumedSk"]})
    zone = int(doc.get("windZone", 2))
    terrain = int(doc.get("terrainCategory", 2))
    vb0 = {1: 22.5, 2: 25.0, 3: 27.5, 4: 30.0}.get(zone) if de else doc.get("enVb", 25.0)
    height, width, depth = doc.get("height", 0.0), doc.get("width", 0.0), max(doc.get("depth", 0.1), 0.1)
    if vb0 is not None and terrain in range(5) and doc.get("mixedTerrainDistance", 0.0) <= 0.0:
        co = max(doc.get("orographyFactor", 1.0), 1.0)
        for face in doc.get("windFaces", []):
            b = max(width, 0.1)
            h = max(height, 0.1)
            z = face["z"]
            ze = h if face.get("zone", "D") != "D" else (h if h <= b else (b if z <= b else h) if h <= 2 * b else (b if z <= b else (h if z >= h - b else z)))
            if de and terrain >= 2:
                qp = na_profile(0.5 * 1.25 * vb0 ** 2, ze, False) * co * co * rho / 1.25
            else:
                qp = en_profile(ze, terrain, vb0, rho, co)[4]
            area = face.get("loadedArea", 10.0)
            c1, c10 = face["cPe1"], face["cPe10"]
            cpe = c1 if area <= 1 else (c10 if area >= 10 else c1 - (c1 - c10) * math.log10(area))
            req = max(abs(qp * (cpe - cpi) * face.get("cS", 1.0) * face.get("cD", 1.0)) for cpi in (0.2, -0.3, face.get("cPi", 0.2)))
            out.append({"id": f"en1991.1-4.wind.{face['id']}", "required": req, "assumed": face["assumedWp"]})
    for row in out:
        row["ok"] = row["assumed"] + 1e-9 >= row["required"]
    return out


def main() -> None:
    if len(sys.argv) > 1 and sys.argv[1] == "--catalogue":
        catalogue_main()
        return
    text = Path(sys.argv[1]).read_text(encoding="utf-8") if len(sys.argv) > 1 else sys.stdin.read()
    print(json.dumps(snapshot_checks(json.loads(text)), indent=2))


if __name__ == "__main__":
    main()
