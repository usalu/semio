"""DIN 4108 independent cross-checks: ISO 6946 combined method in exact rational arithmetic, DIN 4108-2 f_Rsi with Rsi = 0.25, ISO 13788 Glaser with the exact condensation-plane iteration and the DIN 4108-3 Annex A boundary conditions. `rust_*` functions transcribe the Rust source line by line (named in each docstring) so that the divergence is quantified without a build."""
import itertools
import json
import math
from fractions import Fraction as F

from scipy import optimize

from common import psat_iapws, psat_iso13788, psat_magnus_rust

DELTA0 = 2.0e-10
HOURS = 90 * 24


def layer(d, lam=None, segments=None, mu=1.0, mu_segments=None):
    return {"thicknessM": d, "lambda": lam or 0.0, "mu": mu, "segments": [{"fraction": f, "lambda": l} for f, l in (segments or [])]}


def iso6946(layers, rsi, rse):
    """EN ISO 6946 6.7.2 combined method: all section combinations (Fractions), upper/lower bound and mean."""
    rsi, rse = F(rsi).limit_denominator(10**6), F(rse).limit_denominator(10**6)
    secs = []
    for L in layers:
        d = F(L["thicknessM"]).limit_denominator(10**9)
        if L["segments"]:
            secs.append([(F(s["fraction"]).limit_denominator(10**9), d / F(s["lambda"]).limit_denominator(10**9)) for s in L["segments"]])
        else:
            secs.append([(F(1), d / F(L["lambda"]).limit_denominator(10**9))])
    inv_upper = F(0)
    for combo in itertools.product(*secs):
        f = F(1)
        for fr, _ in combo:
            f *= fr
        inv_upper += f / (rsi + rse + sum(r for _, r in combo))
    upper = 1 / inv_upper
    lower = rsi + rse + sum(1 / sum(fr / r for fr, r in s) for s in secs)
    return float(upper), float(lower), float((upper + lower) / 2)


def rust_total_resistance(layers, rsi, rse):
    """`resistance-helpers/🦀️.rs` `total_resistance` (first inhomogeneous layer carries the fractions of every inhomogeneous layer)."""
    def r_layer(L):
        return L["thicknessM"] / L["lambda"] if L["lambda"] > 0 else math.inf
    def r_lower(L):
        if not L["segments"]:
            return r_layer(L)
        s = sum(g["fraction"] / (L["thicknessM"] / g["lambda"]) for g in L["segments"] if g["fraction"] > 0 and g["lambda"] > 0)
        return 1 / s if s > 0 else math.inf
    prim = next((L for L in layers if L["segments"]), None)
    if prim is None:
        return rsi + rse + sum(r_layer(L) for L in layers)
    inv = 0.0
    for j, seg in enumerate(prim["segments"]):
        if seg["fraction"] <= 0:
            continue
        rt = rsi + rse
        for L in layers:
            if not L["segments"]:
                rt += r_layer(L)
            else:
                sec = L["segments"][j] if j < len(L["segments"]) else seg
                rt += L["thicknessM"] / sec["lambda"]
        inv += seg["fraction"] / rt
    upper = 1 / inv
    lower = rsi + rse + sum(r_lower(L) for L in layers)
    return 0.5 * (upper + lower)


U_CASES = [
    ("two-inhomogeneous-layers-different-fractions", 0.13, 0.04, [layer(0.0125, 0.25), layer(0.04, segments=[(0.88, 0.035), (0.12, 0.13)]), layer(0.06, segments=[(0.92, 0.035), (0.08, 0.13)]), layer(0.0125, 0.25)]),
    ("rafter-roof-single-inhomogeneous-layer", 0.10, 0.04, [layer(0.0125, 0.25), layer(0.18, segments=[(0.85, 0.035), (0.15, 0.13)]), layer(0.024, 0.13)]),
    ("upper-lower-ratio-above-1-5", 0.13, 0.04, [layer(0.2, segments=[(0.9, 0.04), (0.1, 2.0)]), layer(0.0125, 0.25)]),
    ("mortar-joints-brick-wall", 0.13, 0.04, [layer(0.365, segments=[(0.92, 0.12), (0.08, 0.9)]), layer(0.015, 0.7)]),
]


def u_vectors(fixture):
    rows = []
    for c in fixture["cases"]:
        up, lo, rt = iso6946(c["layers"], c["rSi"], c["rSe"])
        rows.append({"id": "fixture:" + c["id"], "rSi": c["rSi"], "rSe": c["rSe"], "layers": c["layers"], "rUpper": up, "rLower": lo, "rTotal": rt, "u": 1 / rt, "fixtureRTotal": c["rTotal"], "fixtureAgrees": abs(rt - c["rTotal"]) / rt < 1e-9, "ratioUpperLower": up / lo, "rustRTotal": rust_total_resistance(c["layers"], c["rSi"], c["rSe"])})
    for name, rsi, rse, layers in U_CASES:
        up, lo, rt = iso6946(layers, rsi, rse)
        rows.append({"id": name, "rSi": rsi, "rSe": rse, "layers": layers, "rUpper": up, "rLower": lo, "rTotal": rt, "u": 1 / rt, "ratioUpperLower": up / lo, "rustRTotal": rust_total_resistance(layers, rsi, rse)})
    for r in rows:
        r["rustDeviationRelative"] = (r["rustRTotal"] - r["rTotal"]) / r["rTotal"]
        r["combinedMethodApplicable"] = r["ratioUpperLower"] <= 1.5
    return rows


def f_rsi_vectors():
    out = []
    for name, rsi_rust, rse, layers in [("etics-wall", 0.13, 0.04, [layer(0.015, 0.7), layer(0.24, 0.81), layer(0.14, 0.035), layer(0.008, 0.7)]), ("bare-concrete-lintel", 0.13, 0.04, [layer(0.2, 2.1)]), ("thin-frame", 0.13, 0.04, [layer(0.06, 0.13)]), ("roof", 0.10, 0.04, [layer(0.0125, 0.25), layer(0.16, 0.035)]), ("marginal-wall", 0.13, 0.04, [layer(0.3, 0.9), layer(0.03, 0.2)])]:
        r_layers = sum(L["thicknessM"] / L["lambda"] for L in layers)
        rt_rust = rsi_rust + rse + r_layers
        rt_std = 0.25 + rse + r_layers
        out.append({"id": name, "rLayers": r_layers, "rSiRust": rsi_rust, "fRsiRust": 1 - rsi_rust / rt_rust, "rSiStandard": 0.25, "fRsiStandard": 1 - 0.25 / rt_std, "limit": 0.70, "rustPasses": 1 - rsi_rust / rt_rust >= 0.70, "standardPasses": 1 - 0.25 / rt_std >= 0.70})
    theta_si_min = optimize.brentq(lambda t: psat_iso13788(t) - 0.5 * psat_iso13788(20.0) / 0.8, 0.0, 20.0)
    crit = {"thetaSiMinC": theta_si_min, "fRsiMinAtMinus5": (theta_si_min + 5.0) / 25.0, "thetaSiMinIapwsC": optimize.brentq(lambda t: psat_iapws(t) - 0.5 * psat_iapws(20.0) / 0.8, 0.0, 20.0)}
    return out, crit


def solve_profile(sd, t_c, p_i, p_e, psat, floor):
    """ISO 13788 condensation iteration: the interface with the largest excess becomes a condensation plane (pressure = saturation), repeat; planes with negative flux are dropped."""
    n = len(sd) - 1
    ps = [psat(t) for t in t_c]
    planes = []
    def pressures(planes):
        nodes = [0] + sorted(planes) + [n]
        p = [0.0] * (n + 1)
        fixed = {0: p_i, n: p_e}
        for k in planes:
            fixed[k] = ps[k]
        for a, b in zip(nodes, nodes[1:]):
            for k in range(a, b + 1):
                span = sd[b] - sd[a]
                p[k] = fixed[a] + (fixed[b] - fixed[a]) * ((sd[k] - sd[a]) / span if span > 0 else 0)
        return p, nodes
    while True:
        p, nodes = pressures(planes)
        excess = [(p[k] - ps[k], k) for k in range(1, n) if k not in planes and p[k] > ps[k]]
        if not excess:
            break
        planes.append(max(excess)[1])
    p, nodes = pressures(planes)
    mass = 0.0
    fixed = {0: p_i, n: p_e, **{k: ps[k] for k in planes}}
    for idx, k in enumerate(nodes[1:-1], start=1):
        a, b = nodes[idx - 1], nodes[idx + 1]
        left = max(sd[k] - sd[a], floor)
        right = max(sd[b] - sd[k], floor)
        mass += DELTA0 * HOURS * ((fixed[a] - ps[k]) / left + (fixed[b] - ps[k]) / right)
    return planes, p, ps, max(mass, 0.0)


def interface_temps(layers, rsi, rse, ti, te):
    r = [L["thicknessM"] / L["lambda"] for L in layers]
    rt = rsi + rse + sum(r)
    temps, acc = [ti - rsi / rt * (ti - te)], rsi
    for x in r:
        acc += x
        temps.append(ti - acc / rt * (ti - te))
    return temps


def glaser_standard(layers, rsi, rse, winter=(20.0, 0.5, -5.0, 0.8), summer=(12.0, 0.7, 12.0, 0.7), psat=psat_iso13788, floor=1e-3):
    """DIN 4108-3 Annex A conditions: winter 20 C/50 % to -5 C/80 %, evaporation period 12 C/70 % on both sides, 90 d each."""
    sd = [0.0]
    for L in layers:
        sd.append(sd[-1] + L["thicknessM"] * L["mu"])
    tw = interface_temps(layers, rsi, rse, winter[0], winter[2])
    pw_i, pw_e = winter[1] * psat(winter[0]), winter[3] * psat(winter[2])
    planes, _, ps_w, condensed = solve_profile(sd, tw, pw_i, pw_e, psat, floor)
    ts = interface_temps(layers, rsi, rse, summer[0], summer[2])
    ps_s = [psat(t) for t in ts]
    ps_i, ps_e = summer[1] * psat(summer[0]), summer[3] * psat(summer[2])
    nodes = [0] + sorted(planes) + [len(sd) - 1]
    fixed = {0: ps_i, len(sd) - 1: ps_e, **{k: ps_s[k] for k in planes}}
    evaporated = 0.0
    for idx, k in enumerate(nodes[1:-1], start=1):
        a, b = nodes[idx - 1], nodes[idx + 1]
        evaporated += DELTA0 * HOURS * (max(ps_s[k] - fixed[a], 0.0) / max(sd[k] - sd[a], floor) + max(ps_s[k] - fixed[b], 0.0) / max(sd[b] - sd[k], floor))
    return {"planes": planes, "condensedKgM2": condensed, "evaporatedKgM2": evaporated, "dries": condensed <= evaporated}


def rust_glaser(layers, rsi, rse, ti=20.0, rhi=0.5, winter=(-5.0, 0.8), summer=(12.0, 0.7)):
    """`part-3-moisture-protection/🦀️.rs` glaser_condensed_mass_kg_m2 / glaser_evaporated_mass_kg_m2 transcribed (Magnus, interior 20 C/50 % in summer, linear-profile excess on both sides, 0.1 m floor, surface nodes included)."""
    sat = psat_magnus_rust
    def sdm(L):
        return L["thicknessM"] * L["mu"]
    def profile(te, rhe):
        temps = interface_temps(layers, rsi, rse, ti, te)
        t_c = [temps[i] for i in range(len(layers) + 1)]
        p_int, p_ext = rhi * sat(ti), rhe * sat(te)
        total = sum(sdm(L) for L in layers)
        sd = [0.0]
        for L in layers:
            sd.append(sd[-1] + sdm(L))
        p = [p_int + (p_ext - p_int) * s / total for s in sd]
        return sd, p, [sat(t) for t in t_c]
    sd, p, ps = profile(*winter)
    total = sd[-1]
    condensed = 0.0
    for i in range(len(layers) + 1):
        if p[i] > ps[i]:
            left = 0.1 if i == 0 else max(sd[i], 0.1)
            right = 0.1 if i >= len(layers) else max(total - sd[i], 0.1)
            condensed += (p[i] - ps[i]) * (1 / left + 1 / right) * DELTA0 * HOURS
    sdw, pw, psw = sd, p, ps
    sds, ps_, pss = profile(*summer)
    evap = 0.0
    for i in range(len(pw)):
        if pw[i] > psw[i]:
            l, r = max(sds[i], 0.1), max(sds[-1] - sds[i], 0.1)
            evap += (max(pss[i] - ps_[0], 0.0) / l + max(pss[i] - ps_[-1], 0.0) / r) * DELTA0 * HOURS * 1.0
    return {"condensedKgM2": condensed, "evaporatedKgM2": evap}


def mu(d, lam, m):
    return {"thicknessM": d, "lambda": lam, "mu": m, "segments": []}


G_CASES = [
    ("etics-no-condensation", 0.13, 0.04, [mu(0.015, 0.7, 10), mu(0.24, 0.81, 10), mu(0.14, 0.035, 40), mu(0.008, 0.7, 20)]),
    ("vapour-tight-outside", 0.13, 0.04, [mu(0.0125, 0.25, 1), mu(0.2, 0.035, 1.3), mu(0.02, 0.13, 1000)]),
    ("interior-insulated-brick", 0.13, 0.04, [mu(0.0125, 0.25, 8), mu(0.06, 0.035, 1.3), mu(0.365, 0.8, 10), mu(0.02, 0.87, 15)]),
    ("flat-roof-vapour-barrier-inside", 0.10, 0.04, [mu(0.0125, 0.25, 8), mu(0.0002, 0.5, 50000), mu(0.22, 0.035, 1.3), mu(0.004, 0.17, 10000)]),
    ("timber-wall-osb-outside", 0.13, 0.04, [mu(0.0125, 0.25, 8), mu(0.16, 0.04, 1.2), mu(0.015, 0.13, 50), mu(0.06, 0.04, 1.2), mu(0.01, 0.7, 25)]),
]


def glaser_vectors():
    rows = []
    for name, rsi, rse, layers in G_CASES:
        std = glaser_standard(layers, rsi, rse)
        std_floor = glaser_standard(layers, rsi, rse, floor=0.1)
        iapws = glaser_standard(layers, rsi, rse, psat=psat_iapws)
        rust = rust_glaser(layers, rsi, rse)
        rows.append({"id": name, "rSi": rsi, "rSe": rse, "tIntC": 20.0, "rhInt": 0.5, "winter": {"tC": -5.0, "rh": 0.8}, "summerAnnexA": {"tC": 12.0, "rh": 0.7, "interiorTC": 12.0, "interiorRh": 0.7}, "hours": HOURS, "layers": layers, "reference": std, "referenceSdFloor0_1": std_floor, "referenceIapws": iapws, "rustTranscript": rust})
    return rows


def vectors(fixture_path):
    fixture = json.load(open(fixture_path, encoding="utf8"))
    f_rows, crit = f_rsi_vectors()
    doc = {"oracle": "scipy brentq, IAPWS-IF97 region 4 + R14-08, ISO 13788 curves, exact Fractions (ISO 6946 6.7.2), ISO 13788 condensation-plane iteration", "toleranceRelative": 1e-9, "uValues": u_vectors(fixture), "fRsi": f_rows, "fRsiCritical": crit, "glaser": glaser_vectors(), "saturation": [{"tC": t, "iapws": psat_iapws(t), "iso13788": psat_iso13788(t), "rustMagnus": psat_magnus_rust(t)} for t in (-10, -5, 0, 5, 12, 20, 25, 30)]}
    return doc


def divergences(doc):
    out = []
    for r in doc["uValues"]:
        if abs(r["rustDeviationRelative"]) > 1e-6:
            out.append({"id": "din4108.u." + r["id"], "clause": "EN ISO 6946 6.7.2 combined method", "inputs": r["id"], "rust": r["rustRTotal"], "reference": r["rTotal"], "relative": r["rustDeviationRelative"], "note": "the first inhomogeneous layer's fractions are used for every inhomogeneous layer"})
        if not r["combinedMethodApplicable"]:
            out.append({"id": "din4108.u-ratio." + r["id"], "clause": "EN ISO 6946 6.7.2 / 6.7.3 (ratio R_upper/R_lower > 1.5)", "inputs": r["id"], "rust": r["rustRTotal"], "reference": r["rTotal"], "relative": None, "note": f"ratio {r['ratioUpperLower']:.3f}: the combined method is not applicable, Rust reports no warning"})
    for r in doc["fRsi"]:
        if r["rustPasses"] != r["standardPasses"]:
            out.append({"id": "din4108.frsi." + r["id"], "clause": "DIN 4108-2 6.2 (R_si = 0.25 m2K/W for the mould criterion)", "inputs": r["id"], "rust": r["fRsiRust"], "reference": r["fRsiStandard"], "relative": r["fRsiRust"] / r["fRsiStandard"] - 1, "note": "verdict flips: Rust passes with R_si 0.13/0.10, the standard value fails f_Rsi >= 0.70"})
    for g in doc["glaser"]:
        ref, rust = g["reference"], g["rustTranscript"]
        if ref["condensedKgM2"] == 0.0 and rust["condensedKgM2"] == 0.0:
            continue
        out.append({"id": "din4108.glaser." + g["id"], "clause": "DIN 4108-3 Annex A / EN ISO 13788 Glaser", "inputs": f"winter 20 C/50 % to -5 C/80 %, evaporation 12 C/70 % both sides, 90 d each, {g['id']}", "rust": [rust["condensedKgM2"], rust["evaporatedKgM2"]], "reference": [ref["condensedKgM2"], ref["evaporatedKgM2"]], "relative": [(rust["condensedKgM2"] / ref["condensedKgM2"] - 1) if ref["condensedKgM2"] else None, (rust["evaporatedKgM2"] / ref["evaporatedKgM2"] - 1) if ref["evaporatedKgM2"] else None], "note": "Magnus water curve below 0 C, summer interior 20 C/50 %, linear-profile excess on both sides, sd floor 0.1 m, surface nodes counted"})
    return out
