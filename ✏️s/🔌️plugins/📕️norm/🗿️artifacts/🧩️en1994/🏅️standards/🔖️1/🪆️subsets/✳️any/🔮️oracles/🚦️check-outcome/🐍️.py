"""🚦️ Independent EN 1994 document oracle: a committed snapshot (JSON on stdin or a path) to the utilisation of the composite beam checks per simply supported beam (`--json`, default: `{"checks":[{"id","utilization"}]}`; `--self-check`: the strip-integrated plastic modulus against sectionproperties). Written from EN 1990 6.10 and A1.4, EN 1994-1-1 5.4.1.2, 5.4.2.2, 6.2.1, 6.2.2, 6.6.1.2, 6.6.3.1, 6.6.4.2, 7.3.1 and Annex C by horizontal strip integration of the steel plates and a stress block for the slab, not from the Rust. Continuous spans are not recomputed. SI throughout."""

import importlib.util
import json
import math
import pathlib
import sys

import numpy as np


def _sibling(folder):
    """🔌 The oracle module of a sibling folder, loaded by path (the isolated interpreter has no script directory on its path)."""
    path = pathlib.Path(__file__).resolve().parent / folder / "🐍️.py"
    spec = importlib.util.spec_from_file_location("en1994_oracle_" + str(abs(hash(folder))), path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


CONTINUOUS, COLUMNS, SLABS, FAMILIES = _sibling("📉️continuous"), _sibling("🏛️columns"), _sibling("🧱️slabs"), _sibling("🧭️families")

E_A, E_S = 210e9, 200e9
GAMMA_G_SUP, GAMMA_G_INF = 1.35, 1.0
GAMMA = dict(v=1.25, c=1.5, s=1.15, m0=1.0)
EQUIVALENT_SPAN = {"simplySupported": 1.0, "continuous2Span": 0.85, "endSpan": 0.85, "interiorSpan": 0.7, "cantilever": 2.0}
REBAR_FSK, HOGGING_REBAR_DEPTH = 500e6, 0.04
DIVISOR = {"en": 250.0, "de": 300.0}
STRIPS = 2400


def psi(kind, category):
    """🎚️ The (ψ0, ψ1, ψ2) of EN 1990 Table A1.1 of a composite beam action."""
    if kind == "permanent":
        return 1.0, 1.0, 1.0
    if kind == "construction":
        return 1.0, 1.0, 0.0
    if kind == "imposed":
        return {"C": (0.7, 0.7, 0.6), "D": (0.7, 0.7, 0.6), "E": (1.0, 0.9, 0.8), "H": (0.0, 0.0, 0.0)}.get(category, (0.7, 0.5, 0.3))
    return {"snow": (0.5, 0.2, 0.0), "wind": (0.6, 0.2, 0.0), "fatigue": (0.0, 0.0, 0.0)}.get(kind, (0.7, 0.5, 0.3))


def internals(action, span, spacing):
    """📐 Largest moment and support shear of one action on a simply supported span; a point force stands at `xM` from the first support (unset: mid-span), M = F a b / L, V = F max(a, b) / L."""
    q = action["qAreaPa"] * spacing
    if abs(q) > 1e-12:
        return q * span ** 2 / 8, q * span / 2
    if abs(action["fKN"]) > 1.0:
        xi = min(max(action.get("xM", 0.0) / span, 0.0), 1.0) if action.get("xM", 0.0) > 0.0 else 0.5
        return action["fKN"] * xi * (1 - xi) * span, action["fKN"] * max(xi, 1 - xi)
    return 0.0, 0.0


def beam_internals(action, beam):
    """📐 (sagging, hogging, shear) of one action on the span of a beam, simply supported or continuous (None: a point load on a continuous span)."""
    if beam["support"] == "simplySupported":
        m, v = internals(action, beam["spanM"], beam["spacingM"])
        return m, 0.0, v
    return CONTINUOUS.internals(action, beam["spanM"], beam.get("adjacentSpanM", 0.0), beam["support"], beam["spacingM"])


def effects(beam, stage, mode):
    """⚖️ EN 1990 effects (sagging M, hogging M, shear V) of the actions of one stage, each the largest over the choices of the leading variable action: ultimate 6.10, characteristic, frequent."""
    staged = [a for a in beam["actions"] if a["stage"] in (stage, "any")]
    scale = 1.35 if mode == "uls" else 1.0
    permanent = [a for a in staged if a["kind"] == "permanent"]
    if mode == "uls" and beam["support"] != "simplySupported":
        factored = [CONTINUOUS.permanent_internals(a, beam["spanM"], beam.get("adjacentSpanM", 0.0), beam["support"], beam["spacingM"], GAMMA_G_SUP, GAMMA_G_INF) for a in permanent]
        base = tuple(sum(found[i] for found in factored) for i in range(3))
    else:
        base = tuple(scale * sum(beam_internals(a, beam)[i] for a in permanent) for i in range(3))
    var = [(a, beam_internals(a, beam)) for a in staged if a["kind"] in ("imposed", "snow", "wind", "construction")]
    k_lead, k_other = {"uls": (None, 0), "char": (None, 0), "freq": (1, 2)}[mode]
    gain = 1.5 if mode == "uls" else 1.0
    best = base
    for i, (lead, lead_effect) in enumerate(var):
        w = gain if k_lead is None else psi(lead["kind"], lead["category"])[k_lead]
        total = [base[k] + w * lead_effect[k] for k in range(3)]
        for j, (a, effect) in enumerate(var):
            if j != i:
                f = gain * psi(a["kind"], a["category"])[k_other]
                total = [total[k] + f * effect[k] for k in range(3)]
        best = tuple(max(best[k], total[k], key=abs) for k in range(3))
    return best


class Steel:
    """🧱 A doubly symmetric I section as three plates; the web thickness preserves the catalogue area."""

    def __init__(self, s, ys):
        self.h, self.b, self.tf, self.tw, self.area_cat, self.i_y = s["heightM"], s["widthM"], s["tfM"], s["twM"], s["aM2"], s["iYM4"]
        self.hw = max(self.h - 2 * self.tf, 1e-9)
        self.tw_eff = max(self.tw, (self.area_cat - 2 * self.b * self.tf) / self.hw)
        self.area = 2 * self.b * self.tf + self.hw * self.tw_eff
        self.ys = ys
        edges = np.concatenate([np.linspace(ys, ys + self.tf, STRIPS // 4 + 1)[:-1], np.linspace(ys + self.tf, ys + self.h - self.tf, STRIPS // 2 + 1)[:-1], np.linspace(ys + self.h - self.tf, ys + self.h, STRIPS // 4 + 1)])
        mid = 0.5 * (edges[:-1] + edges[1:])
        width = np.where(mid < ys + self.tf, self.b, np.where(mid > ys + self.h - self.tf, self.b, self.tw_eff))
        self.depth, self.area_strip = mid, width * np.diff(edges)

    def sagging(self, fyd, beff, hc, fcc, nc):
        """📐 Plastic sagging moment about the slab top for the concrete force nc (stress block, plastic neutral axis in the steel)."""
        nc = min(max(nc, 0.0), self.area * fyd, fcc * beff * hc)
        lo, hi = self.ys, self.ys + self.h
        for _ in range(80):
            y = 0.5 * (lo + hi)
            net = fyd * (np.sum(self.area_strip[self.depth > y]) - np.sum(self.area_strip[self.depth <= y]))
            lo, hi = (y, hi) if net > nc else (lo, y)
        y = 0.5 * (lo + hi)
        below, above = self.depth > y, self.depth <= y
        tension = fyd * np.sum(self.area_strip[below] * self.depth[below])
        compression = fyd * np.sum(self.area_strip[above] * self.depth[above])
        x = nc / (fcc * beff)
        return tension - compression - nc * x / 2


    def hogging(self, fyd, as_total, fsd, y_rebar):
        """📐 Plastic hogging moment about the slab top: the bars F_s = min(A_s f_sd, A_a f_yd) in tension at `y_rebar`, the plastic axis where the steel compression exceeds the steel tension by F_s."""
        fs = min(as_total * fsd, self.area * fyd)
        lo, hi = self.ys, self.ys + self.h
        for _ in range(80):
            y = 0.5 * (lo + hi)
            net = fyd * (np.sum(self.area_strip[self.depth > y]) - np.sum(self.area_strip[self.depth <= y]))
            lo, hi = (y, hi) if net > fs else (lo, y)
        y = 0.5 * (lo + hi)
        compression = fyd * np.sum(self.area_strip[self.depth > y] * self.depth[self.depth > y])
        tension = fyd * np.sum(self.area_strip[self.depth <= y] * self.depth[self.depth <= y])
        return compression - tension - fs * y_rebar


def plastic_moment(steel, fyd, beff, hc, fcc, nc):
    return steel.sagging(fyd, beff, hc, fcc, nc)


def required_force(steel, fyd, beff, hc, fcc, m_ed, n_full):
    """🎯 The smallest concrete force whose stress-block moment reaches m_ed, bounded by the full connection force."""
    if steel.sagging(fyd, beff, hc, fcc, 0.0) >= m_ed:
        return 0.0
    if steel.sagging(fyd, beff, hc, fcc, n_full) < m_ed:
        return n_full
    lo, hi = 0.0, n_full
    for _ in range(60):
        mid = 0.5 * (lo + hi)
        lo, hi = (mid, hi) if steel.sagging(fyd, beff, hc, fcc, mid) < m_ed else (lo, mid)
    return hi


def stud_resistance(stud, beam, nr):
    """🔩 P_Rd of a headed stud with `nr` studs per rib: min(P1, P2) / γv times the sheeting reduction (None when out of scope)."""
    d, hsc, fu = stud["diameterM"], stud["heightM"], stud["fUPa"]
    alpha = 1.0 if hsc / d > 4 else 0.2 * (hsc / d + 1)
    p1 = 0.8 * fu * math.pi * d * d / 4 / GAMMA["v"]
    p2 = 0.29 * alpha * d * d * math.sqrt(beam["concreteFCkPa"] * beam["concreteECmPa"]) / GAMMA["v"]
    sheeting = beam["sheeting"]
    hp = sheeting["heightM"]
    if hp <= 1e-9:
        return min(p1, p2)
    g = sheeting["ribWidthM"] / hp * (min(hsc, hp + 0.075) / hp - 1)
    if sheeting["ribsParallelToBeam"]:
        kt = min(max(0.6 * g, 0.0), 1.0)
    elif nr > 2:
        return None
    else:
        thin, welded = sheeting["thicknessM"] <= 1e-3 + 1e-12, d <= 0.020 + 1e-9
        kmax = {(True, 1, True): 0.85, (True, 1, False): 0.75, (True, 2, True): 0.70, (True, 2, False): 0.60, (False, 1, True): 1.0, (False, 1, False): 0.75, (False, 2, True): 0.80, (False, 2, False): 0.60}[(thin, nr, welded)]
        kt = min(max(0.7 / math.sqrt(nr) * g, 0.0), kmax)
    return min(p1, p2) * kt


def beam_checks(doc, beam, annex):
    """🧩 The composite beam utilisations of one beam: simply supported with the shear connection and deflection rows, continuous (end, interior, cantilever) with the resistances at the span and the support."""
    support = beam["support"]
    simple = support == "simplySupported"
    if not simple and (support not in CONTINUOUS.LAYOUTS or any(beam_internals(a, beam) is None for a in beam["actions"])):
        return []
    bid, span, spacing = beam["id"], beam["spanM"], beam["spacingM"]
    steel_in = beam["steel"]
    ys, fy = beam["slabThicknessM"], doc["steelFYPa"]
    fyd, fcc = fy / GAMMA["m0"], 0.85 * beam["concreteFCkPa"] / GAMMA["c"]
    hc = max(ys - beam["sheeting"]["heightM"], 0.04)
    beff = 2 * min(span * EQUIVALENT_SPAN[support] / 8, spacing / 2)
    steel = Steel(steel_in, ys)
    out = [(f"en1994.5.4.1.2.beff.{bid}", beff / spacing)]
    m_ed, h_ed, v_ed = effects(beam, "composite", "uls")
    m_c = effects(beam, "construction", "uls")[0]
    eps = math.sqrt(235e6 / fy)
    c_f = (steel.b - steel.tw) / 2 / steel.tf
    r_w = steel.hw / steel.tw
    cls = max(next((i + 1 for i, lim in enumerate((9, 10, 14)) if c_f <= lim * eps), 4), next((i + 1 for i, lim in enumerate((72, 83, 124)) if r_w <= lim * eps), 4))
    rows = FAMILIES.beam_studs(doc, beam)
    sizes = {r["id"]: len(g) for g in FAMILIES.station_groups(rows) for r in g}
    resist = {r["id"]: stud_resistance(r, beam, sizes[r["id"]]) for r in rows}
    inside = [r for r in rows if FAMILIES.in_span(beam, r)]
    in_scope = all(resist[r["id"]] is not None for r in inside)
    p_of = lambda r: resist[r["id"]] or 0.0
    capacity = min(FAMILIES.half_resistances(beam, rows, p_of))
    p_rd = capacity if in_scope else None
    n_full = max(min(steel.area * fyd, fcc * beff * hc), 1.0)
    if p_rd is not None and not simple:
        nc = min(capacity, n_full)
        if cls <= 2:
            out.append((f"en1994.6.2.1.3.mrd.{bid}", m_ed / plastic_moment(steel, fyd, beff, hc, fcc, nc)))
    if not simple:
        out += hogging(beam, steel, ys, fyd, spacing, span, h_ed)
        out += tension_stiffening(beam, steel, ys, hc, spacing, span)
    if p_rd is not None and simple:
        cap = capacity
        nc = min(cap, n_full)
        if cls <= 2:
            out.append((f"en1994.6.2.1.3.mrd.{bid}", m_ed / plastic_moment(steel, fyd, beff, hc, fcc, nc)))
        n_req = required_force(steel, fyd, beff, hc, fcc, m_ed, n_full)
        if inside:
            out.append((f"en1994.6.6.3.1.prd.{bid}", n_req / cap if cap > 0 else math.inf))
        ductile = bool(inside) and all(0.016 - 1e-9 <= r["diameterM"] <= 0.025 + 1e-9 and r["heightM"] >= 4 * r["diameterM"] - 1e-9 for r in inside)
        if ductile:
            out.append((f"en1994.6.6.2.perstud.{bid}", FAMILIES.per_stud_utilisation(beam, rows, p_of, n_req / (span / 2))))
        eta_min = 1.0 if not ductile or span > 25 else max(1 - 355e6 / fy * (0.75 - 0.03 * span), 0.4)
        eta = min(cap / n_full, 1.0)
        out.append((f"en1994.6.6.1.2.etamin.{bid}", math.inf if eta <= 0 else max(eta_min / eta, 1.0) if eta < eta_min else eta_min / eta))
    out += families(beam, rows, steel, ys, fy, fyd, fcc, beff, hc, v_ed, h_ed, m_ed, span, support)
    m_pla = steel_in["wPlYM3"] * fyd
    out.append((f"en1994.6.2.1.construction.{bid}", m_c / m_pla))
    v_pl = max(steel_in["heightM"] * steel_in["twM"], steel_in["aVM2"]) * fy / math.sqrt(3) / GAMMA["m0"]
    out.append((f"en1994.6.2.2.vpl.{bid}", abs(v_ed) / v_pl))
    if simple:
        out += deflection(beam, annex, steel, ys, beff, hc, spacing, span)
    return out


def families(beam, rows, steel, ys, fy, fyd, fcc, beff, hc, v_ed, h_ed, m_ed, span, support):
    """🧭 The class, stud spacing and layout, slab longitudinal shear, bending-shear interaction and SLS stress rows of one beam (status `warning` for an undeclared stud layout)."""
    bid, simple = beam["id"], support == "simplySupported"
    out = [(f"en1994.5.5.class.{bid}", FAMILIES.section_class(beam["steel"], fy) / 3)]
    spacing = FAMILIES.stud_spacing(beam, hc, rows)
    if spacing is not None:
        out.append((f"en1994.6.6.5.5.spacing.{bid}", spacing))
    total, inside = FAMILIES.stud_layout(beam, rows)
    out.append((f"en1994.6.6.1.1.layout.{bid}", total / max(inside, 1)))
    n_full = max(min(steel.area * fyd, fcc * beff * hc), 1.0)
    n_req = required_force(steel, fyd, beff, hc, fcc, m_ed, n_full)
    out.append((f"en1994.6.6.6.vlrd.{bid}", FAMILIES.slab_longitudinal_shear(n_req, span * EQUIVALENT_SPAN[support], hc, beam["concreteFCkPa"], GAMMA["c"], GAMMA["s"], beam["transverseAsM2PerM"])))
    v_pl = max(beam["steel"]["heightM"] * beam["steel"]["twM"], beam["steel"]["aVM2"]) * fy / math.sqrt(3) / GAMMA["m0"]
    uniform = all(abs(a["fKN"]) <= 1.0 for a in beam["actions"])
    if simple and uniform:
        out.append((f"en1994.6.2.2.4.vm-sagging.{bid}", FAMILIES.sagging_interaction(steel, fyd, beff, hc, fcc, m_ed, abs(v_ed), v_pl)))
    if not simple:
        adjacent = beam.get("adjacentSpanM", 0.0)
        le = 0.25 * (span + (adjacent if adjacent > 1e-9 else span))
        as_total = beam["asHoggingM2PerM"] * 2 * min(le / 8, beam["spacingM"] / 2)
        m_v = FAMILIES.reduced_hogging(steel, fyd, as_total, REBAR_FSK / GAMMA["s"], HOGGING_REBAR_DEPTH, FAMILIES.rho(abs(v_ed), v_pl))
        out.append((f"en1994.6.2.2.4.vm.{bid}", abs(h_ed) / m_v))
    if support != "cantilever":
        out.append(stress(beam, steel, ys, beff, hc, fy))
    return out


def stress(beam, steel, ys, beff, hc, fy):
    """🧵 The staged SLS characteristic stress row (7.2.2): permanent actions on the long-term section, the rest short-term, the construction stage on the steel alone when unpropped."""
    n0, nl = E_A / beam["concreteECmPa"], E_A / beam["concreteECmPa"] * (1 + 1.1 * 2.5)
    m_g = sum(beam_internals(a, beam)[0] for a in beam["actions"] if a["kind"] == "permanent" and a["stage"] in ("composite", "any"))
    m_q = max(effects(beam, "composite", "char")[0] - m_g, 0.0)
    m_a = effects(beam, "construction", "char")[0] if beam["construction"] == "unpropped" else 0.0
    return (f"en1994.7.2.2.stress.{beam['id']}", FAMILIES.stress_limit(steel, ys, beff, hc, n0, nl, m_a, m_g, m_q, fy, beam["concreteFCkPa"], lambda *args: transformed(*args)))


def hogging(beam, steel, ys, fyd, spacing, span, h_ed):
    """🪝 The plastic hogging resistance (steel section and the bars of the effective width at the support, equivalent hogging span 0.25 (L + L_adjacent)) against the hogging moment."""
    adjacent = beam.get("adjacentSpanM", 0.0)
    le = 0.25 * (span + (adjacent if adjacent > 1e-9 else span))
    as_total = beam["asHoggingM2PerM"] * 2 * min(le / 8, spacing / 2)
    if as_total <= 0.0:
        return []
    return [(f"en1994.6.2.1.2.hogging.{beam['id']}", abs(h_ed) / steel.hogging(fyd, as_total, REBAR_FSK / GAMMA["s"], HOGGING_REBAR_DEPTH))]


FCTM_C50 = 0.30
TENSION_STIFFENING_K = 0.4


def fctm(fck_pa):
    """🧱 EN 1992-1-1 Table 3.1: f_ctm = 0.30 f_ck^(2/3) up to C50/60, 2.12 ln(1 + f_cm/10) above (MPa in, Pa out)."""
    c = fck_pa / 1e6
    return (FCTM_C50 * c ** (2 / 3) if c <= 50 else 2.12 * math.log(1 + (c + 8) / 10)) * 1e6


def tension_stiffening(beam, steel, ys, hc, spacing, span):
    """🧵 EN 1994-1-1 7.4.3: the bar stress of the hogging zone sigma_s = sigma_s,0 + Delta sigma_s against 0.8 f_sk. sigma_s,0 = (E_s/E_a) M (y_na - d_r) / I on the cracked steel section with the bars, Delta sigma_s = 0.4 f_ctm / (alpha_st rho_s), alpha_st = A I / (A_a I_a), rho_s = A_s / A_ct with A_ct = b_eff h_c."""
    m_hog = abs(effects(beam, "composite", "char")[1])
    adjacent = beam.get("adjacentSpanM", 0.0)
    beff_h = 2 * min(0.25 * (span + (adjacent if adjacent > 1e-9 else span)) / 8, spacing / 2)
    as_total = beam["asHoggingM2PerM"] * beff_h
    if m_hog <= 1.0 or beam["asHoggingM2PerM"] <= 1e-12:
        return []
    a_a, i_a = steel.area, steel.i_y
    bars = as_total * E_S / E_A
    y_steel = ys + steel.h / 2
    area = a_a + bars
    y_na = (a_a * y_steel + bars * HOGGING_REBAR_DEPTH) / area
    inertia = i_a + a_a * (y_steel - y_na) ** 2 + bars * (HOGGING_REBAR_DEPTH - y_na) ** 2
    sigma_0 = m_hog * E_S / E_A * (y_na - HOGGING_REBAR_DEPTH) / inertia
    rho_s = as_total / max(beff_h * hc, 1e-9)
    delta = TENSION_STIFFENING_K * fctm(beam["concreteFCkPa"]) / ((area * inertia / (a_a * i_a)) * max(rho_s, 1e-12))
    return [(f"en1994.7.4.3.tension-stiffening.{beam['id']}", (sigma_0 + delta) / (0.8 * REBAR_FSK))]


def transformed(steel, ys, beff, hc, n):
    """📐 Area, neutral axis depth below the slab top and second moment of the steel section plus the concrete block hc / n."""
    ac, ic, ya = beff * hc / n, beff * hc ** 3 / 12 / n, ys + steel.h / 2
    area = steel.area + ac
    y_na = (steel.area * ya + ac * hc / 2) / area
    return y_na, steel.i_y + steel.area * (ya - y_na) ** 2 + ic + ac * (hc / 2 - y_na) ** 2


def deflection(beam, annex, steel, ys, beff, hc, spacing, span):
    """🌊 Construction, permanent, variable and shrinkage deflection of a simply supported beam (7.3.1) against L/250 or L/300."""
    if any(abs(a["fKN"]) > 1.0 for a in beam["actions"]):
        return []
    ecm = beam["concreteECmPa"]
    n0, nl, ns = E_A / ecm, E_A / ecm * (1 + 1.1 * 2.5), E_A / ecm * (1 + 0.55 * 2.5)
    unit = span ** 2 / 8
    m_g = sum(internals(a, span, spacing)[0] for a in beam["actions"] if a["kind"] == "permanent" and a["stage"] in ("composite", "any"))
    m_freq = effects(beam, "composite", "freq")[0]
    g, q = m_g / unit, max(m_freq - m_g, 0.0) / unit
    shape = 5 * span ** 4 / 384 / E_A
    d_perm = shape * g / transformed(steel, ys, beff, hc, nl)[1]
    d_var = shape * q / transformed(steel, ys, beff, hc, n0)[1]
    d_c = 0.0
    if beam["construction"] == "unpropped":
        d_c = shape * (effects(beam, "construction", "char")[0] / unit) / steel.i_y
    y_s, i_s = transformed(steel, ys, beff, hc, ns)
    d_sh = 0.3e-3 * (beff * hc / ns) * max(y_s - hc / 2, 0.0) / i_s * span ** 2 / 8
    return [(f"en1994.7.3.1.deflection.{beam['id']}", abs(d_c + d_perm + d_var + d_sh) / (span / DIVISOR[annex])), (f"en1994.7.3.1.deflection-variable.{beam['id']}", abs(d_var) / (span / 300))]


def slab_checks(slab):
    """🧱 The slab rows: the design moment and shear of a one metre strip on its layout (point loads are not covered)."""
    support = slab["support"]
    point = [abs(a["fKN"]) for a in slab["actions"] if abs(a["fKN"]) > 1.0]
    if point:
        q = SLABS.Slab(slab)
        return [(f"en1994.9.7.6.punching.{slab['id']}", FAMILIES.punching(slab, 1.5 * max(point), GAMMA["c"], q.dp, q.ap, q.bo, max(slab["concreteThicknessM"] - slab["sheeting"]["heightM"], 0.04)))]
    strip = {"spanM": slab["spanM"], "spacingM": 1.0, "support": support, "adjacentSpanM": 0.0, "actions": slab["actions"]}
    if support != "simplySupported" and support not in CONTINUOUS.LAYOUTS or any(beam_internals(a, strip) is None or abs(a["fKN"]) > 1.0 for a in slab["actions"]):
        return []
    m_ed, _, v_ed = effects(strip, "composite", "uls")
    return SLABS.checks(slab, m_ed, v_ed)


def evaluate(doc):
    """🚦️ The check rows of every composite beam, column and slab of a snapshot."""
    annex = doc["annex"].lower()
    rows = [row for beam in doc["beams"] for row in beam_checks(doc, beam, annex)]
    rows += [row for col in doc["columns"] for row in COLUMNS.checks(doc, col)]
    rows += [row for slab in doc["slabs"] for row in slab_checks(slab)]
    warning = lambda i, u: {"status": "warning"} if ".layout." in i and u > 1.0 + 1e-9 else {}
    unverified = lambda i, u: {"status": "fail" if u > 1.0 + 1e-9 else "unverifiedBasis"} if ".tension-stiffening." in i else {}
    return {"checks": [{"id": i, "utilization": u, **unverified(i, u), **warning(i, u)} for i, u in rows]}


def root_radius(h, b, tf, tw, area):
    """🔘 The root radius implied by a rolled section area: the two flanges, the web and (4 - pi) r^2 of the four fillets."""
    return math.sqrt(max(area - 2 * b * tf - (h - 2 * tf) * tw, 0.0) / (4 - math.pi))


def flange_outstand_ratio(b, tw, tf, r):
    """📏 c / t_f of an I flange outstand with c = (b - t_w) / 2 - r (EN 1993-1-1 Table 5.2)."""
    return ((b - tw) / 2 - r) / tf


def self_check():
    """🧪 The steel-only plastic moment of the strip integration against the plastic modulus of a sectionproperties I section."""
    assert abs(TENSION_STIFFENING_K * fctm(30e6) / (8 * 0.015) - 9654893.85) < 5.0
    from sectionproperties.analysis import Section
    from sectionproperties.pre.library import i_section

    steel = Steel({"heightM": 0.3, "widthM": 0.3, "tfM": 0.019, "twM": 0.011, "aM2": 2 * 0.3 * 0.019 + 0.262 * 0.011, "iYM4": 0.0}, 0.14)
    ours = steel.sagging(355e6, 1.0, 0.1, 1e7, 0.0) / 355e6
    geometry = i_section(d=300.0, b=300.0, t_f=19.0, t_w=11.0, r=0.0, n_r=1)
    geometry.create_mesh(mesh_sizes=[20.0])
    section = Section(geometry)
    section.calculate_geometric_properties()
    section.calculate_plastic_properties()
    theirs = section.get_s()[0] * 1e-9
    assert abs(ours - theirs) <= 5e-3 * theirs, (ours, theirs)
    rolled = i_section(d=300.0, b=300.0, t_f=19.0, t_w=11.0, r=27.0, n_r=16)
    rolled.create_mesh(mesh_sizes=[20.0])
    rolled_section = Section(rolled)
    rolled_section.calculate_geometric_properties()
    radius = root_radius(300.0, 300.0, 19.0, 11.0, rolled_section.get_area())
    assert abs(radius - 27.0) <= 2e-2 * 27.0, radius
    ratio = flange_outstand_ratio(300.0, 11.0, 19.0, 27.0)
    assert abs(ratio - 117.5 / 19.0) <= 1e-12 and ratio <= 9.0 * math.sqrt(235.0 / 355.0), ratio
    print(json.dumps({"wPl": ours, "sectionproperties": theirs, "rootRadiusMm": radius, "flangeOutstandRatio": ratio, "column": COLUMNS.self_check(), "studs": FAMILIES.stud_self_check(), "families": FAMILIES.self_check(Steel, transformed)}))


if __name__ == "__main__":
    if "--self-check" in sys.argv:
        self_check()
    else:
        paths = [a for a in sys.argv[1:] if not a.startswith("--")]
        print(json.dumps(evaluate(json.load(open(paths[0], encoding="utf8") if paths else sys.stdin))))
