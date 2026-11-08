"""🧮 EN 1991 third-party closed forms: every formula is written from the clause text with sympy symbols and checked with scipy (interp1d, quad, brentq), never derived from the Rust. Entries: E12.04 E12.05 E12.07 E13.03 E13.05 E13.06 E13.07 E13.08 E13.10 E14.01 E14.03 E14.05 E14.06 E14.07 E14.13 E14.14 E14.10 E14.11 E14.16 E16.01 E2.03 E2.04 E2.07 E2.08 E2.12 E2.16 E2.14 E2.17 E3.01 E3.04 E3.05 E3.07 E4.02 E4.03 E4.08 E15.03 E15.06 E17.08 E17.09. Paper: https://eur-lex.europa.eu/ (EN 1991 parts 1-1 to 1-7, 2, 3, 4)."""
from __future__ import annotations

import math

import sympy as sp
from scipy import integrate, interpolate, optimize

KN = 1000.0
TERRAIN = {0: (0.003, 1.0), 1: (0.01, 1.0), 2: (0.05, 2.0), 3: (0.3, 5.0), 4: (1.0, 10.0)}
HOIST = {"HC1": (1.05, 0.17), "HC2": (1.10, 0.34), "HC3": (1.15, 0.51), "HC4": (1.20, 0.68)}
EULER = float(sp.EulerGamma)

z, z0, zmin, vb, rho, co, kr = sp.symbols("z z0 zmin vb rho co kr", positive=True)
_CR = kr * sp.log(sp.Max(z, zmin) / z0)
_VM = _CR * co * vb
_IV = 1 / (co * sp.log(sp.Max(z, zmin) / z0))
_QP = (1 + 7 * _IV) * rho * _VM**2 / 2
_G, _MU, _K, _A = sp.symbols("gamma mu K a", positive=True)


def wind_profile(height: float, terrain: int, v_b: float, density: float, c_o: float) -> dict:
    """🌬️ EN 1991-1-4 4.3.2 (4.4), 4.4 (4.7), 4.5 (4.8): k_r = 0.19 (z0/z0,II)^0.07, c_r = k_r ln(z/z0), I_v = k_I/(c_o ln(z/z0)), q_p = (1 + 7 I_v) 1/2 rho v_m^2."""
    roughness, floor = TERRAIN[terrain]
    k = 0.19 * sp.Rational(1) * (sp.Float(roughness) / sp.Float(0.05)) ** sp.Float(0.07)
    sub = {z: height, z0: roughness, zmin: floor, vb: v_b, rho: density, co: c_o, kr: k}
    out = {"k_r": float(k), "c_r": float(_CR.subs(sub)), "v_m": float(_VM.subs(sub)), "I_v": float(_IV.subs(sub)), "q_p": float(_QP.subs(sub))}
    target = out["c_r"]
    solved = optimize.brentq(lambda h: float(_CR.subs({**sub, z: h})) - target, max(floor, 1e-3), 1e4) if height > floor else floor
    if height > floor and abs(solved - height) > 1e-6 * height:
        raise ArithmeticError(f"c_r inversion {solved} vs {height}")
    return out


def return_period(coefficient: float, p: float, exponent: float, euler: float) -> float:
    """⏳ EN 1991-1-4 4.2(2) Note 2 and EN 1991-1-3 Annex D (D.1): ((1 - K ln(-ln(1-p)))/(1 - K ln(-ln 0.98)))^n."""
    scale = coefficient
    num = 1 - scale * (euler + sp.log(-sp.log(1 - sp.Float(p)))) if euler else 1 - scale * sp.log(-sp.log(1 - sp.Float(p)))
    den = 1 - scale * (euler + sp.log(-sp.log(sp.Float(0.98)))) if euler else 1 - scale * sp.log(-sp.log(sp.Float(0.98)))
    return float((num / den) ** sp.Float(exponent))


def janssen_closed(gamma: float, mu: float, k: float, a: float, depth: float) -> dict:
    """🌾 EN 1991-4 5.2.1: sympy dsolve of dsigma_v/dz = gamma - mu K sigma_v / A, p_hf = K sigma_v, N_wf = integral of mu p_hf."""
    zz = sp.Symbol("zz", positive=True)
    f = sp.Function("f")
    solution = sp.dsolve(sp.Eq(f(zz).diff(zz), _G - _MU * _K * f(zz) / _A), f(zz), ics={f(0): 0}).rhs
    sub = {_G: gamma, _MU: mu, _K: k, _A: a}
    sv = float(solution.subs(sub).subs(zz, depth))
    friction, _ = integrate.quad(lambda x: mu * k * float(solution.subs(sub).subs(zz, x)), 0.0, depth)
    return {"K": k, "z_o": a / (k * mu), "p_ho": gamma * a / mu, "Y_J": k * sv / (gamma * a / mu), "p_hf": k * sv, "p_wf": mu * k * sv, "p_vf": sv, "N_wf": friction}


def derive(entry: str, annex: str, n: dict, s: dict, ndp) -> dict:
    """🧪 Third-party values of one catalogue entry for the keys it can derive independently."""
    out: dict[str, float] = {}
    if entry == "E12.04":
        av, ah, h = sp.Float(n["alpha_v"]), sp.Float(n.get("alpha_h", 0.0)), sp.Float(n["height"])
        bv = sp.Max(10, sp.Rational(25, 2) * (1 + 10 * av - av**2))
        wf = sp.Max(sp.Rational(1, 2), (6 / h) ** sp.Float(0.3) * (sp.Float(0.62) + 90 * (sp.Float(0.4) - av) ** 4 / (1 + bv * ah)))
        out["t_ed"] = float(n["q_fd_mj"] * n["k_b"] * wf * 60)
    elif entry == "E12.07":
        g, q, psi = sp.symbols("g q psi")
        out["eta_fi"] = float(((g + psi * q) / (sp.Float(n.get("gamma_g", 1.35)) * g + sp.Float(n.get("gamma_q", 1.5)) * q)).subs({g: n["g_k"], q: n["q_k"], psi: n["psi"]}))
    elif entry == "E13.03":
        mu1 = interpolate.interp1d([0.0, 30.0, 60.0], [0.8, 0.8, 0.0], bounds_error=False, fill_value=(0.8, 0.0))
        out["mu1"] = float(mu1(n["alpha"]))
    elif entry == "E13.05":
        out["mu3"] = float(sp.Min(2, sp.Float(0.2) + 10 * sp.Float(n["h"]) / sp.Float(n["b"])))
    elif entry == "E13.06":
        raw = sp.Float(2000) * sp.Float(n["h"]) / sp.Float(n["s_k"])
        cap = ndp("snow.drift-mu2-cap", annex)
        out.update({"mu2_raw": float(raw), "mu2": float(sp.Min(cap, sp.Max(sp.Rational(4, 5), raw))), "l_s": float(sp.Max(5, sp.Min(15, 2 * sp.Float(n["h"]))))})
    elif entry == "E12.05":
        d, q, zz = sp.Float(n["d"]), sp.Float(n["q_w"]), sp.Float(n["z"])
        z_virtual = -sp.Float(1.02) * d + sp.Float(0.00524) * q ** sp.Float(0.4)
        plume = 20 + sp.Float(0.25) * (sp.Float(0.8) * q) ** sp.Rational(2, 3) * (zz - z_virtual) ** sp.Rational(-5, 3)
        out.update({"L_f": float(-sp.Float(1.02) * d + sp.Float(0.0148) * q ** sp.Float(0.4)), "z_0": float(z_virtual), "theta": float(sp.Min(900, plume) + sp.Float(273.15))})
    elif entry == "E13.07":
        gamma_snow, depth = sp.Integer(3), sp.Float(n["s"] / KN) / 3
        out["s_e"] = float(sp.Min(3 / depth, depth * gamma_snow) * sp.Float(n["s"] / KN) ** 2 / gamma_snow * KN)
    elif entry == "E13.08":
        out["F_s"] = float(sp.Float(n["s"]) * n["b"] * sp.sin(sp.rad(sp.Float(n["alpha"]))))
    elif entry in ("E13.01", "E13.12"):
        out["s"] = float(sp.Float(n["mu"]) * n.get("c_e", 1.0) * n.get("c_t", 1.0) * n["s_k"])
    elif entry == "E13.10":
        out["factor"] = return_period(n["v"] * math.sqrt(6) / math.pi, 1.0 / n["n_years"], 1.0, EULER)
    elif entry == "E14.01":
        v0 = {1: 22.5, 2: 25.0, 3: 27.5, 4: 30.0}[int(n["zone"])]
        out.update({"v_b": v0, "q_b": float(sp.Rational(1, 2) * sp.Float(n.get("rho", 1.25)) * sp.Float(v0) ** 2)})
    elif entry == "E14.03":
        phi, sf = sp.Float(n["phi"]), sp.Float(n["s"])
        out["c_o"] = float(sp.Piecewise((1, phi < sp.Float(0.05)), (1 + 2 * sf * phi, phi <= sp.Float(0.3)), (1 + sp.Float(0.6) * sf, True)))
    elif entry in ("E14.04", "E14.05"):
        profile = wind_profile(n["z"], int(n["terrain"]), n.get("v_b", 1.0), n.get("rho", 1.25), n.get("c_o", 1.0))
        out.update(profile if entry == "E14.05" else {"I_v": profile["I_v"]})
    elif entry == "E14.06":
        p = n["p"] if "p" in n else 1.0 / n["n_years"]
        out["c_prob"] = return_period(0.2, p, 0.5, 0.0)
    elif entry == "E16.01":
        days = n["duration_days"]
        years = 2.0 if days <= 3 else 5.0 if days <= 90 else 10.0 if days <= 365 else 50.0
        out.update({"c_prob": return_period(0.2, 1 / years, 0.5, 0.0), "snow_factor": return_period(0.4 * math.sqrt(6) / math.pi, 1 / years, 1.0, EULER)})
    elif entry == "E14.07":
        e = sp.Min(sp.Float(n["b"]), 2 * sp.Float(n["h"]))
        out["e"] = float(e)
        out["B"] = float(sp.Piecewise((0, e >= 5 * n["d"]), (sp.Rational(4, 5) * e, e < n["d"]), (n["d"] - e / 5, True)))
    elif entry == "E14.13":
        out["F_w"] = float(sp.Rational(1, 2) * sp.Float(n.get("rho", 1.25)) * sp.Float(n["v_b"]) ** 2 * n["c_e"] * n["c_fx"] * n["a_ref"])
    elif entry == "E14.14":
        out["v_crit"] = float(sp.Float(n["b"]) * n["n"] / sp.Float(n.get("st", 0.18)))
        if "delta_s" in n:
            out["Sc"] = float(2 * sp.Float(n["delta_s"]) * n["m_e"] / (sp.Float(n.get("rho", 1.25)) * sp.Float(n["b"]) ** 2))
    elif entry == "E14.16" and "b" in n:
        out["Re"] = float(sp.Float(n["b"]) * sp.sqrt(2 * sp.Float(n["q_p"]) / sp.Float(n.get("rho", 1.25))) / sp.Float(15e-6))
    elif entry in ("E14.10", "E14.11") and "c_f" in n:
        out["F_w"] = float(sp.Float(n.get("c_s", 1.0)) * n.get("c_d", 1.0) * n["c_f"] * n["q_p"] * n["a_ref"])
        if entry == "E14.10":
            out["z_e"] = float(sp.Float(n.get("z_g", 0.0)) + sp.Float(n["h"]) / 2)
    elif entry == "E15.03":
        out["sigma"] = float(sp.Float(n["e"]) * n["alpha"] * n["dt"])
    elif entry == "E15.06":
        out["movement"] = float(sp.Float(n["alpha"]) * (n["dt"] + n.get("margin_k", 0.0)) * n["length"])
    elif entry == "E2.03":
        axle = sp.Float(n.get("axle", 400e3))
        out.update({"axle": float(axle), "wheel_pressure": float(axle / 2 / (sp.Float(n.get("a", 0.35)) * sp.Float(n.get("b", 0.60))))})
    elif entry == "E2.04":
        out["total"] = float(sp.Float(n["axles"]) * n["axle_load"])
    elif entry == "E2.14":
        out.update({"axle": float(sp.Float(n.get("alpha", 1.0)) * 250 * KN), "udl": float(sp.Float(n.get("alpha", 1.0)) * 80 * KN)})
    elif entry == "E2.07":
        a_q1, a_q11, q1k, q11k, w1, length = sp.symbols("aQ1 aq1 Q1k q1k w1 L")
        value = sp.Min(900 * KN, sp.Max(180 * KN * a_q1, sp.Float(0.6) * a_q1 * 2 * q1k + sp.Float(0.1) * a_q11 * q11k * w1 * length))
        out["Q_lk"] = float(value.subs({a_q1: 1, a_q11: 1, q1k: 300 * KN, q11k: 9 * KN, w1: 3, length: n["loaded_length"]}))
    elif entry == "E2.08":
        qv, r = sp.Float(n["q_v"]), sp.Float(n["radius"])
        out["Q_tk"] = float(sp.Piecewise((sp.Float(0.2) * qv, r < 200), (40 * qv / r, r <= 1500), (0, True)))
    elif entry == "E2.12":
        length = sp.Float(n["span"])
        out["q_fk"] = float(sp.Min(5 * KN, sp.Max(sp.Float(2.5) * KN, (2 + 120 / (length + 30)) * KN)))
    elif entry == "E2.16":
        root = sp.sqrt(sp.Float(n["l_phi"])) - sp.Float(0.2)
        out.update({"phi_2": float(sp.Min(sp.Float(1.67), sp.Max(1, sp.Float(1.44) / root + sp.Float(0.82)))), "phi_3": float(sp.Min(2, sp.Max(1, sp.Float(2.16) / root + sp.Float(0.73))))})
    elif entry == "E2.17":
        v, rad, lf, qvk = (sp.Float(n[k]) for k in ("v_kmh", "radius", "l_f", "q_vk"))
        f = sp.Piecewise((1, v <= 120), (1 - (v - 120) / 1000 * (814 / v + sp.Float(1.75)) * (1 - sp.sqrt(sp.Float(2.88) / lf)), True))
        out.update({"f": float(f), "Q_tk": float(v**2 / (127 * rad) * f * qvk)})
        if "loaded_length" in n:
            out.update({"traction": float(sp.Min(33 * KN * n["loaded_length"], 1000 * KN)), "braking": float(sp.Min(20 * KN * n["loaded_length"], 6000 * KN))})
    elif entry == "E3.01":
        low, slope = HOIST[s["hoist_class"]]
        out["phi_2"] = float(sp.Float(low) + sp.Float(slope) * sp.Float(n.get("v_h", 0.0)))
        if "dm_over_m" in n:
            out["phi_3"] = float(1 - sp.Float(n["dm_over_m"]) * (1 + sp.Float(n["beta_3"])))
    elif entry == "E3.04":
        out.update({"static": float(sp.Float(1.25) * n["q_h"]), "dynamic": float(sp.Float(1.1) * n["q_h"])})
    elif entry == "E3.05":
        out["Q_e"] = float(sp.Float(n["lambda"]) * n["q_max"])
    elif entry == "E3.07":
        out["F_dyn"] = float(sp.Float(n["mass"]) * n["eccentricity"] * (2 * sp.pi * sp.Float(n["rpm"]) / 60) ** 2)
    elif entry == "E4.02":
        phi = sp.rad(sp.Float(n["phi"]))
        out.update({"K_m": float(sp.Float(1.1) * (1 - sp.sin(phi))), "K_a": float(sp.tan(sp.pi / 4 - phi / 2) ** 2), "mu": float(sp.tan(sp.Float(0.75) * phi))})
    elif entry == "E4.03" and "gamma" in n and "z" in n:
        phi = n.get("phi")
        k = n["k"] if "k" in n else float(sp.Float(1.1) * (1 - sp.sin(sp.rad(sp.Float(phi)))))
        mu = n["mu"] if "mu" in n else float(sp.tan(sp.Float(0.75) * sp.rad(sp.Float(phi))))
        a = n["a_over_u"] if "a_over_u" in n else n["dc"] / 4.0
        out.update(janssen_closed(n["gamma"], mu, k, a, n["z"]))
    elif entry == "E4.08":
        out["p"] = float(sp.Float(n["gamma"]) * n["z"])
    elif entry == "E17.08":
        out.update({"energy": float(sp.Rational(1, 2) * sp.Float(n["mass"]) * sp.Float(n["speed"]) ** 2), "F_el": float(sp.Float(n["speed"]) * sp.sqrt(sp.Float(n.get("k", 3e5)) * sp.Float(n["mass"])))})
    elif entry == "E17.09":
        out.update({"p_d1": float((3 + sp.Float(n["p_v"])) * KN), "p_d2": float((3 + sp.Float(n["p_v"]) / 2 + sp.Float(0.04) / sp.Float(n["av_over_v"]) ** 2) * KN)})
    return out
