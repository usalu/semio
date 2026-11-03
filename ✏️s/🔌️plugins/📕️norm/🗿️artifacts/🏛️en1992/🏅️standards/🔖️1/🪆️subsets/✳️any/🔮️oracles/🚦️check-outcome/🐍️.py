"""🚦️ Independent EN 1992 document oracle: a committed snapshot (JSON on stdin or a path) to the utilisation of flexure, shear, crack width, elastic steel and concrete stresses (7.2), span-to-depth, deflection and the stress limits and the decompression (compression positive) of prestressed members (5.10.2, 7.3.1, tendon resolved from its control points) per member (`--json`, default: `{"checks":[{"id","utilization"}]}`; `--self-check`: the formulas against structuralcodes and concreteproperties). Written from EN 1990 6.10, EN 1992-1-1 3.1, 6.1, 6.2.2, 6.2.3, 7.3.4, 7.4.2, 7.4.3, Annex B and DIN EN 1992-1-1/NA by strain compatibility with horizontal strip integration (shapely, numpy), not from the Rust. Units inside: N, mm, MPa; the snapshot is SI and N is positive in tension. Section moments are about the gross centroid."""

import importlib.util
import json
import math
import sys
from pathlib import Path

import numpy as np
from shapely.geometry import Polygon, box

_FACTORS_PATH = next(parent / "⚖️compliance" / "📜️annex-and-tables" / "⚖️action-factors" / "🐍️.py" for parent in Path(__file__).resolve().parents if (parent / "⚖️compliance" / "📜️annex-and-tables" / "⚖️action-factors" / "🐍️.py").is_file())
_SPEC = importlib.util.spec_from_file_location("action_factors", _FACTORS_PATH)
FACTORS = importlib.util.module_from_spec(_SPEC)
_SPEC.loader.exec_module(FACTORS)


_FAMILIES_PATH = Path(__file__).resolve().parent / "🧭️families" / "🐍️.py"
_FAMILIES_SPEC = importlib.util.spec_from_file_location("en1992_families", _FAMILIES_PATH)
FAMILIES = importlib.util.module_from_spec(_FAMILIES_SPEC)
_FAMILIES_SPEC.loader.exec_module(FAMILIES)


_KERNEL_PATH = next(parent / "🧮️mechanics" / "🔮️oracles" / "🔩️fastening" / "🐍️.py" for parent in Path(__file__).resolve().parents if (parent / "🧮️mechanics" / "🔮️oracles" / "🔩️fastening" / "🐍️.py").is_file())
_KERNEL_SPEC = importlib.util.spec_from_file_location("fastening_twin", _KERNEL_PATH)
KERNEL = importlib.util.module_from_spec(_KERNEL_SPEC)
_KERNEL_SPEC.loader.exec_module(KERNEL)


def material(annex):
    """🧱 α_cc, γ_c and γ_s (persistent and accidental) of the annex from the registry rows the Rust reads."""
    n = lambda row: FACTORS.ndp(row, annex)
    return dict(annex=annex, node_k1=n("ndp.en1992.node-k1"), node_k2=n("ndp.en1992.node-k2"), strut_cracked=n("ndp.en1992.strut-cracked"), alpha_cc=n("ndp.en1992.m-01.alpha-cc-persistent"), gamma_c=n("ndp.en1992.m-01.gamma-c-persistent"), gamma_s=n("ndp.en1992.gamma-s"), alpha_cc_acc=n("ndp.en1992.m-01.alpha-cc-accidental"), gamma_c_acc=n("ndp.en1992.m-01.gamma-c-accidental"), gamma_s_acc=n("ndp.en1992.gamma-s-acc"))


NDP = {
    "en": dict(material("en"), crd=0.18, k1=0.15, vmin=(0.035, 0.035), nu_de=None, alpha_cw=None, cot_max=2.5, wk=0.3, wk_xc1=0.4, ld_cap=None),
    "de": dict(material("de"), crd=0.15, k1=0.12, vmin=(0.0525, 0.0375), nu_de=0.75, alpha_cw=1.0, cot_max=3.0, wk=0.3, wk_xc1=0.4, ld_cap=35.0),
}
PSI_KEYS = {"office": "office", "residential": "residential", "congregation": "congregation", "shopping": "retail", "storage": "storage", "snow": "snow", "snowhigh": "snow_high", "wind": "wind", "temperature": "temperature", "accidental": ""}
K_SUPPORT = {"SimplySupported": 1.0, "Continuous": 1.3, "Fixed": 1.5, "Cantilever": 0.4}
TABLE_3_1 = {12: (27.0, 1.6), 16: (29.0, 1.9), 20: (30.0, 2.2), 25: (31.0, 2.6), 30: (33.0, 2.9), 35: (34.0, 3.2), 40: (35.0, 3.5), 45: (36.0, 3.8), 50: (37.0, 4.1), 55: (38.0, 4.2), 60: (39.0, 4.4), 70: (41.0, 4.6), 80: (42.0, 4.8), 90: (44.0, 5.0)}
FCTK_5_TABLE_3_1 = {12: 1.1, 16: 1.3, 20: 1.5, 25: 1.8, 30: 2.0, 35: 2.2, 40: 2.5, 45: 2.7, 50: 2.9, 55: 3.0, 60: 3.1}
STRIPS = 1500


def psi(category, factors):
    """🎚️ The (ψ0, ψ1, ψ2) of the registry rows `ndp.en1990.psi.*` for an action category; an unknown category follows offices."""
    return factors.psi(PSI_KEYS.get(str(category).lower().replace("_", ""), "office"))


def concrete(fck):
    """🧱 The Table 3.1 properties used by the oracle (MPa)."""
    fcm = fck + 8.0
    q = max((90.0 - fck) / 100.0, 0.0) ** 4
    hi = fck > 50.0
    ecm, fctm = TABLE_3_1.get(int(round(fck)), (22.0 * (fcm / 10.0) ** 0.3, 0.3 * fck ** (2 / 3) if not hi else 2.12 * math.log(1 + fcm / 10.0)))
    return dict(fck=fck, fcm=fcm, ecm=ecm * 1e3, fctm=fctm, eps_c2=2.0 + 0.085 * (fck - 50.0) ** 0.53 if hi else 2.0, eps_cu2=2.6 + 35.0 * q if hi else 3.5, n=1.4 + 23.4 * q if hi else 2.0)


def sigma_c(c, fcd, eps):
    """📈 Parabola-rectangle stress (Figure 3.3) for a strain in permille."""
    if eps <= 0.0 or eps > c["eps_cu2"] + 1e-12:
        return 0.0
    return fcd * (1.0 - (1.0 - eps / c["eps_c2"]) ** c["n"]) if eps < c["eps_c2"] else fcd


def sigma_s(fyd, es, k, eps_ud, eps):
    """🔩 Hardening reinforcement stress (Figure 3.8) for a strain in permille."""
    ey = fyd / es
    a = abs(eps) / 1000.0
    s = es * a if a <= ey else fyd + (k - 1.0) * fyd * (min(a, eps_ud / 1000.0) - ey) / (eps_ud / 1000.0 - ey)
    return math.copysign(s, eps)


class Section:
    """🧮 A polygon section with bars: strip integration for M_Rd and the cracked elastic state."""

    def __init__(self, polygon, bars, conc, fcd, fyd, es, k, eps_ud):
        self.polygon, self.bars, self.c, self.fcd, self.fyd, self.es, self.k, self.eps_ud = polygon, bars, conc, fcd, fyd, es, k, eps_ud
        self.xmin, self.zmin, self.xmax, self.zmax = polygon.bounds
        self.zc = polygon.centroid.y
        edges = np.linspace(self.zmin, self.zmax, STRIPS + 1)
        self.area_strip = np.array([polygon.intersection(box(self.xmin - 1, a, self.xmax + 1, b)).area for a, b in zip(edges[:-1], edges[1:])])
        self.z_strip = 0.5 * (edges[:-1] + edges[1:])

    @property
    def depth(self):
        return self.zmax - self.zmin

    def tension_bars(self):
        return [b for b in self.bars if b[1] < self.zc]

    def tension_area(self):
        return sum(b[2] for b in self.tension_bars())

    def effective_depth(self, fallback):
        a = self.tension_area()
        return self.zmax - sum(b[1] * b[2] for b in self.tension_bars()) / a if a > 0 else fallback

    def width_at(self, z):
        return self.polygon.intersection(box(self.xmin - 1, z - 1e-9, self.xmax + 1, z + 1e-9)).area / 2e-9

    def web_width(self):
        a = self.tension_area()
        zs = sum(b[1] * b[2] for b in self.tension_bars()) / a if a > 0 else self.zmin + 0.1 * self.depth
        lo, hi = min(zs, self.zc), max(zs, self.zc)
        eps = 1e-6 * self.depth
        cuts = sorted({p[1] for p in self.polygon.exterior.coords if lo < p[1] < hi} | {lo, hi})
        widths = [self.width_at(z) for u, v in zip(cuts[:-1], cuts[1:]) if v - u > 2 * eps for z in (u + eps, v - eps)]
        return min(widths) if widths else self.width_at(self.zc)

    def top_strain(self, x):
        if x <= self.depth:
            e = self.c["eps_cu2"]
            for _, z, _ in self.bars:
                u = self.zmax - z
                if u > x:
                    e = min(e, self.eps_ud * x / (u - x))
            return max(e, 1e-12)
        uc = self.depth * (1.0 - self.c["eps_c2"] / self.c["eps_cu2"])
        return self.c["eps_c2"] * x / (x - uc)

    def forces(self, x):
        etop = self.top_strain(x)
        strain = lambda u: etop * (x - u) / x
        u_mid = self.zmax - self.z_strip
        e = np.where(u_mid <= x, strain(u_mid), 0.0)
        sig = np.array([sigma_c(self.c, self.fcd, v) for v in e])
        n = float(np.sum(sig * self.area_strip))
        m = float(np.sum(sig * self.area_strip * self.z_strip))
        for _, z, a in self.bars:
            eb = strain(self.zmax - z)
            s = sigma_s(self.fyd, self.es, self.k, self.eps_ud, eb) - (sigma_c(self.c, self.fcd, eb) if eb > 0 else 0.0)
            n += s * a
            m += s * a * z
        return n, m - n * self.zc

    def m_rd(self, n_comp):
        lo, hi = 1e-3 * self.depth, 1e4 * self.depth
        n_lo, n_hi = self.forces(lo)[0], self.forces(hi)[0]
        if n_comp < n_lo - 1e-9 * abs(n_lo) - 1.0 or n_comp > n_hi + 1e-9 * abs(n_hi) + 1.0:
            return 0.0
        for _ in range(70):
            mid = math.sqrt(lo * hi)
            lo, hi = (mid, hi) if self.forces(mid)[0] < n_comp else (lo, mid)
        return self.forces(math.sqrt(lo * hi))[1]

    def second_moment(self):
        return float(np.sum(self.area_strip * (self.z_strip - self.zc) ** 2))

    def cracked(self, m, n_comp, ec):
        z_rel = self.z_strip - self.zc

        def resultants(e0, kappa):
            comp = np.maximum(e0 + kappa * z_rel, 0.0)
            n = float(np.sum(ec * comp * self.area_strip))
            mm = float(np.sum(ec * comp * self.area_strip * z_rel))
            for _, z, a in self.bars:
                f = self.es * (e0 + kappa * (z - self.zc)) * a
                n += f
                mm += f * (z - self.zc)
            return n, mm

        def balance(kappa):
            lo, hi = -0.5, 0.5
            if resultants(lo, kappa)[0] > n_comp or resultants(hi, kappa)[0] < n_comp:
                return None
            for _ in range(60):
                mid = 0.5 * (lo + hi)
                lo, hi = (mid, hi) if resultants(mid, kappa)[0] < n_comp else (lo, mid)
            return 0.5 * (lo + hi)

        lo, hi = -0.05 / self.depth, 0.05 / self.depth
        if balance(lo) is None or balance(hi) is None or not resultants(balance(lo), lo)[1] <= m <= resultants(balance(hi), hi)[1]:
            return None
        for _ in range(70):
            mid = 0.5 * (lo + hi)
            lo, hi = (mid, hi) if resultants(balance(mid), mid)[1] < m else (lo, mid)
        kappa = 0.5 * (lo + hi)
        e0 = balance(kappa)
        x = min(max(self.zmax - self.zc + e0 / kappa, 0.0), self.depth) if kappa > 1e-15 else self.depth
        return dict(x=x, sigma_s=max((-self.es * (e0 + kappa * (z - self.zc)) for _, z, _ in self.bars), default=0.0), sigma_c=ec * (e0 + kappa * (self.zmax - self.zc)), i2=m / (ec * kappa) if abs(kappa) > 1e-18 else 0.0)


def build_section(member, conc, reinf, fcd, fyd):
    """🏗️ The member section in mm: the outline or b x h about its centre and the bars of every layer."""
    b, h, d, cover = (member[k] * 1e3 for k in ("width", "height", "effectiveDepth", "cover"))
    points = member.get("sectionPoints") or []
    outline = Polygon([(p["y"] * 1e3, p["z"] * 1e3) for p in points]) if len(points) >= 3 else box(-b / 2, -h / 2, b / 2, h / 2)
    xmin, _, xmax, zmax = outline.bounds
    bars = []
    for layer in member["longitudinal"]:
        mine = [x for x in member.get("bars") or [] if x["layerId"] == layer["id"]]
        phi, n = layer["diameter"] * 1e3, max(layer["count"], 1)
        if mine:
            bars += [(x["y"] * 1e3, x["z"] * 1e3, math.pi * (x["diameter"] * 1e3) ** 2 / 4) for x in mine]
            continue
        z = zmax - (cover + phi / 2) if layer["position"] == "Top" else zmax - d
        inset = cover + phi / 2
        chord = outline.intersection(box(xmin - 1, z - 1e-9, xmax + 1, z + 1e-9)).bounds
        y0, y1 = (chord[0], chord[2]) if chord else (-b / 2, b / 2)
        lo, hi = (y0 + inset, y1 - inset) if y1 - y0 > 2 * inset else (0.5 * (y0 + y1),) * 2
        bars += [(0.5 * (lo + hi) if n == 1 else lo + (hi - lo) * i / (n - 1), z, math.pi * phi ** 2 / 4) for i in range(n)]
    return Section(outline, bars, conc, fcd, fyd, reinf["eS"] / 1e6, reinf["k"], 0.9 * reinf["epsUk"] * 1000.0)


def line_effects(support, span, q, p):
    """📐 Moment and shear of a line load q and a mid-span point load p (kN-free SI)."""
    l = max(span, 1e-6)
    if support == "SimplySupported":
        return q * l * l / 8 + p * l / 4, q * l / 2 + p / 2
    if support == "Cantilever":
        return q * l * l / 2 + p * l, q * l + p
    return q * l * l / 12 + p * l / 8, q * l / 2 + p / 2


def characteristic(member, a):
    """🏋️ The characteristic (M, N, V, T, V_punch) of one action in SI."""
    if a["source"] == "External" or member["useFem"]:
        return np.array([a["mK"], a["nK"], a["vK"], a["tK"], a["vKPunch"]])
    q = a["gKLine"] if a["kind"] in ("Permanent", "Prestress") else a["qKLine"]
    m, v = line_effects(member["support"], member["span"], q, abs(a["pointForce"]))
    return np.array([m, a["nK"], v, a["tK"], a["vKPunch"]])


def combinations(member, annex, reliability_class):
    """⚖️ EN 1990 6.10 (with the 6.10b reduction and favourable permanent actions), 6.11b and the quasi-permanent SLS effects of a member."""
    fac = FACTORS.ActionFactors(annex, reliability_class=int(str(reliability_class)[-1]))
    g, pre, var, acc = np.zeros(5), np.zeros(5), [], []
    for a in member["actions"]:
        e = characteristic(member, a)
        if a["kind"] == "Permanent":
            g = g + e
        elif a["kind"] == "Prestress":
            pre = pre + e
        elif a["kind"] == "Accidental":
            acc.append(e)
        else:
            var.append((a, e))
    if member["udl"] != 0.0 and not member["useFem"]:
        m, v = line_effects(member["support"], member["span"], member["udl"], 0.0)
        g = g + np.array([m, 0, v, 0, 0])
    gg, gi, gq, xi = fac.gamma_g_sup(), fac.gamma_g_inf(), fac.gamma_q(), fac.xi()
    uls = [gg * g + pre, gi * g + pre] if not var else []
    for i, (_, lead) in enumerate(var):
        others = sum((gq * psi(a["category"], fac)[0] * e for j, (a, e) in enumerate(var) if j != i), np.zeros(5))
        uls += [factor * g + pre + gq * lead + others for factor in (gg, xi * gg, gi)]
    lead_acc = max(range(len(var)), key=lambda j: abs(var[j][1][0])) if var else None
    accidental = [g + pre + e + sum(((psi(a["category"], fac)[1] if j == lead_acc else psi(a["category"], fac)[2]) * ev for j, (a, ev) in enumerate(var)), np.zeros(5)) for e in acc]
    char = [g + pre] if not var else [g + pre + lead + sum((psi(a["category"], fac)[0] * e for j, (a, e) in enumerate(var) if j != i), np.zeros(5)) for i, (_, lead) in enumerate(var)]
    return dict(uls=uls, accidental=accidental, char=char, qp=g + pre + sum((psi(a["category"], fac)[2] * e for a, e in var), np.zeros(5)))


def governing(effects, key=lambda e: abs(e[0])):
    """🎯 The effect with the largest key, ties to the last."""
    best = None
    for e in effects:
        if best is None or key(e) >= key(best):
            best = e
    return best


def shear_resistance_without_links(p, fck, bw, d, asl, sigma_cp):
    """✂️ V_Rd,c (Eq. 6.2a/b) in N for mm and MPa, with the DE/EN C_Rd,c, k1 and v_min."""
    k = min(1 + math.sqrt(200.0 / d), 2.0)
    rho = min(asl / (bw * d), 0.02)
    fcd = p["alpha_cc"] * fck / p["gamma_c"]
    scp = min(sigma_cp, 0.2 * fcd)
    t = min(max((d - 600.0) / 200.0, 0.0), 1.0)
    vmin = ((1 - t) * p["vmin"][0] + t * p["vmin"][1]) / (p["gamma_c"] if p["nu_de"] else 1.0) * k ** 1.5 * math.sqrt(fck)
    return max(p["crd"] / p["gamma_c"] * k * (100 * rho * fck) ** (1 / 3) + p["k1"] * scp, vmin + p["k1"] * scp) * bw * d


def shear_resistance_with_links(p, fck, fyk, bw, z, asw_s, sigma_cp, v_ed):
    """➰ V_Rd,s at the steepest strut inclination that still keeps V_Rd,s at most V_Rd,max (Eq. 6.8, 6.9) in N, with cot theta."""
    fcd = p["alpha_cc"] * fck / p["gamma_c"]
    r = sigma_cp / fcd
    acw = p["alpha_cw"] or (1.0 if r <= 0 else 1 + r if r <= 0.25 else 1.25 if r <= 0.5 else 2.5 * (1 - r))
    nu = p["nu_de"] * min(1.1 - fck / 500.0, 1.0) if p["nu_de"] else 0.6 * (1 - fck / 250)
    cmax = p["cot_max"]
    if p["nu_de"]:
        vcc = 0.5 * 0.48 * fck ** (1 / 3) * (1 - 1.2 * sigma_cp / fcd) * bw * z
        den = 1.0 - vcc / max(v_ed, 1.0)
        cmax = cmax if den <= 0 else min((1.2 + 1.4 * sigma_cp / fcd) / den, cmax)
    fywd = fyk / p["gamma_s"]
    vs = lambda cot: asw_s * z * fywd * cot
    vm = lambda cot: acw * bw * z * nu * fcd * cot / (1 + cot * cot)
    if vs(1.0) >= vm(1.0):
        return vm(1.0), 1.0
    if vs(cmax) <= vm(cmax):
        return vs(cmax), cmax
    lo, hi = 1.0, cmax
    for _ in range(80):
        mid = 0.5 * (lo + hi)
        lo, hi = (mid, hi) if vs(mid) < vm(mid) else (lo, mid)
    return vs(lo), lo


def limiting_span_to_depth(p, fck, rho, rho_prime, k):
    """📉 K [11 + 1.5 sqrt(fck) rho0/rho + ...] of Eq. 7.16a/b, capped by K x 35 under the DE annex."""
    s = math.sqrt(fck)
    rho0 = s * 1e-3
    basic = 11 + 1.5 * s * rho0 / rho + 3.2 * s * (rho0 / rho - 1) ** 1.5 if rho <= rho0 else 11 + 1.5 * s * rho0 / (rho - rho_prime) + s / 12 * math.sqrt(rho_prime / rho0)
    return min(k * basic, k * p["ld_cap"]) if p["ld_cap"] else k * basic


def creep_coefficient(fcm, rh, h0, t0, t):
    """⏳ φ(t, t0) of Annex B.1 for cement class N (t0 unadjusted)."""
    a1, a2, a3 = ((35.0 / fcm) ** e if fcm > 35 else 1.0 for e in (0.7, 0.2, 0.5))
    phi_rh = (1 + (1 - rh / 100) / (0.1 * h0 ** (1 / 3)) * a1) * a2
    beta_h = min(1.5 * (1 + (0.012 * rh) ** 18) * h0 + 250 * a3, 1500 * a3)
    return phi_rh * 16.8 / math.sqrt(fcm) / (0.1 + t0 ** 0.2) * ((t - t0) / (beta_h + t - t0)) ** 0.3


def elastic_line(support, span, line, point, s):
    """🌊 The deflection times EI at x = s L of a line load and a mid-span point load."""
    l, x = span, s * span
    h = min(x, l - x)
    if support == "SimplySupported":
        return line * x * (l ** 3 - 2 * l * x * x + x ** 3) / 24 + point * h * (3 * l * l - 4 * h * h) / 48
    if support == "Cantilever":
        a = l / 2
        return line * x * x * (6 * l * l - 4 * l * x + x * x) / 24 + (point * x * x * (3 * a - x) / 6 if x <= a else point * a * a * (3 * x - a) / 6)
    return line * (x * (l - x)) ** 2 / 24 + point * h * h * (3 * l - 4 * h) / 48


def flexure_and_shear(member, p, sec, fck, fyk, combos):
    """💪✂️ The ULS flexure, accidental flexure and shear utilisations of a member."""
    mid, out = member["id"], []
    uls = governing(combos["uls"])
    fcd = p["alpha_cc"] * fck / p["gamma_c"]
    if uls is not None:
        out.append((f"en1992.6.1.flexure.{mid}", abs(uls[0]) / max(sec.m_rd(-uls[1]) / 1e3, 1.0)))
        uv = governing(combos["uls"], lambda e: abs(e[2]))
        d = sec.effective_depth(member["effectiveDepth"] * 1e3)
        bw, area = sec.web_width(), sec.polygon.area
        asw_s = member["stirrups"]["legs"] * math.pi * member["stirrups"]["diameter"] ** 2 / 4 / member["stirrups"]["spacing"] if member.get("stirrups") and member["stirrups"]["spacing"] > 0 else 0.0
        sigma_cp = -uv[1] / area
        vrdc = shear_resistance_without_links(p, fck, bw, d, sec.tension_area(), sigma_cp)
        vcs, _ = shear_resistance_with_links(p, fck, fyk, bw, 0.9 * d, asw_s * 1e3, max(sigma_cp, 0.0), abs(uv[2]))
        v_ed = abs(uv[2])
        limit = (vrdc if v_ed <= vrdc else vcs) if asw_s > 0 else min(vrdc, 0.5 * bw * d * 0.6 * (1 - fck / 250) * fcd)
        out.append((f"en1992.6.2.shear.{mid}", uv[2] / max(limit, 1.0)))
    acc = governing(combos["accidental"])
    if acc is not None:
        acc_sec = Section(sec.polygon, sec.bars, sec.c, p["alpha_cc_acc"] * fck / p["gamma_c_acc"], fyk / p["gamma_s_acc"], sec.es, sec.k, sec.eps_ud)
        out.append((f"en1992.6.1.flexure.acc.{mid}", abs(acc[0]) / max(acc_sec.m_rd(-acc[1]) / 1e3, 1.0)))
    return out


def crack_and_deflection(member, p, sec, c, es, qp, fck):
    """🪓🌊 The quasi-permanent crack width (7.3.4) and deflection along the span (7.4.3) utilisations."""
    mid, out = member["id"], []
    h, n_comp, m_qp = sec.depth, -qp[1], abs(qp[0]) * 1e3
    state = sec.cracked(m_qp, n_comp, c["ecm"])
    w_max = p["wk_xc1"] if member["exposure"] in ("X0", "Xc1") else p["wk"]
    lower = [x for x in member["longitudinal"] if x["position"] != "Top"] or member["longitudinal"]
    phi = (lower[0]["diameter"] if lower else 0.016) * 1e3
    w_k = 0.0
    if sec.tension_area() <= 0:
        w_k = math.inf
    elif state is not None and state["x"] < h - 1e-6:
        d = sec.effective_depth(0.9 * h)
        hc = max(min(2.5 * (h - d), (h - state["x"]) / 3, h / 2), 1e-6)
        rho = sec.tension_area() / sec.polygon.intersection(box(sec.xmin - 1, sec.zmin, sec.xmax + 1, sec.zmin + hc)).area
        eps = max((state["sigma_s"] - 0.4 * c["fctm"] / rho * (1 + es / c["ecm"] * rho)) / es, 0.6 * state["sigma_s"] / es)
        w_k = (3.4 * member["cover"] * 1e3 + 0.8 * 0.5 * 0.425 * phi / rho) * eps
    out.append((f"en1992.7.3.crack.{mid}", w_k / w_max))
    factor = lambda a: 1.0 if a["kind"] in ("Permanent", "Prestress") else psi(a["category"], FACTORS.ActionFactors(p["annex"]))[2]
    live = [a for a in member["actions"] if a["kind"] != "Accidental"]
    line = member["udl"] + sum(factor(a) * (a["gKLine"] + a["qKLine"]) for a in live)
    point = sum(factor(a) * abs(a["pointForce"]) for a in live)
    derived = not member["useFem"] and all(a["source"] != "External" for a in member["actions"])
    if member["kind"] in ("Beam", "Slab", "FlatSlab", "RibbedSlab", "Bridge", "LiquidRetaining") and derived and member["span"] > 0 and line + point > 0:
        area = sec.polygon.area
        phi_c = creep_coefficient(fck + 8, 100 * min(max(member["relativeHumidity"], 0.2), 1.0), 2 * area / sec.polygon.exterior.length, 28, 18250)
        e_eff = c["ecm"] / (1 + phi_c)
        i1 = sec.second_moment()
        cracked = sec.cracked(m_qp, n_comp, e_eff)
        i2 = cracked["i2"] if cracked and cracked["x"] < h - 1e-6 else i1
        m_cr = (c["fctm"] + n_comp / area) * i1 / (sec.zc - sec.zmin)
        zeta = 1 - 0.5 * (m_cr / m_qp) ** 2 if m_qp > m_cr else 0.0
        ei = e_eff * 1e6 / ((1 - zeta) / (i1 * 1e-12) + zeta / max(i2 * 1e-12, 1e-18))
        span, support = member["span"], member["support"]
        out.append((f"en1992.7.4.deflection.{mid}", max(elastic_line(support, span, line, point, i / 10) for i in range(11)) / ei / ((2 * span if support == "Cantilever" else span) / 250)))
    return out


def elastic_stresses(member, sec, c, combos, fck, fyk):
    """🔩 The elastic steel and concrete stress of the characteristic SLS combination in state II (7.2(2), (5)) against 0.8 f_yk and 0.6 f_ck; a section that stays uncracked has no row."""
    governing_char = governing(combos["char"])
    state = sec.cracked(abs(governing_char[0]) * 1e3, -governing_char[1], c["ecm"]) if governing_char is not None and sec.bars else None
    if state is None or state["x"] >= sec.depth - 1e-6:
        return []
    return [(f"en1992.7.2.sigma-s.{member['id']}", state["sigma_s"] / (0.8 * fyk)), (f"en1992.7.2.sigma-c.{member['id']}", state["sigma_c"] / (0.6 * fck))]


def parabolic_through(points, x):
    """〰️ Parabolic interpolation through control points sorted by x: a line for two points, from three on each interval blends the parabola through the three points ending at its right end with the one starting at its left end by the position in the interval (Overhauser); passes through every point, exact on a parabola, x clamped to the range."""
    n = len(points)
    if n == 0:
        return 0.0
    if n == 1:
        return points[0][1]
    x = min(max(x, points[0][0]), points[-1][0])
    if n == 2:
        return float(np.interp(x, [points[0][0], points[1][0]], [points[0][1], points[1][1]]))
    i = max(next((k for k, q in enumerate(points) if q[0] >= x), n - 1), 1)

    def through(a):
        (x0, y0), (x1, y1), (x2, y2) = points[a], points[a + 1], points[a + 2]
        return y0 * (x - x1) * (x - x2) / ((x0 - x1) * (x0 - x2)) + y1 * (x - x0) * (x - x2) / ((x1 - x0) * (x1 - x2)) + y2 * (x - x0) * (x - x1) / ((x2 - x0) * (x2 - x1))

    t = (x - points[i - 1][0]) / (points[i][0] - points[i - 1][0])
    if i >= 2 and i + 1 < n:
        return (1 - t) * through(i - 2) + t * through(i - 1)
    return through(i - 2) if i >= 2 else through(i - 1)


def tendon_eccentricity(member, ps):
    """🧵 The eccentricity of the tendon at midspan, positive below the centroid: parabolic through the control points (a line for two, clamped at the ends), else the stored value."""
    points = sorted(((q["x"], q["z"]) for q in member.get("tendonPoints") or []), key=lambda q: q[0])
    if not points:
        return ps["eccentricity"]
    return -parabolic_through(points, member["span"] / 2)


def prestress_row(doc, member, sec, fck):
    """🧵 The stress limits of 5.10.2: initial and final tendon stress and the concrete stress at the tendon side as the worst of five ratios (all in MPa)."""
    ps, mid = member.get("prestress"), member["id"]
    steel = next((s for s in doc["prestressSteels"] if s["id"] == member["prestressSteelId"]), None)
    if not ps or steel is None:
        return []
    area, ecc = sec.polygon.area * 1e-6, tendon_eccentricity(member, ps)
    fpk, fp01k = steel["fPk"] / 1e6, steel["fP01k"] / 1e6
    sigma_0, sigma_inf = ps["force"] / max(ps["area"], 1e-12) / 1e6, ps["force"] * (1 - min(max(ps["lossRatio"], 0.0), 1.0)) / max(ps["area"], 1e-12) / 1e6
    lever = (sec.zc - sec.zmin) if ecc >= 0 else (sec.zmax - sec.zc)
    sigma_c = ps["force"] / area / 1e6 + ps["force"] * abs(ecc) * (lever * 1e-3) / (sec.second_moment() * 1e-12) / 1e6
    ratios = [sigma_0 / (0.75 * fpk), sigma_0 / (0.85 * fp01k), sigma_0 / min(0.8 * fpk, 0.9 * fp01k), sigma_c / (0.6 * fck), sigma_inf / min(0.75 * fpk, 0.85 * fp01k)]
    return [(f"en1992.5.10.prestress.{mid}", max(ratios))]


def decompression_stresses(force, loss, ecc, moment, sec):
    """🗜️ The fibre stresses (bottom, top; MPa, compression positive) of the section under the final prestress force (N), the tendon eccentricity (m, positive below the centroid) and the sagging quasi-permanent moment (N m): P/A + P e/W_bottom - M/W_bottom and P/A - P e/W_top + M/W_top (EN 1992-1-1 7.3.1, 5.10)."""
    p_inf, area, inertia = force * (1 - min(max(loss, 0.0), 1.0)), sec.polygon.area, sec.second_moment()
    w_bot, w_top = inertia / (sec.zc - sec.zmin), inertia / (sec.zmax - sec.zc)
    mean, e, m = p_inf / area, ecc * 1e3, moment * 1e3
    return mean + p_inf * e / w_bot - m / w_bot, mean - p_inf * e / w_top + m / w_top


def decompression_minimal_force(doc, member, sec):
    """🩹 The smallest initial prestressing force (N) that keeps both fibres compressive under the quasi-permanent moment (closed form of the decompression remedy): per fibre with a load tension P_inf >= tension / (1/A +- e/W) (infinite when the prestress alone tensions that fibre), divided by the retained share 1 - loss; an upper bound from a fibre the prestress alone tensions is not modelled."""
    ps = member["prestress"]
    qp, inertia, ecc = combinations(member, doc["annex"], doc["reliabilityClass"])["qp"], sec.second_moment(), tendon_eccentricity(member, ps) * 1e3
    w_bot, w_top, area, m = inertia / (sec.zc - sec.zmin), inertia / (sec.zmax - sec.zc), sec.polygon.area, qp[0] * 1e3
    needs = []
    for load, k in ((m / w_bot, 1 / area + ecc / w_bot), (-m / w_top, 1 / area - ecc / w_top)):
        needs.append(0.0 if load <= 0.0 else (load / k if k > 0.0 else math.inf))
    return max(needs) / (1 - min(max(ps["lossRatio"], 0.0), 1.0))


def decompression_row(doc, member, sec):
    """🗜️ Decompression under the quasi-permanent combination: per fibre the load tension against the compression the prestress alone gives (a fibre the prestress puts in tension is measured against the mean prestress), the worse fibre decides; at most 1 keeps the net stress compressive."""
    ps, mid = member.get("prestress"), member["id"]
    if not ps or next((s for s in doc["prestressSteels"] if s["id"] == member["prestressSteelId"]), None) is None:
        return []
    qp = combinations(member, doc["annex"], doc["reliabilityClass"])["qp"]
    p_inf, inertia, ecc = ps["force"] * (1 - min(max(ps["lossRatio"], 0.0), 1.0)), sec.second_moment(), tendon_eccentricity(member, ps) * 1e3
    w_bot, w_top, mean, m = inertia / (sec.zc - sec.zmin), inertia / (sec.zmax - sec.zc), p_inf / sec.polygon.area, qp[0] * 1e3
    ratios = []
    for prestress, load in ((mean + p_inf * ecc / w_bot, m / w_bot), (mean - p_inf * ecc / w_top, -m / w_top)):
        if prestress > 0.0:
            ratios.append(max(load, 0.0) / prestress)
        else:
            reference = max(mean, 1e-6)
            ratios.append(max(reference + load - prestress, 0.0) / reference)
    return [(f"en1992.5.10.decompression.{mid}", max(ratios))]


def span_to_depth(member, p, sec, fck):
    """📏 The 7.4.2 span-to-depth utilisation of a beam or slab."""
    if not (member["span"] > 0 and member["kind"] in ("Beam", "Slab", "FlatSlab")):
        return []
    d = sec.effective_depth(member["effectiveDepth"] * 1e3) / 1e3
    b_c, bw = sec.width_at(sec.zmax - 1e-6 * sec.depth) / 1e3, sec.web_width() / 1e3
    b_ref = max(b_c, bw)
    area_s = sec.tension_area() / 1e6
    rho = max(area_s / (b_ref * d), 1e-4) if b_ref * d > 0 else 1e-4
    a_top = sum(x["count"] * math.pi * x["diameter"] ** 2 / 4 for x in member["longitudinal"] if x["position"] == "Top")
    k = K_SUPPORT[member["support"]]
    table = (0.8 if bw > 0 and b_c / bw > 3.0 else 1.0) * limiting_span_to_depth(p, fck, rho, a_top / (b_ref * d) if b_ref * d > 0 else 0.0, k)
    cap = k * p["ld_cap"] if p["ld_cap"] else math.inf
    if p["ld_cap"] and member["deflectionSensitive"]:
        cap = min(cap, k * k * 150 / member["span"])
    return [(f"en1992.7.4.ld.{member['id']}", member["span"] / d / max(min(table, cap), 1.0))]


def reinforcement_limits(member, sec, c, fyk):
    """📉 The 9.2.1.1 minimum (Eq. 9.1N, 0.26 f_ctm/f_yk b_t d, at least 0.0013 b_t d) as the ratio of required to provided tension steel and the maximum (0.04 A_c) as the ratio of provided steel to it."""
    a_s = sec.tension_area()
    if a_s <= 0:
        return []
    d, b_t = sec.effective_depth(member["effectiveDepth"] * 1e3), sec.width_at(sec.zmin + 1e-6 * sec.depth)
    a_min = max(0.26 * c["fctm"] / fyk * b_t * d, 0.0013 * b_t * d)
    return [(f"en1992.9.minas.{member['id']}", a_min / a_s), (f"en1992.9.maxas.{member['id']}", a_s / (0.04 * sec.polygon.area))]


def ring_perimeter(position, c1, c2, radius):
    """⭕ Closed form of the line at distance `radius` from the face of a c1 x c2 column (c1 normal to the free edge): interior all around, edge flush in one free edge, corner flush in two; checked against shapely in `--self-check`."""
    return {"Edge": c2 + 2.0 * c1 + math.pi * radius, "Corner": c1 + c2 + 0.5 * math.pi * radius}.get(position, 2.0 * (c1 + c2) + 2.0 * math.pi * radius)


def column_frame(position, c1, c2):
    """🧭 Column box, slab box (None for an interior column) and the column centre in the shapely frame where the free edges are x = 0 and y = 0 and the column lies at x <= 0, y <= 0 (corner)."""
    column, slab = {"Edge": (box(-c1, -c2 / 2, 0.0, c2 / 2), box(-100.0, -100.0, 0.0, 100.0)), "Corner": (box(-c1, -c2, 0.0, 0.0), box(-100.0, -100.0, 0.0, 0.0))}.get(position, (box(-c1 / 2, -c2 / 2, c1 / 2, c2 / 2), None))
    return column, slab, (column.centroid.x, column.centroid.y)


def perimeter_line(position, c1, c2, offset):
    """⭕ The line at distance `offset` from a column face, by shapely: the buffered column cut by the slab (the free edges) without the part on the free edges."""
    column, slab, _ = column_frame(position, c1, c2)
    region = column.buffer(offset, quad_segs=256)
    if slab is None:
        return region.boundary
    region = region.intersection(slab)
    return region.boundary.difference(slab.boundary)


def control_perimeter(position, c1, c2, offset):
    """⭕ Length of the line at distance `offset` from a column face, by shapely: the buffered column cut by the slab (the free edges) without the part on the free edges."""
    column, slab, _ = column_frame(position, c1, c2)
    region = column.buffer(offset, quad_segs=256)
    if slab is None:
        return region.length
    region = region.intersection(slab)
    return region.length - region.boundary.intersection(slab.boundary).length


def opening_wedges(position, c1, c2, d, openings):
    """🕳️ EN 1992-1-1 6.4.2(3): for every opening within 6 d of the column periphery (shapely distance) the triangle spanned from the column centre by the two tangents to the opening outline, found as the two hull edges that meet in the centre of the convex hull of the centre and the four corners (openings in the column frame, origin at the column centre, converted to the shapely frame)."""
    from shapely.geometry import MultiPoint, Point
    column, _, (cx, cy) = column_frame(position, c1, c2)
    wedges = []
    for o in openings:
        rect = box(cx + o["x"] - o["width"] / 2, cy + o["y"] - o["depth"] / 2, cx + o["x"] + o["width"] / 2, cy + o["y"] + o["depth"] / 2)
        if column.distance(rect) > 6.0 * d:
            continue
        assert not rect.contains(Point(cx, cy)), "the opening contains the column centre"
        ring = list(MultiPoint([(cx, cy)] + list(rect.exterior.coords)[:4]).convex_hull.exterior.coords)[:-1]
        k = next(i for i, q in enumerate(ring) if abs(q[0] - cx) < 1e-12 and abs(q[1] - cy) < 1e-12)
        a, b = ring[(k - 1) % len(ring)], ring[(k + 1) % len(ring)]
        far = lambda q: (cx + 1000.0 * (q[0] - cx), cy + 1000.0 * (q[1] - cy))
        wedges.append(Polygon([(cx, cy), far(a), far(b)]))
    return wedges


def effective_perimeter(position, c1, c2, offset, d, openings):
    """📏 Length of the perimeter at `offset` without the part between the tangents of the openings within 6 d (shapely difference with the tangent triangles)."""
    line = perimeter_line(position, c1, c2, offset)
    for wedge in opening_wedges(position, c1, c2, d, openings):
        line = line.difference(wedge)
    return line.length


def w1_of_perimeter(position, c1, c2, offset, steps=4000, axis=0):
    """🧮 W1 = integral of |e| dl (EN 1992-1-1 Eq. 6.40) along the control perimeter at `offset` by shapely: the buffered column cut by the slab without the free edge part, e the distance of dl from the axis through the column centroid normal to c1 (interior columns only: for edge and corner columns 6.4.3(4) measures e from the perimeter centroid, not asserted yet), integrated on a fine resampling of the boundary."""
    column, slab = {"Edge": (box(-c1, -c2 / 2, 0.0, c2 / 2), box(-100.0, -100.0, 0.0, 100.0)), "Corner": (box(-c1, -c2, 0.0, 0.0), box(-100.0, -100.0, 0.0, 0.0))}.get(position, (box(-c1 / 2, -c2 / 2, c1 / 2, c2 / 2), None))
    region = column.buffer(offset, quad_segs=512)
    line = region.boundary if slab is None else region.intersection(slab).boundary.difference(slab.boundary)
    lines = list(getattr(line, "geoms", [line]))
    centre = (column.centroid.x, column.centroid.y)[axis]
    return sum(abs((g.interpolate(t, normalized=True).x, g.interpolate(t, normalized=True).y)[axis] - centre) * g.length / steps for g in lines for t in (np.arange(steps) + 0.5) / steps)


def centroidal_w1(position, c1, c2, offset, axis=0, steps=20000):
    """🧭 Shapely sampling of the control perimeter at `offset` of a flush edge or corner column: W1 = integral of |e| dl (EN 1992-1-1 Eq. 6.40) of the full perimeter about the axis through the perimeter centroid normal to `axis` (0 = c1, perpendicular to the first free edge), used for an outward eccentricity."""
    column, slab = {"Edge": (box(-c1, -c2 / 2, 0.0, c2 / 2), box(-100.0, -100.0, 0.0, 100.0)), "Corner": (box(-c1, -c2, 0.0, 0.0), box(-100.0, -100.0, 0.0, 0.0))}[position]
    line = column.buffer(offset, quad_segs=512).intersection(slab).boundary.difference(slab.boundary)
    parts = list(getattr(line, "geoms", [line]))
    points = [(g.interpolate(t, normalized=True), g.length / steps) for g in parts for t in (np.arange(steps) + 0.5) / steps]
    values = np.array([(q.x, q.y)[axis] for q, _ in points])
    weights = np.array([w for _, w in points])
    centroid = float((values * weights).sum() / weights.sum())
    return float((np.abs(values - centroid) * weights).sum())


def column_position(spec):
    """🧭 The perimeter position of a punching spec (a wall end is treated as an interior column, like the production check)."""
    return spec["columnPosition"] if spec["columnPosition"] in ("Edge", "Corner") else "Interior"


def stud_face_distance(angle, c1, c2):
    """🔩 Distance from the column centre to the column face along the ray at `angle`, by shapely (the ray cut with the column rectangle)."""
    from shapely.geometry import LineString, Point
    ray = LineString([(0.0, 0.0), (100.0 * math.cos(angle), 100.0 * math.sin(angle))])
    return ray.intersection(box(-c1 / 2.0, -c2 / 2.0, c1 / 2.0, c2 / 2.0).exterior).distance(Point(0.0, 0.0))


def stud_layout_row(member):
    """🔩 Utilisation of the stud layout (EN 1992-1-1 6.4.5 and 9.4.3, limits unverified): first stud 0.3 d to 0.5 d from the column face, radial spacing at most 0.75 d, tangential spacing (distance of the k-th studs of angularly adjacent rows by shapely, wrapping around an interior column only) at most 1.5 d within 2 d of the face and 2 d beyond, no stud beyond a free slab edge (flush at +x / +y)."""
    punching = member.get("punching")
    rows = sorted((r for r in member.get("studRows", []) if r["count"] > 0 and r["diameter"] > 0), key=lambda r: r["angle"] % (2.0 * math.pi))
    if member["kind"] != "FlatSlab" or not punching or not rows:
        return []
    from shapely.geometry import Point
    d, c1, c2, position = member["effectiveDepth"], punching["columnWidth"], punching["columnDepth"], column_position(punching)
    at = lambda r, k: Point((stud_face_distance(r["angle"], c1, c2) + r["spacingFirst"] + k * r["spacingRadial"]) * math.cos(r["angle"]), (stud_face_distance(r["angle"], c1, c2) + r["spacingFirst"] + k * r["spacingRadial"]) * math.sin(r["angle"]))
    s0 = min(r["spacingFirst"] for r in rows)
    s_r = max([r["spacingRadial"] for r in rows if r["count"] > 1] or [0.0])
    s_t, outside = 0.0, False
    for k in range(max(r["count"] for r in rows)):
        here = [r for r in rows if r["count"] > k]
        outside = outside or any((position == "Edge" and at(r, k).x > c1 / 2 + 1e-9) or (position == "Corner" and (at(r, k).x > c1 / 2 + 1e-9 or at(r, k).y > c2 / 2 + 1e-9)) for r in here)
        pairs = len(here) if position == "Interior" and len(here) > 2 else max(len(here) - 1, 0)
        for j in range(pairs):
            a, b = here[j], here[(j + 1) % len(here)]
            s_t = max(s_t, at(a, k).distance(at(b, k)) / ((1.5 if a["spacingFirst"] + k * a["spacingRadial"] <= 2.0 * d else 2.0) * d))
    low = 0.3 * d / max(s0, 1e-9) if s0 < 0.3 * d else 0.0
    return [(f"en1992.9.4.3.studs.{member['id']}", max(s0 / (0.5 * d), low, s_r / (0.75 * d), s_t, 1.0 + 1e-6 if outside else 0.0))]


def reduced_perimeter(position, c1, c2, d):
    """📏 u1* of an edge or corner column (EN 1992-1-1 Figure 6.20, unverified reading): the shapely perimeter at 2 d with the legs along the column sides cut to min(1.5 d, 0.5 c)."""
    leg = lambda c: min(1.5 * d, 0.5 * c)
    cut = {"Edge": 2.0 * (c1 - leg(c1)), "Corner": (c1 - leg(c1)) + (c2 - leg(c2))}[position]
    return control_perimeter(position, c1, c2, 2.0 * d) - cut


def beta_unbalanced(position, c1, c2, d, load, m1, m2):
    """🎯 β from the unbalanced column moments m1 (eccentricity along c1, perpendicular to the free edge) and m2 (along c2), EN 1992-1-1 6.4.3(3) to (5): interior Eq. 6.39 (k of Table 6.1 by numpy interpolation, W1 by the shapely integral of |e| dl) for one axis and Eq. 6.43 for both, edge and corner u1/u1* (Eq. 6.44, 6.46) plus k u1/W1 e_par for an edge moment parallel to the edge (Eq. 6.45); an outward eccentricity (negative moment) of an edge or corner column uses Eq. 6.39 with the full u1 about the shapely perimeter centroid (M_Ed taken about it) and never gives less than the inward value."""
    k_of = lambda ratio: float(np.interp(ratio, [0.5, 1.0, 2.0, 3.0], [0.45, 0.60, 0.70, 0.80]))
    u1 = control_perimeter(position, c1, c2, 2.0 * d)
    if position == "Interior":
        if m1 == 0.0 and m2 == 0.0:
            return 1.0
        if m1 != 0.0 and m2 != 0.0:
            return 1.0 + 1.8 * math.hypot(abs(m1) / load / (c2 + 4.0 * d), abs(m2) / load / (c1 + 4.0 * d))
        along_c1 = m1 != 0.0
        a, b = (c1, c2) if along_c1 else (c2, c1)
        return 1.0 + k_of(a / b) * abs(m1 if along_c1 else m2) / load * u1 / w1_of_perimeter("Interior", a, b, 2.0 * d, axis=0)
    base = u1 / reduced_perimeter(position, c1, c2, d)
    if position == "Edge":
        parallel = k_of(c1 / (2.0 * c2)) * u1 / w1_of_perimeter("Edge", c1, c2, 2.0 * d, axis=1) * abs(m2) / load if m2 != 0.0 else 0.0
        if m1 >= 0.0:
            return base + parallel
        w1 = centroidal_w1("Edge", c1, c2, 2.0 * d, axis=0)
        return max(1.0 + k_of(c1 / c2) * u1 / w1 * abs(m1) / load + parallel, base + parallel)
    if m1 >= 0.0 and m2 >= 0.0:
        return base
    terms = 0.0
    for axis, (a, b, m) in enumerate(((c1, c2, m1), (c2, c1, m2))):
        terms += k_of(a / b) * u1 / centroidal_w1("Corner", c1, c2, 2.0 * d, axis=axis) * max(-m, 0.0) / load
    return max(1.0 + terms, base)


def punching_rows(doc, member, combos):
    """🕳️ Punching utilisations of a flat slab (EN 1992-1-1 6.4.3, 6.4.4, 6.4.5): v_Ed = beta V_Ed/(u1 d) of the governing ultimate combination over v_Rd,cs = 0.75 v_Rd,c + 1.5 d/s_r A_sw f_ywd,ef/(u1 d) with s_r the largest radial spacing of a stud row with two or more studs (0.75 d without) and A_sw the stud area of one perimeter (one stud of every row), u1 from shapely; the crushing limit 1.4 v_Rd,c only for the German annex."""
    spec = member.get("punching")
    if member["kind"] != "FlatSlab" or not spec or not combos["uls"]:
        return []
    annex, p = doc["annex"], NDP[doc["annex"].lower()]
    grade = next(g for g in doc["concreteGrades"] if g["id"] == member["concreteGradeId"])
    reinf = next(g for g in doc["reinforcementGrades"] if g["id"] == member["reinforcementGradeId"])
    fck, fyd = grade["fCk"] / 1e6, reinf["fYk"] / 1e6 / p["gamma_s"]
    d, d_mm, position = member["effectiveDepth"], member["effectiveDepth"] * 1e3, column_position(spec)
    openings = member.get("openings") or []
    u1 = effective_perimeter(position, spec["columnWidth"], spec["columnDepth"], 2.0 * d, d, openings)
    layers = [x for x in member["longitudinal"] if x["position"] in ("Bottom", "Tension")]
    area = sum(sum(math.pi * (b["diameter"] / 2) ** 2 for b in member.get("bars") or [] if b["layerId"] == x["id"]) or x["count"] * math.pi * (x["diameter"] / 2) ** 2 for x in layers)
    rho = min(area / (member["width"] * d), 0.02)
    if annex == "De":
        rho = min(rho, 0.5 * p["alpha_cc"] * fck / p["gamma_c"] / fyd)
    k = min(1.0 + math.sqrt(200.0 / d_mm), 2.0)
    vmin = float(np.interp(d_mm, [600.0, 800.0], [v / p["gamma_c"] for v in p["vmin"]]) if annex == "De" else p["vmin"][0]) * k**1.5 * math.sqrt(fck)
    c1, c2 = spec["columnWidth"], spec["columnDepth"]
    u0 = {"Interior": 2.0 * (c1 + c2), "Edge": min(c2 + 3.0 * d, c2 + 2.0 * c1), "Corner": min(3.0 * d, c1 + c2)}[position]
    crd = FACTORS.ndp("ndp.en1992.crd-c-punch", annex)
    if position == "Interior" and u0 / d < FACTORS.ndp("ndp.en1992.punch-small-u0.limit", annex):
        crd = max(crd * (FACTORS.ndp("ndp.en1992.punch-small-u0.slope", annex) * u0 / d + FACTORS.ndp("ndp.en1992.punch-small-u0.offset", annex)), FACTORS.ndp("ndp.en1992.punch-small-u0.floor", annex))
    v_rd_c = max(crd / p["gamma_c"] * k * (100.0 * rho * fck) ** (1.0 / 3.0), vmin) * 1e6
    valid = [r for r in member.get("studRows") or [] if r["count"] > 0 and r["diameter"] > 0]
    asw = sum(math.pi * (r["diameter"] / 2) ** 2 for r in valid) or spec["asw"]
    fywd_ef = min(250.0 + 0.25 * d_mm, fyd) * 1e6
    s_r = max([r["spacingRadial"] for r in valid if r["count"] > 1] or [0.0]) or 0.75 * d
    v_rd_cs = 0.75 * v_rd_c + 1.5 * (d / s_r) * asw * fywd_ef / (u1 * d) if asw > 0 else v_rd_c
    load = max(abs(e[4]) for e in combos["uls"])
    if spec["betaMethod"] == "Simplified":
        beta = FACTORS.ndp(f"ndp.en1992.beta-{position.lower()}", annex)
    elif spec["columnPosition"] == "WallEnd" or load <= 0.0 and (spec["momentC1"] or spec["momentC2"]):
        return []
    else:
        beta = beta_unbalanced(position, c1, c2, d, load, spec["momentC1"], spec["momentC2"])
    v_ed = beta * load / (u1 * d)
    rows = [(f"en1992.6.4.punching.{member['id']}", v_ed / max(v_rd_cs, 1.0))]
    if annex == "De":
        rows.append((f"en1992.6.4.punching-max.{member['id']}", v_ed / max(FACTORS.ndp("ndp.en1992.punch-vrdmax-factor", annex) * v_rd_c, 1.0)))
    else:
        rows.append((f"en1992.6.4.punching-max.{member['id']}", beta * load / (u0 * d) / max(0.5 * 0.6 * (1.0 - fck / 250.0) * p["alpha_cc"] * fck / p["gamma_c"] * 1e6, 1.0)))
    if valid:
        u_out = beta * load / (max(v_rd_c, 1.0) * d)
        lo, hi = 0.0, 100.0
        for _ in range(60):
            mid = 0.5 * (lo + hi)
            lo, hi = (mid, hi) if effective_perimeter(position, c1, c2, mid, d, openings) < u_out else (lo, mid)
        a_out = 0.0 if u_out <= effective_perimeter(position, c1, c2, 0.0, d, openings) else 0.5 * (lo + hi)
        rows.append((f"en1992.6.4.uout.{member['id']}", a_out / (max(r["spacingFirst"] + (r["count"] - 1) * r["spacingRadial"] for r in valid) + FACTORS.ndp("ndp.en1992.punch-k-out", annex) * d)))
    return rows


def anchorage_rows(doc, member, fyk, p):
    """⚓ EN 1992-1-1 8.4 and 8.7 per longitudinal layer: f_bd = 2.25 eta_1 eta_2 f_ctd (8.2, f_ctd with the C60/75 cap, f_ctk,0.05 = 0.7 f_ctm), l_b,rqd = phi/4 sigma_sd/f_bd (8.3), l_bd = max(alpha_2 l_b,rqd, l_b,min) with l_b,min = max(0.3 l_b,rqd, 10 phi, 100 mm) and alpha_2 = 1 - 0.15 (c_d - phi)/phi in [0.7, 1] (Table 8.2), l_0 = max(alpha_2 alpha_6 l_b,rqd, 0.3 alpha_6 l_b,rqd, 15 phi, 200 mm) with alpha_6 = sqrt(rho_1/25) in [1, 1.5] (8.10, Table 8.3); the utilisation is the required over the provided length, a passing row stays `unverifiedBasis` (the registry rows b-01.1 and b-04.1 carry no value)."""
    grade = next(g for g in doc["concreteGrades"] if g["id"] == member["concreteGradeId"])
    annex = doc["annex"].lower()
    fck = grade["fCk"] / 1e6
    f_ctd = FACTORS.ndp("ndp.en1992.m-01.alpha-ct", annex) * FCTK_5_TABLE_3_1.get(int(round(min(fck, 60.0))), 0.7 * concrete(min(fck, 60.0))["fctm"]) / p["gamma_c"]
    sigma_sd = fyk / p["gamma_s"]
    rows = []
    for layer in member["longitudinal"]:
        phi, cover = layer["diameter"], member["cover"]
        eta1 = 0.7 if layer["bondCondition"] == "Poor" else 1.0
        eta2 = 1.0 if phi * 1000 <= 32 else (132 - phi * 1000) / 100
        f_bd = FACTORS.ndp("ndp.en1992.fbd-eta", annex) * eta1 * eta2 * f_ctd
        l_rqd = phi / 4 * sigma_sd / f_bd
        spacing = (member["width"] - 2 * cover - phi) / (layer["count"] - 1) if layer["count"] > 1 else math.inf
        c_d = max(min(spacing / 2, cover), phi)
        alpha2 = min(max(1 - 0.15 * (c_d - phi) / phi, 0.7), 1.0)
        l_bd = max(alpha2 * l_rqd, 0.3 * l_rqd, 10 * phi, 0.1)
        alpha6 = min(max(math.sqrt(100 * min(max(layer["lapShare"], 0.0), 1.0) / 25), 1.0), 1.5)
        l_0 = max(alpha2 * alpha6 * l_rqd, 0.3 * alpha6 * l_rqd, 15 * phi, 0.2)
        for key, required, provided in (("anchorage", l_bd, layer["anchorageLength"]), ("lap", l_0, layer["lapLength"])):
            u = min(required / provided, 1e6) if provided > 0 else 1e6
            rows.append((f"en1992.8.{4 if key == 'anchorage' else 7}.{key}.{member['id']}.{layer['id']}", u))
    return rows


def torsion_and_truss_rows(member, p, sec, combos, fck, fyk):
    """🌀🕸️ The torsion rows of the member with a torsion moment (links and chords from the shear inclination of the shear-governing combination) and the strut-and-tie rows of a deep beam."""
    uls, out = governing(combos["uls"]), []
    if uls is None:
        return out
    uv, ut = governing(combos["uls"], lambda e: abs(e[2])), governing(combos["uls"], lambda e: abs(e[3]))
    d, bw = sec.effective_depth(member["effectiveDepth"] * 1e3), sec.web_width()
    stirrups = member.get("stirrups")
    asw_s = stirrups["legs"] * math.pi * stirrups["diameter"] ** 2 / 4 / stirrups["spacing"] if stirrups and stirrups["spacing"] > 0 else 0.0
    if abs(ut[3]) > 0.0:
        _, cot = shear_resistance_with_links(p, fck, fyk, bw, 0.9 * d, asw_s * 1e3, max(-uv[1] / sec.polygon.area, 0.0), abs(uv[2]))
        mid_z = 0.5 * (sec.zmax + sec.zmin)
        below = sum(a for _, z, a in sec.bars if z <= mid_z)
        above = sum(a for _, z, a in sec.bars if z > mid_z)
        out += FAMILIES.torsion_rows(member, p, fck, fyk, bw, d, below, above, ut[3], uv[2], uls[0], sec.m_rd(-uls[1]) / 1e3, cot, stirrups)
    return out


def member_checks(doc, member):
    """🏛️ Every check the oracle recomputes for one member."""
    p = NDP[doc["annex"].lower()]
    grade = next((g for g in doc["concreteGrades"] if g["id"] == member["concreteGradeId"]), None)
    reinf = next((g for g in doc["reinforcementGrades"] if g["id"] == member["reinforcementGradeId"]), None)
    if grade is None or reinf is None:
        return []
    fck, fyk = grade["fCk"] / 1e6, reinf["fYk"] / 1e6
    c = concrete(fck)
    sec = build_section(member, c, reinf, p["alpha_cc"] * fck / p["gamma_c"], fyk / p["gamma_s"])
    combos = combinations(member, doc["annex"], doc["reliabilityClass"])
    out = flexure_and_shear(member, p, sec, fck, fyk, combos)
    if sec.bars:
        out += crack_and_deflection(member, p, sec, c, reinf["eS"] / 1e6, combos["qp"], fck) + elastic_stresses(member, sec, c, combos, fck, fyk) + reinforcement_limits(member, sec, c, fyk)
    rows = torsion_and_truss_rows(member, p, sec, combos, fck, fyk)
    truss = [(r["id"], r["utilization"], {k: v for k, v in r.items() if k not in ("id", "utilization")}) for r in FAMILIES.strut_and_tie_rows(member, p, fck, fyk, governing(combos["uls"])[2], sec.tension_area())] if combos["uls"] else []
    return out + rows + [(i, u) if not extra else (i, u, extra) for i, u, extra in truss] + anchorage_rows(doc, member, fyk, p) + punching_rows(doc, member, combos) + stud_layout_row(member) + span_to_depth(member, p, sec, fck) + prestress_row(doc, member, sec, fck) + decompression_row(doc, member, sec)


def anchor_effects(anchor, fac):
    """⚓ EN 1990 design (N_Ed, V_Ed) of an anchor in N: line and point sources add to N_k, M_k acts as a force couple over h_ef, T_k over c1, punching shear adds to V; returns the governing 6.10b ultimate and quasi-permanent effects (ties to the last, like the Rust `max_by` on the unused |M|), the permanent-only 6.10 effect without variable actions."""
    g, var = np.zeros(2), []
    for a in anchor["actions"]:
        n = a["nK"] + (abs(a["gKLine"]) + abs(a["qKLine"]) if a["source"] == "Udl" else abs(a["pointForce"]) if a["source"] == "Point" else 0.0)
        n += abs(a["mK"]) / anchor["hEf"] if anchor["hEf"] > 0 else 0.0
        v = a["vK"] + (abs(a["tK"]) / anchor["c1"] if anchor["c1"] > 0 else 0.0) + abs(a["vKPunch"])
        if a["kind"] in ("Permanent", "Prestress"):
            g = g + np.array([n, v])
        elif a["kind"] in ("Imposed", "Snow", "Wind"):
            var.append((a, np.array([n, v])))
    gg, gq = fac.gamma_g_sup(), fac.gamma_q()
    uls, qp = [], []
    if not var:
        uls = [gg * g]
    for i, (lead, e) in enumerate(var):
        rest = [(psi(o["category"], fac), oe) for j, (o, oe) in enumerate(var) if j != i]
        uls.append(gg * g + gq * e + sum((gq * ps[0] * oe for ps, oe in rest), np.zeros(2)))
        qp.append(g + psi(lead["category"], fac)[2] * e + sum((ps[2] * oe for ps, oe in rest), np.zeros(2)))
    return (uls[-1] if uls else None), (qp[-1] if qp else None)


def anchor_checks(doc, anchor):
    """⚓ The EN 1992-4 anchor utilisations (N, mm, MPa through the Python twin of the mechanics fastening kernel): steel tension and its quasi-permanent serviceability twin, concrete cone, pull-out (k2 A_h f_ck), splitting (min of cone and pull-out times 0.5 + 0.5 min(c1, c_cr)/c_cr), pry-out (k8 N_Rd,c), edge breakout with the member thickness 2 h_ef, steel shear, and the tension-shear interactions (concrete p = 1.5 over the smaller tension and shear resistances, steel p = 2)."""
    annex, aid = doc["annex"], anchor["id"]
    fac = FACTORS.ActionFactors(annex, reliability_class=int(str(doc["reliabilityClass"])[-1]))
    uls, qp = anchor_effects(anchor, fac)
    if uls is None:
        return []
    n_ed, v_ed = max(uls[0], 0.0), max(uls[1], 0.0)
    ndp = lambda key: FACTORS.ndp(key, annex)
    g_mc = ndp("ndp.en1992.m-01.gamma-c-persistent") * ndp("ndp.en1992.gamma-inst")
    f_ck, hef, c1, d = anchor["fCk"] / 1e6, anchor["hEf"] * 1e3, anchor["c1"] * 1e3, anchor["d"] * 1e3
    f_uk, f_yk, a_s = anchor["fUk"] / 1e6, anchor["fYk"] / 1e6, anchor["aS"] * 1e6
    cracked = anchor["cracked"]
    steel = KERNEL.steel({"a_s": a_s, "f_uk": f_uk, "f_yk": f_yk})
    n_rd_s, v_rd_s = steel["n_rk_s"] / steel["gamma_n"], 0.5 * a_s * f_uk / steel["gamma_v"]
    ratio = lambda ed, rd: ed / max(rd, 1.0)
    out = [(f"en1992-4.steel.{aid}", ratio(n_ed, n_rd_s))]
    if qp is not None:
        out.append((f"en1992-4.steel.sls.{aid}", ratio(max(qp[0], 0.0), anchor["aS"] * anchor["fYk"] / ndp("ndp.en1992.gamma-s"))))
    cone = KERNEL.cone({"h_ef": hef, "c1": c1, "k1": ndp("ndp.en1992.cone-k-cast-cracked" if cracked else "ndp.en1992.cone-k-cast-uncracked"), "f_ck": f_ck, "anchors": [(0.0, 0.0)], "e_nx": 0.0, "e_ny": 0.0})["n_rk_c"]
    head = anchor["headDiameter"] * 1e3 if anchor["headDiameter"] > anchor["d"] else 2.0 * d
    pull = KERNEL.pullout({"k2": ndp("ndp.en1992.pullout-k2-cracked" if cracked else "ndp.en1992.pullout-k2-uncracked"), "d_h": head, "d": d, "f_ck": f_ck})["n_rk_p"]
    c_cr = 1.5 * hef
    n_rd_c, n_rd_p = cone / g_mc, pull / g_mc
    n_rd_sp = min(cone, pull) * (min(c1, c_cr) / c_cr * 0.5 + 0.5) / g_mc
    v_rd_cp = KERNEL.pryout({"h_ef": hef, "n_rk_c": n_rd_c})["v_rk_cp"]
    v_rd_c = KERNEL.edge({"c1": c1, "d_nom": d, "h_ef": hef, "k9": 1.7 if cracked else 2.4, "f_ck": f_ck, "h": 2.0 * hef})["v_rk_c"] / g_mc
    out += [(f"en1992-4.cone.{aid}", ratio(n_ed, n_rd_c)), (f"en1992-4.pullout.{aid}", ratio(n_ed, n_rd_p)), (f"en1992-4.splitting.{aid}", ratio(n_ed, n_rd_sp)), (f"en1992-4.pryout.{aid}", ratio(v_ed, v_rd_cp)), (f"en1992-4.edge.{aid}", ratio(v_ed, v_rd_c)), (f"en1992-4.steel-shear.{aid}", ratio(v_ed, v_rd_s))]
    term = lambda ed, rd, p: (ed / rd) ** p if rd > 0 else 0.0
    n_rd_t, v_rd_t = min(n_rd_c, n_rd_p, n_rd_sp), min(v_rd_c, v_rd_cp)
    out.append((f"en1992-4.interaction.{aid}", max(term(n_ed, n_rd_t, 1.5) + term(v_ed, v_rd_t, 1.5), term(n_ed, n_rd_s, 2.0) + term(v_ed, v_rd_s, 2.0))))
    return out


CITES = (
    (".6.1.flexure.acc.", ("ndp.en1992.m-01.alpha-cc-accidental", "ndp.en1992.m-01.gamma-c-accidental")),
    (".6.1.flexure.", ("ndp.en1992.m-01.alpha-cc-persistent", "ndp.en1992.m-01.alpha-ct", "ndp.en1992.m-01.gamma-c-persistent")),
    (".6.2.shear.", ("ndp.en1992.v-01.1", "ndp.en1992.v-02.1", "ndp.en1992.v-03.1", "ndp.en1992.v-03.2", "ndp.en1992.e-03.1")),
    (".6.4.punching-max.", ("ndp.en1992.p-03.3", "ndp.en1992.p-04.1", "ndp.en1992.p-05.1")),
    (".6.4.uout.", ("ndp.en1992.punch-k-out", "ndp.en1992.p-04.1")),
    (".6.4.punching.", ("ndp.en1992.p-03.1", "ndp.en1992.p-03.2", "ndp.en1992.p-03.3", "ndp.en1992.p-04.1")),
    (".9.4.3.studs.", ("ndp.en1992.p-04.1",)),
    (".7.3.crack.", ("ndp.en1992.sl-06.1",)),
    (".7.4.ld.", ("ndp.en1992.sl-07.1", "ndp.en1992.defl-limit-qp")),
    (".7.4.deflection.", ("ndp.en1992.defl-limit-qp", "ndp.en1992.sigma-s-char")),
    (".7.2.sigma-s.", ("ndp.en1992.sl-02.1",)),
    (".7.2.sigma-c.", ("ndp.en1992.sl-02.1", "ndp.en1992.sigma-c-qp")),
    (".5.10.prestress.", ("ndp.en1992.pr-01.1", "ndp.en1992.pr-05.1", "ndp.en1992.sl-02.1")),
    (".5.10.decompression.", ("ndp.en1992.pr-05.1", "ndp.en1992.sl-02.1")),
    (".8.4.anchorage.", ("ndp.en1992.b-01.1",)),
    (".8.7.lap.", ("ndp.en1992.b-04.1",)),
    ("en1992-4.", ("ndp.en1992.gamma-s",)),
)


def cited(row_id):
    """🔖 The registry rows the Rust check of `row_id` cites (its `ndps` list plus the values it reads); empty for the checks that read no registry row (minimum and maximum reinforcement, the strut-and-tie tie)."""
    return next((ids for key, ids in CITES if key in row_id), ())


def evaluate(doc):
    """🚦️ The check rows of every member of a snapshot, with the kernel status rule: a pass that cites an unverified registry row is `unverifiedBasis` (the registry flips it once the rows are verified), a failure stays a failure."""
    outcome = lambda i, u: {"status": "fail" if u > 1.0 + 1e-9 else "unverifiedBasis"} if any(c not in FAMILIES._VERIFIED for c in cited(i)) else {}
    return {"checks": [{"id": i, "utilization": u, **outcome(i, u), **(extra[0] if extra else {})} for member in doc["members"] for i, u, *extra in member_checks(doc, member)] + [{"id": i, "utilization": u, **outcome(i, u)} for anchor in doc.get("anchors", []) for i, u in anchor_checks(doc, anchor)]}


def _concreteproperties_section(ecm, width, height, bars, fck):
    from concreteproperties import Concrete, ConcreteLinear, ConcreteSection, EurocodeParabolicUltimate, SteelBar, SteelElasticPlastic, add_bar
    from sectionproperties.pre.library import rectangular_section

    concrete = Concrete(name="c", density=2.4e-6, stress_strain_profile=ConcreteLinear(elastic_modulus=ecm), colour="lightgrey", ultimate_stress_strain_profile=EurocodeParabolicUltimate(compressive_strength=0.85 * fck / 1.5, compressive_strain=0.002, ultimate_strain=0.0035, n=2.0), flexural_tensile_strength=3.2)
    steel = SteelBar(name="s", density=7.85e-6, stress_strain_profile=SteelElasticPlastic(yield_strength=500.0 / 1.15, elastic_modulus=200000.0, fracture_strain=0.05), colour="grey")
    geometry = rectangular_section(d=height, b=width, material=concrete)
    for y, z, area in bars:
        geometry = add_bar(geometry, area=area, material=steel, x=y + width / 2, y=z)
    from sectionproperties.pre import CompoundGeometry

    return ConcreteSection(geometry if bars else CompoundGeometry([geometry]))


def concreteproperties_stress(fck, ecm, width, height, bars, moment, normal_comp, es):
    """📚 The cracked steel and concrete stress of concreteproperties (MPa, tension positive for the steel) for bars given as (y, z from the bottom, area)."""
    section = _concreteproperties_section(ecm, width, height, bars, fck)
    cracked = section.calculate_cracked_properties(theta=0)
    stress = section.calculate_cracked_stress(cracked_results=cracked, n=-normal_comp, m=moment)
    return {"sigma_s": -min(float(v) for v in stress.lumped_reinforcement_stresses), "sigma_c": max(float(v.max()) for v in stress.concrete_stresses)}


def concreteproperties_uncracked_fibres(fck, ecm, width, height, normal, moment):
    """📚 The least and the greatest fibre stress (MPa, compression positive) of the uncracked section under a normal force (compression negative) and a moment (N mm, positive compresses the bottom), by concreteproperties."""
    section = _concreteproperties_section(ecm, width, height, [], fck)
    stress = section.calculate_uncracked_stress(n=normal, m_x=moment)
    return -float(max(v.max() for v in stress.concrete_stresses)), -float(min(v.min() for v in stress.concrete_stresses))


def concreteproperties_uncracked(fck, ecm, width, height, normal, moment):
    """📚 The extreme compressive concrete stress (MPa) of the uncracked section under a normal force (compression negative) and a moment, by concreteproperties."""
    return concreteproperties_uncracked_fibres(fck, ecm, width, height, normal, moment)[1]


def self_check():
    """🧪 V_Rd,c, the creep coefficient and M_Rd of a tank wall section against structuralcodes."""
    from structuralcodes.codes import ec2_2004 as ec2

    p = NDP["en"]
    ours = shear_resistance_without_links(p, 30.0, 300.0, 500.0, 1500.0, 100e3 / (300.0 * 550.0)) / 1e3
    theirs = ec2.VRdc(30.0, 500.0, 1500.0, 300.0, 100e3, 300.0 * 550.0, 30.0 / 1.5) / 1e3
    assert abs(ours - theirs) <= 5e-3 * theirs, (ours, theirs)
    for fck, table in FCTK_5_TABLE_3_1.items():
        formula = ec2.fctk_5(ec2.fctm(float(fck)))
        assert abs(table - formula) <= 0.04 * formula, (fck, table, formula)
    fcm, rh, h0 = 38.0, 50.0, 200.0
    theirs_phi = ec2.phi_0(ec2.phi_RH(h0, fcm, rh, ec2.alpha_1(fcm), ec2.alpha_2(fcm)), ec2.beta_fcm(fcm), ec2.beta_t0(28.0))
    ours_phi = creep_coefficient(fcm, rh, h0, 28.0, 1e9)
    assert abs(ours_phi - theirs_phi) <= 5e-3 * theirs_phi and ours_phi > 0, (ours_phi, theirs_phi)
    from shapely import Polygon as ShapelyPolygon
    column = ShapelyPolygon([(-0.2, -0.15), (0.2, -0.15), (0.2, 0.15), (-0.2, 0.15)])
    assert all(abs(ring_perimeter("Interior", 0.4, 0.3, r) - column.buffer(r, quad_segs=2048).length) <= 1e-5 for r in (0.1, 0.2, 0.4)), "ring perimeter against shapely buffer"
    assert all(abs(ring_perimeter(position, 0.4, 0.3, r) - control_perimeter(position, 0.4, 0.3, r)) <= 1e-4 for position in ("Interior", "Edge", "Corner") for r in (0.1, 0.2, 0.4, 0.84)), "closed-form ring perimeters against the shapely cut buffer"
    near, far = [{"x": 1.0, "y": 0.0, "width": 0.2, "depth": 0.2}], [{"x": 2.5, "y": 0.0, "width": 0.2, "depth": 0.2}]
    assert abs(control_perimeter("Interior", 0.3, 0.3, 0.5) - effective_perimeter("Interior", 0.3, 0.3, 0.5, 0.25, near) - 2.0 * 0.65 / 9.0) <= 1e-6, "6.4.2(3) worked example: the tangents y = +-x/9 cut 2 * 0.65 / 9 m from the right side of the 2 d perimeter"
    assert abs(control_perimeter("Interior", 0.3, 0.3, 0.5) - effective_perimeter("Interior", 0.3, 0.3, 0.5, 0.25, far)) <= 1e-9, "an opening beyond 6 d leaves the perimeter whole"
    c1, c2, a = 0.4, 0.3, 0.51
    assert abs(w1_of_perimeter("Interior", c1, c2, a) - (c1**2 / 2 + c1 * c2 + 2 * a * c2 + math.pi * a * c1 + 4 * a**2)) <= 5e-4, "W1 of Eq. 6.40 against the shapely integral (Interior, d = a/2; tolerance of the 4000-step midpoint rule)"
    assert abs(w1_of_perimeter("Edge", c1, c2, a, axis=1) - (c2**2 / 4 + c1 * c2 + 2 * a * c1 + 2 * a**2 + math.pi * (a / 2) * c2)) <= 5e-4, "W1 of the edge column (6.4.3(4)) against the shapely integral of |y| dl"
    hand = dict(interior_c1=1.1077500, interior_both=1.1286452, edge=1.1362267, corner=1.2723, edge_parallel=1.1935251)
    sq, dd, v = 0.3, 0.255, 500e3
    assert abs(beta_unbalanced("Interior", sq, sq, dd, v, 40e3, 0.0) - hand["interior_c1"]) <= 2e-4, "beta Eq. 6.39 worked example (c = 300 mm, d = 255 mm, V = 500 kN, M = 40 kNm)"
    assert abs(beta_unbalanced("Interior", sq, sq, dd, v, 40e3, 25e3) - hand["interior_both"]) <= 1e-6, "beta Eq. 6.43 worked example"
    assert abs(beta_unbalanced("Edge", sq, sq, dd, v, 0.0, 0.0) - hand["edge"]) <= 1e-4 and abs(beta_unbalanced("Edge", sq, sq, dd, v, 10e3, 30e3) - hand["edge_parallel"]) <= 2e-3, "beta Eq. 6.44/6.45 worked examples"
    assert abs(beta_unbalanced("Corner", sq, sq, dd, v, 0.0, 0.0) - (sq * 2 + math.pi * dd) / (2 * min(1.5 * dd, 0.5 * sq) + math.pi * dd)) <= 1e-4 and beta_unbalanced("Edge", sq, sq, dd, v, -1.0, 0.0) >= hand["edge"], "beta Eq. 6.46 and the outward floor"
    free = KERNEL.cone({"h_ef": 100.0, "c1": 0.0, "k1": 12.7, "f_ck": 30.0, "anchors": [(0.0, 0.0)], "e_nx": 0.0, "e_ny": 0.0})["n_rk_c"]
    assert abs(free - 12.7 * math.sqrt(30.0) * 100.0**1.5) <= 1e-6 * free, "EN 1992-4 7.2.1.4 isolated anchor without edge: N0_Rk,c = k1 sqrt(fck) hef^1.5"
    edge_cone = KERNEL.cone({"h_ef": 100.0, "c1": 75.0, "k1": 12.7, "f_ck": 30.0, "anchors": [(0.0, 0.0)], "e_nx": 0.0, "e_ny": 0.0})["n_rk_c"]
    assert abs(edge_cone - free * (75.0 + 150.0) / 300.0 * (0.7 + 0.3 * 75.0 / 150.0)) <= 1e-6 * free, "one free edge at 75 mm: A_c,N/A0 = (c1 + c_cr)/(2 c_cr) and psi_s = 0.7 + 0.3 c1/c_cr"
    from structuralcodes.geometry import SurfaceGeometry, add_reinforcement_line
    from structuralcodes.materials.concrete import ConcreteEC2_2004
    from structuralcodes.materials.reinforcement import ReinforcementEC2_2004
    from structuralcodes.sections import GenericSection

    pde = NDP["de"]
    conc = concrete(35.0)
    mine = build_section({"width": 0.35, "height": 0.5, "effectiveDepth": 0.45, "cover": 0.04, "longitudinal": [{"id": "l", "diameter": 0.016, "count": 6, "position": "bottom"}]}, conc, {"eS": 200e9, "k": 1.08, "epsUk": 0.05}, pde["alpha_cc"] * 35.0 / 1.5, 500.0 / 1.15)
    m_ours = mine.m_rd(-63e3) / 1e6
    geometry = SurfaceGeometry(ShapelyPolygon([(-175, -250), (175, -250), (175, 250), (-175, 250)]), ConcreteEC2_2004(fck=35.0, alpha_cc=0.85, gamma_c=1.5))
    geometry = add_reinforcement_line(geometry, (-175 + 55, -250 + 50), (175 - 55, -250 + 50), 16.0, ReinforcementEC2_2004(fyk=500.0, Es=200000.0, density=7850, ftk=540.0, epsuk=0.05, gamma_s=1.15), n=6)
    m_theirs = abs(GenericSection(geometry).section_calculator.calculate_bending_strength(n=63e3).m_y) / 1e6
    assert abs(m_ours - m_theirs) <= 5e-3 * m_theirs, (m_ours, m_theirs)
    cracked_ours = mine.cracked(150e6, 0.0, conc["ecm"])
    cp_stress = concreteproperties_stress(35.0, conc["ecm"], 350.0, 500.0, [(y, z + 250.0, a) for y, z, a in mine.bars], 150e6, 0.0, 200e3)
    assert abs(cracked_ours["sigma_s"] - cp_stress["sigma_s"]) <= 5e-3 * cp_stress["sigma_s"], (cracked_ours, cp_stress)
    assert abs(cracked_ours["sigma_c"] - cp_stress["sigma_c"]) <= 5e-3 * cp_stress["sigma_c"], (cracked_ours, cp_stress)
    force, ecc = 1.0e6, 0.12
    ours_c = force / (mine.polygon.area * 1e-6) / 1e6 + force * ecc * (mine.zc - mine.zmin) * 1e-3 / (mine.second_moment() * 1e-12) / 1e6
    cp_uncracked = concreteproperties_uncracked(35.0, conc["ecm"], 350.0, 500.0, -force, force * ecc * 1e3)
    assert abs(ours_c - cp_uncracked) <= 5e-3 * cp_uncracked, (ours_c, cp_uncracked)
    xs = [0.0, 2.0, 5.0, 8.0, 10.0]
    parabola = lambda v: -0.05 - 0.4 * (1 - ((v - 5.0) / 5.0) ** 2)
    pts = [(v, parabola(v)) for v in xs]
    assert all(abs(parabolic_through(pts, v) - parabola(v)) < 1e-12 for v in (0.0, 1.0, 3.3, 5.0, 7.7, 10.0)), "parabolic_through on a parabola"
    three = [(0.0, 0.0), (3.75, -0.12), (7.5, 0.0)]
    fit = np.polyfit([q[0] for q in three], [q[1] for q in three], 2)
    assert all(abs(parabolic_through(three, v) - np.polyval(fit, v)) < 1e-12 for v in (0.5, 2.0, 3.75, 6.1)), "three points equal the polyfit parabola"
    moment = 115.0e3
    bottom, top = decompression_stresses(force, 0.0, ecc, moment, mine)
    cp_least, cp_greatest = concreteproperties_uncracked_fibres(35.0, conc["ecm"], 350.0, 500.0, -force, force * ecc * 1e3 - moment * 1e3)
    assert abs(min(bottom, top) - cp_least) <= 5e-3 * abs(cp_least) and abs(max(bottom, top) - cp_greatest) <= 5e-3 * abs(cp_greatest), (bottom, top, cp_least, cp_greatest)
    print(json.dumps({"decompressionBottomMPa": bottom, "decompressionTopMPa": top, "concretepropertiesDecompressionLeastMPa": cp_least, "vRdc": ours, "structuralcodesVRdc": theirs, "phi": ours_phi, "structuralcodesPhi": theirs_phi, "mRdKNm": m_ours, "structuralcodesMRdKNm": m_theirs, "sigmaSMPa": cracked_ours["sigma_s"], "concretepropertiesSigmaSMPa": cp_stress["sigma_s"], "sigmaCMPa": cracked_ours["sigma_c"], "concretepropertiesSigmaCMPa": cp_stress["sigma_c"], "prestressSigmaCMPa": ours_c, "concretepropertiesPrestressSigmaCMPa": cp_uncracked}))


if __name__ == "__main__":
    if "--self-check" in sys.argv:
        self_check()
    else:
        paths = [a for a in sys.argv[1:] if not a.startswith("--")]
        print(json.dumps(evaluate(json.load(open(paths[0], encoding="utf8") if paths else sys.stdin))))
