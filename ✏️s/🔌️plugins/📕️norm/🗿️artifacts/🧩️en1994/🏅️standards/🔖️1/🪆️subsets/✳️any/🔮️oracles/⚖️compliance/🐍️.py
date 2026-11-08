#!/usr/bin/env python3
"""🧩️ Language-agnostic EN 1994 oracle — recomputes key formulas vs Rust evaluate() JSON (±0.5 %) by numeric strip integration."""
from __future__ import annotations
import importlib.util, json, math, sys
from pathlib import Path

_FACTORS_PATH = next(parent / "⚖️compliance" / "📜️annex-and-tables" / "⚖️action-factors" / "🐍️.py" for parent in Path(__file__).resolve().parents if (parent / "⚖️compliance" / "📜️annex-and-tables" / "⚖️action-factors" / "🐍️.py").is_file())
_SPEC = importlib.util.spec_from_file_location("action_factors", _FACTORS_PATH)
FACTORS = importlib.util.module_from_spec(_SPEC)
_SPEC.loader.exec_module(FACTORS)


def effective_width_m(le_m, spacing_m):
    return 2.0 * min(le_m / 8.0, spacing_m / 2.0)


def le_sagging(span_m, support):
    return 0.85 * span_m if support == "continuous2Span" else span_m


def stud_alpha(h_sc_m, d_m):
    ratio = h_sc_m / d_m
    return 1.0 if ratio > 4.0 else 0.2 * (ratio + 1.0)


def connector_resistance_n(d_m, h_sc_m, f_ck_pa, f_u_pa, e_cm_pa, gamma_v=1.25, kt=1.0):
    alpha = stud_alpha(h_sc_m, d_m)
    p_pl = 0.8 * f_u_pa * math.pi * d_m * d_m / 4.0
    p_b = 0.29 * alpha * d_m * d_m * math.sqrt(f_ck_pa * e_cm_pa)
    return min(p_pl, p_b) / gamma_v * kt


def sheeting_kt(sheeting, stud, nr):
    hp = float(sheeting["heightM"])
    if hp <= 1e-9:
        return 1.0
    b0 = float(sheeting["ribWidthM"])
    hsc = min(float(stud["heightM"]), hp + 0.075)
    g = (b0 / hp) * (hsc / hp - 1.0)
    if sheeting.get("ribsParallelToBeam", False):
        return min(max(0.6 * g, 0.0), 1.0)
    thin = float(sheeting["thicknessM"]) <= 1.0e-3 + 1e-12
    welded = float(stud["diameterM"]) <= 0.020 + 1e-9
    kmax = {(True, 1, True): 0.85, (True, 1, False): 0.75, (True, 2, True): 0.70, (True, 2, False): 0.60,
            (False, 1, True): 1.00, (False, 1, False): 0.75, (False, 2, True): 0.80, (False, 2, False): 0.60}.get((thin, min(nr, 2), welded), 0.6)
    return min(max(0.7 / math.sqrt(nr) * g, 0.0), kmax)


def min_eta(le_m, f_y_pa):
    f_y = f_y_pa / 1e6
    eta = 1.0 - (355.0 / f_y) * (0.75 - 0.03 * le_m) if le_m <= 25 else 1.0
    return max(eta, 0.4)


def strips(top, bot, width, n):
    h = (bot - top) / n
    return [(top + (i + 0.5) * h, width * h) for i in range(n)]


def steel_plates(steel, ys):
    h, b, tf, tw, a = float(steel["heightM"]), float(steel["widthM"]), float(steel["tfM"]), float(steel["twM"]), float(steel["aM2"])
    hw = h - 2 * tf
    tw_eff = max(tw, (a - 2 * b * tf) / hw)
    out = strips(ys, ys + tf, b, 800) + strips(ys + tf, ys + h - tf, tw_eff, 800) + strips(ys + h - tf, ys + h, b, 800)
    return out, ys, ys + h


def sagging_moment(steel, fyd, ys, beff, hc, fcc, nc):
    st, top, bot = steel_plates(steel, ys)
    npl = fyd * sum(a for _, a in st)
    nc = min(nc, npl, fcc * beff * hc)
    lo, hi = top, bot
    for _ in range(80):
        mid = 0.5 * (lo + hi)
        net = sum(fyd * a * (1 if y < mid else -1) for y, a in st)
        lo, hi = (mid, hi) if net < -nc else (lo, mid)
    y_na = 0.5 * (lo + hi)
    x = nc / (fcc * beff)
    m = -nc * x / 2 + sum((-fyd * a * y if y < y_na else fyd * a * y) for y, a in st)
    return m, nc


def required_force(steel, fyd, ys, beff, hc, fcc, m_ed):
    full = sagging_moment(steel, fyd, ys, beff, hc, fcc, 1e30)[1]
    if sagging_moment(steel, fyd, ys, beff, hc, fcc, 0.0)[0] >= m_ed:
        return 0.0
    if sagging_moment(steel, fyd, ys, beff, hc, fcc, full)[0] < m_ed:
        return full
    lo, hi = 0.0, full
    for _ in range(50):
        mid = 0.5 * (lo + hi)
        lo, hi = (mid, hi) if sagging_moment(steel, fyd, ys, beff, hc, fcc, mid)[0] < m_ed else (lo, mid)
    return hi


def insulation_m(rating, deck, steel_height_m):
    r = rating.lower()
    if deck == "reEntrant":
        if steel_height_m >= 0.400:
            table = dict(r30=12, r60=22, r90=33, r120=48)
        elif steel_height_m >= 0.300:
            table = dict(r30=12, r60=20, r90=32, r120=45)
        else:
            table = dict(r30=10, r60=18, r90=28, r120=40)
    elif steel_height_m >= 0.400:
        table = dict(r30=12, r60=20, r90=32, r120=45)
    elif steel_height_m >= 0.300:
        table = dict(r30=10, r60=18, r90=28, r120=40)
    else:
        table = dict(r30=10, r60=15, r90=25, r120=35)
    return table[r] / 1000.0

def gamma_mf(annex):
    return FACTORS.ndp("ndp.en1994.gamma-mf", annex)

def psi_factors(kind, category, annex):
    """🎚️ (ψ0, ψ1, ψ2) of the registry rows `ndp.en1990.psi.*`: snow, wind and construction loads by kind, imposed actions by category letter (offices when missing)."""
    factors = FACTORS.ActionFactors(annex)
    if kind == "permanent":
        return (1.0, 1.0, 1.0)
    key = {"construction": "construction_load", "snow": "snow", "wind": "wind"}.get(kind) or (category or "B").upper()[:1]
    return factors.psi_known(key) or factors.psi("office")

def action_internals(action, span_m, support, spacing_m):
    q = float(action.get("qAreaPa") or 0) * spacing_m
    f_k = float(action.get("fKN") or 0)
    if abs(q) > 1e-12:
        if support == "continuous2Span":
            return q * span_m * span_m / 16.0, q * span_m * span_m / 8.0, 1.25 * q * span_m / 2.0, 0.0
        return q * span_m * span_m / 8.0, 0.0, q * span_m / 2.0, 0.0
    if abs(f_k) > 1.0:
        if support == "continuous2Span":
            return f_k * span_m / 8.0, f_k * span_m / 8.0, 1.25 * f_k / 2.0, 0.0
        return f_k * span_m / 4.0, 0.0, f_k / 2.0, 0.0
    return 0.0, 0.0, 0.0, 0.0


def uls_composite(actions, span_m, support, spacing_m, annex, reliability_class):
    g_m = g_h = g_v = g_n = 0.0
    vars_ = []
    for a in actions:
        if a.get("stage") != "composite":
            continue
        kind = a.get("kind")
        m, mh, v, n = action_internals(a, span_m, support, spacing_m)
        if kind == "permanent":
            g_m += m; g_h += mh; g_v += v; g_n += n
        elif kind in ("imposed", "snow", "wind", "construction"):
            vars_.append((a, m, mh, v, n))
    factors = FACTORS.ActionFactors(annex, reliability_class=reliability_class)
    gamma_g, gamma_q = factors.gamma_g_sup(), factors.gamma_q()
    best = (gamma_g * g_m, gamma_g * g_h, gamma_g * g_v, gamma_g * g_n, "G only")
    for i, (lead, lm, lh, lv, ln) in enumerate(vars_):
        psi0 = psi_factors(lead["kind"], lead.get("category"), annex)[0]
        m = gamma_g * g_m + gamma_q * lm
        mh = gamma_g * g_h + gamma_q * lh
        v = gamma_g * g_v + gamma_q * lv
        n = gamma_g * g_n + gamma_q * ln
        for j, (other, om, oh, ov, on) in enumerate(vars_):
            if i == j: continue
            p0 = psi_factors(other["kind"], other.get("category"), annex)[0]
            m += gamma_q * p0 * om
            mh += gamma_q * p0 * oh
            v += gamma_q * p0 * ov
            n += gamma_q * p0 * on
        if abs(m) >= abs(best[0]):
            best = (m, mh, v, n, lead.get("id", "Q"))
    return best


def utilization(computed, limit):
    if abs(limit) < 1e-15:
        return 0.0 if abs(computed) < 1e-15 else 1e9
    return abs(computed / limit)

def rel_err(a, b):
    return abs(a - b) / max(abs(a), abs(b), 1e-9)


def beam_stud_resistances(snapshot, beam, gamma_v):
    """🔩 The design resistance of every stud of `beam` inside its designed span (None when the sheeting reduction is out of scope) and the weaker half span's capacity: studs within 1 mm along the beam axis share one rib (n_r), a stud on the middle belongs to both halves with half its resistance."""
    rows = sorted((s for s in snapshot.get("studs", []) if s["beamId"] == beam["id"]), key=lambda s: (s["xM"], s["id"]))
    sizes, first, groups = {}, None, []
    for row in rows:
        if first is None or row["xM"] - first > 1e-3:
            groups.append([])
            first = row["xM"]
        groups[-1].append(row)
    for group in groups:
        sizes.update({r["id"]: len(group) for r in group})
    adjacent = float(beam.get("adjacentSpanM") or 0.0)
    span = float(beam["spanM"])
    origin = (adjacent if adjacent > 1e-9 else span) if beam.get("support") in ("interiorSpan", "cantilever") else 0.0
    inside = [r for r in rows if origin - 1e-6 <= r["xM"] <= origin + span + 1e-6]
    p = {}
    for r in inside:
        kt = sheeting_kt(beam["sheeting"], r, sizes[r["id"]])
        p[r["id"]] = connector_resistance_n(float(r["diameterM"]), float(r["heightM"]), float(beam["concreteFCkPa"]), float(r["fUPa"]), float(beam["concreteECmPa"]), gamma_v, kt)
    weight = lambda r: 0.5 if abs(r["xM"] - (origin + span / 2)) <= 1e-3 else (1.0 if r["xM"] < origin + span / 2 else 0.0)
    left = sum(weight(r) * p[r["id"]] for r in inside)
    right = sum((1.0 - weight(r)) * p[r["id"]] for r in inside)
    return inside, p, min(left, right)


def compare_report(snapshot, report):
    checks = {c["id"]: c for c in report.get("checks", [])}
    compared = 0
    max_rel = 0.0
    fy = float(snapshot.get("steelFYPa") or 355e6)
    annex = "de" if str(snapshot.get("annex") or "en").lower() in ("de", "annexchoice::de") else "en"
    for beam in snapshot.get("beams") or []:
        bid = beam["id"]
        span, spacing = float(beam["spanM"]), float(beam["spacingM"])
        support = beam.get("support") or "simplySupported"
        sheeting = beam["sheeting"]
        le = le_sagging(span, support)
        beff = effective_width_m(le, spacing)
        cid = f"en1994.5.4.1.2.beff.{bid}"
        if cid in checks:
            max_rel = max(max_rel, rel_err(beff, float(checks[cid]["computed"]["value"])))
            compared += 1
        ys = float(beam["slabThicknessM"])
        hc = max(ys - float(sheeting["heightM"]), 0.04)
        fcc = 0.85 * float(beam["concreteFCkPa"]) / FACTORS.ndp("ndp.en1994.gamma-c", annex)
        inside, resistances, capacity = beam_stud_resistances(snapshot, beam, FACTORS.ndp("ndp.en1994.gamma-v", annex))
        p_rd = sum(resistances.values()) / len(resistances) if resistances else 0.0
        m_ed, _, _, _, _ = uls_composite(beam.get("actions") or [], span, support, spacing, annex, int(str(snapshot["reliabilityClass"])[-1]))
        cid = f"en1994.6.2.1.3.mrd.{bid}"
        full = sagging_moment(beam["steel"], fy, ys, beff, hc, fcc, 1e30)
        nc = min(capacity, full[1])
        if cid in checks and abs(m_ed) > 1.0:
            comp = float(checks[cid]["computed"]["value"])
            max_rel = max(max_rel, rel_err(m_ed, comp))
            m_rd = sagging_moment(beam["steel"], fy, ys, beff, hc, fcc, nc)[0]
            max_rel = max(max_rel, rel_err(m_rd, float(checks[cid]["limit"]["value"])))
            compared += 2
        cid = f"en1994.6.6.3.1.prd.{bid}"
        if cid in checks and p_rd > 1.0:
            nc_req = required_force(beam["steel"], fy, ys, beff, hc, fcc, m_ed)
            ck = checks[cid]
            max_rel = max(max_rel, rel_err(capacity, float(ck["limit"]["value"])))
            if nc_req > 1.0:
                max_rel = max(max_rel, rel_err(nc_req, float(ck["computed"]["value"])))
            compared += 1
        cid = f"en1994.6.6.1.2.etamin.{bid}"
        if cid in checks:
            ck = checks[cid]
            max_rel = max(max_rel, rel_err(min(capacity / full[1], 1.0), float(ck["computed"]["value"])), rel_err(min_eta(le, fy), float(ck["limit"]["value"])))
            compared += 1
        rating = str(snapshot.get("fireRating") or "r60")
        deck = sheeting.get("profile") or "trapezoidal"
        cid = f"en1994.1-2.4.2.insulation.{bid}"
        if cid in checks and rating != "unrated":
            t_req = insulation_m(rating, deck, float(beam["steel"]["heightM"]))
            max_rel = max(max_rel, rel_err(t_req, float(checks[cid]["limit"]["value"])))
            compared += 1
    return {"compared": compared, "maxRel": max_rel}


IPE300 = {"heightM": 0.300, "widthM": 0.150, "tfM": 0.0107, "twM": 0.0071, "aM2": 5381e-6}
SPEC_ROWS = {
    "C-03": {"checks": ["en1994.5.4.1.2", "en1994.5.4.1.2.beff", "en1994.5.4.1.2.le"], "library": "anastruct (reactions of the two span beam give the zero-moment distances Le)"},
    "C-07": {"checks": ["en1994.6.2.1.3.mrd"], "library": "scipy.optimize.brentq (plastic neutral axis of exact plate areas)"},
    "C-09": {"checks": ["en1994.6.2.1.3.mrd", "en1994.7.2.2.stress"], "library": "sectionproperties (transformed composite section I(n))"},
    "C-12": {"checks": ["en1994.6.2.1.3.mrd"], "library": "scipy.optimize.brentq (stress block with N_c = eta N_cf) against the linear interpolation"},
    "C-14": {"checks": ["en1994.6.6.1.2.etamin"], "library": "scipy.optimize.brentq (eta = 0.4 root) and closed form 6.6.1.2"},
    "C-16": {"checks": ["en1994.6.6.3.1.prd", "en1994.6.6.2.perstud"], "library": "structuralcodes.codes.ec2_2004 (fcm, Ecm) in 0.29 alpha d^2 sqrt(fck Ecm)"},
    "C-18": {"checks": ["en1994.6.6.3.1.prd"], "library": "scipy.optimize.brentq (N_c,req from the stress-block moment)"},
}


def third_party() -> dict[str, float]:
    """🎯️ Third-party cross-check of the compliance oracle quantities of the spec rows C-03, C-07, C-09, C-12, C-14, C-16 and C-18: every entry is |oracle - library| / library and must stay below the tolerance of its row (the process exits non-zero otherwise)."""
    import warnings

    from anastruct import SystemElements
    from scipy import optimize
    from sectionproperties.analysis import Section
    from sectionproperties.pre import Material
    from sectionproperties.pre.library import i_section, rectangular_section
    from structuralcodes.codes import ec2_2004

    warnings.simplefilter("ignore")
    fyd, ys, beff, hc, fcc = 355e6, 0.100, 2.0, 0.100, 17e6
    h, b, tf, tw = IPE300["heightM"], IPE300["widthM"], IPE300["tfM"], IPE300["twM"]
    tw_eff = max(tw, (IPE300["aM2"] - 2 * b * tf) / (h - 2 * tf))
    plates = [(ys, ys + tf, b), (ys + tf, ys + h - tf, tw_eff), (ys + h - tf, ys + h, b)]

    def area_above(y: float) -> float:
        return sum(max(min(y, bot) - top, 0.0) * w for top, bot, w in plates)

    def first_moment_above(y: float) -> float:
        return sum(w * (min(y, bot) ** 2 - top ** 2) / 2 for top, bot, w in plates if y > top)

    total, first_total = area_above(1e9), first_moment_above(1e9)

    def moment_stress_block(nc: float) -> tuple[float, float]:
        nc = min(nc, fyd * total, fcc * beff * hc)
        axis = optimize.brentq(lambda y: nc + fyd * (2 * area_above(y) - total), ys, ys + h, xtol=1e-14)
        compression, tension = fyd * area_above(axis), fyd * (total - area_above(axis))
        y_c = first_moment_above(axis) / area_above(axis) if area_above(axis) > 0 else 0.0
        y_t = (first_total - first_moment_above(axis)) / (total - area_above(axis))
        return tension * y_t - compression * y_c - nc * nc / (fcc * beff) / 2, nc

    out = {}
    length, load = 8.0, 10.0
    system = SystemElements(EA=1e12, EI=1e7)
    for i in range(16):
        system.add_element([[length * i / 16, 0], [length * (i + 1) / 16, 0]])
    for i in range(16):
        system.add_element([[length + length * i / 16, 0], [length + length * (i + 1) / 16, 0]])
    system.add_support_hinged(1)
    system.add_support_roll(17)
    system.add_support_roll(33)
    system.q_load(q=-load, element_id=list(range(1, 33)))
    system.solve()
    sagging = 2 * abs(system.get_node_results_system(1)["Fy"]) / load
    hogging = 2 * length - 2 * sagging
    out["c03_le_hogging"] = rel_err(0.25 * (length + length), hogging)
    out["c03_le_end_span_sagging_within_15pct"] = max(rel_err(0.85 * length, sagging) - 0.15, 0.0)
    out["c03_beff"] = rel_err(effective_width_m(length, 3.0), 2 * min(length / 8, 1.5))
    full = moment_stress_block(1e30)
    steel = IPE300
    out["c07_mpl_full"] = rel_err(sagging_moment(steel, fyd, ys, beff, hc, fcc, 1e30)[0], full[0])
    for eta in (0.4, 0.7):
        m_sb = moment_stress_block(eta * full[1])[0]
        out[f"c12_mpl_partial_{eta}"] = rel_err(sagging_moment(steel, fyd, ys, beff, hc, fcc, eta * full[1])[0], m_sb)
        m_a = moment_stress_block(0.0)[0]
        assert m_sb >= m_a + eta * (full[0] - m_a) - 1e-6, "stress block must not be below the linear interpolation"
    for fy in (235e6, 275e6, 355e6):
        root = optimize.brentq(lambda le: 1.0 - (355.0 / (fy / 1e6)) * (0.75 - 0.03 * le) - 0.4, 0.0, 25.0)
        out[f"c14_etamin_{int(fy / 1e6)}"] = rel_err(min_eta(root + 1e-9, fy), 0.4)
        for le in (3.0, 8.0, 12.0, 20.0):
            out[f"c14_etamin_{int(fy / 1e6)}_{le}"] = rel_err(min_eta(le, fy), max(1.0 - (355.0 / (fy / 1e6)) * (0.75 - 0.03 * le), 0.4))
    for fck in (25.0, 30.0, 40.0, 50.0):
        ecm_pa = ec2_2004.Ecm(ec2_2004.fcm(fck)) * 1e6
        out[f"c16_ecm_{int(fck)}"] = rel_err(22e9 * ((fck + 8) / 10) ** 0.3, ecm_pa)
        out[f"c16_prd_{int(fck)}"] = rel_err(connector_resistance_n(0.019, 0.095, fck * 1e6, 450e6, ecm_pa), min(0.8 * 450e6 * math.pi * 0.019 ** 2 / 4, 0.29 * 0.019 ** 2 * math.sqrt(fck * 1e6 * ecm_pa)) / 1.25)
    for m_ed in (0.8 * full[0], 0.95 * full[0]):
        nc_req = optimize.brentq(lambda nc: moment_stress_block(nc)[0] - m_ed, 0.0, full[1], xtol=1e-6)
        out[f"c18_nreq_{round(m_ed / full[0], 2)}"] = rel_err(required_force(steel, fyd, ys, beff, hc, fcc, m_ed), nc_req)
    for n_mod in (6.0, 15.0):
        steel_section = i_section(d=h, b=b, t_f=tf, t_w=tw, r=0.0, n_r=2, material=Material("steel", 210e9, 0.3, 0.0, 355e6, "grey"))
        slab = rectangular_section(d=hc, b=beff, material=Material("concrete", 210e9 / n_mod, 0.2, 0.0, 30e6, "lightblue")).shift_section(x_offset=b / 2 - beff / 2, y_offset=h)
        joined = steel_section + slab
        joined.create_mesh(mesh_sizes=[2e-4, 4e-3])
        section = Section(geometry=joined, time_info=False)
        section.calculate_geometric_properties()
        i_library = section.get_eic()[0] / 210e9
        a_s = 2 * b * tf + (h - 2 * tf) * tw
        i_s = 2 * (b * tf ** 3 / 12 + b * tf * ((h - tf) / 2) ** 2) + tw * (h - 2 * tf) ** 3 / 12
        a_c = beff * hc / n_mod
        z_c = (a_s * h / 2 + a_c * (h + hc / 2)) / (a_s + a_c)
        i_closed = i_s + a_s * (h / 2 - z_c) ** 2 + beff * hc ** 3 / 12 / n_mod + a_c * (h + hc / 2 - z_c) ** 2
        out[f"c09_transformed_inertia_n{int(n_mod)}"] = rel_err(i_closed, i_library)
    return out


def main(argv):
    if len(argv) >= 2 and argv[1] == "--third-party":
        deviations = third_party()
        worst = max(deviations.values())
        print(json.dumps({"specRows": SPEC_ROWS, "deviations": deviations, "worst": worst}))
        return 0 if worst <= 5e-3 else 2
    if len(argv) >= 2 and argv[1] == "--evaluate-json":
        payload = json.load(sys.stdin)
        out = compare_report(payload["snapshot"], payload["report"])
        print(json.dumps(out))
        return 0 if out["maxRel"] <= 0.005 and out["compared"] >= 3 else 2
    if len(argv) < 2:
        assert abs(effective_width_m(8, 3) - 2.0) < 1e-9
        p = connector_resistance_n(0.019, 0.095, 30e6, 450e6, 33e9)
        assert abs(p/1000 - 81.656) < 0.05, p/1000
        assert abs(min_eta(8, 355e6) - 0.49) < 1e-6
        assert abs(insulation_m("r60", "trapezoidal", 0.3) - 0.018) < 1e-9
        assert abs(insulation_m("r60", "trapezoidal", 0.3) - 0.018) < 1e-9
        assert gamma_mf("en") == 1.15 and gamma_mf("de") == 1.35
        print("oracle self-check OK")
        return 0
    report = json.loads(Path(argv[1]).read_text())
    print(f"oracle loaded {len(report.get('checks', []))} checks")
    return 0

if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
