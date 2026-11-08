"""📇️ Independent oracle of the EN 1992 catalogue (`--catalogue`: line protocol `entry|case|annex=..|n:k=v|s:k=v` for the norm step layer, otherwise `{"cases":[...]}` on stdin): numpy/scipy/shapely formulations (numerical integration over shapely polygon strips, brentq root finding, least-squares truss, polygon union areas) instead of the Rust closed forms; stdin `{"cases":[{id,annex,num,text,list}]}` -> stdout `{"results":[{id,values}]}` in SI (Pa, N, m, N·m, m²)."""

import importlib.util
import json
import math
import sys
from pathlib import Path

import numpy as np
from scipy import integrate, optimize
from shapely.geometry import box, Polygon
from shapely.ops import unary_union

MM = 1e-3
ECM = {12: 27e3, 16: 29e3, 20: 30e3, 25: 31e3, 30: 33e3, 35: 34e3, 40: 35e3, 45: 36e3, 50: 37e3, 55: 38e3, 60: 39e3, 70: 41e3, 80: 42e3, 90: 44e3}


_FACTORS_PATH = next(parent / "⚖️compliance" / "📜️annex-and-tables" / "⚖️action-factors" / "🐍️.py" for parent in Path(__file__).resolve().parents if (parent / "⚖️compliance" / "📜️annex-and-tables" / "⚖️action-factors" / "🐍️.py").is_file())
_SPEC = importlib.util.spec_from_file_location("action_factors", _FACTORS_PATH)
FACTORS = importlib.util.module_from_spec(_SPEC)
_SPEC.loader.exec_module(FACTORS)


def ndp(annex, ec, de):
    return de if annex == "De" else ec


def alpha_cc(annex):
    """🧱 α_cc of the annex: the registry row `ndp.en1992.m-01.alpha-cc-persistent` the Rust reads."""
    return FACTORS.ndp("ndp.en1992.m-01.alpha-cc-persistent", annex)


def gamma_c(annex):
    """🧱 γ_c of the annex: the registry row `ndp.en1992.m-01.gamma-c-persistent`."""
    return FACTORS.ndp("ndp.en1992.m-01.gamma-c-persistent", annex)


def conc(fck):
    fcm = fck + 8
    fctm = 0.3 * fck ** (2 / 3) if fck <= 50 else 2.12 * math.log(1 + fcm / 10)
    table = {12: 1.1, 16: 1.3, 20: 1.5, 25: 1.8, 30: 2.0, 35: 2.2, 40: 2.5, 45: 2.7, 50: 2.9}
    tfctm = {12: 1.6, 16: 1.9, 20: 2.2, 25: 2.6, 30: 2.9, 35: 3.2, 40: 3.5, 45: 3.8, 50: 4.1}
    hi = fck > 50
    q = ((90 - fck) / 100) ** 4
    return dict(fck=fck, fcm=fcm, fctm=tfctm.get(int(fck), fctm), fctk05=table.get(int(fck), 0.7 * fctm), ec2=2.0 + 0.085 * (fck - 50) ** 0.53 if hi else 2.0, ecu2=2.6 + 35 * q if hi else 3.5, n=1.4 + 23.4 * q if hi else 2.0)


def fcd(annex, fck):
    return alpha_cc(annex) * fck / gamma_c(annex)


def fyd(fyk):
    return fyk / 1.15


def sigma_c(c, f, e):
    return 0.0 if e <= 0 or e > c["ecu2"] + 1e-12 else (f * (1 - (1 - e / c["ec2"]) ** c["n"]) if e < c["ec2"] else f)


def m05(case):
    c = conc(case["num"]["fck"])
    f = fcd(case["annex"], c["fck"])
    e = case["num"]["eps"]
    top = max(min(e, c["ecu2"]), 1e-9)
    force = integrate.quad(lambda x: sigma_c(c, f, x), 0, top, points=[c["ec2"]] if top > c["ec2"] else None)[0]
    moment = integrate.quad(lambda x: x * sigma_c(c, f, x), 0, top, points=[c["ec2"]] if top > c["ec2"] else None)[0]
    return {"sigma_c": sigma_c(c, f, e) * 1e6, "alpha_r": force / (f * top), "delta_r": 1 - moment / force / top}


def section_nm(case, x, alpha_deg=90.0, n_strips=1500):
    num, lst = case["num"], case["list"]
    c = conc(num["fck"])
    f = fcd(case["annex"], num["fck"])
    fy = fyd(num.get("fyk", 500.0))
    poly = box(-num["b"] / 2, -num["h"] / 2, num["b"] / 2, num["h"] / 2) if "b" in num else Polygon(list(zip(lst["poly_y"], lst["poly_z"])))
    cy, cz = poly.centroid.x, poly.centroid.y
    a = math.radians(alpha_deg)
    nvec = np.array([math.cos(a), math.sin(a)])
    pts = np.array(poly.exterior.coords) - [cy, cz]
    proj = pts @ nvec
    hi, depth = proj.max(), proj.max() - proj.min()
    ecu, ec2 = c["ecu2"], c["ec2"]
    bars = [(y - cy, z - cz, ar) for y, z, ar in zip(lst["bar_y"], lst["bar_z"], lst["bar_a"])]
    etop = ecu
    if x <= depth:
        for y, z, _ in bars:
            u = hi - (y * nvec[0] + z * nvec[1])
            if u > x:
                etop = min(etop, 45.0 * x / (u - x))
    else:
        etop = ec2 * x / (x - depth * (1 - ec2 / ecu))
    eps = lambda u: etop * (x - u) / x
    zone = min(x, depth)
    n = 0.0
    my = 0.0
    mz = 0.0
    rot = Polygon(pts)
    big = 10 * (depth + 1)
    tangent = np.array([-nvec[1], nvec[0]])
    for i in range(n_strips):
        u0, u1 = zone * i / n_strips, zone * (i + 1) / n_strips
        s = sigma_c(c, f, eps(0.5 * (u0 + u1)))
        if s == 0:
            continue
        c0, c1 = hi - u1, hi - u0
        strip = Polygon([nvec * c0 + tangent * big, nvec * c1 + tangent * big, nvec * c1 - tangent * big, nvec * c0 - tangent * big])
        part = rot.intersection(strip)
        if part.is_empty:
            continue
        n += s * part.area
        my += s * part.area * part.centroid.y
        mz -= s * part.area * part.centroid.x
    for y, z, ar in bars:
        e = eps(hi - (y * nvec[0] + z * nvec[1]))
        ey = fy / 200000.0 * 1000
        st = e * 200.0 if abs(e) <= ey else (fy + (1.08 * fy - fy) / (45.0 - ey) * (min(abs(e), 45.0) - ey)) * np.sign(e)
        if e > 0:
            st -= sigma_c(c, f, e)
        n += st * ar
        my += st * ar * z
        mz -= st * ar * y
    return n, my, mz, depth


def sc_section(case):
    from shapely import Polygon
    from structuralcodes.geometry import SurfaceGeometry, add_reinforcement
    from structuralcodes.materials.concrete import ConcreteEC2_2004
    from structuralcodes.materials.reinforcement import ReinforcementEC2_2004
    from structuralcodes.sections import GenericSection

    n, l = case["num"], case["list"]
    fyk = n.get("fyk", 500.0)
    concrete = ConcreteEC2_2004(fck=n["fck"], gamma_c=gamma_c(case["annex"]), alpha_cc=alpha_cc(case["annex"]))
    steel = ReinforcementEC2_2004(fyk=fyk, Es=200000, ftk=1.08 * fyk, epsuk=n.get("eps_uk", 75.0) / 1000.0, gamma_s=1.15)
    geo = SurfaceGeometry(Polygon(list(zip(l["poly_y"], l["poly_z"]))), concrete)
    for y, z, a in zip(l["bar_y"], l["bar_z"], l["bar_a"]):
        geo = add_reinforcement(geo, (y, z), 2.0 * math.sqrt(a / math.pi), steel)
    return GenericSection(geo)


def sc_s04(case):
    r = sc_section(case).section_calculator.calculate_bending_strength(theta=0, n=0)
    return {"m_rd": abs(r.m_y) / 1e3}


def sc_s11(case):
    res = sc_section(case).section_calculator.calculate_moment_curvature(theta=0, n=0)
    kappa = [abs(float(k)) * 1000.0 for k in res.chi_y][4:]
    given = case["list"]["kappa"]
    if len(kappa) != len(given) or any(abs(a - b) > 1e-9 * abs(b) for a, b in zip(kappa, given)):
        raise ValueError("structuralcodes curvature ladder differs from the committed vector")
    return {f"m_{i}": abs(float(m)) / 1e3 for i, m in enumerate(res.m_y[4:])}


def s04(case):
    if case["text"].get("oracle") == "structuralcodes":
        return sc_s04(case)
    ned = case["num"]["n_ed"]
    depth = case["num"]["h"] if "h" in case["num"] else 400.0
    x = optimize.brentq(lambda x: section_nm(case, x)[0] - ned, 1e-3 * depth, 1e3 * depth, xtol=1e-6)
    return {"m_rd": section_nm(case, x)[1] / 1e3, "x": x * MM}


def s06(case):
    ned, my, mz = case["num"]["n_ed"], case["num"]["m_edy"], case["num"]["m_edz"]
    curve = []
    for ang in range(0, 360, int(case["num"].get("step_deg", 5))):
        depth = 400.0
        try:
            x = optimize.brentq(lambda x: section_nm(case, x, ang, 300)[0] - ned, 1e-3 * depth, 1e3 * depth, xtol=1e-5)
        except ValueError:
            continue
        _, a, b, _ = section_nm(case, x, ang, 300)
        curve.append((a, b))
    poly = Polygon(curve + [curve[0]])
    d = np.array([my, mz]) / math.hypot(my, mz)
    from shapely.geometry import LineString
    hit = LineString([(0, 0), tuple(d * 1e12)]).intersection(poly.exterior)
    r = math.hypot(hit.x, hit.y) if hit.geom_type == "Point" else max(math.hypot(p.x, p.y) for p in hit.geoms)
    return {"util": math.hypot(my, mz) / r}


def creep(fcm, rh, h0, t0, cement="N"):
    alpha = {"S": -1, "N": 0, "R": 1}[cement]
    t0 = max(t0 * (9 / (2 + t0**1.2) + 1) ** alpha, 0.5)
    a1, a2, a3 = (35 / fcm) ** 0.7, (35 / fcm) ** 0.2, (35 / fcm) ** 0.5
    phirh = (1 + (1 - rh / 100) / (0.1 * h0 ** (1 / 3)) * a1) * a2 if fcm > 35 else 1 + (1 - rh / 100) / (0.1 * h0 ** (1 / 3))
    return phirh * 16.8 / math.sqrt(fcm) / (0.1 + t0**0.2)


def a01(case):
    n = case["num"]
    return {"phi_inf": creep(n["fck"] + 8, n["rh"], n["h0"], n["t0"], case["text"].get("cement", "N"))}


def a02(case):
    n = case["num"]
    cem = case["text"].get("cement", "N")
    ads1, ads2 = {"S": (3, 0.13), "N": (4, 0.12), "R": (6, 0.11)}[cem]
    fcm = n["fck"] + 8
    ecd0 = 0.85 * (220 + 110 * ads1) * math.exp(-ads2 * fcm / 10) * 1e-6 * 1.55 * (1 - (n["rh"] / 100) ** 3)
    kh = float(np.interp(n["h0"], [100, 200, 300, 500], [1.0, 0.85, 0.75, 0.70]))
    return {"eps_cs_inf": kh * ecd0 + 2.5 * (n["fck"] - 10) * 1e-6}


def fbd(annex, fck, phi, good=True):
    c = conc(min(fck, 60))
    fctd = alpha_cc(annex) * c["fctk05"] / gamma_c(annex)
    return 2.25 * (1.0 if good else 0.7) * (1.0 if phi <= 32 else (132 - phi) / 100) * fctd


def b02(case):
    n = case["num"]
    return {"lb_rqd": n["phi"] / 4 * fyd(n.get("fyk", 500.0)) / fbd(case["annex"], n["fck"], n["phi"]) * MM}


def v02(case):
    n = case["num"]
    annex = case["annex"]
    k = min(2.0, 1 + math.sqrt(200 / n["d"]))
    rho = min(0.02, n["asl"] / (n["bw"] * n["d"]))
    crd = ndp(annex, 0.18, 0.15) / 1.5
    vmin = 0.035 * k**1.5 * math.sqrt(n["fck"])
    return {"v_rd_c": max(crd * k * (100 * rho * n["fck"]) ** (1 / 3), vmin) * n["bw"] * n["d"]}


def v03(case):
    n = case["num"]
    annex = case["annex"]
    fck, bw, z, ved = n["fck"], n["bw"], n["z"], n["v_ed"]
    f = fcd(annex, fck)
    nu = 0.75 * min(1.1 - fck / 500, 1.0) if annex == "De" else 0.6 * (1 - fck / 250)
    vcc = 0.5 * 0.48 * fck ** (1 / 3) * bw * z
    cmax = min(3.0, 1.2 / (1 - vcc / ved)) if annex == "De" and vcc < ved else (3.0 if annex == "De" else 2.5)
    vmax = lambda cot: bw * z * nu * f / (cot + 1 / cot)
    res = optimize.minimize_scalar(lambda cot: -cot, bounds=(1.0, cmax), method="bounded", options={"xatol": 1e-12})
    cot = cmax if vmax(cmax) >= ved else optimize.brentq(lambda c: vmax(c) - ved, 1.0, cmax)
    return {"cot_theta": cot, "v_rd_max": vmax(cot), "asw_req": ved / (z * fyd(500.0) * cot)}


def p03(case):
    n = case["num"]
    annex = case["annex"]
    d, c1, ved = n["d"], n["c1"], n["v_ed"]
    u1 = 4 * c1 + 2 * math.pi * 2 * d
    k = min(2.0, 1 + math.sqrt(200 / d))
    rho = min(math.sqrt(n["rho_ly"] * n["rho_lz"]), 0.02)
    crd = max(0.18 * (0.1 * 4 * c1 / d + 0.6), 0.15) if annex == "De" and 4 * c1 / d < 4 else 0.18
    vrdc = max(crd / 1.5 * k * (100 * rho * n["fck"]) ** (1 / 3), 0.035 * k**1.5 * math.sqrt(n["fck"]))
    beta = 1.10 if annex == "De" else 1.15
    ved_s = beta * ved / (u1 * d)
    return {"v_ed": ved_s * 1e6, "util": ved_s / vrdc}


def sl06(case):
    n = case["num"]
    b, h, d, a, c, phi, m = n["b"], n["h"], n["d"], n["as"], n["c"], n["phi"], n["m_qp"]
    ae = n["alpha_e"]
    x = optimize.brentq(lambda x: b * x * x / 2 - ae * a * (d - x), 1e-6, h)
    i2 = b * x**3 / 3 + ae * a * (d - x) ** 2
    ss = ae * m * (d - x) / i2
    hc = min(2.5 * (h - d), (h - x) / 3, h / 2)
    rho = a / (b * hc)
    fct = conc(n["fck"])["fctm"]
    eps = max((ss - 0.4 * fct / rho * (1 + ae * rho)) / 200000.0, 0.6 * ss / 200000.0)
    sr = 3.4 * c + 0.8 * 0.5 * 0.425 * phi / rho
    return {"x": x * MM, "sr_max": sr * MM, "eps_diff": eps}


def torsion_geometry(n):
    """🌀 Thin-walled section of 6.3.1 built with shapely: t_ef = max(A/u, 2 (c + phi_l/2)) and the wall centre line as the inward mitre buffer by t_ef/2 (mm)."""
    outline = box(0.0, 0.0, n["b"], n["h"])
    t = max(outline.area / outline.length, 2.0 * (n.get("c", 0.0) + n.get("phi_l", 0.0) / 2.0))
    centre = outline.buffer(-t / 2.0, join_style=2)
    return t, centre.area, centre.exterior.length


def strut_nu(annex, fck):
    return FACTORS.ndp("ndp.en1992.nu1-factor", annex) * min(1.1 - fck / 500, 1.0) if annex == "De" else 0.6 * (1 - fck / 250)


def fctd(annex, fck):
    return FACTORS.ndp("ndp.en1992.m-01.alpha-ct", annex) * conc(fck)["fctk05"] / gamma_c(annex)


def torsion_limits(case):
    n, annex = case["num"], case["annex"]
    t, ak, uk = torsion_geometry(n)
    f = fcd(annex, n["fck"])
    theta = math.atan(1.0 / n.get("cot_theta", 1.0))
    t_max = 2.0 * strut_nu(annex, n["fck"]) * f * ak * t * math.sin(theta) * math.cos(theta)
    return t, ak, uk, f, t_max, 2.0 * ak * t * fctd(annex, n["fck"])


def t02(case):
    n, annex = case["num"], case["annex"]
    t, ak, uk, f, t_max, t_c = torsion_limits(case)
    cot, ted, fy = n.get("cot_theta", 1.0), n["t_ed"], fyd(n.get("fyk", 500.0))
    asw_s, asl = ted / (2.0 * ak * fy * cot), ted * uk * cot / (2.0 * ak * fy)
    values = {"util": ted / t_max, "asw_s": asw_s, "asl": asl * 1e-6, "t_rd_c": t_c * 1e-3}
    if "asw_s_provided" in n:
        values["util_links"] = asw_s / n["asw_s_provided"]
    share = min(max(n.get("tension_share", 1.0), 0.0), 1.0)
    if "asl_provided" in n:
        values["util_long"] = (n.get("asl_bending", 0.0) + share * asl) / n["asl_provided"]
    if "asl_compression_provided" in n:
        values["util_long_comp"] = (1.0 - share) * asl / n["asl_compression_provided"]
    return values


def t04(case):
    n, annex = case["num"], case["annex"]
    t, ak, uk, f, t_max, t_c = torsion_limits(case)
    fck, bw, d, asl, ted, ved, cot = n["fck"], n["bw"], n["d"], max(n.get("asl", 0.0), 1e-9), n["t_ed"], n["v_ed"], n.get("cot_theta", 1.0)
    k = min(2.0, 1 + math.sqrt(200 / d))
    rho = min(0.02, asl / (bw * d))
    crd = ndp(annex, 0.18, 0.15) / 1.5
    v_c = max(crd * k * (100 * rho * fck) ** (1 / 3), 0.035 * k**1.5 * math.sqrt(fck)) * bw * d
    v_max = bw * n.get("z", 0.9 * d) * strut_nu(annex, fck) * f / (cot + 1 / cot)
    return {"util_min": ted / t_c + ved / v_c, "util_max": ted / t_max + ved / v_max, "t_rd_max": t_max * 1e-3, "t_rd_c": t_c * 1e-3, "v_rd_c": v_c, "v_rd_max": v_max}


def a10(case):
    n, annex = case["num"], case["annex"]
    fck, b, h, ned, l0, a, i_s = n["fck"], n["b"], n["h"], n["n_ed"], n["l0"], n["as"], n["is"]
    f, fy, es, phi = fcd(annex, fck), fyd(n.get("fyk", 500.0)), n.get("es", 200000.0), n.get("phi_ef", 0.0)
    ac, d = b * h, h / 2.0 + i_s
    omega, nn = a * fy / (ac * f), ned / (ac * f)
    kr = min(1.0, max(0.0, (1.0 + omega - nn) / (1.0 + omega - 0.4)))
    lam = l0 / (min(h, b) / math.sqrt(12.0))
    kphi = max(1.0, 1.0 + (0.35 + fck / 200.0 - lam / 150.0) * phi)
    inv_r = kr * kphi * (fy / es) / (0.45 * d)
    e2 = inv_r * l0**2 / n.get("c", math.pi**2)
    e_tot = max(h / 30.0, 20.0) + n.get("ei", l0 / 400.0) + e2
    return {"e2": e2 * MM, "inv_r": inv_r, "m_ed": ned * e_tot * 1e-3}


def sl07(case):
    n = case["num"]
    fck = n["fck"]
    rho = n["as"] / (n["b"] * n["d"])
    s = math.sqrt(fck)
    rho0 = s * 1e-3
    v = 11 + 1.5 * s * rho0 / rho + 3.2 * s * (rho0 / rho - 1) ** 1.5 if rho <= rho0 else 11 + 1.5 * s * rho0 / rho
    return {"ld_limit": min(v, 35.0) if case["annex"] == "De" else v}


def fa03(case):
    n, l = case["num"], case["list"]
    hef, fck = n["hef"], n["fck"]
    ccr = 1.5 * hef
    xs, ys = l.get("ax", [0.0]), l.get("ay", [0.0])
    region = box(n.get("x_min", -1e9), n.get("y_min", -1e9), n.get("x_max", 1e9), n.get("y_max", 1e9))
    cone = unary_union([box(x - ccr, y - ccr, x + ccr, y + ccr) for x, y in zip(xs, ys)]).intersection(region)
    edge = min(min(x - region.bounds[0], region.bounds[2] - x, y - region.bounds[1], region.bounds[3] - y) for x, y in zip(xs, ys))
    k1 = 8.9 if n.get("cracked", 1.0) > 0 else 12.7
    psi_s = min(1.0, 0.7 + 0.3 * edge / ccr)
    psi_re = min(1.0, 0.5 + hef / 200)
    return {"n_rk_c": k1 * math.sqrt(fck) * hef**1.5 * cone.area / (2 * ccr) ** 2 * psi_s * psi_re}


def pa01(case):
    n = case["num"]
    f = fcd(case["annex"], n["fck"])
    return {"f_rdu": min(n["ac0"] * f * math.sqrt(n["ac1"] / n["ac0"]), 3 * f * n["ac0"])}


def pr04(case):
    n, l = case["num"], case["list"]
    return {"p_0": n["pmax"] * math.exp(-n["mu"] * (l["theta"][0] + n["k"] * l["x"][0]))}


def f04(case):
    n, l = case["num"], case["list"]
    rd = 162.5 / 1.15
    if "ranges" in l:
        dmg = sum(c / (1e6 * (rd / s) ** 5) for s, c in zip(l["ranges"], l["cycles"]))
        return {"damage": dmg}
    return {"util_equ": n["delta_sigma_equ"] / rd}


def st01(case):
    n, t, annex = case["num"], case["text"], case["annex"]
    lim = fcd(annex, n["fck"])
    if t.get("kind", "free") == "cracked":
        lim *= FACTORS.ndp("ndp.en1992.strut-cracked", annex) * (1 - n["fck"] / 250)
    values = {"sigma_rd_max": lim * 1e6}
    if "strut_force" in n:
        values["util"] = n["strut_force"] / (n["b"] * n["w"]) / lim
    return values


def st03(case):
    n, t, annex = case["num"], case["text"], case["annex"]
    key = {"CCT": "ndp.en1992.node-k2", "CTT": "ndp.en1992.node-k3"}.get(t.get("node", "CCC"), "ndp.en1992.node-k1")
    lim = FACTORS.ndp(key, annex) * (1 - n["fck"] / 250) * fcd(annex, n["fck"]) * min(max(n.get("confinement", 1.0), 1.0), 3.0)
    values = {"sigma_rd_max": lim * 1e6}
    if "sigma_c" in n:
        values["util"] = n["sigma_c"] / lim
    return values


def st04(case):
    l = case["list"]
    x, y = np.array(l["node_x"]), np.array(l["node_y"])
    mi, mj = [int(v) for v in l["member_i"]], [int(v) for v in l["member_j"]]
    nn, m = len(x), len(mi)
    sup = list(zip([int(v) for v in l["support_node"]], [int(v) for v in l["support_dof"]]))
    a = np.zeros((2 * nn, m + len(sup)))
    for k, (i, j) in enumerate(zip(mi, mj)):
        d = np.array([x[j] - x[i], y[j] - y[i]])
        d /= np.linalg.norm(d)
        a[2 * i : 2 * i + 2, k] += d
        a[2 * j : 2 * j + 2, k] -= d
    for k, (nd, dof) in enumerate(sup):
        a[2 * nd + dof, m + k] = 1
    p = np.zeros(2 * nn)
    p[1::2] = l.get("load_y", [0.0] * nn)
    sol = np.linalg.lstsq(a, -p, rcond=None)[0]
    return {f"force_{k}": sol[k] for k in range(m)}


def plane(sx, sy, txy, cot):
    t = abs(txy)
    th = math.atan(1.0 / cot)
    c, sn = math.cos(th), math.sin(th)
    rx, ry, sc = np.linalg.solve(np.array([[1.0, 0.0, -c * c], [0.0, 1.0, -sn * sn], [0.0, 0.0, sn * c]]), np.array([sx, sy, t]))
    if rx < 0 or ry < 0:
        fix_x = rx < 0
        s_n, s_t = (sx, sy) if fix_x else (sy, sx)
        normal, other_axis = (math.cos, math.sin) if fix_x else (math.sin, math.cos)
        sc, ang = optimize.fsolve(lambda v: [s_n + v[0] * normal(v[1]) ** 2, t - v[0] * math.sin(v[1]) * math.cos(v[1])], [abs(s_n) + t, 0.7])
        other = s_t + sc * other_axis(ang) ** 2
        rx, ry = (0.0, max(other, 0.0)) if fix_x else (max(other, 0.0), 0.0)
    return rx, ry, sc


def af01(case):
    n = case["num"]
    fy = fyd(n.get("fyk", 500.0))
    rx, ry, sc = plane(n["sx"], n["sy"], n["txy"], n.get("cot_theta", 1.0))
    return {"as_x": rx * n["t"] / fy, "as_y": ry * n["t"] / fy, "sigma_c": sc * 1e6}


def aj03(case):
    n = case["num"]
    ft = n["f_ed"] * n["a_c"] / n["z0"] + n.get("h_ed", 0.0) * (1 + n.get("z_h", 0.0) / n["z0"])
    return {"f_td": ft, "as_main": ft / fyd(n.get("fyk", 500.0)) * 1e-6, "a_links": 0.25 * ft / fyd(n.get("fyk", 500.0)) * 1e-6, "c_strut": n["f_ed"] * math.hypot(n["a_c"], n["z0"]) / n["z0"]}


def t05(case):
    n = case["num"]
    mf = n["bimoment"] / n["h_s"]
    f_cd = fcd(case["annex"], n["fck"])
    mu = mf / (f_cd * n["t_f"] * n["d_f"] ** 2)
    u = optimize.brentq(lambda x: x * (1 - x / 2) - mu, 0.0, 1.0)
    return {"m_f": mf * 1e-3, "as_req": f_cd * n["t_f"] * n["d_f"] * u / fyd(n.get("fyk", 500.0)) * 1e-6}


def ak01(case):
    n = case["num"]
    ecm = 33000.0
    return {"e_adj": ecm / (1 + n.get("chi", 0.8) * n["phi"]) * 1e6, "xi": 1 - math.exp(-n["phi"])}


def al01(case):
    n = case["num"]
    z, t = n["h"] - 2 * n["a"], 2 * n["a"]
    fy = fyd(n.get("fyk", 500.0))
    out = {}
    for name, sg in (("bot", 1.0), ("top", -1.0)):
        lx, ly, lxy = n.get("nx", 0.0) / 2 + sg * n.get("mx", 0.0) / z, n.get("ny", 0.0) / 2 + sg * n.get("my", 0.0) / z, n.get("nxy", 0.0) / 2 + sg * n.get("mxy", 0.0) / z
        rx, ry, _ = plane(lx / t, ly / t, lxy / t, n.get("cot_theta", 1.0))
        out[f"as_x_{name}"] = rx * t / fy
        out[f"as_y_{name}"] = ry * t / fy
    return out


def s03(case):
    n = case["num"]
    lam, eta = (0.8, 1.0) if n["fck"] <= 50 else (0.8 - (n["fck"] - 50) / 400, 1.0 - (n["fck"] - 50) / 200)
    f, fy = fcd(case["annex"], n["fck"]), fyd(n.get("fyk", 500.0))
    x = optimize.brentq(lambda x: eta * f * n["b"] * lam * x * (n["d"] - lam * x / 2) - n["m_ed"], 1e-6, n["d"] / lam)
    return {"as1": eta * f * n["b"] * lam * x / fy * 1e-6}


def b03(case):
    n = case["num"]
    phi = n["phi"]
    lb = phi / 4 * fyd(n.get("fyk", 500.0)) / fbd(case["annex"], n["fck"], phi)
    a2 = min(max(1 - 0.15 * (n.get("cd", 3 * phi) - phi) / phi, 0.7), 1.0)
    return {"lbd": max(a2 * lb, 0.3 * lb, 10 * phi, 100.0) * MM}


def a15(case):
    n = case["num"]
    k, hb_max = (70.0, 3.5) if n.get("transient", 0.0) > 0.5 else (50.0, 2.5)
    hb = n["h"] / n["b"]
    return {"util_l": n["l0t"] / n["b"] / (k / hb ** (1 / 3)), "util_h": hb / hb_max}


def polygon_of(case):
    from shapely import Polygon

    l = case["list"]
    holes = [list(zip(l["hole_y"], l["hole_z"]))] if "hole_y" in l else []
    return Polygon(list(zip(l["poly_y"], l["poly_z"])), holes)


def sg01(case):
    from sectionproperties.analysis import Section
    from sectionproperties.pre import Geometry
    from shapely import LineString, Polygon, box

    poly, n, l = polygon_of(case), case["num"], case["list"]
    geometry = Geometry(poly)
    geometry.create_mesh(mesh_sizes=[max(poly.area / 4000.0, 1.0)])
    section = Section(geometry)
    section.calculate_geometric_properties()
    ixx, iyy, _ = section.get_ic()
    cz = section.get_c()[1]
    zmin, zmax = poly.bounds[1], poly.bounds[3]
    bars = [(z, a) for z, a in zip(l.get("bar_z", []), l.get("bar_a", [])) if z < cz]
    out = {"area": poly.area * 1e-6, "i_y": ixx, "i_z": iyy, "z_bot": (cz - zmin) * MM}
    if bars:
        zs = sum(z * a for z, a in bars) / sum(a for _, a in bars)
        lo, hi = min(zs, cz), max(zs, cz)
        width = lambda z: poly.intersection(LineString([(poly.bounds[0] - 1, z), (poly.bounds[2] + 1, z)])).length
        out.update({"b_w": min(width(lo + (hi - lo) * (i + 0.5) / 400) for i in range(400)) * MM, "a_s": sum(a for _, a in bars) * 1e-6, "d": (zmax - zs) * MM})
    out["b_t"] = poly.intersection(box(poly.bounds[0] - 1, zmin - 1, poly.bounds[2] + 1, cz)).area / (cz - zmin) * MM
    outer = Polygon(poly.exterior)
    wall = (poly.area / ((poly.exterior.length + sum(r.length for r in poly.interiors)) / 2)) if poly.interiors else float("inf")
    t = max(min(outer.area / outer.length, wall), 2 * n.get("c", 0.0) + n.get("phi_l", 0.0))
    inner = outer.buffer(-t / 2, join_style=2, mitre_limit=100.0)
    out.update({"t_ef": t * MM, "a_k": inner.area * 1e-6, "u_k": inner.exterior.length * MM})
    return out


def sg02(case):
    from concreteproperties import Concrete, ConcreteSection, SteelBar, add_bar
    from concreteproperties.pre import Geometry as ConcreteGeometry
    from concreteproperties.stress_strain_profile import ConcreteLinear, EurocodeParabolicUltimate, SteelElasticPlastic

    n, l = case["num"], case["list"]
    ecm = ECM[int(n["fck"])] / (1 + n.get("phi_inf", 0.0))
    concrete = Concrete(name="c", density=2.4e-6, stress_strain_profile=ConcreteLinear(elastic_modulus=ecm), colour="lightgrey", ultimate_stress_strain_profile=EurocodeParabolicUltimate(compressive_strength=fcd(case["annex"], n["fck"]), compressive_strain=0.002, ultimate_strain=0.0035, n=2.0), flexural_tensile_strength=conc(n["fck"])["fctm"])
    steel = SteelBar(name="s", density=7.85e-6, stress_strain_profile=SteelElasticPlastic(yield_strength=500.0 / 1.15, elastic_modulus=n.get("es", 200000.0), fracture_strain=0.05), colour="grey")
    geometry = ConcreteGeometry(polygon_of(case), concrete)
    for y, z, a in zip(l["bar_y"], l["bar_z"], l["bar_a"]):
        geometry = add_bar(geometry, area=a, material=steel, x=y, y=z)
    section = ConcreteSection(geometry)
    cracked = section.calculate_cracked_properties(theta=0)
    stress = section.calculate_cracked_stress(cracked_results=cracked, n=-n.get("n_ed", 0.0), m=n["m_ed"])
    steel_stress = [float(s) for s in stress.lumped_reinforcement_stresses]
    return {
        "x": cracked.d_nc * MM,
        "sigma_c": max(float(s.max()) for s in stress.concrete_stresses) * 1e6,
        "sigma_s": -min(steel_stress) * 1e6,
        "i_ii": cracked.e_ixx_c_cr / ecm,
    }


KC_TABLE = ([20, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200], [1.0, 1.0, 0.95, 0.85, 0.75, 0.60, 0.45, 0.30, 0.15, 0.08, 0.04, 0.01, 0.0])


def r04(case):
    n, zones = case["num"], case["list"]["zone_theta"]
    kc = lambda t: float(np.interp(t, *KC_TABLE))
    km = (1 - 0.2 / len(zones)) / len(zones) * sum(kc(t) for t in zones)
    return {"kc_m": km, "az": n["w"] * (1 - km / kc(n["theta_m"])) * MM}


def b07(case):
    n = case["num"]
    strand = case["text"].get("tendon", "strand") == "strand"
    age = n.get("t", 28.0)
    s = {"R": 0.20, "S": 0.38}.get(case["text"].get("cement", "N"), 0.25)
    fctd = alpha_cc(case["annex"]) * conc(n["fck"])["fctk05"] / gamma_c(case["annex"]) * math.exp(s * (1 - math.sqrt(28.0 / age))) ** (1.0 if age < 28 else 2 / 3)
    fbpt = (3.2 if strand else 2.7) * (1.0 if case["text"].get("bond", "good") == "good" else 0.7) * fctd
    lpt = (1.0 if case["text"].get("release", "gradual") == "gradual" else 1.25) * (0.19 if strand else 0.25) * n["phi"] * n["sigma_pm0"] / fbpt
    return {"lpt": lpt * MM, "lpt1": 0.8 * lpt * MM, "lpt2": 1.2 * lpt * MM}


def pr05(case):
    from blueprints.codes.eurocode.en_1992_1_1_2004.chapter_3_materials.formula_3_28 import Form3Dot28RatioLossOfPreStressClass1
    from blueprints.codes.eurocode.en_1992_1_1_2004.chapter_3_materials.formula_3_29 import Form3Dot29RatioLossOfPreStressClass2
    from blueprints.codes.eurocode.en_1992_1_1_2004.chapter_3_materials.formula_3_30 import Form3Dot30RatioLossOfPreStressClass3
    from blueprints.codes.eurocode.en_1992_1_1_2004.chapter_3_materials.sub_formula_3_28_29_30 import SubForm3Dot28And29And30Mu
    from blueprints.codes.eurocode.en_1992_1_1_2004.chapter_5_structural_analysis.formula_5_46 import Form5Dot46Part2TimeDependentStressLosses

    n = case["num"]
    relaxation = int(n.get("relax_class", 2))
    rho_1000 = n.get("rho1000", {1: 8.0, 2: 2.5, 3: 4.0}[relaxation])
    mu = float(SubForm3Dot28And29And30Mu(sigma_pi=n["sigma_pi"], f_pk=n["fpk"]))
    ratio = {1: Form3Dot28RatioLossOfPreStressClass1, 2: Form3Dot29RatioLossOfPreStressClass2, 3: Form3Dot30RatioLossOfPreStressClass3}[relaxation](rho_1000=rho_1000, mu=mu, t=n.get("t_hours", 500000.0))
    relax = float(ratio) * n["sigma_pi"]
    loss = Form5Dot46Part2TimeDependentStressLosses(a_p=n["ap"], epsilon_cs=abs(n["eps_cs"]), e_p=n.get("ep", 195000.0), e_cm=n["ecm"], delta_sigma_pr=relax, phi_t_t0=n["phi"], sigma_c_qp=n["sigma_c_qp"], a_c=n["ac"], i_c=n["ic"], z_cp=n["zcp"])
    return {"relax": relax * 1e6, "loss": float(loss) * 1e6}


def dt09(case):
    n = case["num"]
    phi = n.get("phi", 12.0 if case["annex"] == "De" else 8.0)
    return {"scl_max": min((12.0 if case["annex"] == "De" else 20.0) * phi, n.get("b", math.inf), 400.0) * MM}


def m02(case):
    n = case["num"]
    c = conc(n["fck"])
    return {"fcm": c["fcm"] * 1e6, "fctm": c["fctm"] * 1e6, "fctk05": c["fctk05"] * 1e6, "ecm": ECM.get(int(n["fck"]), 22000.0 * (c["fcm"] / 10) ** 0.3) * 1e6}


def m08(case):
    n, annex = case["num"], case["annex"]
    fy = n["fyk"] / FACTORS.ndp("ndp.en1992.gamma-s", annex)
    es, k, euk = n.get("es", 200000.0), n.get("k", 1.08), n.get("eps_uk", 50.0)
    eps_yd, eps_ud = fy / es * 1000, 0.9 * euk
    a = abs(n["eps"])
    s = es * a / 1000 if a <= eps_yd else (fy + (k - 1) * fy * (min(a, eps_ud) - eps_yd) / (eps_ud - eps_yd) if n.get("inclined", 1.0) > 0 else fy)
    return {"sigma_s": math.copysign(s, n["eps"]) * 1e6, "eps_yd": eps_yd, "eps_ud": eps_ud}


def b01(case):
    n = case["num"]
    return {"fbd": fbd(case["annex"], n["fck"], n["phi"], case["text"].get("bond", "good") == "good") * 1e6}


def a16(case):
    n, annex = case["num"], case["annex"]
    lim = min(FACTORS.ndp("ndp.en1992.p-k7", annex) * n["fpk"], FACTORS.ndp("ndp.en1992.p-k8", annex) * n["fp01k"])
    values = {"sigma_pm0_max": lim * 1e6, "pm0_max": n["ap"] * lim}
    if "pm0" in n:
        values["util"] = n["pm0"] / n["ap"] / lim
    return values


def dt14(case):
    n, t, annex = case["num"], case["text"], case["annex"]
    fy = n.get("fyk", 500.0) / FACTORS.ndp("ndp.en1992.gamma-s-acc", annex)
    q1, q2, internal, column = (FACTORS.ndp(f"ndp.en1992.tie-{key}", annex) for key in ("q1", "q2", "internal", "column"))
    force = {"peripheral": min(n["li"] * q1, q2), "internal": max(internal, 15.0) * n.get("width_m", 1.0), "column": column * n.get("wall_length_m", 1.0)}[t.get("tie", "peripheral")] * 1000.0
    values = {"f_tie": force, "as_req": force / fy * 1e-6}
    if "as_provided" in n:
        values["util"] = force / fy / n["as_provided"]
    return values


HANDLERS = {"T-02": t02, "T-04": t04, "A-10": a10, "S-03": s03, "B-03": b03, "A-15": a15, "S-11": sc_s11, "AF-01": af01, "AJ-03": aj03, "T-05": t05, "AK-01": ak01, "AL-01": al01, "M-05": m05, "S-04": s04, "S-06": s06, "A-01": a01, "A-02": a02, "M-11": a02, "B-02": b02, "V-02": v02, "V-03": v03, "P-03": p03, "SL-06": sl06, "SL-07": sl07, "FA-03": fa03, "PA-01": pa01, "PR-04": pr04, "F-04": f04, "ST-04": st04, "ST-01": st01, "ST-03": st03, "SG-01": sg01, "SG-02": sg02, "R-04": r04, "B-07": b07, "DT-09": dt09, "PR-05": pr05, "M-02": m02, "M-08": m08, "B-01": b01, "A-16": a16, "DT-14": dt14}


def load_clause_forms():
    """📐 Handlers of every `📐️clause-forms/<domain>/🐍️.py` module: independent third-party or clause-text oracles per catalogue entry."""
    found = {"M-10": a01}
    shared = Path(__file__).resolve().parent / "📐️clause-forms" / "🧰️shared" / "🐍️.py"
    shared_spec = importlib.util.spec_from_file_location("clause_forms_shared", shared)
    sys.modules["clause_forms_shared"] = importlib.util.module_from_spec(shared_spec)
    shared_spec.loader.exec_module(sys.modules["clause_forms_shared"])
    for index, path in enumerate(p for p in sorted((Path(__file__).resolve().parent / "📐️clause-forms").glob("*/🐍️.py")) if p.parent != shared.parent):
        spec = importlib.util.spec_from_file_location(f"clause_forms_{index}", path)
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        found.update(module.HANDLERS)
    return found


HANDLERS.update(load_clause_forms())
sys.modules["clause_forms_shared"].REGISTRY = HANDLERS


def catalogue_case(line):
    entry, _, *fields = line.rstrip("\n").split("|")
    case = {"id": entry, "annex": "De", "num": {}, "text": {}, "list": {}}
    for field in fields:
        if field.startswith("annex="):
            case["annex"] = field[len("annex="):]
        elif field[:2] in ("n:", "s:"):
            key, value = field[2:].split("=", 1)
            case["num" if field[0] == "n" else "text"][key] = float(value) if field[0] == "n" else value
    return case


def main():
    if "--catalogue" in sys.argv:
        for line in filter(str.strip, sys.stdin):
            case = catalogue_case(line)
            for key, value in HANDLERS[case["id"]](case).items():
                print("|".join([case["id"], "feature", key, repr(float(value))]))
        return
    cases = json.load(sys.stdin)["cases"]
    out = [{"id": c["id"], "values": {k: float(v) for k, v in HANDLERS[c["id"]](c).items()}} for c in cases]
    json.dump({"results": out}, sys.stdout)


if __name__ == "__main__":
    main()
