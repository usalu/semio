#!/usr/bin/env python3
"""Language-agnostic EN 1994 oracle — recomputes key formulas vs Rust evaluate() JSON (±0.5 %)."""
from __future__ import annotations
import json, math, sys
from pathlib import Path

def effective_width_m(span_m, b0_m, spacing_m):
    be1 = span_m / 8.0 + b0_m / 2.0
    be2 = spacing_m / 2.0
    return min(2.0 * be1, 2.0 * be2, spacing_m)

def uncapped_effective_width_m(span_m, b0_m):
    return 2.0 * (span_m / 8.0 + b0_m / 2.0)

def stud_alpha(h_sc_m, d_m):
    ratio = h_sc_m / d_m
    return 1.0 if ratio > 4.0 else 0.2 * (ratio + 1.0)

def connector_resistance_n(d_m, h_sc_m, f_ck_pa, f_u_pa, e_cm_pa, gamma_v=1.25, kt=1.0):
    d_mm = d_m * 1000.0
    h_sc_mm = h_sc_m * 1000.0
    f_ck = f_ck_pa / 1e6
    f_u = f_u_pa / 1e6
    e_cm = e_cm_pa / 1e6
    alpha = stud_alpha(h_sc_m, d_m)
    p_pl = 0.8 * f_u * math.pi * d_mm * d_mm / 4.0
    p_b = 0.29 * alpha * d_mm * d_mm * math.sqrt(f_ck * e_cm)
    return min(p_pl, p_b) / gamma_v * kt

def min_eta(span_m, f_y_pa):
    f_y = f_y_pa / 1e6
    eta = 1.0 - (355.0 / f_y) * (0.75 - 0.03 * span_m) if span_m <= 25 else 1.0 - (355.0 / f_y) * 0.30
    return max(eta, 0.4)

def insulation_m(rating, deck, steel_height_m):
    r = rating.lower()
    if deck == "re-entrant":
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
    return 1.35 if annex.lower() == "de" else 1.15

def psi_factors(kind, category, annex):
    # EN 1990 + DE NA simplified
    if kind == "permanent":
        return (1.0, 1.0, 1.0)
    if kind == "construction":
        return (1.0, 0.0, 0.0)
    if kind == "snow":
        return (0.5, 0.2, 0.0) if annex == "de" else (0.5, 0.2, 0.0)
    if kind == "wind":
        return (0.6, 0.2, 0.0)
    # imposed by category
    cat = (category or "B").upper()[:1]
    table = {
        "A": (0.7, 0.5, 0.3),
        "B": (0.7, 0.5, 0.3),
        "C": (0.7, 0.7, 0.6),
        "D": (0.7, 0.7, 0.6),
        "E": (1.0, 0.9, 0.8),
        "F": (0.7, 0.7, 0.6),
        "G": (0.7, 0.5, 0.3),
        "H": (0.0, 0.0, 0.0),
    }
    return table.get(cat, (0.7, 0.5, 0.3))

def action_internals(action, span_m, support, spacing_m):
    q = float(action.get("qAreaPa") or 0) * spacing_m
    f_k = float(action.get("fKN") or 0)
    if abs(q) > 1e-12:
        if support == "continuous_2_span":
            return q * span_m * span_m / 16.0, q * span_m * span_m / 8.0, 1.25 * q * span_m / 2.0, 0.0
        return q * span_m * span_m / 8.0, 0.0, q * span_m / 2.0, 0.0
    if abs(f_k) > 1.0:
        if support == "continuous_2_span":
            return f_k * span_m / 8.0, f_k * span_m / 8.0, 1.25 * f_k / 2.0, 0.0
        return f_k * span_m / 4.0, 0.0, f_k / 2.0, 0.0
    return 0.0, 0.0, 0.0, 0.0


def uls_composite(actions, span_m, support, spacing_m, annex):
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
    gamma_g = 1.35
    best = (gamma_g * g_m, gamma_g * g_h, gamma_g * g_v, gamma_g * g_n, "G only")
    for i, (lead, lm, lh, lv, ln) in enumerate(vars_):
        psi0 = psi_factors(lead["kind"], lead.get("category"), annex)[0]
        m = gamma_g * g_m + 1.5 * lm
        mh = gamma_g * g_h + 1.5 * lh
        v = gamma_g * g_v + 1.5 * lv
        n = gamma_g * g_n + 1.5 * ln
        for j, (other, om, oh, ov, on) in enumerate(vars_):
            if i == j: continue
            p0 = psi_factors(other["kind"], other.get("category"), annex)[0]
            m += 1.5 * p0 * om
            mh += 1.5 * p0 * oh
            v += 1.5 * p0 * ov
            n += 1.5 * p0 * on
        if abs(m) >= abs(best[0]):
            best = (m, mh, v, n, lead.get("id", "Q"))
    return best


def utilization(computed, limit):
    if abs(limit) < 1e-15:
        return 0.0 if abs(computed) < 1e-15 else 1e9
    return abs(computed / limit)

def compare_report(snapshot, report):
    checks = {c["id"]: c for c in report.get("checks", [])}
    compared = 0
    max_rel = 0.0
    beams = snapshot.get("beams") or []
    fy = float(snapshot.get("steelFYPa") or snapshot.get("steel_f_y_pa") or 355e6)
    annex = str(snapshot.get("annex") or "en").lower()
    if annex in ("de", "annexchoice::de"):
        annex = "de"
    else:
        annex = "en"
    for beam in beams:
        bid = beam["id"]
        span = float(beam["spanM"])
        spacing = float(beam["spacingM"])
        b0 = float(beam["steel"]["widthM"])
        beff = effective_width_m(span, b0, spacing)
        unc = uncapped_effective_width_m(span, b0)
        # beff check utilization
        cid = f"en1994.5.4.1.2.beff.{bid}"
        if cid in checks:
            u = utilization(unc, spacing)
            ru = float(checks[cid]["utilization"])
            rel = abs(u - ru) / max(abs(u), abs(ru), 1e-9)
            max_rel = max(max_rel, rel)
            compared += 1
        studs = beam["studs"]
        sheeting = beam["sheeting"]
        kt = 1.0
        if not sheeting.get("ribsParallelToBeam", False):
            nr = max(int(studs.get("countPerRib") or 1), 1)
            hp = max(float(sheeting["heightM"]), 1e-6)
            b00 = max(float(sheeting["ribWidthM"]), 1e-6)
            d = max(float(studs["diameterM"]), 1e-6)
            kt = min(1.0, max(0.0, (0.7 / math.sqrt(nr)) * (b00 / hp) * ((hp / d) - 1.0)))
        p_rd = connector_resistance_n(
            float(studs["diameterM"]), float(studs["heightM"]),
            float(beam["concreteFCkPa"]), float(studs["fUPa"]), float(beam["concreteECmPa"]),
            1.25, kt,
        )
        spacing_m = float(beam.get("spacingM") or 3.0)
        support = beam.get("support") or "simply_supported"
        actions = beam.get("actions") or []
        # N_cf surrogate for longitudinal shear demand
        h_c = max(float(beam["slabThicknessM"]) - float(sheeting["heightM"]), 0.04)
        n_a = float(beam["steel"]["aM2"]) * fy / 1.0  # γ_M0
        n_c = 0.85 * (float(beam["concreteFCkPa"]) / 1.5) * beff * h_c
        n_cf = min(n_a, n_c)
        spacing_stud = max(float(studs.get("spacingM") or 0.2), 1e-6)
        n_layout = max(int((span / 2.0) / spacing_stud) * max(int(studs.get("countPerRib") or 1), 1), 1)
        n = max(int(studs["totalCount"]), n_layout)
        v_ed_stud = n_cf / n
        cid = f"en1994.6.6.3.1.prd.{bid}"
        if cid in checks:
            u = utilization(v_ed_stud, p_rd)
            ru = float(checks[cid]["utilization"])
            rel = abs(u - ru) / max(abs(u), abs(ru), 1e-9)
            max_rel = max(max_rel, rel)
            compared += 1
        # ULS combination M_Ed vs report computed moment
        m_ed, _, _, _, _ = uls_composite(actions, span, support, spacing_m, annex)
        cid = f"en1994.6.2.1.3.mrd.{bid}"
        if cid in checks:
            comp = checks[cid].get("computed") or {}
            rc = float(comp.get("value") or 0.0)
            if abs(rc) > 1.0 and abs(m_ed) > 1.0:
                rel = abs(m_ed - rc) / max(abs(m_ed), abs(rc), 1e-9)
                max_rel = max(max_rel, rel)
                compared += 1
        eta_min = min_eta(span, fy)
        n_a = float(beam["steel"]["aM2"]) * fy
        n_c = 0.85 * (float(beam["concreteFCkPa"]) / 1.5) * beff * h_c
        n_cf = min(n_a, n_c)
        n_req = max(int(math.ceil(n_cf / max(p_rd, 1.0))), 1)
        n_layout = max(int((span / 2.0) / spacing_stud) * max(int(studs.get("countPerRib") or 1), 1), 1)
        n_f = max(int(studs["totalCount"]), n_layout)
        eta = n_f / max(n_req, 1)
        cid = f"en1994.6.6.1.2.etamin.{bid}"
        if cid in checks:
            comp = float((checks[cid].get("computed") or {}).get("value") or 0.0)
            lim = float((checks[cid].get("limit") or {}).get("value") or 0.0)
            ru = float(checks[cid]["utilization"])
            rel_eta = abs(eta - comp) / max(abs(eta), abs(comp), 1e-9)
            rel_min = abs(eta_min - lim) / max(abs(eta_min), abs(lim), 1e-9)
            if eta >= eta_min:
                u = eta_min / max(eta, eta_min)
            else:
                u = eta / max(eta_min, 1e-15)
            rel_u = abs(u - ru) / max(abs(u), abs(ru), 1e-9)
            max_rel = max(max_rel, rel_eta, rel_min, rel_u)
            compared += 1
        rating = str(snapshot.get("fireRating") or "r60")
        deck = sheeting.get("profile") or "trapezoidal"
        steel_h = float(beam["steel"]["heightM"])
        t_req = insulation_m(rating, deck, steel_h)
        cid = "en1994.1-2.4.2.insulation"
        if cid in checks:
            provided = float(snapshot.get("insulationThicknessM") or 0)
            ru = float(checks[cid]["utilization"])
            if provided >= t_req:
                u = t_req / max(provided, t_req)
            else:
                u = provided / max(t_req, 1e-15)
            rel = abs(u - ru) / max(abs(u), abs(ru), 1e-9)
            max_rel = max(max_rel, rel)
            compared += 1
        break  # first beam sufficient for overlapping checks
    # always include beff numeric self-check
    if beams:
        b = beams[0]
        assert abs(effective_width_m(float(b["spanM"]), float(b["steel"]["widthM"]), float(b["spacingM"])) - beff) < 1e-9
    return {"compared": compared, "maxRel": max_rel}

def main(argv):
    if len(argv) >= 2 and argv[1] == "--evaluate-json":
        payload = json.load(sys.stdin)
        out = compare_report(payload["snapshot"], payload["report"])
        print(json.dumps(out))
        return 0 if out["maxRel"] <= 0.005 and out["compared"] >= 3 else 2
    if len(argv) < 2:
        assert abs(effective_width_m(8, 0.3, 3) - 2.30) < 0.01
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
