"""🚦️ Independent EN 1998 document oracle: a committed snapshot (JSON on stdin or a path) to the utilisation of the building checks of EN 1998-1 per building, system, storey and member (`--json`, default: `{"checks":[{"id","utilization"}]}`; `--self-check`: the shear-building Rayleigh period and the spectrum plateau against scipy). Written from EN 1998-1 3.1.2, 3.2.2.2, 3.2.2.5, 3.2.4, 4.2.3, 4.3.3.2, 4.3.5, 4.4.2, 4.4.3.2, 5.4.3, 6.5 and DIN EN 1998-1/NA Table NA.4 (Tables 3.2, 3.3 and NA.4 keyed here, not read from the Rust). EN 1998-5 Annex E and 5.4.1.1 (wall `.thrust`, `.water`, foundation `.sliding`, Mononobe-Okabe Eq. E.4 keyed here as the Python twin of the shared kernel). Covered families (spec-trace SEI-11, 20, 30, 35, 40, 41, 44, 45, 46, 47, 49, 50, 51): `baseShear`, `qLimit`, `t1Given`, `torsion`, `storeyForces.<storey>`, `drift.<storey>`, `pdelta.<storey>`, `overturning`, `componentCombination`, `planRegularity`, `elevationRegularity`, `plan.slenderness`, `plan.envelope`, `elevation.setback`, `joint.gap`, `nonstructural.force`, `capacityDesign.nodes`, `rc.rho`, `rc.omegaWdBeam`, `rc.noSteelClass`, `steel.class`. SI throughout; the stiffnesses of the shear building are N/m."""

import json
import math
import sys
import importlib.util
import pathlib


def _limit_rule():
    for parent in pathlib.Path(__file__).resolve().parents:
        path = parent / "🔮️oracles" / "🏃️harness" / "📏️tolerance" / "🐍️.py"
        if path.is_file():
            spec = importlib.util.spec_from_file_location("norm_limit_rule", path)
            module = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(module)
            return module
    raise FileNotFoundError("oracle tolerance module")


LIMIT = _limit_rule()
_spec_families = importlib.util.spec_from_file_location("en1998_site_and_structures", pathlib.Path(__file__).resolve().parent.parent / "🌍️site-and-structures" / "🐍️.py")
families = importlib.util.module_from_spec(_spec_families)
_spec_families.loader.exec_module(families)
_spec_geotechnics = importlib.util.spec_from_file_location("en1998_geotechnical_details", pathlib.Path(__file__).resolve().parent.parent / "🪨️geotechnical-details" / "🐍️.py")
geotechnics = importlib.util.module_from_spec(_spec_geotechnics)
_spec_geotechnics.loader.exec_module(geotechnics)

G = 9.81
TABLE_32 = {"A": (1.0, 0.15, 0.40, 2.0), "B": (1.2, 0.15, 0.50, 2.0), "C": (1.15, 0.20, 0.60, 2.0), "D": (1.35, 0.20, 0.80, 2.0), "E": (1.4, 0.15, 0.50, 2.0)}
TABLE_33 = {"A": (1.0, 0.05, 0.25, 1.2), "B": (1.35, 0.05, 0.25, 1.2), "C": (1.5, 0.10, 0.25, 1.2), "D": (1.8, 0.10, 0.30, 1.2), "E": (1.6, 0.05, 0.25, 1.2)}
NA4 = {"A-R": (1.0, 0.05, 0.20, 2.0), "B-R": (1.25, 0.05, 0.25, 2.0), "C-R": (1.5, 0.05, 0.30, 2.0), "B-T": (1.25, 0.10, 0.30, 2.0), "C-T": (1.5, 0.10, 0.40, 2.0), "C-S": (0.75, 0.10, 0.50, 2.0)}
GAMMA_I = {"I": 0.8, "CC1": 0.8, "II": 1.0, "CC2": 1.0, "III": 1.2, "CC3": 1.2, "IV": 1.4, "CC4": 1.4}
PSI2 = {"a": 0.3, "residential": 0.3, "b": 0.3, "office": 0.3, "c": 0.6, "congregation": 0.6, "d": 0.6, "retail": 0.6, "e": 0.8, "storage": 0.8, "f": 0.6, "traffic": 0.6, "g": 0.3, "vehicles": 0.3, "h": 0.0, "roofs": 0.0, "snow": 0.2, "wind": 0.0}
Q_MAX = {("rc", "frame"): (1.5, 3.9, 5.85), ("rc", "wall"): (1.5, 3.0, 4.4), ("rc", "dual"): (1.5, 3.6, 5.2), ("steel", "frame"): (1.5, 4.0, 6.5), ("steel", "wall"): (1.5, 4.0, 5.0), ("steel", "dual"): (1.5, 4.0, 5.0), ("composite", None): (1.5, 4.0, 6.0), ("timber", None): (1.5, 2.5, 5.0), ("masonry", None): (1.5, 2.5, 2.5)}
RHO = {"beam": {"dch": (0.0035, 0.04), "dcm": (0.0025, 0.04), "low": (0.0025, 0.04)}, "column": {"dch": (0.01, 0.04), "dcm": (0.01, 0.04), "low": (0.01, 0.04)}, "other": {"dch": (0.002, 0.04), "dcm": (0.002, 0.04), "low": (0.002, 0.04)}}
COLUMN_MIN_B = {"dch": 0.25, "dcm": 0.25, "low": 0.20}
BEAM_MIN_B = 0.20
COLUMN_OMEGA_WD_MIN = {"dch": 0.12, "dcm": 0.08, "low": 0.0}
DRIFT_THETA = {"brittle": 0.005, "ductile": 0.0075, "isolated": 0.01}
NU = {"low": 0.5, "high": 0.4}
CRACKED_FACTOR = 0.5


def site_parameters(doc):
    """🌋 (a_gR, S, TB, TC, TD) of the document: Table NA.4 and a_gR = S_aP,R / 2.5 under the DE annex, Table 3.2 or 3.3 under EN."""
    site = doc["site"]
    if doc.get("annex", "de").lower() == "en":
        ground = (site.get("enGroundType") or "B")[0].upper()
        table = TABLE_33 if "2" in (site.get("enSpectrumType") or "").lower() else TABLE_32
        return (site.get("aGr", 0.0),) + table[ground if ground in table else "B"]
    return (site.get("sApR", 0.0) / 2.5,) + NA4[site.get("deGroundCombo", "B-R")]


def design_spectrum(a_g, s, tb, tc, td, q, t):
    """📉 EN 1998-1 Eq. 3.13 to 3.16 with beta = 0.2 and a_g = gamma_I a_gR already applied."""
    q = max(q, 1.0)
    if t <= tb:
        return a_g * s * (2 / 3 + t / tb * (2.5 / q - 2 / 3))
    if t <= tc:
        return a_g * s * 2.5 / q
    if t <= td:
        return max(a_g * s * 2.5 / q * tc / t, 0.2 * a_g)
    return max(a_g * s * 2.5 / q * tc * td / t ** 2, 0.2 * a_g)


def phi(category, is_roof, correlated):
    """🏗 Table 4.2: D to F and archives 1.0, otherwise the roof 1.0, correlated storeys 0.8, independent storeys 0.5."""
    if category.strip().lower() in ("retail", "d", "storage", "e", "traffic", "f", "archives", "archive") or is_roof:
        return 1.0
    return 0.8 if correlated else 0.5


def weight(storey, is_roof):
    """⚖️ G_k + sum psi_E,i Q_k,i of one storey [N] (EN 1998-1 3.2.4)."""
    total = storey["permanentGkN"]
    for v in storey.get("variables", []):
        cat = v["category"].strip().lower()
        total += phi(cat, is_roof, storey.get("correlatedOccupancy", True)) * PSI2.get(cat, 0.3) * v["qkN"]
    return total


def rayleigh(masses, stiffness):
    """🎻 Rayleigh quotient of the shear building under the weights as lateral loads: K u = m g, T = 2 pi sqrt(sum m u^2 / sum m g u)."""
    n = len(masses)
    shear = [sum(masses[j:]) * G for j in range(n)]
    u, acc = [], 0.0
    for j in range(n):
        acc += shear[j] / stiffness[j]
        u.append(acc)
    return 2 * math.pi * math.sqrt(sum(m * x * x for m, x in zip(masses, u)) / sum(m * G * x for m, x in zip(masses, u)))


def stiffness_factor(b):
    """🪨️ Factor on the entered storey stiffnesses (EN 1998-1 4.3.1(6)): basis `simplified` selects the cracked factor 0.5 (unverified), basis `declared` the declared effective share in (0, 1]."""
    return CRACKED_FACTOR if b["storeyStiffnessBasis"] == "simplified" else min(b["storeyStiffnessRatio"], 1.0)


def fundamental_period(b, masses, height):
    method = b.get("t1Method", "ct").lower()
    ct_period = b["ct"] * height ** 0.75
    if method == "given":
        return b["t1GivenS"], ct_period
    if method == "rayleigh":
        periods = [rayleigh(masses, [stiffness_factor(b) * s[k] for s in b["storeys"]]) for k in ("stiffnessX", "stiffnessY") if all(s[k] > 0 for s in b["storeys"])]
        return (min(periods) if periods else ct_period), ct_period
    return ct_period, ct_period


def q_limit(system):
    key = (system["material"].lower().replace("concrete", "rc"), system["systemType"].lower())
    row = Q_MAX.get(key) or Q_MAX.get((key[0], None))
    cls = {"dcm": 1, "dch": 2}.get(system["ductilityClass"].lower(), 0)
    return row[cls] if row else 1.5


def design_base_shear(b, sy, masses, t1, a_g, s, tb, tc, td):
    """🪨 EN 1998-1 4.3.3.2.2 design base shear of one system, the single source of the `baseShear` check and the foundation sliding demand: q capped at q_max and reduced by 0.8 for irregular elevation, S_d >= 0.2 a_g, lambda; no system: q = 1.5 uncapped. Returns (q, q_max, F_b)."""
    q = 1.5 if sy is None else max(sy["q0"] * sy["alphaUOverAlpha1"] * min(max(sy["kW"], 0.5), 1.0), 1.5)
    q_max = float("inf") if sy is None else q_limit(sy)
    lam = 0.85 if t1 <= 2 * tc and len(masses) > 2 else 1.0
    q_design = min(q, q_max) * (1.0 if b["elevationRegular"] else 0.8)
    return q, q_max, design_spectrum(a_g, s, tb, tc, td, q_design, t1) * sum(masses) * lam


def amplification(theta):
    return 1.0 if theta <= 0.1 else (1 / (1 - theta) if theta <= 0.3 else 1 / 0.7)


CF = {"kl1": 1.35, "kl2": 1.20, "kl3": 1.00}
GAMMA_C, GAMMA_S, E_S = 1.5, 1.15, 200000.0
SHEAR = {"axial": 0.55, "cap": 5.0, "slope": 0.05, "base": 0.16, "rho_min": 0.5, "lv_slope": 0.16, "lv_cap": 5.0}


def yield_neutral_axis_depth(h, b, d, a_s1, a_s2, n_ed, f_c, f_y, strips=120):
    """🧵 Depth x [m] of the compression zone at the yield curvature of the strain-plane section (parabola-rectangle concrete eps_c2 = 2 permille and exponent 2, elastic plastic steel E_s = 200 GPa, 120 midpoint strips, compression steel displacing the concrete): the yield curvature is the first of the extreme concrete fibre at eps_c2 and the tension steel at f_y/E_s, found by bisection with the force balance solved by scipy.optimize.brentq; `None` above 58 MPa (outside the registry range) or without a balanced plane."""
    import numpy as np
    from scipy.optimize import brentq

    e_s, eps_c2, expo, top = 200000.0, 2.0, 2.0, 0.5 * h * 1e3
    if f_c <= 0 or f_c > 58.0 or f_y <= 0:
        return None
    dz = h * 1e3 / strips
    z_c = -top + (np.arange(strips) + 0.5) * dz
    bars = [(a_s1 * 1e6, (0.5 * h - d) * 1e3), (a_s2 * 1e6, (d - 0.5 * h) * 1e3)]
    bars = [(area, z) for area, z in bars if area > 0]

    def concrete(e):
        e = np.asarray(e, dtype=float)
        return np.where(e <= 0, 0.0, np.where(e < eps_c2, f_c * (1 - (1 - np.minimum(e, eps_c2) / eps_c2) ** expo), f_c))

    def force(eps_m, kappa):
        n = float(np.sum(concrete(eps_m + kappa * z_c)) * b * 1e3 * dz)
        for area, z in bars:
            e = eps_m + kappa * z
            stress = min(max(e_s * e / 1000.0, -f_y), f_y) - (float(concrete(e)) if e > 0 else 0.0)
            n += stress * area
        return n

    def balanced(kappa):
        lo, hi = -200.0, 10.0 * 3.5 - kappa * top
        if hi <= lo or force(lo, kappa) > n_ed or force(hi, kappa) < n_ed:
            return None
        return brentq(lambda m: force(m, kappa) - n_ed, lo, hi, xtol=1e-13, rtol=1e-14, maxiter=500)

    eps_y = f_y / e_s * 1e3

    def ratio(kappa):
        m = balanced(kappa)
        if m is None:
            return math.inf
        tension = max(-(m + kappa * z) for _, z in bars) if bars else -math.inf
        return max((m + kappa * top) / eps_c2, tension / eps_y)

    if ratio(0.0) >= 1.0:
        return None
    hi = (eps_c2 + eps_y) / (2 * top)
    while ratio(hi) < 1.0:
        hi *= 2.0
    lo = 0.0
    for _ in range(100):
        mid = 0.5 * (lo + hi)
        lo, hi = (lo, mid) if ratio(mid) >= 1.0 else (mid, hi)
    kappa_y = 0.5 * (lo + hi)
    return (top + balanced(kappa_y) / kappa_y) * 1e-3


def cyclic_shear_utilisation(a):
    """🧱 E_d/R_d of the cyclic shear capacity of EN 1998-3 A.3.3 for an assessment without chord rotation demand (plastic ductility 0, so no yield rotation is needed): strengths f_m/(CF gamma_m), x the yield neutral axis of the cracked section with the strengths over CF; None while the section or the shear demand is missing."""
    h, b, d, lv, v_ed = a["sectionDepthM"], a["sectionWidthM"], a["effectiveDepthM"], a["shearSpanM"], a.get("shearForceDemandN", 0.0)
    if v_ed <= 0 or a["chordRotationDemandRad"] > 0 or min(h, b, d, lv, a["tensionSteelAreaM2"], a["steelYieldMpa"], a["concreteStrengthMpa"]) <= 0 or d >= h or a["gammaEl"] <= 0:
        return None
    cf = 1.0 if a["limitState"] == "dl" else CF[a["knowledgeLevel"]]
    fy, fc = a["steelYieldMpa"] / cf, a["concreteStrengthMpa"] / cf
    rho, rho_c, delta = a["tensionSteelAreaM2"] / (b * d), a["compressionSteelAreaM2"] / (b * d), (h - d) / d
    nu = a["axialForceN"] / (b * d * fy * 1e6)
    alpha = E_S / (22000.0 * (fc / 10.0) ** 0.3)
    big_a, big_b = rho + rho_c + nu, rho + rho_c * delta + nu
    x = (math.sqrt(max(alpha * alpha * big_a * big_a + 2 * alpha * big_b, 0.0)) - alpha * big_a) * d
    fibre = yield_neutral_axis_depth(h, b, d, a["tensionSteelAreaM2"], a["compressionSteelAreaM2"], a["axialForceN"], fc, fy)
    x = x if fibre is None else fibre
    fc_b, fyw_b = a["concreteStrengthMpa"] / (cf * GAMMA_C), a["stirrupYieldMpa"] / (cf * GAMMA_S)
    area = b * h
    axial = (h - x) / (2 * lv) * min(max(a["axialForceN"], 0.0), SHEAR["axial"] * area * fc_b * 1e6)
    concrete = SHEAR["base"] * max(SHEAR["rho_min"], 100 * (a["tensionSteelAreaM2"] + a["compressionSteelAreaM2"] + a.get("intermediateSteelAreaM2", 0.0)) / area) * (1 - SHEAR["lv_slope"] * min(SHEAR["lv_cap"], lv / h)) * math.sqrt(fc_b) * area * 1e6
    stirrup = (a["stirrupLegAreaM2"] / a["stirrupSpacingM"] if a["stirrupSpacingM"] > 0 else 0.0) * max(2 * d - h, 0.0) * fyw_b * 1e6
    return v_ed / ((axial + (concrete + stirrup)) / a["gammaEl"])


REGION = {
    "min_length": 0.45, "clear_divisor": 6.0, "whole_height": 3.0, "beam_depth_factor": {"dcm": 1.0, "dch": 1.5}, "dbw_min": 0.006, "dch_dbw_factor": 0.4,
    "col_core_divisor": {"dcm": 2.0, "dch": 3.0}, "col_max": {"dcm": 0.175, "dch": 0.125}, "col_bar_factor": {"dcm": 8.0, "dch": 6.0},
    "beam_depth_divisor": 4.0, "beam_dbw_factor": 24.0, "beam_max": {"dcm": 0.225, "dch": 0.175}, "beam_bar_factor": {"dcm": 8.0, "dch": 6.0},
    "bar_gap_max": {"dcm": 0.20, "dch": 0.15}, "omega_min": {"dcm": 0.08, "dch": 0.12}, "confinement_factor": 30.0, "confinement_offset": 0.035, "nu_max": {"dcm": 0.65, "dch": 0.55},
}


def region_utilisations(r, role, cls, q0, t1, tc):
    """🔩 (id suffix, utilisation) of the DCM/DCH critical region checks of EN 1998-1 5.4.3 and 5.5.3 for one beam or column: the numpy evaluation of critical length, hoop diameter, hoop spacing, restrained bar spacing, the confinement alpha omega_wd of Eq. (5.15) and rho_tot; the utilisation of a minimum is required over provided, of a maximum provided over limit."""
    import numpy as np

    k = REGION
    column = role == "column"
    bc, hc, b0, h0, s = r["sectionWidthM"], r["sectionDepthM"], r["coreWidthM"], r["coreDepthM"], r["hoopSpacingM"]
    large, core = max(bc, hc), min(b0, h0)
    whole = column and r["clearLengthM"] / large < k["whole_height"]
    l_cr = (k["beam_depth_factor"][cls] * hc) if not column else (r["clearLengthM"] if whole else max(large, r["clearLengthM"] / k["clear_divisor"], k["min_length"]))
    if column:
        s_max = min(core / k["col_core_divisor"][cls], k["col_max"][cls], k["col_bar_factor"][cls] * r["longitudinalBarDiameterM"])
        dbw_min = max(k["dbw_min"], k["dch_dbw_factor"] * r["longitudinalBarMaxDiameterM"] * math.sqrt(r["steelDesignYieldMpa"] / r["hoopDesignYieldMpa"]) if cls == "dch" else 0.0)
    else:
        s_max = min(hc / k["beam_depth_divisor"], k["beam_dbw_factor"] * r["hoopDiameterM"], k["beam_max"][cls], k["beam_bar_factor"][cls] * r["longitudinalBarDiameterM"])
        dbw_min = k["dbw_min"]
    over = lambda required, provided: 0.0 if required <= 0 else (required / provided if provided > 0 else 1e9)
    out = [("length", over(l_cr, r["hoopedLengthM"])), ("hoopDiameter", over(dbw_min, r["hoopDiameterM"]))]
    if s > 0:
        out.append(("hoopSpacing", s / max(s_max, 1e-9)))
    if r["restrainedBarCount"] > 0:
        out.append(("barSpacing", r["restrainedBarSpacingM"] / max(k["bar_gap_max"][cls], 1e-9)))
    if column and min(r["concreteDesignStrengthMpa"], r["steelDesignYieldMpa"], r["hoopDesignYieldMpa"]) > 0 and r["axialForceN"] != 0:
        out.append(("nuD", r["axialForceN"] / (bc * hc * r["concreteDesignStrengthMpa"] * 1e6) / k["nu_max"][cls]))
    if column and min(r["concreteDesignStrengthMpa"], r["hoopDesignYieldMpa"], r["steelDesignYieldMpa"], s, r["hoopDiameterM"]) > 0:
        gaps = np.full(int(r["restrainedBarCount"]), r["restrainedBarSpacingM"])
        alpha_n = max(1 - float(np.sum(gaps ** 2)) / (6 * b0 * h0), 0.0)
        alpha_s = max((1 - s / (2 * b0)) * (1 - s / (2 * h0)), 0.0)
        hoop_area = float(np.pi * r["hoopDiameterM"] ** 2 / 4)
        omega = hoop_area * (r["hoopLegsX"] * b0 + r["hoopLegsY"] * h0) / (b0 * h0 * s) * r["hoopDesignYieldMpa"] / r["concreteDesignStrengthMpa"]
        mu_phi = 2 * q0 - 1 if t1 >= tc else 1 + 2 * (q0 - 1) * tc / t1
        nu = r["axialForceN"] / (bc * hc * r["concreteDesignStrengthMpa"] * 1e6)
        required = k["confinement_factor"] * mu_phi * nu * (r["steelDesignYieldMpa"] / E_S) * bc / b0 - k["confinement_offset"]
        out.append(("confinement", over(required, alpha_n * alpha_s * omega)))
        out.append(("omegaWdMin", over(k["omega_min"][cls], omega)))
    if column and r["longitudinalSteelAreaM2"] > 0:
        rho = r["longitudinalSteelAreaM2"] / (bc * hc)
        lo, hi = RHO["column"][cls]
        out.append(("rhoTot", rho / (lo if rho < lo else hi)))
    return out


TIE_COEFFICIENT = {"b": 0.3, "c": 0.4, "d": 0.6}
M2 = 1.25
SLOPE_FACTOR = 0.5


def foundation_gravity(doc, foundation):
    """🏗 Seismic weight of the building a foundation supports [N]."""
    wanted = foundation["supportedBuildingId"] or (doc.get("buildings") or [{"id": ""}])[0]["id"]
    building = next((b for b in doc.get("buildings", []) if b["id"] == wanted), None)
    storeys = building["storeys"] if building else []
    return sum(weight(st, i + 1 == len(storeys)) for i, st in enumerate(storeys))


def tie_beams(doc, a_g, s, out, cited):
    """🔗 EN 1998-5 5.4.1.2: N = coefficient alpha S N_Ed of ground type B, C or D with N_Ed the larger weight of the two foundations, against A_s f_yd; the relative displacement against its admissible value."""
    coefficient = TIE_COEFFICIENT.get((doc["site"].get("enGroundType") or "b").lower())
    by_id = {f["id"]: f for f in doc.get("foundations", [])}
    for t in doc.get("tieBeams", []):
        ends = [by_id[i] for i in (t["foundationAId"], t["foundationBId"]) if i in by_id]
        if coefficient is not None and len(ends) == 2:
            force = coefficient * a_g / G * s * max(foundation_gravity(doc, f) for f in ends)
            cited(f"en1998.5.{t['id']}.tie", force / (t["steelArea"] * t["steelDesignStrength"]))
        out(f"en1998.5.{t['id']}.displacement", abs(t["relativeDisplacement"]) / t["displacementLimit"])


def slope_bishop(h, beta, xc, yc, rad, gamma, tan_phi, c, kh, share):
    """⛰ Least simplified Bishop factor of both vertical signs of one circle in a dry slope (40 slices, brentq between the pole of the slice denominators and 1e4), `None` without a sliding mass."""
    import numpy as np
    from scipy import optimize

    tb = math.tan(math.radians(beta))
    run = h / tb
    ground = lambda x: h if x <= 0.0 else (0.0 if x >= run else h - x * tb)
    lower = lambda x: yc - math.sqrt(max(rad * rad - (x - xc) ** 2, 0.0))
    inside = [x for x in np.linspace(xc - rad, xc + rad, 4001) if lower(x) < ground(x)]
    if not inside or inside[-1] - inside[0] < 1e-6:
        return None
    width = (inside[-1] - inside[0]) / 40
    xs = [inside[0] + (i + 0.5) * width for i in range(40)]
    base = [lower(x) for x in xs]
    height = [max(ground(x) - b, 0.0) for x, b in zip(xs, base)]
    w = [gamma * hh * width for hh in height]
    alpha = [math.asin((xc - x) / rad) for x in xs]
    factors = []
    for sign in (1.0, -1.0):
        fh = [kh * wi for wi in w]
        wv = [wi + sign * share * f for wi, f in zip(w, fh)]
        drive = sum(v * math.sin(a) + f * (yc - (b + 0.5 * hh)) / rad for v, a, f, b, hh in zip(wv, alpha, fh, base, height))
        if drive <= 0.0:
            return None
        residual = lambda f: sum((c * width + max(v, 0.0) * tan_phi) / (math.cos(a) * (1.0 + math.tan(a) * tan_phi / f)) for v, a in zip(wv, alpha)) / drive - f
        pole = max(0.0, -min(math.tan(a) for a in alpha) * tan_phi)
        low = pole * (1.0 + 1e-9) + 1e-9
        if residual(low) * residual(1.0e4) > 0.0:
            return None
        factors.append(optimize.brentq(residual, low, 1.0e4, xtol=1e-13))
    return min(factors)


def seismic_slopes(doc, a_g, s, out):
    """⛰ EN 1998-5 4.1.3.3: F_H = 0.5 alpha S S_T W and F_V = +-share F_H (0.5 above a_vg / a_g = 0.6, else 0.33; 0.9 for spectrum type 1, 0.45 for type 2) with the M2 strengths; the utilisation is the limit 1.0 over the least factor of the given circle and of the grid of circles (centres x from -h/2 to the toe plus h/2, z from 0.8 h to 2.4 h, radii 0.85 to 1.3 times the toe distance)."""
    import numpy as np

    type2 = "2" in (doc["site"].get("enSpectrumType") or "").lower()
    share = 0.33 if type2 else 0.5
    layers = {layer["id"]: layer for layer in doc.get("soilLayers", [])}
    for sl in doc.get("slopes", []):
        layer = layers.get(sl["governingLayerId"])
        if layer is None:
            continue
        kh = SLOPE_FACTOR * a_g / G * s * (sl["topographicAmplification"] if sl["topographicAmplification"] > 0.0 else 1.0)
        tan_phi, c = math.tan(math.radians(sl["frictionAngle"])) / M2, sl["cohesion"] / M2
        h, beta, gamma = sl["height"], sl["angle"], layer["unitWeightNM3"]
        run = h / math.tan(math.radians(beta))
        grid = [slope_bishop(h, beta, xc, yc, math.hypot(xc - run, yc) * scale, gamma, tan_phi, c, kh, share) for xc in -0.5 * h + (run + h) * np.arange(9) / 8.0 for yc in h * (0.8 + 1.6 * np.arange(7) / 6.0) for scale in (0.85, 1.0, 1.15, 1.3)]
        found = [f for f in grid + [slope_bishop(h, beta, sl["circleCentreX"], sl["circleCentreZ"], sl["circleRadius"], gamma, tan_phi, c, kh, share)] if f is not None]
        out(f"en1998.5.{sl['id']}.slope", max(1.0 / min(found), 0.0) if found else 0.0)


GAMMA_W = 9810.0
PERMEABILITY_LIMIT = 5.0e-4
SLIDING_GAMMA_RD = 1.0
VERTICAL_THRESHOLD, VERTICAL_HIGH, VERTICAL_LOW = 0.6, 0.5, 0.33


def mo_active(phi_deg, theta, delta_deg=0.0, psi_deg=90.0, beta_deg=0.0):
    """🧱 EN 1998-5 Eq. E.4 active coefficient K for the seismic angle theta [rad]; `None` where phi - theta - beta < 0 (no limit equilibrium)."""
    phi, delta, psi, beta = (math.radians(x) for x in (phi_deg, delta_deg, psi_deg, beta_deg))
    if phi - theta - beta < 0.0:
        return None
    root = math.sqrt(math.sin(phi + delta) * math.sin(phi - theta - beta) / (math.sin(psi - theta - delta) * math.sin(psi + beta)))
    return math.sin(psi + phi - theta) ** 2 / (math.cos(theta) * math.sin(psi) ** 2 * math.sin(psi - theta - delta) * (1.0 + root) ** 2)


def wall_thrust(wall, kh, share):
    """🧱 EN 1998-5 Annex E: E_d = 1/2 gamma (1 +- k_v) K H^2 above the water, the submerged layer with gamma' = gamma_sat - gamma_w and tan theta_w = gamma (permeable) or gamma_sat (low permeability) / gamma' x k_h / (1 +- k_v), hydrostatic 1/2 gamma_w H_w^2 and, for a permeable backfill, Westergaard 7/12 k_h gamma_w H_w^2; the larger total of both signs of k_v = share k_h governs. Returns (soil, static, dynamic) or `None`."""
    h, gamma, phi = wall["heightM"], wall["soilGamma"], wall["phiDeg"]
    wet = min(max(wall["waterHeightM"], 0.0), h)
    dry = h - wet
    sat = wall["saturatedGamma"] if wall["saturatedGamma"] > 0.0 else gamma
    permeable = wall["permeabilityMS"] >= PERMEABILITY_LIMIT
    best = None
    for sign in (1.0, -1.0):
        v = 1.0 + sign * share * kh
        k_dry = mo_active(phi, math.atan(kh / v))
        if k_dry is None:
            continue
        soil, static, dynamic = 0.5 * gamma * v * dry * dry * k_dry, 0.0, 0.0
        if wet > 0.0:
            eff = sat - GAMMA_W
            if eff <= 0.0:
                continue
            k_wet = mo_active(phi, math.atan(kh * (gamma if permeable else sat) / eff / v))
            if k_wet is None:
                continue
            soil += k_wet * v * (gamma * dry * wet + 0.5 * eff * wet * wet)
            static = 0.5 * GAMMA_W * wet * wet
            dynamic = 7 / 12 * kh * GAMMA_W * wet * wet if permeable else 0.0
        row = (soil, static, dynamic)
        if best is None or sum(row) > sum(best):
            best = row
    return best


def retaining_walls(doc, a_g, s, out):
    """🧱 EN 1998-5 Annex E thrust of every retaining wall against H_Rd (`.thrust`: soil + water; `.water`: hydrostatic + Westergaard, only with retained water); k_h = a_g S / (g r), k_v = share k_h with share 0.5 above a_vg / a_g = 0.6 (0.9 type 1) and 0.33 below (0.45 type 2). No row where the backfill has no limit equilibrium (the Rust check fails without utilisation)."""
    type2 = "2" in (doc["site"].get("enSpectrumType") or "").lower()
    ratio = 0.45 if type2 else 0.9
    share = VERTICAL_HIGH if ratio > VERTICAL_THRESHOLD else VERTICAL_LOW
    for w in doc.get("retainingWalls", []):
        kh = a_g / G * s / max(w["r"], 1.0)
        load = wall_thrust(w, kh, share)
        if load is None:
            continue
        out(f"en1998.5.{w['id']}.thrust", sum(load) / max(w["hRdNPerM"], 1e-9))
        if w["waterHeightM"] > 0.0:
            out(f"en1998.5.{w['id']}.water", (load[1] + load[2]) / max(w["hRdNPerM"], 1e-9))


def foundation_sliding(doc, a_gr, gamma_i, s, tb, tc, td, out):
    """🧱 EN 1998-5 5.4.1.1 sliding: H_Ed is the design base shear F_b of the first system of the supported building (spectrum with q capped and reduced for elevation irregularity, T1, lambda), H_Rd = (V - u) tan(delta) / gamma_Rd + passive resistance, V the seismic weight of the building, u the base pore pressure times the area."""
    buildings = doc.get("buildings", [])
    for f in doc.get("foundations", []):
        wanted = f["supportedBuildingId"] or (buildings[0]["id"] if buildings else "")
        b = next((x for x in buildings if x["id"] == wanted), None)
        if b is None:
            continue
        storeys, n = b["storeys"], len(b["storeys"])
        weights = [weight(st, i + 1 == n) for i, st in enumerate(storeys)]
        masses = [w / G for w in weights]
        t1, _ = fundamental_period(b, masses, sum(st["heightM"] for st in storeys))
        h_ed = design_base_shear(b, b["systems"][0] if b["systems"] else None, masses, t1, gamma_i * a_gr, s, tb, tc, td)[2]
        effective = max(sum(weights) - f["basePorePressurePa"] * f["areaM2"], 0.0)
        h_rd = effective * math.tan(math.radians(f["baseFrictionAngleDeg"])) / SLIDING_GAMMA_RD + f["passiveResistanceN"]
        out(f"en1998.5.{f['id']}.sliding", h_ed / max(h_rd, 1e-9))


def evaluate(doc):
    a_gr, s, tb, tc, td = site_parameters(doc)
    if a_gr <= 0:
        return [{"id": f"en1998.1.{b['id']}.zone0", "utilization": 0.0, "status": "notApplicable"} for b in doc.get("buildings", [])]
    gamma_i = GAMMA_I.get(doc["site"].get("importanceClass", "ii").strip().upper(), 1.0)
    a_g = gamma_i * a_gr
    rows = []
    out = lambda key, value: rows.append({"id": key, "utilization": value})
    cited = lambda key, value: rows.append({"id": key, "utilization": value, "status": "unverifiedBasis" if value <= 1.0 else "fail"})
    buildings = doc.get("buildings", [])
    for index, b in enumerate(buildings):
        bid, storeys = b["id"], b["storeys"]
        n = len(storeys)
        weights = [weight(st, i + 1 == n) for i, st in enumerate(storeys)]
        masses = [w / G for w in weights]
        heights = [st["heightM"] for st in storeys]
        height = sum(heights)
        t1, ct_period = fundamental_period(b, masses, height)
        lx, ly = b["planWidthM"], b["planLengthM"]
        if b.get("t1Method", "ct").lower() == "given":
            out(f"en1998.1.{bid}.t1Given", b["t1GivenS"] / max(1.5 * ct_period, 1e-9))
        if b.get("t1Method", "ct").lower() == "rayleigh" and not any(all(st[k] > 0 for st in storeys) for k in ("stiffnessX", "stiffnessY")):
            rows.append({"id": f"en1998.1.{bid}.t1Rayleigh", "utilization": 1.0, "status": "fail"})
        smaller = min(lx, ly)
        out(f"en1998.1.{bid}.planDims", min(0.01 / smaller, 1e6) if smaller > 0 else 1e6)
        types = {sy["systemType"].lower() for sy in b["systems"]}
        has_frame, has_wall = bool(types & {"frame", "dual"}), bool(types & {"wall", "dual"})
        if b.get("multipleResistingSystems", False) and not (has_frame and has_wall and b["planRegular"]):
            rows.append({"id": f"en1998.1.{bid}.multipleSystems", "utilization": 1.0, "status": "fail"})
        else:
            out(f"en1998.1.{bid}.multipleSystems", 0.0)
        applicable = b["elevationRegular"] and t1 <= 4.0 * tc and t1 <= 2.0
        rows.append({"id": f"en1998.1.{bid}.lfm", "utilization": 0.0, "status": "pass" if applicable else "warning"})
        if lx > 0 and ly > 0:
            for st in storeys:
                e0x, e0y = abs(st["centreOfMassXM"] - st["centreOfStiffnessXM"]), abs(st["centreOfMassYM"] - st["centreOfStiffnessYM"])
                kx, ky = stiffness_factor(b) * st["stiffnessX"], stiffness_factor(b) * st["stiffnessY"]
                k_theta = kx * (ly / 2) ** 2 + ky * (lx / 2) ** 2
                rx, ry = math.sqrt(k_theta / max(ky, 1e-9)), math.sqrt(k_theta / max(kx, 1e-9))
                ls = math.sqrt((lx * lx + ly * ly) / 12)
                out(f"en1998.1.{bid}.storey.{st['id']}.planRegularity", 1e9 if min(rx, ry) <= 0 else min(max(e0x / (0.30 * rx), e0y / (0.30 * ry), ls / rx, ls / ry), 1e9))
            for i in range(n - 1):
                lo, hi = storeys[i], storeys[i + 1]
                soft = lambda kl, ku: 0.0 if ku <= 0 else (math.inf if kl <= 0 else 0.70 * ku / kl)
                m_lo, m_hi = weight(lo, False) / G, weight(hi, i + 2 == n) / G
                u = max(soft(lo["stiffnessX"], hi["stiffnessX"]), soft(lo["stiffnessY"], hi["stiffnessY"]), (m_hi / m_lo if m_lo > 0 else 0.0) / 1.5)
                out(f"en1998.1.{bid}.elevationRegularity.{lo['id']}-{hi['id']}", min(u, 1e9))
        if lx > 0 and ly > 0:
            out(f"en1998.1.{bid}.plan.slenderness", max(lx, ly) / min(lx, ly) / 4)
            out(f"en1998.1.{bid}.plan.envelope", max(b.get("planEnvelopeGapRatio", 0.0), 0.0) / 0.05)
        out(f"en1998.1.{bid}.elevation.setback", max(b.get("setbackRatio", 0.0), 0.0) / 0.20)
        nxt = buildings[index + 1] if index + 1 < len(buildings) else None
        if nxt is not None and b.get("jointGapM", 0.0) > 0:
            d = lambda x: sum(st["driftXM"] for st in x["storeys"])
            out(f"en1998.1.{bid}.joint.gap", math.hypot(d(b), d(nxt)) / max(b["jointGapM"], 1e-9))
        if b.get("nonstructuralWeightN", 0.0) > 0:
            ratio = b["nonstructuralPeriodS"] / t1 if t1 > 0 else 0.0
            alpha = gamma_i * a_gr / G
            sa = max(alpha * s * (3 * (1 + b["nonstructuralHeightM"] / max(height, 1e-9)) / (1 + (1 - ratio) ** 2) - 0.5), alpha * s)
            out(f"en1998.1.{bid}.nonstructural.force", sa * b["nonstructuralWeightN"] / max(b["nonstructuralQ"], 1.0) / max(b["nonstructuralResistanceN"], 1e-9))
        ductile_frame = any(sy["systemType"].lower() in ("frame", "dual") and sy["ductilityClass"].lower() != "dcl" for sy in b["systems"])
        if b.get("beamMomentSumNm", 0.0) > 0 and ductile_frame:
            out(f"en1998.1.{bid}.capacityDesign.nodes", 1.3 * b["beamMomentSumNm"] / max(b["columnMomentSumNm"], 1e-9))
        for joint in doc.get("joints", []):
            top_exempt = n > 1 and joint["storeyId"] == storeys[-1]["id"]
            if joint["buildingId"] == bid and ductile_frame and joint["beamMomentRdNm"] > 0 and not top_exempt:
                out(f"en1998.1.{bid}.capacityDesign.joint.{joint['id']}", 1.3 * joint["beamMomentRdNm"] / max(joint["columnMomentRdNm"], 1e-9))
        directional = []
        for sy in b["systems"]:
            sid = sy["id"]
            direction = sy["direction"].lower()
            rows.append({"id": f"en1998.1.{bid}.{sid}.direction", "utilization": 1.0, "status": "fail"} if sum(1 for o in b["systems"] if o["direction"].lower() == direction) > 1 else {"id": f"en1998.1.{bid}.{sid}.direction", "utilization": 0.0})
            q, q_max, f_b = design_base_shear(b, sy, masses, t1, a_g, s, tb, tc, td)
            out(f"en1998.1.{bid}.{sid}.qLimit", 1.0 if 1.0 < q / q_max and LIMIT.within_limit(q / q_max) else q / q_max)
            out(f"en1998.1.{bid}.{sid}.baseShear", f_b / max(sy["baseShearResistanceN"], 1e-9))
            directional.append((sid, direction, f_b, sy["baseShearResistanceN"]))
            zs = [sum(heights[: i + 1]) for i in range(n)]
            denominator = sum(m * z for m, z in zip(masses, zs))
            forces = [f_b * m * z / denominator if denominator > 0 else 0.0 for m, z in zip(masses, zs)]
            if lx > 0 and ly > 0:
                across = lx if direction == "y" else ly
                e0 = max(abs((st["centreOfMassXM"] - st["centreOfStiffnessXM"]) if direction == "y" else (st["centreOfMassYM"] - st["centreOfStiffnessYM"])) for st in storeys)
                delta = 1 + 0.6 * (0.5 * across + e0) / max(across, 1e-9)
                out(f"en1998.1.{bid}.{sid}.torsion", delta / 1.6)
            theta_lim = DRIFT_THETA.get(b.get("driftLimitClass", "ductile").lower(), DRIFT_THETA["ductile"])
            weights_above = [sum(weights[i:]) for i in range(n)]
            shear = [f_b - sum(forces[:i]) for i in range(n)]
            for i, st in enumerate(storeys):
                drift = st["driftYM"] if direction == "y" else st["driftXM"]
                theta = weights_above[i] * drift / (shear[i] * st["heightM"]) if abs(shear[i]) > 1e-12 else math.inf
                v_amplified = shear[i] * amplification(theta)
                out(f"en1998.1.{bid}.{sid}.storeyForces.{st['id']}", v_amplified / max(st["shearResistanceN"], 1e-9))
                out(f"en1998.1.{bid}.{sid}.drift.{st['id']}", min(drift * b["nu"] / (theta_lim * st["heightM"]), 1e9))
                out(f"en1998.1.{bid}.{sid}.pdelta.{st['id']}", min(theta / 0.2, 1e9))
            m_ov = sum(f * z for f, z in zip(forces, zs))
            if m_ov > 0:
                entered, plan = (b["overturningBaseYM"], ly) if direction == "y" else (b["overturningBaseXM"], lx)
                base = entered if entered > 0 else plan
                m_stab = 1.0 * sum(st["permanentGkN"] for st in storeys) * base / 2 + b["tieDownMomentNm"]
                out(f"en1998.1.{bid}.{sid}.overturning", m_ov / max(m_stab, 1e-9))
            cls = {"dcm": "dcm", "dch": "dch"}.get(sy["ductilityClass"].lower(), "low")
            for m in b.get("members", []):
                base_id = f"en1998.1.{bid}.{sid}.member.{m['id']}"
                rows.append({"id": f"{base_id}.qDetail", "utilization": 0.0} if m["detailingCompatibleWithQ"] else {"id": f"{base_id}.qDetail", "utilization": 1.0, "status": "fail"})
                if m["material"].lower() in ("rc", "concrete"):
                    role = m["role"].lower() if m["role"].lower() in ("column", "beam") else "other"
                    lo, hi = RHO[role][cls]
                    out(f"{base_id}.rc.rho", m["rho"] / (lo if m["rho"] < lo else hi))
                    width = m["minDimensionM"]
                    if role == "column":
                        out(f"{base_id}.rc.minB", min(COLUMN_MIN_B[cls] / width, 1e6) if width > 0 else 1e6)
                    if role == "beam":
                        out(f"{base_id}.rc.beamMinB", min(BEAM_MIN_B / width, 1e6) if width > 0 else 1e6)
                    if role == "column" and COLUMN_OMEGA_WD_MIN[cls] > 0:
                        out(f"{base_id}.rc.omegaWd", min(COLUMN_OMEGA_WD_MIN[cls] / m["omegaWd"], 1e6) if m["omegaWd"] > 0 else 1e6)
                    if role == "beam":
                        ratio = m["rhoPrime"] / m["rho"] if m["rho"] > 1e-12 else 0.0
                        out(f"{base_id}.rc.rhoPrime", min(0.5 / ratio, 1e6) if ratio > 0 else 1e6)
                    out(f"{base_id}.rc.rhoPrimeAbs", min(m["rhoPrime"] / max(m["rho"], 1e-6), 1e6))
                    out(f"{base_id}.rc.noSteelClass", m["steelSectionClass"] / 0.5)
                    if role != "column":
                        out(f"{base_id}.rc.omegaWdBeam", m["omegaWd"] / 0.30)
                if m["material"].lower() == "steel":
                    max_class = 1 if q > 4 else (2 if q > 2 else 3)
                    out(f"{base_id}.steel.class", m["steelSectionClass"] / max_class)
        ranked = sorted(b["systems"], key=lambda sy: {"dch": 2, "dcm": 1}.get(sy["ductilityClass"].lower(), 0))
        if ranked and ranked[-1]["ductilityClass"].lower() in ("dcm", "dch"):
            governing = ranked[-1]
            for r in doc.get("criticalRegions", []):
                member = next((m for m in b.get("members", []) if m["id"] == r["memberId"] and m["material"].lower() in ("rc", "concrete")), None)
                if r["buildingId"] == bid and member and member["role"].lower() in ("column", "beam"):
                    for suffix, value in region_utilisations(r, member["role"].lower(), governing["ductilityClass"].lower(), governing["q0"], t1, tc):
                        out(f"en1998.1.{bid}.region.{r['id']}.{suffix}", value)
        first = lambda d: next((e for e in directional if e[1] == d), None)
        x, y = first("x"), first("y")
        if x and y:
            for own, other in ((x, y), (y, x)):
                envelope = max(own[2] + 0.3 * other[2], 0.3 * own[2] + other[2])
                out(f"en1998.1.{bid}.{own[0]}.componentCombination", envelope / max(own[3], 1e-9))
    for a in doc.get("assessments", []):
        ratio = cyclic_shear_utilisation(a)
        if ratio is not None:
            out(f"en1998.3.{a['id']}.cyclicShear", ratio)
    tie_beams(doc, a_g, s, out, cited)
    seismic_slopes(doc, a_g, s, cited)
    retaining_walls(doc, a_g, s, cited)
    foundation_sliding(doc, a_gr, gamma_i, s, tb, tc, td, cited)
    rows.extend(families.rows(doc, sys.modules[__name__], a_gr, s, tb, tc, td, gamma_i))
    rows.extend(geotechnics.rows(doc, a_gr, gamma_i, s))
    return rows


def self_check():
    """🧪 The Rayleigh period against the generalised eigenproblem of scipy.linalg.eigh (Rayleigh <= exact T1 as an upper bound of the frequency, equal for one storey), the plateau of the spectrum and the corner periods against scipy.optimize.brentq."""
    import numpy as np
    from scipy.linalg import eigh
    from scipy.optimize import brentq

    masses, k = [250e3, 240e3, 230e3, 210e3], [1.8e8, 1.7e8, 1.6e8, 1.5e8]
    n = len(masses)
    stiffness = np.zeros((n, n))
    for i in range(n):
        stiffness[i, i] += k[i] + (k[i + 1] if i + 1 < n else 0)
        if i + 1 < n:
            stiffness[i, i + 1] = stiffness[i + 1, i] = -k[i + 1]
    exact = 2 * math.pi / math.sqrt(eigh(stiffness, np.diag(masses), eigvals_only=True)[0])
    approx = rayleigh(masses, k)
    single = rayleigh([300e3], [2e8]), 2 * math.pi / math.sqrt(eigh(np.array([[2e8]]), np.array([[300e3]]), eigvals_only=True)[0])
    plateau = brentq(lambda t: design_spectrum(1.0, 1.25, 0.05, 0.25, 2.0, 3.9, t) - 1.0 * 1.25 * 2.5 / 3.9 * 0.5, 0.26, 2.0)
    results = {"rayleigh_over_eigh": approx / exact, "single_storey_ratio": single[0] / single[1], "spectrum_branch_T_at_half_plateau": plateau}
    assert exact * 0.97 <= approx <= exact, results
    assert abs(single[0] / single[1] - 1) < 1e-12, results
    assert abs(plateau - 0.5) > 0 and abs(design_spectrum(1.0, 1.25, 0.05, 0.25, 2.0, 3.9, plateau) - 1.25 * 2.5 / 3.9 * 0.5) < 1e-9, results
    return results


if __name__ == "__main__":
    if "--self-check" in sys.argv:
        print(json.dumps(self_check()))
    else:
        paths = [a for a in sys.argv[1:] if not a.startswith("--")]
        text = open(paths[0], encoding="utf8").read() if paths else sys.stdin.read()
        print(json.dumps({"checks": evaluate(json.loads(text))}, ensure_ascii=False))
