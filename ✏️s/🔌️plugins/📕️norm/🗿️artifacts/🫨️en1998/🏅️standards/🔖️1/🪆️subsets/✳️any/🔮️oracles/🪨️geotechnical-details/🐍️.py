"""🪨 Independent EN 1998-5 geotechnical families of the fifth oracle wave, written from the textbook closed forms (never from the Rust): the liquefaction safety factor of a foundation layer and of every layer of a soil profile by Annex B (cyclic stress ratio 0.65 (a_g S/g) sigma_v/sigma'_v without depth reduction, CRR_7.5 of Youd et al. 2001 from (N1)60cs, the magnitude scaling factor 10^2.24/M^2.56), the Youd et al. (2001) comparison factor with the Liao-Whitman depth reduction r_d, the screening criteria of 4.1.4, and the pile moments of 5.4.2 (Hetenyi long pile on a Winkler foundation by `scipy.integrate.quad` of the bending moment course, Nikolaou et al. 2001 kinematic moment at a layer interface). Every row is `{id, utilization, status[, value]}`; a passing row that cites a registry row is `unverifiedBasis`, informative rows are `info`. SI throughout."""

import json
import math
import pathlib

G = 9.81
WATER = 9810.0


def registry():
    root = next(q for q in pathlib.Path(__file__).resolve().parents if q.name == "📕️norm")
    entries = json.loads((root / "⚖️compliance" / "📜️annex-and-tables" / "🔖️ndp" / "🔣️.json").read_text(encoding="utf-8"))["entries"]
    return {e["id"]: e for e in entries}


def value(entries, key, annex):
    """🔢 The EN recommended value, or the NA-DE value under the German annex."""
    e = entries[key]
    return e["naDe"] if annex.lower() == "de" and e.get("naDe") is not None else e["recommended"]


def r_d(z):
    """🌊 Youd et al. (2001) depth reduction (Liao and Whitman): 1 - 0.00765 z to 9.15 m, 1.174 - 0.0267 z to 23 m, 0.744 - 0.008 z to 30 m, 0.5 below."""
    if z <= 9.15:
        return 1.0 - 0.00765 * z
    if z <= 23.0:
        return 1.174 - 0.0267 * z
    if z <= 30.0:
        return 0.744 - 0.008 * z
    return 0.5


def n_clean(n, fines):
    """🌊 (N1)60cs = alpha + beta (N1)60 (Youd et al. 2001, eq. 5): alpha = 0, beta = 1 up to 5 % fines; alpha = exp(1.76 - 190/FC^2), beta = 0.99 + FC^1.5/1000 up to 35 %; alpha = 5, beta = 1.2 above."""
    if fines <= 5.0:
        return n
    if fines < 35.0:
        return math.exp(1.76 - 190.0 / fines**2) + (0.99 + fines**1.5 / 1000.0) * n
    return 5.0 + 1.2 * n


def crr75(n_cs):
    """🌊 Cyclic resistance ratio of a magnitude 7.5 earthquake (Youd et al. 2001, eq. 4, N1,60cs < 30), not liquefiable from 30 on."""
    if n_cs >= 30.0:
        return math.inf
    return 1.0 / (34.0 - n_cs) + n_cs / 135.0 + 50.0 / (10.0 * n_cs + 45.0) ** 2 - 1.0 / 200.0


def msf(magnitude):
    return 10.0**2.24 / magnitude**2.56


def samples(lo, hi):
    return [lo + (hi - lo) * k / 8.0 for k in range(9)]


def smallest(points):
    best = (math.inf, None)
    for z, f in points:
        if f < best[0]:
            best = (f, z)
    return best


def row(out, key, util, status, value=None):
    out.append({"id": key, "utilization": util, "status": status, **({"value": value} if value is not None else {})})


def verdict(util, cites):
    return "fail" if util > 1.0 + 1e-9 else "unverifiedBasis" if cites else "pass"


def foundation_rows(doc, f, a_gs, entries, annex, out):
    """🌊 EN 1998-5 4.1.4 and Annex B for the layer of a foundation: `liquefaction` (required factor over F_L, the worst of nine depths in the layer band), `liquefaction.youd` and `liquefaction.screen` (informative)."""
    if min(f["liqN160"], f["liqUnitWeightNM3"], f["liqDepthM"], f["liqMagnitude"]) <= 0:
        return
    required = value(entries, "ndp.en1998.en5.liquefaction.min-safety-factor", annex)
    c = value(entries, "ndp.en1998.en5.liquefaction.annex-b.csr-factor", annex)
    half = 0.5 * max(f["liqLayerThicknessM"], 0.0)
    band = (max(f["liqDepthM"] - half, 0.1), f["liqDepthM"] + half)
    resistance = crr75(n_clean(f["liqN160"], f["liqFinesPercent"])) * msf(f["liqMagnitude"])

    def stresses(z):
        total = f["liqUnitWeightNM3"] * z
        return total, total - WATER * max(z - f["liqWaterDepthM"], 0.0)

    def annex_b(z):
        total, effective = stresses(z)
        csr = c * a_gs / G * total / effective if effective > 0 else math.inf
        return math.inf if csr <= 0 else resistance / csr

    def youd(z):
        total, effective = stresses(z)
        csr = 0.65 * a_gs / G * total / effective * r_d(z) if effective > 0 else math.inf
        return math.inf if csr <= 0 else resistance / csr

    f_l, _ = smallest([(z, annex_b(z)) for z in samples(*band)])
    f_y, z_y = smallest([(z, youd(z)) for z in samples(*band)])
    util = required / min(f_l, 1e9)
    row(out, f"en1998.5.{f['id']}.liquefaction", util, verdict(util, True))
    row(out, f"en1998.5.{f['id']}.liquefaction.youd", 0.0, "info", min(f_y, 1e9))
    row(out, f"en1998.5.{f['id']}.liquefaction.screen", 0.0, "info", a_gs / G)


def profile_rows(doc, f, a_gs, entries, annex, out):
    """🪨 EN 1998-5 Annex B over the layers of `soilLayers`: `layer.<id>.liquefaction` (required factor over the smallest F_L of nine depths from just below the top, at least 0.1 m, to the bottom of the layer; stresses integrate the layer unit weights) and the informative `profile.liquefaction` (smallest of all layers)."""
    layers = [l for l in doc.get("soilLayers", []) if l["foundationId"] == f["id"]]
    if not layers or f["liqMagnitude"] <= 0 or any(l["unitWeightNM3"] <= 0 or l["thicknessM"] <= 0 for l in layers):
        return
    required = value(entries, "ndp.en1998.en5.liquefaction.min-safety-factor", annex)
    c = value(entries, "ndp.en1998.en5.liquefaction.annex-b.csr-factor", annex)
    bottoms, depth = [], 0.0
    for l in layers:
        depth += l["thicknessM"]
        bottoms.append(depth)

    def stresses(z):
        top, total = 0.0, 0.0
        for l in layers:
            total += l["unitWeightNM3"] * max(0.0, min(l["thicknessM"], z - top))
            top += l["thicknessM"]
        total += layers[-1]["unitWeightNM3"] * max(0.0, z - top)
        return total, total - WATER * max(z - f["liqWaterDepthM"], 0.0)

    def factor(z):
        i = next((k for k, b in enumerate(bottoms) if z <= b + 1e-12), len(layers) - 1)
        l = layers[i]
        resistance = math.inf if l["n160"] <= 0 else crr75(n_clean(l["n160"], l["finesPercent"])) * msf(f["liqMagnitude"])
        total, effective = stresses(z)
        csr = c * a_gs / G * total / effective if effective > 0 else math.inf
        return math.inf if csr <= 0 else resistance / csr

    governing = []
    for i, l in enumerate(layers):
        top = 0.0 if i == 0 else bottoms[i - 1]
        lo = min(max(top + 1e-6, 0.1), bottoms[i])
        f_l, z = smallest([(z, factor(z)) for z in samples(lo, bottoms[i])])
        governing.append((f_l, z, i))
        if l["n160"] > 0:
            util = required / min(f_l, 1e9)
            row(out, f"en1998.5.{f['id']}.layer.{l['id']}.liquefaction", util, verdict(util, True))
    finite = [g for g in governing if math.isfinite(g[0])]
    if finite:
        row(out, f"en1998.5.{f['id']}.profile.liquefaction", 0.0, "info", min(min(g[0] for g in finite), 1e9))


def pile_rows(f, a_gs, entries, annex, out):
    """🏗 EN 1998-5 5.4.2: Winkler characteristic number beta = (k_h D / (4 E I))^(1/4), long pile beta L >= pi (`pile.length`), inertial moment of the head shear (free head: the maximum of V/beta exp(-x) sin x by quadrature of the course, fixed head: V/(2 beta) at the head, `pile.inertial`), kinematic moment at the interface of two layers with G2/G1 >= 6 (Nikolaou et al. 2001, `pile.kinematic`) and the sum against M_Rd (`pile.moment`)."""
    if min(f["pileDiameterM"], f["pileLengthM"], f["pileModulusPa"], f["pileSubgradeModulusNM3"]) <= 0:
        return
    d, length, e_p, k_h, shear = f["pileDiameterM"], f["pileLengthM"], f["pileModulusPa"], f["pileSubgradeModulusNM3"], f["pileHeadShearN"]
    beta = (k_h * d / (4.0 * e_p * math.pi * d**4 / 64.0)) ** 0.25
    fixed = f["pileHeadFixed"] >= 0.5
    row(out, f"en1998.5.{f['id']}.pile.length", math.pi / (beta * length), "pass" if beta * length >= math.pi else "fail")
    xs = [i * math.pi / 2000.0 for i in range(2001)]
    if fixed:
        m_in = max(abs(shear / (2.0 * beta) * math.exp(-x) * (math.cos(x) - math.sin(x))) for x in xs)
    else:
        m_in = max(shear / beta * math.exp(-x) * math.sin(x) for x in xs)
    row(out, f"en1998.5.{f['id']}.pile.inertial", 0.0, "info", m_in)
    contrast = (f["pileLowerVsMS"] / f["pileUpperVsMS"]) ** 2 if f["pileUpperVsMS"] > 0 else math.inf
    limit = value(entries, "ndp.en1998.en5.pile.stiffness-contrast", annex)
    applies = f["pileInterfaceDepthM"] > 0 and f["pileUpperVsMS"] > 0 and f["pileLowerVsMS"] > 0 and f["liqUnitWeightNM3"] > 0 and contrast >= limit
    m_k = 0.0
    if applies:
        n = lambda k: value(entries, f"ndp.en1998.ref.pile.nikolaou-2001.{k}", annex)
        tau = a_gs / G * f["liqUnitWeightNM3"] * f["pileInterfaceDepthM"]
        e_1 = 3.0 * f["liqUnitWeightNM3"] / G * f["pileUpperVsMS"] ** 2
        m_k = n("coefficient") * tau * d**3 * (length / d) ** n("length-exponent") * (e_p / e_1) ** n("modulus-exponent") * (f["pileLowerVsMS"] / f["pileUpperVsMS"]) ** n("velocity-exponent")
    util = (m_in + m_k) / f["pileMomentRdNm"] if f["pileMomentRdNm"] > 0 else (0.0 if m_in + m_k < 2.2e-16 else 1.0e6)
    if applies:
        row(out, f"en1998.5.{f['id']}.pile.kinematic", 0.0, "info", m_k)
    row(out, f"en1998.5.{f['id']}.pile.moment", min(util, 1.0e6), "fail" if util > 1.0 + 1e-9 else "pass")
    if f["pileInterfaceDepthM"] > 0 and not applies and f["pileUpperVsMS"] > 0 and f["pileLowerVsMS"] > 0:
        row(out, f"en1998.5.{f['id']}.pile.kinematic", 0.0, "info")


def rows(doc, a_gr, gamma_i, s):
    entries, annex = registry(), doc.get("annex", "de")
    a_gs = gamma_i * a_gr * s
    out = []
    for f in doc.get("foundations", []):
        foundation_rows(doc, f, a_gs, entries, annex, out)
        profile_rows(doc, f, a_gs, entries, annex, out)
        pile_rows(f, a_gs, entries, annex, out)
    return out


def self_check():
    """🔬 Hetenyi free head pile: the maximum moment 0.3224 V/beta against `scipy.integrate.quad` of the shear course (M(x) = integral of the shear V exp(-x) cos x - ... ) and CRR_7.5 for (N1)60cs = 10 against the hand sum 1/24 + 10/135 + 50/145^2 - 1/200 = 0.113119."""
    from scipy.integrate import quad

    peak = max(math.exp(-x) * math.sin(x) for x in (i * math.pi / 4000.0 for i in range(4001)))
    shear_integral = quad(lambda x: math.exp(-x) * (math.cos(x) - math.sin(x)), 0.0, math.pi / 4.0)[0]
    assert abs(peak - 0.3224) < 1e-3 and abs(shear_integral - peak) < 1e-9, (peak, shear_integral)
    assert abs(crr75(10.0) - (1 / 24 + 10 / 135 + 50 / 145**2 - 1 / 200)) < 1e-12, crr75(10.0)
    return {"hetenyi": peak, "crr10": crr75(10.0)}
