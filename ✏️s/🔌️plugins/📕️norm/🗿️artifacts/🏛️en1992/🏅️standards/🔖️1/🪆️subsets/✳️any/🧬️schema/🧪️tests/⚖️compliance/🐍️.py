#!/usr/bin/env python3
"""Language-agnostic EN 1992 evaluate oracle — EN 1990 combinations + ULS/SLS twins."""
from __future__ import annotations

import json
import math
import sys
from typing import Any


def annex_params(annex: str) -> dict[str, float]:
    a = (annex or "De").strip().lower()
    if a in ("de", "de-na", "din"):
        return dict(alpha_cc=0.85, alpha_ct=0.85, gamma_c=1.5, gamma_s=1.15)
    return dict(alpha_cc=1.0, alpha_ct=1.0, gamma_c=1.5, gamma_s=1.15)


def is_de(annex: str) -> bool:
    return (annex or "").strip().lower() in ("de", "de-na", "din")


def f_cd(f_ck: float, p: dict[str, float]) -> float:
    return p["alpha_cc"] * f_ck / p["gamma_c"]


def f_yd(f_yk: float, p: dict[str, float]) -> float:
    return f_yk / p["gamma_s"]


def psi_factors(category: str) -> tuple[float, float, float]:
    c = (category or "").lower()
    if c in ("office", "residential", "congregation"):
        return 0.7, 0.5, 0.3
    if c in ("shopping", "storage"):
        return 0.7, 0.7, 0.6
    if c in ("snow", "snow_high"):
        return 0.5, 0.2, 0.0
    if c == "wind":
        return 0.6, 0.2, 0.0
    return 0.7, 0.5, 0.3


def characteristic_effects(member: dict[str, Any], action: dict[str, Any]) -> tuple[float, float, float, float, float]:
    source = str(action.get("source", "udl")).lower()
    use_fem = bool(member.get("useFem", False))
    if source == "external" or use_fem:
        return (
            float(action.get("mK", 0.0)),
            float(action.get("nK", 0.0)),
            float(action.get("vK", 0.0)),
            float(action.get("tK", 0.0)),
            float(action.get("vKPunch", 0.0)),
        )
    l = max(float(member.get("span", 0.0)), 1e-6)
    udl = float(member.get("udl", 0.0))
    kind = str(action.get("kind", "")).lower()
    g_line = float(action.get("gKLine", 0.0)) + (udl if kind == "permanent" else 0.0)
    q_line = float(action.get("qKLine", 0.0))
    q = g_line if kind in ("permanent", "prestress") else q_line
    support = str(member.get("support", "")).lower()
    if "cantilever" in support:
        m, v = q * l * l / 2.0, q * l
    elif "continuous" in support or "fixed" in support:
        m, v = q * l * l / 12.0, q * l / 2.0
    else:
        m, v = q * l * l / 8.0, q * l / 2.0
    pf = abs(float(action.get("pointForce", 0.0)))
    if pf > 0.0:
        m += pf * l / 4.0
        v += pf / 2.0
    return m, 0.0, v, float(action.get("tK", 0.0)), float(action.get("vKPunch", 0.0))


def combine_member_actions(member: dict[str, Any]) -> list[dict[str, Any]]:
    g = [0.0, 0.0, 0.0, 0.0, 0.0]
    variables: list[tuple[dict[str, Any], tuple[float, float, float, float, float]]] = []
    accidentals: list[tuple[dict[str, Any], tuple[float, float, float, float, float]]] = []
    has_g = False
    for a in member.get("actions") or []:
        eff = characteristic_effects(member, a)
        kind = str(a.get("kind", "")).lower()
        if kind in ("permanent", "prestress"):
            has_g = True
            for i in range(5):
                g[i] += eff[i]
        elif kind == "accidental":
            accidentals.append((a, eff))
        else:
            variables.append((a, eff))
    if not has_g and float(member.get("udl", 0.0)) > 0.0 and not bool(member.get("useFem", False)):
        l = max(float(member.get("span", 0.0)), 1e-6)
        q = float(member.get("udl", 0.0))
        support = str(member.get("support", "")).lower()
        if "cantilever" in support:
            g = [q * l * l / 2.0, 0.0, q * l, 0.0, 0.0]
        elif "continuous" in support or "fixed" in support:
            g = [q * l * l / 12.0, 0.0, q * l / 2.0, 0.0, 0.0]
        else:
            g = [q * l * l / 8.0, 0.0, q * l / 2.0, 0.0, 0.0]
    gamma_g, gamma_q = 1.35, 1.5
    out: list[dict[str, Any]] = []
    if not variables:
        out.append({"situation": "uls", "combination_id": "ULS-6.10-G", "m_ed": gamma_g * g[0], "n_ed": gamma_g * g[1], "v_ed": gamma_g * g[2], "t_ed": gamma_g * g[3], "v_ed_punch": gamma_g * g[4]})
        out.append({"situation": "sls_qp", "combination_id": "SLS-qp", "m_ed": g[0], "n_ed": g[1], "v_ed": g[2], "t_ed": g[3], "v_ed_punch": g[4]})
        out.append({"situation": "sls_char", "combination_id": "SLS-char", "m_ed": g[0], "n_ed": g[1], "v_ed": g[2], "t_ed": g[3], "v_ed_punch": g[4]})
    else:
        for lead_i, (lead_a, lead_e) in enumerate(variables):
            m = gamma_g * g[0] + gamma_q * lead_e[0]
            n = gamma_g * g[1] + gamma_q * lead_e[1]
            v = gamma_g * g[2] + gamma_q * lead_e[2]
            t = gamma_g * g[3] + gamma_q * lead_e[3]
            vp = gamma_g * g[4] + gamma_q * lead_e[4]
            for j, (oa, oe) in enumerate(variables):
                if j == lead_i:
                    continue
                psi0, _, _ = psi_factors(str(oa.get("category", "")))
                m += gamma_q * psi0 * oe[0]
                n += gamma_q * psi0 * oe[1]
                v += gamma_q * psi0 * oe[2]
                t += gamma_q * psi0 * oe[3]
                vp += gamma_q * psi0 * oe[4]
            xi = 0.85
            m_b = xi * gamma_g * g[0] + gamma_q * lead_e[0]
            n_b = xi * gamma_g * g[1] + gamma_q * lead_e[1]
            v_b = xi * gamma_g * g[2] + gamma_q * lead_e[2]
            t_b = xi * gamma_g * g[3] + gamma_q * lead_e[3]
            vp_b = xi * gamma_g * g[4] + gamma_q * lead_e[4]
            for j, (oa, oe) in enumerate(variables):
                if j == lead_i:
                    continue
                psi0, _, _ = psi_factors(str(oa.get("category", "")))
                m_b += gamma_q * psi0 * oe[0]
                n_b += gamma_q * psi0 * oe[1]
                v_b += gamma_q * psi0 * oe[2]
                t_b += gamma_q * psi0 * oe[3]
                vp_b += gamma_q * psi0 * oe[4]
            if abs(m) >= abs(m_b):
                mu, nu, vu, tu, vpu, tag = m, n, v, t, vp, "6.10a"
            else:
                mu, nu, vu, tu, vpu, tag = m_b, n_b, v_b, t_b, vp_b, "6.10b"
            lid = str(lead_a.get("id", lead_i))
            out.append({"situation": "uls", "combination_id": f"ULS-{tag}-{lid}", "m_ed": mu, "n_ed": nu, "v_ed": vu, "t_ed": tu, "v_ed_punch": vpu})
            ms = g[0] + lead_e[0]
            ns = g[1] + lead_e[1]
            vs = g[2] + lead_e[2]
            ts = g[3] + lead_e[3]
            vps = g[4] + lead_e[4]
            for j, (oa, oe) in enumerate(variables):
                if j == lead_i:
                    continue
                psi0, _, _ = psi_factors(str(oa.get("category", "")))
                ms += psi0 * oe[0]
                ns += psi0 * oe[1]
                vs += psi0 * oe[2]
                ts += psi0 * oe[3]
                vps += psi0 * oe[4]
            out.append({"situation": "sls_char", "combination_id": f"SLS-char-{lid}", "m_ed": ms, "n_ed": ns, "v_ed": vs, "t_ed": ts, "v_ed_punch": vps})
        mq, nq, vq, tq, vpq = g[0], g[1], g[2], g[3], g[4]
        for oa, oe in variables:
            _, _, psi2 = psi_factors(str(oa.get("category", "")))
            mq += psi2 * oe[0]
            nq += psi2 * oe[1]
            vq += psi2 * oe[2]
            tq += psi2 * oe[3]
            vpq += psi2 * oe[4]
        out.append({"situation": "sls_qp", "combination_id": "SLS-qp", "m_ed": mq, "n_ed": nq, "v_ed": vq, "t_ed": tq, "v_ed_punch": vpq})
    for aa, ae in accidentals:
        m = g[0] + ae[0]
        n = g[1] + ae[1]
        v = g[2] + ae[2]
        t = g[3] + ae[3]
        vp = g[4] + ae[4]
        for oa, oe in variables:
            _, psi1, _ = psi_factors(str(oa.get("category", "")))
            m += psi1 * oe[0]
            n += psi1 * oe[1]
            v += psi1 * oe[2]
            t += psi1 * oe[3]
            vp += psi1 * oe[4]
        out.append({"situation": "accidental", "combination_id": f"ACC-6.11-{aa.get('id','A')}", "m_ed": m, "n_ed": n, "v_ed": v, "t_ed": t, "v_ed_punch": vp})
    return out


def governing(effects: list[dict[str, Any]], sit: str) -> dict[str, Any] | None:
    cand = [e for e in effects if e["situation"] == sit]
    if not cand:
        return None
    return max(cand, key=lambda e: abs(e["m_ed"]))


def shear_v_rd_c_n(b: float, d: float, f_ck: float, rho_l: float, n_ed: float, annex: str) -> float:
    p = annex_params(annex)
    f_ck_mpa = f_ck / 1.0e6
    d_mm = d * 1000.0
    b_mm = b * 1000.0
    k = min(1.0 + math.sqrt(200.0 / max(d_mm, 1e-9)), 2.0)
    rho = min(rho_l, 0.02)
    a_c_mm2 = b_mm * d_mm
    sigma_cp_mpa = 0.0
    if a_c_mm2 > 0.0:
        sigma_cp_mpa = (n_ed / 1000.0) / a_c_mm2 * 1000.0
    sigma_cp_mpa = max(0.0, min(sigma_cp_mpa, 0.2 * f_ck_mpa))
    c_rd_c = (0.15 if is_de(annex) else 0.18) / p["gamma_c"]
    v_min = 0.035 * (k ** 1.5) * math.sqrt(max(f_ck_mpa, 0.0))
    v1 = (c_rd_c * k * (100.0 * rho * f_ck_mpa) ** (1.0 / 3.0) + 0.15 * sigma_cp_mpa) * b_mm * d_mm
    v2 = (v_min + 0.15 * sigma_cp_mpa) * b_mm * d_mm
    return max(v1, v2)


def cot_theta(annex: str, sigma_cp: float, f_cd_pa: float) -> float:
    if is_de(annex):
        ratio = max(0.0, sigma_cp / f_cd_pa) if f_cd_pa > 0 else 0.0
        return min(3.0, max(1.0, 1.2 + 0.2 * ratio))
    return max(1.0, min(2.5, 3.0))


def shear_v_rd_s_n(asw_per_s: float, z: float, f_ywd: float, cot: float) -> float:
    return asw_per_s * z * f_ywd * cot


def shear_v_rd_max_n(b: float, z: float, f_ck: float, cot: float, annex: str) -> float:
    p = annex_params(annex)
    fcd = f_cd(f_ck, p)
    f_ck_mpa = f_ck / 1.0e6
    nu1 = 0.6 * (1.0 - f_ck_mpa / 250.0)
    sin_cos = cot / (1.0 + cot * cot)
    return 1.0 * b * z * nu1 * fcd * sin_cos


def eps_cu2_of(f_ck: float) -> float:
    fck = f_ck / 1.0e6
    if fck <= 50.0:
        return 3.5e-3
    return (2.6 + 35.0 * ((90.0 - fck) / 100.0) ** 4) * 1.0e-3

def n_parabola_of(f_ck: float) -> float:
    fck = f_ck / 1.0e6
    if fck <= 50.0:
        return 2.0
    return 1.4 + 23.4 * ((90.0 - fck) / 100.0) ** 4

def flexural_resistance_nm(f_ck: float, b: float, d: float, a_s: float, f_yk: float, n_ed: float, annex: str, e_s: float = 200.0e9) -> float:
    p = annex_params(annex)
    n_parabola = n_parabola_of(f_ck)
    eps_cu2 = eps_cu2_of(f_ck)
    eta = 1.0 if n_parabola <= 2.0 else max(1.0 - (f_ck / 1.0e6 - 50.0) / 200.0, 0.8)
    lam = 0.8 if n_parabola <= 2.0 else max(0.8 - (f_ck / 1.0e6 - 50.0) / 400.0, 0.6)
    fcd = eta * f_cd(f_ck, p)
    fyd = f_yd(f_yk, p)
    f_s = a_s * fyd
    f_c_req = max(f_s - n_ed, 0.0)
    x = f_c_req / (fcd * b * lam) if fcd * b * lam > 0 else 0.0
    eps_yd = fyd / e_s if e_s > 0 else 0.0025
    x_bal = d * eps_cu2 / (eps_cu2 + eps_yd) if eps_cu2 + eps_yd > 0 else 0.45 * d
    x_lim = min(x_bal, 0.45 * d)
    x_use = max(min(x, x_lim), 0.0)
    z = d - 0.5 * lam * x_use
    m_from_steel = f_s * z
    if abs(n_ed) > 1.0 and b * d > 0.0:
        n_rd = fcd * b * d + fyd * a_s
        util_n = max(-n_ed, 0.0) / max(n_rd, 1.0)
        return m_from_steel * max(1.0 - util_n, 0.0)
    return m_from_steel


def c_min_dur_m(exposure: str) -> float:
    e = (exposure or "").replace("_", "").lower()
    table = {
        "x0": 10.0, "xc1": 15.0, "xc2": 20.0, "xc3": 20.0, "xc4": 25.0,
        "xd1": 40.0, "xs1": 40.0, "xd2": 40.0, "xs2": 40.0, "xd3": 45.0, "xs3": 45.0,
        "xf1": 20.0, "xf2": 25.0, "xf3": 25.0, "xf4": 30.0,
        "xa1": 25.0, "xa2": 30.0, "xa3": 40.0,
    }
    return table.get(e, 20.0) / 1000.0


def basic_ld_limit(support: str, rho: float, rho_prime: float = 0.0) -> float:
    s = (support or "").lower()
    if "cantilever" in s:
        k = 0.4
    elif "continuous" in s or "fixed" in s:
        k = 1.5
    else:
        k = 1.0
    f_yk_ref = 500.0
    rho0 = 1.0e-3 * math.sqrt(f_yk_ref)
    rho = max(rho, 1e-6)
    rho_prime = max(rho_prime, 0.0)
    if rho <= rho0:
        return k * (11.0 + 1.5 * math.sqrt(f_yk_ref) * rho0 / rho + 3.2 * max(rho0 / rho - 1.0, 0.0) ** 1.5)
    return k * (11.0 + 1.5 * math.sqrt(f_yk_ref) * rho0 / max(rho, rho0) + rho_prime / rho0 * math.sqrt(f_yk_ref) / 12.0)


def de_ld_caps(k: float, span: float, sensitive: bool) -> float:
    cap1 = k * 35.0
    if sensitive and span > 0:
        return min(cap1, (k * k) * 150.0 / span)
    return cap1


def support_k(support: str) -> float:
    s = (support or "").lower()
    if "cantilever" in s:
        return 0.4
    if "continuous" in s or "fixed" in s:
        return 1.5
    return 1.0


def lambda_lim_de(n: float) -> float:
    if n >= 0.41:
        return 25.0
    return 16.0 / math.sqrt(max(n, 1e-6))


def grade_map(snap: dict[str, Any], key: str, value_field: str) -> dict[str, float]:
    return {str(g.get("id", "")): float(g.get(value_field, 0.0)) for g in (snap.get(key) or [])}


def member_as(member: dict[str, Any]) -> float:
    total = 0.0
    for layer in member.get("longitudinal") or []:
        dia = float(layer.get("diameter", 0.0))
        count = float(layer.get("count", 0.0))
        total += count * math.pi * (dia * 0.5) ** 2
    return total


def asw_per_s(member: dict[str, Any]) -> float:
    st = member.get("stirrups")
    if not st:
        return 0.0
    dia = float(st.get("diameter", 0.0))
    spacing = float(st.get("spacing", 0.0))
    legs = float(st.get("legs", 0.0))
    if spacing <= 0:
        return 0.0
    return legs * math.pi * (dia * 0.5) ** 2 / spacing



def cracked_sigma_s_pa(m: float, b: float, d: float, a_s: float, e_s: float, e_cm: float) -> float:
    if a_s <= 0.0 or d <= 0.0:
        return 0.0
    alpha_e = e_s / max(e_cm, 1.0)
    rho = a_s / (b * d)
    x = d * ((alpha_e * rho) ** 2 + 2.0 * alpha_e * rho) ** 0.5 - alpha_e * rho * d
    z = d - x / 3.0
    if z <= 0.0:
        return 0.0
    return abs(m) / (a_s * z)


def cracked_sigma_c_pa(m: float, b: float, d: float, a_s: float, e_s: float, e_cm: float) -> float:
    if a_s <= 0.0 or d <= 0.0 or b <= 0.0:
        return 0.0
    alpha_e = e_s / max(e_cm, 1.0)
    rho = a_s / (b * d)
    x = d * ((alpha_e * rho) ** 2 + 2.0 * alpha_e * rho) ** 0.5 - alpha_e * rho * d
    i_cr = b * x ** 3 / 3.0 + alpha_e * a_s * (d - x) ** 2
    if i_cr <= 0.0:
        return 0.0
    return abs(m) * x / i_cr

def util_minimum(computed: float, minimum: float) -> float:
    if computed >= minimum:
        return minimum / max(computed, minimum)
    return computed / max(minimum, 1e-15)


def torsion_t_rd_nm(f_ck_pa: float, b: float, h: float, annex: str) -> float:
    p = annex_params(annex)
    f_cd_pa = f_cd(f_ck_pa, p)
    t_eff = min(b, h) / 6.0
    a_k = (b - t_eff) * (h - t_eff)
    f_ck_mpa = f_ck_pa / 1.0e6
    nu = 0.6 * (1.0 - f_ck_mpa / 250.0)
    return 2.0 * a_k * t_eff * nu * f_cd_pa * 0.5

def concrete_e_cm(f_ck_pa: float) -> float:
    f_cm = f_ck_pa + 8.0e6
    return 22.0e3 * ((f_cm / 1.0e6) / 10.0) ** 0.3 * 1.0e6

def reinforcement_e_s(_f_yk_pa: float = 500.0e6) -> float:
    return 200.0e9


def second_order_moment_nm(n_ed: float, l_0: float, d: float, f_yd: float, e_s: float) -> float:
    curv = (0.45 * f_yd / max(e_s, 1.0)) / max(d, 1e-6)
    r = 1.0 / max(curv, 1e-9)
    e2 = l_0 * l_0 / (r * 10.0)
    return abs(n_ed) * e2


def evaluate(snap: dict[str, Any]) -> dict[str, Any]:
    annex = str(snap.get("annex", "De"))
    p = annex_params(annex)
    fcks = grade_map(snap, "concreteGrades", "fCk")
    fyks = grade_map(snap, "reinforcementGrades", "fYk")
    delta_c = float(snap.get("deltaCDev", 0.01))
    life = float(snap.get("designWorkingLifeYears", 50.0))
    cement = str(snap.get("cementType", "N"))
    checks = []
    for m in snap.get("members") or []:
        mid = m.get("id", "?")
        f_ck = fcks.get(str(m.get("concreteGradeId", "")), 30.0e6)
        f_yk = fyks.get(str(m.get("reinforcementGradeId", "")), 500.0e6)
        b = float(m.get("width", 0.0))
        d = float(m.get("effectiveDepth", 0.0))
        h = float(m.get("height", 0.0))
        a_s = member_as(m)
        rho = a_s / max(b * d, 1e-12)
        layers = m.get("longitudinal") or []
        phi = float(layers[0].get("diameter", 0.012)) if layers else 0.012
        agg = float(layers[0].get("aggregateSize", 0.016)) if layers else 0.016
        c_min_b = phi + (0.005 if agg > 0.032 else 0.0)
        c_min_dur = c_min_dur_m(str(m.get("exposure", "Xc3")))
        if life >= 100:
            c_min_dur += 0.010
        elif life <= 25:
            c_min_dur -= 0.005
        if "R" in cement.upper():
            c_min_dur -= 0.005
        c_min_dur = max(c_min_dur, 0.010)
        c_nom = max(c_min_b, c_min_dur, 0.010) + delta_c
        cover = float(m.get("cover", 0.0))
        checks.append({"id": f"en1992.4.4.cover.{mid}", "utilization": util_minimum(cover, c_nom)})

        effects = combine_member_actions(m)
        uls = governing(effects, "uls")
        sls_qp = governing(effects, "sls_qp")
        sls_char = governing(effects, "sls_char")
        if uls is not None:
            m_ed = abs(float(uls["m_ed"]))
            n_ed = float(uls["n_ed"])
            v_ed = float(uls["v_ed"])
            m_rd = flexural_resistance_nm(f_ck, b, d, a_s, f_yk, n_ed, annex)
            checks.append({"id": f"en1992.6.1.flexure.{mid}", "utilization": m_ed / max(m_rd, 1.0)})
            v_rd_c = shear_v_rd_c_n(b, d, f_ck, rho, n_ed, annex)
            z = 0.9 * d
            sigma_cp = max(-n_ed, 0.0) / (b * h) if b * h > 0 else 0.0
            fcd = f_cd(f_ck, p)
            cot = cot_theta(annex, sigma_cp, fcd)
            asw_s = asw_per_s(m)
            f_ywd = f_yd(f_yk, p)
            v_rd_s = shear_v_rd_s_n(asw_s, z, f_ywd, cot) if asw_s > 0 else 0.0
            v_rd_max = shear_v_rd_max_n(b, z, f_ck, cot, annex)
            v_limit = min(v_rd_s, v_rd_max) if asw_s > 0 else v_rd_c
            checks.append({"id": f"en1992.6.2.shear.{mid}", "utilization": abs(v_ed) / max(v_limit, 1.0)})
            if abs(float(uls["t_ed"])) > 1.0:
                t_rd = torsion_t_rd_nm(f_ck, b, h, annex)
                checks.append({"id": f"en1992.6.3.torsion.{mid}", "utilization": abs(float(uls["t_ed"])) / max(t_rd, 1.0)})

        f_ctm = 0.30 * (f_ck / 1e6) ** (2.0 / 3.0) * 1e6 if f_ck / 1e6 <= 50 else 2.12 * math.log(1.0 + (f_ck / 1e6 + 8.0) / 10.0) * 1e6
        a_min = max(0.26 * f_ctm / f_yk * b * d, 0.0013 * b * d)
        a_max = 0.04 * b * h
        checks.append({"id": f"en1992.9.minas.{mid}", "utilization": util_minimum(a_s, a_min)})
        checks.append({"id": f"en1992.9.maxas.{mid}", "utilization": a_s / max(a_max, 1e-12)})

        span = float(m.get("span", 0.0))
        kind = str(m.get("kind", "")).lower().replace("_", "-")
        if d > 0 and span > 0 and kind in ("beam", "slab", "flatslab", "flat-slab"):
            k = support_k(str(m.get("support", "")))
            lim_table = basic_ld_limit(str(m.get("support", "")), max(rho, 1e-4))
            lim_de = de_ld_caps(k, span, bool(m.get("deflectionSensitive", False)))
            ld_lim = min(lim_table, lim_de)
            checks.append({"id": f"en1992.7.4.ld.{mid}", "utilization": (span / d) / max(ld_lim, 1.0)})

        if kind in ("column", "wall") and float(m.get("bucklingLength", 0.0)) > 0:
            l0 = float(m.get("bucklingLength", 0.0))
            i = min(b, h) / math.sqrt(12.0)
            lam = l0 / i if i > 0 else 0.0
            n_ed = float(uls["n_ed"]) if uls else 0.0
            n_ratio = min(abs(n_ed) / max(b * h * f_cd(f_ck, p), 1.0), 1.0)
            lim = lambda_lim_de(n_ratio)
            fyd = f_yd(f_yk, p)
            e_s_col = reinforcement_e_s(f_yk)
            m2 = second_order_moment_nm(n_ed, l0, d, fyd, e_s_col) if lam > lim else 0.0
            m_ed1 = abs(float(uls["m_ed"])) if uls else 0.0
            m_ed_tot = m_ed1 + m2
            m_rd = flexural_resistance_nm(f_ck, b, d, a_s, f_yk, n_ed, annex)
            checks.append({"id": f"en1992.5.8.slender.{mid}", "utilization": abs(m_ed_tot) / max(m_rd, 1.0)})

        e_cm = concrete_e_cm(f_ck)
        e_s = reinforcement_e_s(f_yk)
        if sls_char is not None:
            sig_s = cracked_sigma_s_pa(float(sls_char["m_ed"]), b, d, a_s, e_s, e_cm)
            lim_s = 0.8 * f_yk
            checks.append({"id": f"en1992.7.2.sigma-s.{mid}", "utilization": abs(sig_s) / max(lim_s, 1.0)})
            sig_c = cracked_sigma_c_pa(float(sls_char["m_ed"]), b, d, a_s, e_s, e_cm)
            lim_c = 0.60 * f_ck
            checks.append({"id": f"en1992.7.2.sigma-c.{mid}", "utilization": abs(sig_c) / max(lim_c, 1.0)})
        if sls_qp is not None:
            sig_c = cracked_sigma_c_pa(float(sls_qp["m_ed"]), b, d, a_s, e_s, e_cm)
            lim = 0.45 * f_ck
            checks.append({"id": f"en1992.7.2.creep.{mid}", "utilization": abs(sig_c) / max(lim, 1.0)})

        f_ctk = 0.7 * (0.30 * (f_ck / 1e6) ** (2.0 / 3.0) * 1e6 if f_ck / 1e6 <= 50 else 2.12 * math.log(1.0 + (f_ck / 1e6 + 8.0) / 10.0) * 1e6)
        for layer in layers:
            lid = layer.get("id", "L")
            phi = float(layer.get("diameter", 0.012))
            eta1 = 0.7 if str(layer.get("bondCondition", "good")).lower() == "poor" else 1.0
            eta2 = 1.0 if phi <= 0.032 else max(0.7, min(1.0, (132.0 - phi * 1000.0) / 100.0))
            f_bd = 2.25 * eta1 * eta2 * f_ctk
            sigma_sd = f_yk / p["gamma_s"]
            lb_rqd = (phi / 4.0) * (sigma_sd / f_bd) if f_bd > 0 else 50.0 * phi
            lbd = max(1.0 * lb_rqd, 0.3 * lb_rqd, 10.0 * phi, 0.100)
            l0 = max(1.5 * lbd, 0.3 * 1.5 * lbd, 15.0 * phi, 0.200)
            checks.append({"id": f"en1992.8.4.anchorage.{mid}.{lid}", "utilization": util_minimum(float(layer.get("anchorageLength", 0.0)), lbd)})
            checks.append({"id": f"en1992.8.7.lap.{mid}.{lid}", "utilization": util_minimum(float(layer.get("lapLength", 0.0)), l0)})

        if m.get("fire"):
            rating = str(m["fire"].get("rating", "r60")).lower().replace("_","")
            support = str(m.get("support", "")).lower()
            # Tables 5.2a–5.11 minima (a, dim) aligned with Rust part_1_2_fire::required_for
            table = {
                "r30": (0.025, 0.080),
                "r60": (0.040, 0.120),
                "r90": (0.055, 0.150),
                "r120": (0.065, 0.200),
            }
            if kind in ("column",):
                table = {"r30": (0.025, 0.200), "r60": (0.035, 0.250), "r90": (0.045, 0.350), "r120": (0.045, 0.350)}
            elif kind in ("wall", "liquid-retaining", "liquidretaining"):
                table = {"r30": (0.010, 0.100), "r60": (0.010, 0.120), "r90": (0.020, 0.140), "r120": (0.030, 0.160)}
            elif kind in ("flat-slab", "flatslab"):
                table = {"r30": (0.010, 0.150), "r60": (0.015, 0.180), "r90": (0.025, 0.200), "r120": (0.035, 0.200)}
            elif kind == "slab":
                table = {"r30": (0.010, 0.060), "r60": (0.020, 0.080), "r90": (0.030, 0.100), "r120": (0.040, 0.120)}
            elif "continuous" in support or "fixed" in support:
                table = {"r30": (0.015, 0.080), "r60": (0.025, 0.120), "r90": (0.035, 0.150), "r120": (0.045, 0.200)}
            a_req, dim_min = table.get(rating, (0.040, 0.120))
            checks.append({"id": f"en1992.1-2.a.{mid}", "utilization": util_minimum(float(m["fire"].get("axisDistance", 0.0)), a_req)})
            # Rust fire b_min check uses member.width (b) against tabulated min dimension.
            checks.append({"id": f"en1992.1-2.bmin.{mid}", "utilization": util_minimum(b, dim_min)})

        if kind in ("flat-slab", "flatslab") and m.get("punching") and uls is not None:
            pspec = m["punching"]
            c1 = float(pspec.get("columnWidth", 0.3))
            c2 = float(pspec.get("columnDepth", 0.3))
            pos = str(pspec.get("columnPosition", "interior")).lower()
            beta = 1.40 if pos == "edge" else (1.50 if pos == "corner" else 1.10)
            u0 = 2.0 * (c1 + c2)
            u1 = u0 + 2.0 * math.pi * 2.0 * d
            u0_over_d = u0 / max(d, 1e-6)
            f_ck_mpa = f_ck / 1e6
            d_mm = d * 1000.0
            k = min(1.0 + math.sqrt(200.0 / max(d_mm, 1e-9)), 2.0)
            rho_u = min(rho, 0.02)
            c_rd = 0.18 / p["gamma_c"] if is_de(annex) else 0.18 / p["gamma_c"]
            if is_de(annex) and u0_over_d < 4.0:
                c_rd *= min(1.0, 0.5 + 0.125 * u0_over_d)
            elif is_de(annex):
                c_rd = 0.18 / p["gamma_c"]
            else:
                c_rd = 0.18 / p["gamma_c"]
            # EN default punching in Rust part_1_1 for En uses 0.18; De path overridden above
            v_min = 0.035 * (k ** 1.5) * math.sqrt(max(f_ck_mpa, 0.0))
            v_rd_c_pa = max(c_rd * k * (100.0 * rho_u * f_ck_mpa) ** (1.0 / 3.0), v_min) * 1e6
            asw = float(pspec.get("asw", 0.0))
            v_rd_cs_pa = v_rd_c_pa + (0.75 * asw * (f_yk / p["gamma_s"]) / max(u1 * d, 1e-9) if asw > 0 else 0.0)
            v_ed_pa = beta * abs(float(uls["v_ed_punch"])) / max(u1 * d, 1e-12)
            checks.append({"id": f"en1992.6.4.punching.{mid}", "utilization": v_ed_pa / max(v_rd_cs_pa, 1.0)})

    return {"checks": checks, "params": {"gamma_c": p["gamma_c"], "gamma_s": p["gamma_s"], "alpha_cc": p["alpha_cc"]}}


def main(argv: list[str]) -> int:
    if "--json" in argv:
        snap = json.load(sys.stdin)
        json.dump(evaluate(snap), sys.stdout)
        return 0
    v = shear_v_rd_c_n(0.30, 0.450, 30.0e6, 0.01, 0.0, "De")
    assert abs(v / 1e3 - 69.9) < 2.0, v
    assert shear_v_rd_c_n(0.30, 0.450, 30.0e6, 0.01, 0.0, "En") > v
    assert abs(annex_params("De")["alpha_cc"] - 0.85) < 1e-12
    assert abs(c_min_dur_m("Xc3") - 0.020) < 1e-12
    print("oracle self-check ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
