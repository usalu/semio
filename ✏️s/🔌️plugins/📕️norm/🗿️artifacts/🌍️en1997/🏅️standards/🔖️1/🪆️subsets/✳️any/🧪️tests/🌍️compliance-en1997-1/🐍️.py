#!/usr/bin/env python3
"""Language-agnostic EN 1997 evaluate oracle — bearing / sliding / pile / Bishop parity."""
from __future__ import annotations

import json
import math
import sys
from typing import Any


def n_q(phi_deg: float) -> float:
    phi = math.radians(phi_deg)
    return math.exp(math.pi * math.tan(phi)) * math.tan(math.pi / 4 + phi / 2) ** 2


def n_c(phi_deg: float) -> float:
    if abs(phi_deg) < 1e-9:
        return 5.14
    return (n_q(phi_deg) - 1.0) / math.tan(math.radians(phi_deg))


def n_gamma(phi_deg: float) -> float:
    return 2.0 * (n_q(phi_deg) - 1.0) * math.tan(math.radians(phi_deg))



def base_inclination_factors(phi_deg, alpha_deg):
    alpha = abs(math.radians(alpha_deg))
    phi = math.radians(phi_deg)
    if alpha < 1e-12:
        return 1.0, 1.0, 1.0
    b_q = max(0.0, 1.0 - math.tan(alpha) * math.tan(phi)) ** 2
    b_g = b_q
    nc = n_c(phi_deg)
    if abs(phi_deg) < 1e-9:
        b_c = 1.0 - 2.0 * alpha / math.pi
    else:
        b_c = b_q - (1.0 - b_q) / max(nc * math.tan(phi), 1e-9)
    return max(0.0, b_c), max(0.0, b_q), max(0.0, b_g)

def effective_gamma_below(gamma, gamma_prime, embedment, gwl, width):
    base = embedment
    zone = max(width, 0.1)
    if gwl >= base + zone:
        return gamma
    if gwl <= base:
        return max(gamma_prime, 1.0)
    wet = ((base + zone) - gwl) / zone
    return gamma * (1.0 - wet) + max(gamma_prime, 1.0) * wet

def shaft_kn(alpha_s: float, d: float, q_s_kpa: float, length: float) -> float:
    return alpha_s * math.pi * d * q_s_kpa * length


def parse_situation(value: str) -> str:
    key = (value or "bsP").strip().lower().replace("-", "").replace("_", "")
    if key in ("bst", "transient"):
        return "bsT"
    if key in ("bsa", "accidental"):
        return "bsA"
    return "bsP"


def resolve_params(approach: str, annex: str, situation: str) -> dict[str, float]:
    a = (approach or "da2").lower()
    ax = (annex or "de").lower()
    sit = parse_situation(situation)
    # Base DA2* DE table
    if a in ("da2", "geo2") and ax == "de":
        p = dict(gamma_g=1.35, gamma_q=1.5, gamma_r_v=1.4, gamma_r_h=1.1, gamma_c=1.0, gamma_phi=1.0, gamma_gamma=1.0, gamma_b=1.1, gamma_s=1.1)
    elif a in ("da2", "geo2"):
        p = dict(gamma_g=1.35, gamma_q=1.5, gamma_r_v=1.0, gamma_r_h=1.0, gamma_c=1.0, gamma_phi=1.0, gamma_gamma=1.0, gamma_b=1.1, gamma_s=1.1)
    elif a == "geo3":
        p = dict(gamma_g=1.0, gamma_q=1.3, gamma_r_v=1.0, gamma_r_h=1.0, gamma_c=1.25, gamma_phi=1.25, gamma_gamma=1.0, gamma_b=1.1, gamma_s=1.1)
    elif a == "da1geo":
        p = dict(gamma_g=1.0, gamma_q=1.3, gamma_r_v=1.4, gamma_r_h=1.1, gamma_c=1.0, gamma_phi=1.0, gamma_gamma=1.0, gamma_b=1.1, gamma_s=1.1)
    elif a == "da1str" and ax == "en":
        p = dict(gamma_g=1.35, gamma_q=1.5, gamma_r_v=1.0, gamma_r_h=1.0, gamma_c=1.25, gamma_phi=1.25, gamma_gamma=1.0, gamma_b=1.0, gamma_s=1.0)
    elif a == "da1str":
        p = dict(gamma_g=1.35, gamma_q=1.5, gamma_r_v=1.0, gamma_r_h=1.0, gamma_c=1.4, gamma_phi=1.25, gamma_gamma=1.0, gamma_b=1.0, gamma_s=1.0)
    else:
        p = dict(gamma_g=1.35, gamma_q=1.5, gamma_r_v=1.4, gamma_r_h=1.1, gamma_c=1.0, gamma_phi=1.0, gamma_gamma=1.0, gamma_b=1.1, gamma_s=1.1)
    if ax == "de" and a in ("da2", "geo2"):
        p["gamma_c"] = 1.0
        p["gamma_phi"] = 1.0
        if sit == "bsT":
            p.update(gamma_g=1.20, gamma_q=1.30, gamma_r_v=1.30, gamma_r_h=1.10)
        elif sit == "bsA":
            p.update(gamma_g=1.00, gamma_q=1.00, gamma_r_v=1.20, gamma_r_h=1.10)
        else:
            p.update(gamma_g=1.35, gamma_q=1.50, gamma_r_v=1.40, gamma_r_h=1.10)
    elif sit == "bsT":
        p["gamma_g"] = 1.20
        p["gamma_q"] = 1.30
    elif sit == "bsA":
        p["gamma_g"] = 1.00
        p["gamma_q"] = 1.00
    return p


def design_actions(p: dict[str, float], g: float, q: float) -> float:
    return p["gamma_g"] * g + p["gamma_q"] * q


def effective_width(b: float, e: float) -> float:
    return max(b - 2.0 * abs(e), 1e-6)


def shape_factors(phi_deg: float, b_p: float, l_p: float) -> tuple[float, float, float]:
    nq = n_q(phi_deg)
    nc = n_c(phi_deg)
    ratio = min(1.0, max(0.0, b_p / max(l_p, b_p)))
    s_q = 1.0 + ratio * math.sin(math.radians(phi_deg))
    s_g = max(0.7, 1.0 - 0.3 * ratio)
    s_c = 1.0 if abs(nc) < 1e-9 else (s_q * nq - 1.0) / max(nc, 1e-9)
    return s_c, s_q, s_g


def depth_factors(phi_deg: float, embedment: float, b_p: float) -> tuple[float, float, float]:
    phi = math.radians(phi_deg)
    d_b = embedment / max(b_p, 1e-6)
    d_q = 1.0 + 2.0 * math.tan(phi) * (1.0 - math.sin(phi)) ** 2 * math.atan(d_b)
    if abs(phi_deg) < 1e-9:
        d_c = 1.0 + 0.4 * math.atan(d_b)
    else:
        d_c = d_q - (1.0 - d_q) / max(n_c(phi_deg) * math.tan(phi), 1e-9)
    return d_c, d_q, 1.0


def inclination_factors(phi_deg: float, h: float, v: float, c: float, b_p: float, l_p: float) -> tuple[float, float, float]:
    a = b_p * l_p
    m = (2.0 + (b_p / max(l_p, b_p))) / (1.0 + (b_p / max(l_p, b_p)))
    nc = n_c(phi_deg)
    denom = max(v + a * c / max(nc, 1e-6), 1e-6)
    ratio = min(1.0, max(0.0, 1.0 - abs(h) / denom))
    i_q = ratio**m
    if abs(phi_deg) < 1e-9:
        i_c = 1.0 - m * abs(h) / max(a * c * nc + 1e-6, 1e-6)
    else:
        i_c = i_q - (1.0 - i_q) / max(nc * math.tan(math.radians(phi_deg)), 1e-9)
    i_g = ratio ** (m + 1.0)
    return max(0.0, i_c), max(0.0, i_q), max(0.0, i_g)


def ultimate_bearing_pa(phi_deg, c_pa, gamma_over, gamma_below, b, l, embedment, e_b, e_l, h, v, alpha_deg=0.0) -> float:
    b_p = effective_width(b, e_b)
    l_p = effective_width(l, e_l)
    s_c, s_q, s_g = shape_factors(phi_deg, b_p, l_p)
    d_c, d_q, d_g = depth_factors(phi_deg, embedment, b_p)
    i_c, i_q, i_g = inclination_factors(phi_deg, h, v, c_pa, b_p, l_p)
    b_c, b_q, b_g = base_inclination_factors(phi_deg, alpha_deg)
    q = gamma_over * embedment
    return (
        c_pa * n_c(phi_deg) * s_c * d_c * i_c * b_c
        + q * n_q(phi_deg) * s_q * d_q * i_q * b_q
        + 0.5 * gamma_below * b_p * n_gamma(phi_deg) * s_g * d_g * i_g * b_g
    )


def bearing_rd(phi_deg, c_pa, gamma_over, gamma_below, b, l, embedment, e_b, h, v, p, alpha_deg=0.0) -> float:
    phi_d = math.degrees(math.atan(math.radians(phi_deg) / p["gamma_phi"]))
    c_d = c_pa / p["gamma_c"]
    go = gamma_over / p["gamma_gamma"]
    gb = gamma_below / p["gamma_gamma"]
    b_p = effective_width(b, e_b)
    l_p = effective_width(l, 0.0)
    q_u = ultimate_bearing_pa(phi_d, c_d, go, gb, b, l, embedment, e_b, 0.0, h, v, alpha_deg)
    return q_u * b_p * l_p / p["gamma_r_v"]


def ka_coulomb(phi_deg: float, delta_deg: float) -> float:
    phi = math.radians(phi_deg)
    delta = math.radians(delta_deg)
    num = math.cos(phi) ** 2
    inner = math.sin(phi) * max(math.sin(phi - delta), 0.0) / max(math.cos(delta), 1e-6)
    den = (math.cos(phi) + math.sqrt(max(0.0, inner))) ** 2
    den = max(den, 1e-6)
    return max(0.1, num / den / max(math.cos(phi), 1e-6))


def kp_coulomb(phi_deg: float, delta_deg: float) -> float:
    return min(20.0, 1.0 / ka_coulomb(phi_deg, delta_deg))


def sliding_rd(phi_deg, c_pa, v_n, a, p, embedment, length, gamma, permanent_embedment) -> float:
    phi_d = math.atan(math.radians(phi_deg) / p["gamma_phi"])
    c_d = c_pa / p["gamma_c"]
    base = c_d * a + v_n * math.tan(phi_d)
    passive = 0.0
    if permanent_embedment and embedment > 0.0:
        kp = kp_coulomb(phi_deg, 0.0)
        e_pk = 0.5 * gamma * embedment * embedment * kp * length
        passive = 0.5 * e_pk
    return (base + passive) / p["gamma_r_h"]


def layer_at(layers, depth: float) -> dict[str, Any]:
    for layer in layers:
        if layer["depthTop"] - 1e-9 <= depth <= layer["depthBottom"] + 1e-9:
            return layer
    return layers[0] if layers else dict(
        id="default",
        phiPrimeDeg=30.0,
        cohesionEffective=0.0,
        gamma=18000.0,
        oedometricModulus=30e6,
        poissonRatio=0.3,
        depthTop=0.0,
        depthBottom=10.0,
    )


def stress_2to1(q, b, l, z) -> float:
    bb, ll = max(b, 1e-6), max(l, b)
    zz = max(z, 0.0)
    return q * (bb * ll) / ((bb + zz) * (ll + zz))


def settlement_oedometric(layers, width, length, embedment, q_sls, gwl=1e9) -> float:
    s = 0.0
    for layer in layers:
        z_top = max(0.0, layer["depthTop"] - embedment)
        z_bot = max(0.0, layer["depthBottom"] - embedment)
        if z_bot <= z_top + 1e-12:
            continue
        dz = z_bot - z_top
        z_mid = 0.5 * (z_top + z_bot)
        dsig = stress_2to1(q_sls, width, length, z_mid)
        depth_abs = embedment + z_mid
        if gwl < depth_abs:
            gamma = max(layer.get("gamma", 1.0), 1.0)
            gamma_p = layer.get("gammaPrime", gamma * 0.55)
            dsig *= max(0.4, min(1.0, gamma_p / gamma))
        e = max(layer.get("oedometricModulus", 1.0), 1.0)
        s += dsig / e * dz
    return s


def pile_xi(n: int) -> tuple[float, float]:
    if n <= 1:
        return 1.40, 1.40
    if n == 2:
        return 1.35, 1.27
    if n in (3, 4):
        t = (n - 2) / 3.0
        return 1.35 + t * (1.30 - 1.35), 1.27 + t * (1.15 - 1.27)
    return 1.30, 1.15


def pile_type_factors(pile_type: str, n: int, gamma_b: float, gamma_s: float) -> tuple[float, float, float, float]:
    xi3, xi4 = pile_xi(n)
    key = (pile_type or "bored").strip().lower()
    if key in ("driven", "ramm", "rammpfahl"):
        if n <= 1:
            xi3, xi4 = 1.35, 1.35
        elif n == 2:
            xi3, xi4 = 1.30, 1.25
        else:
            xi3, xi4 = 1.25, 1.15
        return max(gamma_b, 1.10), max(gamma_s, 1.15), xi3, xi4
    if key in ("cfa", "sob", "schnecke"):
        return max(gamma_b, 1.15), max(gamma_s, 1.15), max(xi3, 1.40), max(xi4, 1.40)
    return max(gamma_b, 1.10), max(gamma_s, 1.10), xi3, xi4


def layer_props_at(layers, depth: float):
    for layer in layers:
        if layer["depthTop"] <= depth <= layer["depthBottom"]:
            return layer
    return layers[-1] if layers else {
        "gamma": 18_000.0, "gammaPrime": 10_000.0, "phiPrimeDeg": 30.0,
        "cohesionEffective": 0.0, "cohesionUndrained": 0.0,
    }


def bishop_fos_slices(layers, height, angle_deg, length, gwl, gamma_phi, gamma_c) -> float:
    """Independent simplified Bishop FoS — mirrors Rust bishop_fos_slices_inner (GEO-3)."""
    beta = max(0.05, min(1.45, math.radians(angle_deg)))
    if length > 0.5:
        horiz = max(length, height / max(math.tan(beta), 0.05))
    else:
        horiz = height / max(math.tan(beta), 0.05)
    n_slices = 16
    best = float("inf")
    for xc_frac in (0.25, 0.4, 0.55, 0.7, 0.85):
        for yc_frac in (0.8, 1.1, 1.4, 1.8, 2.2):
            xc = xc_frac * horiz
            yc = height * yc_frac
            r_toe = max(math.hypot(xc, yc), height * 0.5)
            r_crest = max(math.hypot(xc - horiz, yc - height), height * 0.5)
            for r in (r_toe, r_crest, 0.5 * (r_toe + r_crest), r_toe * 1.15, r_crest * 1.15):
                fos = _bishop_one_circle(layers, height, beta, horiz, xc, yc, r, gwl, gamma_phi, gamma_c, n_slices)
                if fos is not None and math.isfinite(fos) and 0.05 < fos < 50.0:
                    best = min(best, fos)
    if best == float("inf"):
        layer = layer_props_at(layers, height * 0.5)
        return bishop_fos_closed(layer["phiPrimeDeg"], layer.get("cohesionEffective", 0.0), layer["gamma"], height, angle_deg, gamma_phi, gamma_c)
    return best


def _bishop_one_circle(layers, height, beta, horiz, xc, yc, r, gwl, gamma_phi, gamma_c, n_slices):
    gamma_w = 9_810.0
    dx = horiz / n_slices
    f = 1.5
    for _ in range(40):
        num = 0.0
        den = 0.0
        used = 0
        for i in range(n_slices):
            x = (i + 0.5) * dx
            y_surface = max(height - x * math.tan(beta), 0.0)
            under = r * r - (x - xc) ** 2
            if under <= 0.0:
                continue
            y_slip = yc - math.sqrt(under)
            if y_slip >= y_surface - 1e-9:
                continue
            h_slice = max(y_surface - y_slip, 0.0)
            if h_slice < 1e-6:
                continue
            mid_depth = max(height - 0.5 * (y_surface + y_slip), 0.0)
            layer = layer_props_at(layers, mid_depth)
            phi = layer["phiPrimeDeg"]
            c = layer.get("cohesionEffective", 0.0)
            gamma = layer["gamma"]
            gamma_p = layer.get("gammaPrime", gamma * 0.55)
            phi_d = math.atan(math.tan(math.radians(phi)) / gamma_phi)
            c_d = c / gamma_c
            slip_depth_from_crest = height - y_slip
            submerged = gwl < slip_depth_from_crest
            gamma_eff = max(gamma_p, 1.0) if submerged else gamma
            b = dx
            w = gamma_eff * h_slice * b
            u = gamma_w * max(gwl - (height - 0.5 * (y_surface + y_slip)), 0.0) if submerged else 0.0
            alpha = max(-1.2, min(1.2, math.asin(max(-1.0, min(1.0, (x - xc) / r)))))
            m_alpha = max(math.cos(alpha) + math.sin(alpha) * math.tan(phi_d) / max(f, 0.1), 0.05)
            num += (c_d * b + max(w - u * b, 0.0) * math.tan(phi_d)) / m_alpha
            den += w * math.sin(alpha)
            used += 1
        if used < n_slices // 3 or abs(den) < 1e-6:
            return None
        f_new = num / den
        if not math.isfinite(f_new) or f_new <= 0.0:
            return None
        if abs(f_new - f) < 1e-4:
            return max(0.05, min(50.0, f_new))
        f = 0.5 * (f + f_new)
    return max(0.05, min(50.0, f))


def bishop_fos_closed(phi_deg, c_pa, gamma, height, angle_deg, gamma_phi, gamma_c) -> float:
    phi_d = math.atan(math.tan(math.radians(phi_deg)) / gamma_phi)
    c_d = c_pa / gamma_c
    beta = math.radians(angle_deg)
    weight = gamma * height * height / (2.0 * max(math.tan(beta), 0.2))
    resisting = c_d * height / max(math.sin(beta), 0.2) + weight * math.tan(phi_d) * math.cos(beta) / max(math.sin(beta), 0.2)
    driving = weight * max(math.sin(beta), 0.05)
    return resisting / max(driving, 1.0)


def bishop_fos(phi_deg, c_pa, gamma, height, angle_deg, gamma_phi, gamma_c) -> float:
    return bishop_fos_closed(phi_deg, c_pa, gamma, height, angle_deg, gamma_phi, gamma_c)


def evaluate(doc: dict[str, Any]) -> dict[str, Any]:
    checks = []
    approach = doc.get("designApproach", "da2")
    annex = doc.get("annex", "de")
    layers = doc.get("layers", [])
    for footing in doc.get("footings", []):
        layer = layer_at(layers, footing.get("embedment", 0.0))
        for lc in footing.get("loadCases", []):
            sit = lc.get("designSituation") or doc.get("designSituation") or "bsP"
            p = resolve_params(approach, annex, sit)
            v_d = design_actions(p, lc.get("verticalPermanent", 0.0), lc.get("verticalVariable", 0.0))
            h_d = design_actions(p, lc.get("horizontalPermanent", 0.0), lc.get("horizontalVariable", 0.0))
            m_d = design_actions(p, lc.get("momentPermanent", 0.0), lc.get("momentVariable", 0.0))
            e_b = m_d / max(v_d, 1.0)
            a = footing["width"] * footing["length"]
            gwl = doc.get("groundwaterLevel", 0.0)
            gamma_below = effective_gamma_below(
                layer["gamma"], layer.get("gammaPrime", layer["gamma"] * 0.55),
                footing.get("embedment", 0.0), gwl, footing["width"],
            )
            r_v = bearing_rd(
                layer["phiPrimeDeg"],
                layer.get("cohesionEffective", 0.0),
                layer["gamma"],
                gamma_below,
                footing["width"],
                footing["length"],
                footing.get("embedment", 0.0),
                e_b,
                h_d,
                v_d,
                p,
                footing.get("baseInclinationDeg", 0.0),
            )
            checks.append(
                {
                    "id": f"en1997.6.5.bearing.{footing['id']}.{lc['id']}",
                    "utilization": v_d / r_v if r_v else 0.0,
                }
            )
            r_h = sliding_rd(
                layer["phiPrimeDeg"],
                layer.get("cohesionEffective", 0.0),
                v_d,
                a,
                p,
                footing.get("embedment", 0.0),
                footing["length"],
                layer["gamma"],
                footing.get("embedment", 0.0) > 0.0,
            )
            checks.append(
                {
                    "id": f"en1997.6.5.3.sliding.{footing['id']}.{lc['id']}",
                    "utilization": h_d / r_h if r_h else 0.0,
                }
            )
        q_sls = 0.0
        for lc in footing.get("loadCases", []):
            v_sls = lc.get("verticalPermanent", 0.0) + lc.get("verticalVariable", 0.0)
            q_sls = max(q_sls, v_sls / max(footing["width"] * footing["length"], 1e-9))
        s = settlement_oedometric(layers, footing["width"], footing["length"], footing.get("embedment", 0.0), q_sls, doc.get("groundwaterLevel", 1e9))
        limit = max(footing.get("settlementLimit", 0.025), 1e-12)
        checks.append({"id": f"en1997.6.6.settlement.{footing['id']}", "utilization": s / limit})

    for pile in doc.get("piles", []):
        p = resolve_params(approach, annex, doc.get("designSituation", "bsP"))
        profiles = pile.get("testProfiles") or []
        a_base = math.pi * (pile["diameter"] * 0.5) ** 2
        if not profiles:
            rs = pile.get("alphaS", 0.7) * math.pi * pile["diameter"] * pile.get("unitShaftResistance", 0.0) * pile["length"]
            rb = pile.get("unitBaseResistance", 0.0) * a_base
            shafts = [rs]
            bases = [rb]
        else:
            shafts = [t["shaftResistance"] for t in profiles]
            bases = [t["baseResistance"] for t in profiles]
        n = len(shafts)
        r_s_mean = sum(shafts) / n
        r_s_min = min(shafts)
        r_b_mean = sum(bases) / n
        r_b_min = min(bases)
        gamma_b, gamma_s, xi3, xi4 = pile_type_factors(pile.get("pileType", "bored"), n, p["gamma_b"], p["gamma_s"])
        r_s_k = min(r_s_mean / xi3, r_s_min / xi4)
        r_b_k = min(r_b_mean / xi3, r_b_min / xi4)
        r_c_d = (r_b_k / gamma_b + r_s_k / gamma_s) * pile.get("count", 1)
        n_ed = design_actions(p, pile.get("compressionPermanent", 0.0), pile.get("compressionVariable", 0.0))
        checks.append({"id": f"en1997.7.6.2.compression.{pile['id']}", "utilization": n_ed / r_c_d if r_c_d else 0.0})

    # Bishop simplified FoS for each slope (independent slice implementation, ±0.5 % vs Rust).
    for slope in doc.get("slopes", []):
        geo3 = resolve_params("geo3", annex, "bsP")
        fos = bishop_fos_slices(
            layers,
            slope.get("height", 1.0),
            slope.get("angleDeg", 30.0),
            slope.get("length", 0.0),
            doc.get("groundwaterLevel", 1e9),
            geo3.get("gamma_phi", 1.25),
            geo3.get("gamma_c", 1.25),
        )
        checks.append({"id": f"en1997.11.bishop.{slope['id']}", "utilization": 1.0 / max(fos, 1e-9)})

    return {"ok": True, "checks": checks}


def self_check() -> None:
    assert abs(n_q(30) - 18.401) < 0.05
    assert abs(n_c(30) - 30.14) < 0.1
    assert abs(n_gamma(30) - 20.093) < 0.15
    assert abs(shaft_kn(0.7, 0.6, 80, 12) - 1266.69) < 0.1


def main(argv: list[str]) -> int:
    self_check()
    if "--json" in argv:
        doc = json.load(sys.stdin)
        json.dump(evaluate(doc), sys.stdout)
        sys.stdout.write("\n")
        return 0
    print(json.dumps({"ok": True, "n_q_30": n_q(30), "n_c_30": n_c(30), "n_gamma_30": n_gamma(30), "shaft_kn": shaft_kn(0.7, 0.6, 80, 12)}))
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
