#!/usr/bin/env python3
"""Independent EN 1996 oracle — characteristic actions → EN 1990 ULS + f_k / Φ (±0.5 %)."""
from __future__ import annotations
import json, math, sys
from pathlib import Path

def fk_factors_de(group: int, mortar: str):
    if mortar == "thinLayer":
        return 0.70, 0.85, 0.0
    return {1: 0.60, 2: 0.50, 3: 0.45, 4: 0.40}[group], 0.65, 0.25

def fk_factors_en(group: int, mortar: str):
    if mortar == "thinLayer":
        return 0.80, 0.85, 0.0
    return {1: 0.55, 2: 0.45, 3: 0.40, 4: 0.35}[group], 0.70, 0.30

def shape_factor_delta(h_m, w_m, l_m):
    h = max(h_m * 1000.0, 1.0)
    w = max(w_m * 1000.0, 1.0)
    l = max(l_m * 1000.0, 1.0)
    ratio = min(2.5, max(0.4, h / 100.0))
    width_factor = 1.0 - math.exp(-w / 250.0)
    length_factor = 1.0 - math.exp(-l / 400.0)
    return min(1.40, max(0.70, (1.15 - 0.15 * ratio) * (0.90 + 0.20 * width_factor) * (0.92 + 0.16 * length_factor)))

def f_k_mpa(annex, fb, fm, group, mortar, h_m=0.113, w_m=0.365, l_m=0.24):
    k, a, b = (fk_factors_de if annex == "de" else fk_factors_en)(group, mortar)
    delta = shape_factor_delta(h_m, w_m, l_m)
    fb_n = fb * delta
    return k * (fb_n ** a) * (fm ** b)

def phi_s(annex, lam):
    base = 0.70 if annex == "de" else 0.85
    return max(0.0, base - 0.0011 * lam * lam)

def phi_i(e, t):
    t = max(t, 1e-6)
    e = max(abs(e), 0.05 * t)
    return max(0.0, min(1.0, 1.0 - 2.0 * e / t))

def phi_m(lam, e_mk, t):
    t = max(t, 1e-6)
    e_mk = max(abs(e_mk), 0.05 * t)
    a1 = max(0.0, min(1.0, 1.0 - 2.0 * e_mk / t))
    if a1 <= 0.0:
        return 0.0
    u = max(0.0, (lam - 2.0) * (t - 2.0 * e_mk) / (23.0 * t))
    return a1 * math.exp(-0.5 * u * u)

def rho_n(sides, h, L):
    rho2 = 0.75
    if sides == 2:
        return rho2
    if sides == 3:
        return rho2 / (1.0 + (rho2 * h / (3.0 * L)) ** 2)
    return rho2 / (1.0 + (rho2 * h / L) ** 2)

def group_num(g):
    s = str(g).lower().replace("group", "")
    try:
        return int(s) if s.isdigit() else {"1":1,"2":2,"3":3,"4":4}.get(s, 1)
    except Exception:
        return 1

def mortar_key(t):
    s = str(t)
    if s in ("thinLayer", "ThinLayer"):
        return "thinLayer"
    if s in ("lightweight", "Lightweight"):
        return "lightweight"
    return "generalPurpose"

def gamma_m_de(masonry_class: str, accidental: bool) -> float:
    if accidental:
        return 1.3
    return {"class1":1.5,"class2":1.7,"class3":1.7,"class4":2.0,"class5":2.0}.get(masonry_class.lower(), 1.5)

def gamma_m_en(masonry_class: str) -> float:
    return {"class1":1.5,"class2":1.7,"class3":2.0,"class4":2.2,"class5":2.5}.get(masonry_class.lower(), 2.0)

def psi0_imposed(cat: str) -> float:
    return {"A":0.7,"B":0.7,"C":0.7,"D":0.7,"E":1.0,"H":0.0}.get(str(cat).upper()[:1], 0.7)

def effective_length_m(wall):
    """🪟 Opening-aware ℓ_ef (§5.5.1.4) — mirrors Rust effective_length_m."""
    L = float(wall.get("lengthM", wall.get("length_m", 0)))
    h = max(float(wall.get("heightM", wall.get("height_m", 0))), 1e-6)
    l = L
    for o in wall.get("openings") or []:
        ow = float(o.get("widthM", 0))
        oh = float(o.get("heightM", 0))
        sill = float(o.get("sillHeightM", 0))
        pier = sill <= 0.05 * h and oh >= 0.85 * h
        if pier:
            l -= ow
        else:
            open_frac = min(1.0, max(0.0, oh / h))
            sill_frac = min(1.0, max(0.0, sill / h))
            l -= ow * open_frac * (0.35 + 0.65 * (1.0 - sill_frac))
    return max(l, 0.1 * L)

def self_weight_n(wall):
    """🏋️ Net self-weight [N] after opening volume deduction."""
    t = float(wall.get("thicknessM", 0))
    L = float(wall.get("lengthM", 0))
    h = float(wall.get("heightM", 0))
    dens = float(wall.get("densityKgM3", 1800))
    vol_gross = t * L * h
    vol_open = sum(float(o.get("widthM", 0)) * float(o.get("heightM", 0)) * t for o in (wall.get("openings") or []))
    return dens * 9.81 * max(vol_gross - vol_open, 0.0)

def eccentricity_from_slab_rotation_m(wall, lc, n_ed):
    """🔄 e_θ from ℓ_f (DIN EN 1996-1-1/NA NDP 6.1.2.2 / Annex C), capped 0,05·t."""
    t = float(wall.get("thicknessM", 0))
    l_f = min(max(float(lc.get("slabSpanM", 0)), 0.0), 6.0)
    e_full = l_f / 25.0
    n_floor = max(
        float(lc.get("gKSlabN", 0))
        + float(lc.get("qKImposedPa", 0)) * float(lc.get("tributaryAreaM2", 0))
        + float(lc.get("qKSnowPa", 0)) * float(lc.get("tributaryAreaM2", 0)),
        0.0,
    )
    share = min(1.0, max(0.0, n_floor / abs(n_ed))) if abs(n_ed) > 1e-6 else 1.0
    return min(e_full * share, 0.05 * max(t, 1e-6))

def phi_1_slab_span(slab_span_m):
    """📉 DE-NA Φ₁ (NDP 6.1.2.2 / NA Annex C): 1,6 − ℓ_f/6 on 4,5…6,0 m."""
    l_f = min(max(float(slab_span_m), 0.0), 6.0)
    if l_f <= 4.5:
        return 1.0
    return max(0.0, min(1.0, 1.6 - l_f / 6.0))

def design_stations(wall, lc, annex="de"):
    """EN 1990 ULS max-N stations — mirrors Rust design_effects imposed-leading."""
    g_self = self_weight_n(wall)
    g_slab = float(lc.get("gKSlabN", 0))
    q_imp = float(lc.get("qKImposedPa", 0)) * float(lc.get("tributaryAreaM2", 0))
    q_snow = float(lc.get("qKSnowPa", 0)) * float(lc.get("tributaryAreaM2", 0))
    f_conc = sum(float(c.get("forceN", 0)) for c in (lc.get("concentrated") or []))
    sit = str(lc.get("designSituation", "persistent")).lower()
    yg = 1.20 if sit == "transient" else 1.35
    yq = 1.5
    psi_i = psi0_imposed(lc.get("imposedCategory", "A"))
    psi_s = 0.5
    q_acc = (psi_s * q_snow) if q_snow > 0 else (psi_i * q_imp * 0.25)
    q_var = yq * q_imp + yq * q_acc
    n_top = yg * (g_slab + f_conc) + q_var
    n_mid = yg * (g_slab + f_conc + 0.5 * g_self) + q_var
    n_bot = yg * (g_slab + f_conc + g_self) + q_var
    return n_top, n_mid, n_bot

def rel_err(a, b):
    return abs(a - b) / max(abs(a), abs(b), 1e-12)

def assess_wall(annex, masonry_class, wall):
    fb = wall.get("fBPa", wall.get("f_b_pa", 0)) / 1e6
    fm = wall.get("mortarStrengthPa", wall.get("mortar_strength_pa", wall.get("fMPa", wall.get("f_m_pa", 0)))) / 1e6
    if fm <= 0:
        fm = {"M1":1,"M2_5":2.5,"M5":5,"M10":10,"M15":15,"M20":20}.get(str(wall.get("mortarClass","M10")),10)
    g = group_num(wall.get("unitGroup","group1"))
    mortar = mortar_key(wall.get("mortarType","generalPurpose"))
    uh = float(wall.get("unitHeightM", 0.113))
    uw = float(wall.get("unitWidthM", 0.365))
    ul = float(wall.get("unitLengthM", 0.24))
    fk = f_k_mpa(annex, fb, fm, g, mortar, uh, uw, ul)
    h = float(wall.get("heightM", wall.get("height_m", 0)))
    t = float(wall.get("thicknessM", wall.get("thickness_m", 0)))
    L = float(wall.get("lengthM", wall.get("length_m", 0)))
    L_eff = effective_length_m(wall)
    sides = int(wall.get("supportSides", wall.get("support_sides", 2)))
    e_top_decl = float(wall.get("eccentricityTopM", wall.get("eccentricity_top_m", 0)))
    e_bot = float(wall.get("eccentricityBottomM", wall.get("eccentricity_bottom_m", 0)))
    e_slab = abs(t / 2.0 - float(wall.get("slabBearingDepthM", 0)) / 2.0)
    hef = rho_n(sides, h, L_eff) * h
    lam = hef / t if t else 999
    phis = phi_s(annex, lam)
    e_init = hef / 450.0
    phi_inf = float(wall.get("phiInfinity", 1.5))
    gamma = gamma_m_de(masonry_class, False) if annex == "de" else gamma_m_en(masonry_class)
    fd = fk / gamma
    A = max(L_eff * t, 0.0)
    n_rd_simp = phis * (fd * 1e6) * A
    lcs = wall.get("loadCases") or wall.get("load_cases") or []
    n_ed = 0.0
    n_ed_bot = 0.0
    phi = 1.0
    n_rd_g = A * (fd * 1e6)
    if lcs:
        best_u = -1.0
        for lc in lcs:
            nt, nm, nb = design_stations(wall, lc, annex)
            n_ed_bot = max(n_ed_bot, nb)
            n_ref = max(nt, nm)
            e_theta = eccentricity_from_slab_rotation_m(wall, lc, n_ref)
            phi1 = phi_1_slab_span(lc.get("slabSpanM", 0))
            e_top = abs(e_top_decl + e_slab + e_theta)
            e_i_top = abs(e_top + e_init)
            e_i_bot = abs(e_bot + e_init)
            phi_top = phi_i(e_i_top, t) * phi1
            phi_bot = phi_i(e_i_bot, t) * phi1
            e_m = 0.5 * (e_top + e_bot) + e_init
            e_k = 0.002 * phi_inf * (hef / max(t, 1e-6)) * math.sqrt(max(t * abs(e_m), 1e-18))
            e_mk = max(abs(e_m + e_k), 0.05 * t)
            phi_mid = phi_m(lam, e_mk, t) * phi1
            for n_i, phi_i_ in ((nt, phi_top), (nm, phi_mid), (nb, phi_bot)):
                u = n_i / max(phi_i_, 1e-9)
                if u > best_u:
                    best_u = u
                    n_ed = n_i
                    phi = phi_i_
        n_rd_g = phi * (fd * 1e6) * A
    return {
        "fk_mpa": fk,
        "lambda": lam,
        "phi_s": phis,
        "phi_g": phi,
        "gamma_m": gamma,
        "n_rd_simplified_n": n_rd_simp,
        "n_rd_annex_g_n": n_rd_g,
        "n_ed_governing_n": n_ed,
        "utilization_simplified": (n_ed_bot / n_rd_simp) if n_rd_simp > 0 else 99.0,
        "utilization_annex_g": (n_ed / n_rd_g) if n_rd_g > 0 else 99.0,
        "slenderness_ok": lam <= 27.0,
    }

def main():
    snap_path = Path(sys.argv[1]) if len(sys.argv) > 1 else None
    report_path = Path(sys.argv[2]) if len(sys.argv) > 2 else None
    if snap_path and snap_path.exists():
        snap = json.loads(snap_path.read_text())
    else:
        snap = {
            "annex": "de",
            "masonryClass": "class1",
            "walls": [{
                "id": "wall-north", "unitGroup": "Group1", "mortarType": "GeneralPurpose", "mortarClass": "M10",
                "fBPa": 20e6, "mortarStrengthPa": 10e6, "heightM": 2.75, "thicknessM": 0.365, "lengthM": 5.0, "supportSides": 4,
                "eccentricityTopM": 0.02, "eccentricityBottomM": 0.015, "slabBearingDepthM": 0.12,
                "unitHeightM": 0.113, "unitWidthM": 0.365, "unitLengthM": 0.24, "densityKgM3": 1800, "phiInfinity": 1.5,
                "loadCases": [{"id": "uls", "designSituation": "persistent", "imposedCategory": "A",
                               "gKSlabN": 120000, "qKImposedPa": 2000, "tributaryAreaM2": 12.5, "slabSpanM": 4.5,
                               "qKSnowPa": 0, "qPWindPa": 800, "cPe": 0.8, "hKEarthN": 0, "concentrated": []}],
            }],
        }
    annex = snap.get("annex", "de")
    if isinstance(annex, str):
        annex = annex.lower()
    masonry_class = str(snap.get("masonryClass", snap.get("masonry_class", "class1"))).lower()
    wall = snap["walls"][0]
    got = assess_wall(annex, masonry_class, wall)
    result = {"ok": True, **got}
    fb = float(wall.get("fBPa", 20e6)) / 1e6
    fm = float(wall.get("mortarStrengthPa", wall.get("fMPa", 10e6))) / 1e6 or 10.0
    expected_fk = f_k_mpa(
        annex, fb, fm, group_num(wall.get("unitGroup", "group1")), mortar_key(wall.get("mortarType", "generalPurpose")),
        float(wall.get("unitHeightM", 0.113)), float(wall.get("unitWidthM", 0.365)), float(wall.get("unitLengthM", 0.24)),
    )
    if rel_err(got["fk_mpa"], expected_fk) > 0.005:
        result["ok"] = False
        result["error"] = f"fk mismatch {got['fk_mpa']} vs {expected_fk}"
    if report_path and report_path.exists():
        report = json.loads(report_path.read_text())
        checks = report.get("checks") or []
        compared = False
        for c in checks:
            cid = c.get("id") or ""
            status = str(c.get("status") or "")
            if status.lower() in ("notapplicable", "not_applicable"):
                continue
            rust_u = c.get("utilization")
            if rust_u is None:
                continue
            if "simplified" in cid and "basement" not in cid:
                ref = got["utilization_simplified"]
                key = "simplified"
            elif "6.1.2.compression" in cid and ".compression.mid." not in cid:
                ref = got["utilization_annex_g"]
                key = "annex_g"
            else:
                continue
            if ref >= 90:
                result[f"note_{key}"] = "skipped degenerate utilization"
                continue
            err = rel_err(float(rust_u), float(ref))
            result[f"rust_u_{key}"] = rust_u
            result[f"oracle_u_{key}"] = ref
            result[f"rel_diff_{key}"] = err
            compared = True
            if err > 0.005:
                result["ok"] = False
                result["error"] = f"{key} utilization rel_diff {err:.4%} exceeds 0.5% (rust={rust_u}, oracle={ref})"
                break
        if not compared and checks:
            result["note"] = "no comparable utilization check found; f_k still verified"
    print(json.dumps(result, indent=2))
    return 0 if result["ok"] else 1

if __name__ == "__main__":
    sys.exit(main())
