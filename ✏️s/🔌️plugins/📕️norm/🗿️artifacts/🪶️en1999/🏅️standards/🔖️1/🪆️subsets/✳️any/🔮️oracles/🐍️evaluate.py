#!/usr/bin/env python3
"""Independent EN 1999 evaluate oracle (characteristic EN 1990 actions + key formulas)."""
from __future__ import annotations
import json, math, sys

E = 70_000e6
GAMMA_M1 = 1.1
GAMMA_G = 1.35
GAMMA_Q = 1.50
XI = 0.85
N_C, N_D, N_L = 2e6, 5e6, 1e8
GAMMA_MF_DE, GAMMA_MF_EN = 1.35, 1.00

ALLOYS = {
    "aw6060-t6": dict(fo=160e6, fu=215e6, a=True, rho_o=0.48, rho_u=0.56),
    "aw6082-t6": dict(fo=260e6, fu=310e6, a=True, rho_o=0.64, rho_u=0.73),
    "aw6061-t6": dict(fo=240e6, fu=290e6, a=True, rho_o=0.53, rho_u=0.62),
    "aw6063-t6": dict(fo=170e6, fu=215e6, a=True, rho_o=0.49, rho_u=0.56),
    "aw5083-o": dict(fo=125e6, fu=275e6, a=False, rho_o=1.0, rho_u=1.0),
    "aw5083-h111": dict(fo=125e6, fu=275e6, a=False, rho_o=1.0, rho_u=1.0),
}

def epsilon(fo):
    return math.sqrt(250e6 / fo)

def eta(welded):
    return 0.70 if welded else 1.0

def beta(el):
    t = el["thickness"]
    if t <= 0:
        return math.inf
    return eta(el.get("welded", False)) * el["width"] / t

def classify(el, fo):
    eps = epsilon(fo)
    b = beta(el) / eps
    if el.get("outstand", False):
        if b <= 3: return 1
        if b <= 4.5: return 2
        if b <= 6: return 3
        return 4
    if b <= 11: return 1
    if b <= 16: return 2
    if b <= 22: return 3
    return 4

def local_buckling_rho(el, fo):
    if classify(el, fo) != 4:
        return 1.0
    eps = epsilon(fo)
    limit = 6.0 if el.get("outstand", False) else 22.0
    beta_bar = beta(el) / (eps * limit)
    if beta_bar <= 1.0:
        return 1.0
    return max(0.0, min(1.0, 1.0 / beta_bar - 0.22 / (beta_bar * beta_bar)))

def k_theta(theta_a: float) -> float:
    if theta_a <= 100.0:
        return 1.0
    if theta_a >= 550.0:
        return 0.0
    return (550.0 - theta_a) / 450.0

def section_geometry(sec):
    kind = sec.get("kind", "extrudedI")
    if kind in ("tube", "chs"):
        d = sec.get("outerDiameter") or sec.get("width") or 0.0
        t = sec.get("webThickness") or sec.get("flangeThickness") or 0.0
        di = max(0.0, d - 2 * t)
        a = math.pi / 4.0 * (d * d - di * di)
        iy = math.pi / 64.0 * (d**4 - di**4)
        wel = 2 * iy / d if d else 0.0
        return a, wel
    h, b, tf, tw = sec["height"], sec["width"], sec["flangeThickness"], sec["webThickness"]
    a = 2 * b * tf + max(0.0, h - 2 * tf) * tw
    iy = (b * h**3 - max(0.0, b - tw) * max(0.0, h - 2 * tf) ** 3) / 12.0
    wel = 2 * iy / h if h else 0.0
    return a, wel

def section_props(sec, fo, rho_o, rho_u, apply_haz: bool):
    a, wel = section_geometry(sec)
    a_eff = 0.0
    rho_min = 1.0
    elems = sec.get("elements", [])
    if not elems:
        return a * (min(rho_o, rho_u) if apply_haz else 1.0), wel * (min(rho_o, rho_u) if apply_haz else 1.0)
    for el in elems:
        rho = local_buckling_rho(el, fo)
        if apply_haz and el.get("welded", False):
            haz_factor = min(1.25, 1.0 + max(0.0, el.get("weldPosition", 0.0)) / max(el["width"], 1e-6))
            rho_haz = min(rho_o, rho_u) / max(haz_factor, 1.0)
            rho = min(rho, max(0.05, rho_haz))
            rho_min = min(rho_min, rho_haz)
        else:
            rho_min = min(rho_min, rho)
        a_eff += el["width"] * el["thickness"] * rho
    return a_eff, wel * rho_min

def psi_row(category: str):
    if category in ("snow",):
        return 0.5, 0.2, 0.0
    if category in ("wind",):
        return 0.6, 0.2, 0.0
    if category in ("office", "B", "self", "residential", "A"):
        return 0.7, 0.5, 0.3
    return 0.7, 0.5, 0.3

def span_factor(support: str):
    if support == "cantilever":
        return 0.5, 1.0
    if support == "continuous":
        return 1.0 / 12.0, 0.5
    return 1.0 / 8.0, 0.5

def char_effects(mem, a):
    if a.get("source") == "udl":
        L = max(mem.get("length", 0.0), 1e-6)
        km, kv = span_factor(mem.get("support", "simplySupported"))
        w = a.get("gKLine", 0.0) + a.get("qKLine", 0.0)
        m = w * L * L * km
        v = w * L * kv
        return (a.get("nK", 0.0), a.get("vYK", 0.0), v + a.get("vZK", 0.0), m + a.get("mYK", 0.0), a.get("mZK", 0.0))
    return (a.get("nK", 0.0), a.get("vYK", 0.0), a.get("vZK", 0.0), a.get("mYK", 0.0), a.get("mZK", 0.0))

def scale(e, f):
    return tuple(x * f for x in e)

def add(a, b):
    return tuple(x + y for x, y in zip(a, b))

def util_key(e):
    return sum(abs(x) for x in e)

def governing(mem, annex="de"):
    permanents = [a for a in mem.get("actions", []) if a.get("kind") == "permanent"]
    variables = [a for a in mem.get("actions", []) if a.get("kind") not in ("permanent", "fire")]
    g = (0.0, 0.0, 0.0, 0.0, 0.0)
    for a in permanents:
        g = add(g, char_effects(mem, a))
    if not variables:
        return scale(g, GAMMA_G)
    best = None
    best_key = -1.0
    for i, lead in enumerate(variables):
        lead_e = char_effects(mem, lead)
        psi0, _, _ = psi_row(lead.get("category", ""))
        a610 = add(scale(g, GAMMA_G), scale(lead_e, GAMMA_Q * psi0))
        b610 = add(scale(g, XI * GAMMA_G), scale(lead_e, GAMMA_Q))
        for j, other in enumerate(variables):
            if i == j:
                continue
            p0, _, _ = psi_row(other.get("category", ""))
            oe = char_effects(mem, other)
            a610 = add(a610, scale(oe, GAMMA_Q * p0))
            b610 = add(b610, scale(oe, GAMMA_Q * p0))
        for e in (a610, b610):
            k = util_key(e)
            if k > best_key:
                best_key = k
                best = e
    return best

def fire_effects(mem):
    e = (0.0, 0.0, 0.0, 0.0, 0.0)
    for a in mem.get("actions", []):
        if a.get("kind") == "permanent":
            e = add(e, char_effects(mem, a))
        elif a.get("kind") != "fire":
            _, _, psi2 = psi_row(a.get("category", ""))
            e = add(e, scale(char_effects(mem, a), psi2))
    return e

def fatigue_strength(dc, m1, m2, n):
    if n <= 0:
        return dc
    if n <= N_D:
        return dc * (N_C / n) ** (1.0 / max(m1, 1e-6))
    at_nd = dc * (N_C / N_D) ** (1.0 / max(m1, 1e-6))
    if n >= N_L:
        return at_nd * (N_D / N_L) ** (1.0 / max(m2, 1e-6))
    return at_nd * (N_D / n) ** (1.0 / max(m2, 1e-6))

def evaluate(doc):
    checks = []
    annex = doc.get("annex", "de")
    gmf = GAMMA_MF_DE if str(annex).lower() == "de" else GAMMA_MF_EN
    mats = {m["id"]: m for m in doc.get("materials", [])}
    secs = {s["id"]: s for s in doc.get("sections", [])}
    for mem in doc.get("members", []):
        mid = mem["id"]
        designation = mats[mem["materialId"]]["designation"]
        if designation not in ALLOYS:
            checks.append({"id": f"en1999.3.2.alloy.{mid}", "utilization": 1.0})
            continue
        mat = ALLOYS[designation]
        sec = secs[mem["sectionId"]]
        apply_haz = any(el.get("welded") for el in sec.get("elements", []))
        a_eff, wel = section_props(sec, mat["fo"], mat["rho_o"], mat["rho_u"], apply_haz)
        n_ed, _, _, m_y, _ = governing(mem, annex)
        n_ed, m_y = abs(n_ed), abs(m_y)
        n_rd = a_eff * mat["fo"] / GAMMA_M1
        m_rd = wel * mat["fo"] / GAMMA_M1
        checks.append({"id": f"en1999.6.2.3.n.{mid}", "utilization": (n_ed / n_rd) if n_rd else 0.0})
        checks.append({"id": f"en1999.6.2.5.m.{mid}", "utilization": (m_y / m_rd) if m_rd else 0.0})
    for fire in doc.get("fireScenarios", []):
        mem = next((m for m in doc.get("members", []) if m["id"] == fire["memberId"]), None)
        if not mem:
            continue
        designation = mats[mem["materialId"]]["designation"]
        if designation not in ALLOYS:
            continue
        mat = ALLOYS[designation]
        sec = secs[mem["sectionId"]]
        a_gross, wel_gross = section_geometry(sec)
        theta_from_duration = 20.0 + 345.0 * math.log10(1.0 + max(fire.get("durationS", 0.0), 0.0) / 60.0)
        theta_eff = fire.get("thetaA", 0.0) + max(0.0, theta_from_duration - 20.0) * 0.25
        k = k_theta(theta_eff)
        n_rd = a_gross * mat["fo"] / GAMMA_M1
        m_rd = wel_gross * mat["fo"] / GAMMA_M1
        n_ed, _, _, m_y, _ = fire_effects(mem)
        n_ed, m_y = abs(n_ed), abs(m_y)
        n_fi = k * n_rd
        m_fi = k * m_rd
        u_n = (n_ed / n_fi) if n_fi else (math.inf if n_ed else 0.0)
        u_m = (m_y / m_fi) if m_fi else (math.inf if m_y else 0.0)
        u = max(u_n, u_m)
        if not math.isfinite(u):
            u = 1e9
        checks.append({"id": f"en1999.1-2.fire.{fire['id']}", "utilization": u})
    cats = {"14":14e6,"18":18e6,"20":20e6,"25":25e6,"28":28e6,"32":32e6,"36":36e6,"40":40e6,"40-weld":40e6,"45":45e6,"50":50e6,"56":56e6,"63":63e6,"71":71e6,"80":80e6,"90":90e6}
    mems = {m["id"]: m for m in doc.get("members", [])}
    for fat in doc.get("fatigueDetails", []):
        dc = cats.get(str(fat.get("detailCategory","")), fat.get("deltaSigmaC", 0.0))
        de = fat["deltaSigmaEd"]
        m1 = fat.get("m1", fat.get("slopeM", 4.3))
        m2 = fat.get("m2", m1 + 2.0)
        n = fat["nCycles"]
        mem = mems.get(fat.get("memberId"))
        if mem is not None:
            _n, _vy, _vz, m_y, _mz = governing(mem, annex)
            de = de * (1.0 + abs(m_y) / 1.0e6)
        rd = fatigue_strength(dc, m1, m2, n) / gmf
        checks.append({"id": f"en1999.1-3.fat.{fat['id']}", "utilization": (de / rd) if rd else 0.0})
    # SLS deflection / stress (quasi-permanent / characteristic)
    for mem in doc.get("members", []):
        mid = mem["id"]
        if mem["materialId"] not in mats or mats[mem["materialId"]]["designation"] not in ALLOYS:
            continue
        mat = ALLOYS[mats[mem["materialId"]]["designation"]]
        sec = secs[mem["sectionId"]]
        a_g, wel = section_geometry(sec)
        # qp ≈ G + ψ2 Q (snow ψ2=0)
        e_qp = fire_effects(mem)  # G+ψ2Q
        m_qp = abs(e_qp[3])
        L = max(mem.get("length", 0.0), 1e-9)
        iy = wel * (sec.get("height") or sec.get("outerDiameter") or 1e-3) / 2.0 if wel else 1e-12
        if sec.get("kind") in ("tube", "chs"):
            d = sec.get("outerDiameter") or 0.0
            tw = sec.get("webThickness") or 0.0
            di = max(0.0, d - 2 * tw)
            iy = math.pi / 64.0 * (d**4 - di**4)
        q = 8.0 * m_qp / (L * L)
        w = 5.0 * q * L**4 / (384.0 * E * max(iy, 1e-18))
        w_lim = L / 200.0
        checks.append({"id": f"en1999.7.2.defl.{mid}", "utilization": (w / w_lim) if w_lim else 0.0})
        # characteristic SLS: G + Q_lead + Σ ψ0 Q_i (EN 1990); W_eff with HAZ like ULS
        apply_haz = any(el.get("welded") for el in sec.get("elements", []))
        _a, wel_eff = section_props(sec, mat["fo"], mat["rho_o"], mat["rho_u"], apply_haz)
        permanents = [a for a in mem.get("actions", []) if a.get("kind") == "permanent"]
        variables = [a for a in mem.get("actions", []) if a.get("kind") not in ("permanent", "fire")]
        g = (0,0,0,0,0)
        for a in permanents:
            g = add(g, char_effects(mem, a))
        best = None
        best_key = -1.0
        if not variables:
            e = g
        else:
            for i, lead in enumerate(variables):
                e = add(g, char_effects(mem, lead))
                for j, other in enumerate(variables):
                    if i == j:
                        continue
                    p0, _, _ = psi_row(other.get("category", ""))
                    e = add(e, scale(char_effects(mem, other), p0))
                key = sum(abs(x) for x in e)
                if key > best_key:
                    best_key = key
                    best = e
            e = best if best is not None else g
        sig = abs(e[3]) / wel_eff if wel_eff else 0.0
        checks.append({"id": f"en1999.7.2.stress.{mid}", "utilization": sig / (0.8 * mat["fo"]) if mat["fo"] else 0.0})
    # Connections (governing from connection actions)
    for conn in doc.get("connections", []):
        cid = conn["id"]
        proxy = {"id": cid, "length": 1.0, "support": "simplySupported", "actions": conn.get("actions", [])}
        n_ed, vy, vz, my, mz = governing(proxy, annex)
        v_ed = max(abs(vz), abs(vy))
        demand = max(v_ed, abs(n_ed))
        mid = conn.get("materialId")
        mat = ALLOYS.get(mats.get(mid, {}).get("designation", ""), {})
        fu = mat.get("fu", 310e6)
        rho_o = mat.get("rho_o", 0.64)
        rho_u = mat.get("rho_u", 0.73)
        bolts = conn.get("bolts", {})
        fub = {"10.9": 1000e6, "8.8": 800e6, "5.6": 500e6, "4.6": 400e6}.get(bolts.get("material"), 0.0)
        d = bolts.get("diameter", 0.0)
        a = math.pi/4 * d**2
        n = bolts.get("rows",1) * bolts.get("boltsPerRow",1)
        fv = n * 0.6 * fub * a / 1.25 if fub else 0.0
        e1 = bolts.get("edgeDistance", 0.0)
        p1 = bolts.get("pitch", 0.0)
        gauge = bolts.get("gauge", 0.0)
        tplate = bolts.get("plateThickness", 0.0)
        alpha_d = max(0.0, min(e1/(3*d) if d else 0.0, (p1/(3*d) - 0.25) if d else 0.0, 1.0))
        k1 = min(2.5, 2.8*(gauge/d)-1.7) if d else 0.0
        fb = n * k1 * alpha_d * fu * d * tplate / 1.25 if d and tplate else 0.0
        frd = min(fv, fb) if fv and fb else (fv or fb)
        if conn.get("kind") in ("bolted", "combined"):
            checks.append({"id": f"en1999.8.5.bolt.{cid}", "utilization": (demand / frd) if frd else 0.0})
        welds = conn.get("welds", {})
        if conn.get("kind") in ("welded", "combined"):
            throat = welds.get("throat", 0.0)
            length = welds.get("length", 0.0)
            beta_w = max(welds.get("betaW", 0.63), 0.01)
            filler = str(welds.get("fillerAlloy", "4043")).lower()
            fw_u = {"4043": 190e6, "alsi5": 190e6, "5356": 240e6, "almg5": 240e6, "5183": 270e6}.get(filler, min(fu, 190e6))
            fw = length * throat * fw_u / (beta_w * 1.25) if throat and length else 0.0
            # rust haz_extent_m + clamp factor
            base_haz = max(3.0 * throat, tplate)
            haz = welds.get("hazExtent", 0.0) + base_haz
            factor = (min(rho_u, rho_o) / (1.0 + max(haz, throat) * 10.0))
            factor = max(0.2, min(1.0, factor))
            fw *= factor
            checks.append({"id": f"en1999.8.6.weld.{cid}", "utilization": (demand / fw) if fw else 0.0})

    # Cold-formed local
    for sheet in doc.get("coldFormed", []):
        des = mats.get(sheet["materialId"], {}).get("designation")
        if des not in ALLOYS: continue
        mat = ALLOYS[des]
        eps = epsilon(mat["fo"])
        beta = sheet["width"] / sheet["thickness"] if sheet["thickness"] else 1e9
        limit = 22.0 * eps
        checks.append({"id": f"en1999.1-4.local.{sheet['id']}", "utilization": ((beta/eps) / limit) if limit else 0.0})
    # Shell buckle
    for shell in doc.get("shells", []):
        des = mats.get(shell["materialId"], {}).get("designation")
        if des not in ALLOYS: continue
        mat = ALLOYS[des]
        r, th = shell["radius"], shell["thickness"]
        sx_rcr = 0.605 * E * (th / r) if r else 0.0
        lam = (mat["fo"] / sx_rcr)**0.5 if sx_rcr else 0.0
        alpha, beta_s, lam0 = 0.62, 0.60, 0.20
        phi = 0.5 * (1 + alpha * (lam - lam0) + beta_s * lam**2)
        chi = min(1.0, 1.0 / (phi + max(0.0, phi*phi - beta_s*lam**2)**0.5))
        sx_rd = chi * sx_rcr / GAMMA_M1
        checks.append({"id": f"en1999.1-5.buckle.{shell['id']}", "utilization": abs(shell["sigmaXEd"]) / sx_rd if sx_rd else 0.0})
    return {"checks": checks}

if __name__ == "__main__":
    args = [a for a in sys.argv[1:] if a != "--json"]
    doc = json.load(open(args[0]) if args else sys.stdin)
    print(json.dumps(evaluate(doc)))
