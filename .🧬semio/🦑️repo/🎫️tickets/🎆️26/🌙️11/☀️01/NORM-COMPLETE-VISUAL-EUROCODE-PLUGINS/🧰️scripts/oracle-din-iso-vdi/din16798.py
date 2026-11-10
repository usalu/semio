"""DIN EN 16798-1 / ISO 7730 independent cross-checks: PMV by scipy brentq on the clothing heat balance with IAPWS vapour pressure, PPD, the temperature inversion by brentq against the Rust 40-step bisection, band-edge PMV consistency, adaptive limits, steady-state CO2 by integrating the mass balance with scipy, saturation pressure."""
import math

import numpy as np
from scipy import integrate, optimize

from common import psat_iapws, psat_iso7730

EPS_RAD = 3.96e-8


def pmv_brent(ta, rh, v, met, clo, pa_model="iapws"):
    pa = rh / 100.0 * (psat_iapws(ta) if pa_model == "iapws" else psat_iso7730(ta))
    m, icl = met * 58.15, 0.155 * clo
    fcl = 1 + 1.29 * icl if icl <= 0.078 else 1.05 + 0.645 * icl
    hcf = 12.1 * math.sqrt(v)

    def f(tcl):
        hc = max(hcf, 2.38 * abs(tcl - ta) ** 0.25)
        return tcl - (35.7 - 0.028 * m - icl * (EPS_RAD * fcl * ((tcl + 273) ** 4 - (ta + 273) ** 4) + fcl * hc * (tcl - ta)))

    tcl = optimize.brentq(f, ta - 30.0, ta + 50.0, xtol=1e-12)
    hc = max(hcf, 2.38 * abs(tcl - ta) ** 0.25)
    loss = 3.05e-3 * (5733 - 6.99 * m - pa) + (0.42 * (m - 58.15) if m > 58.15 else 0.0) + 1.7e-5 * m * (5867 - pa) + 0.0014 * m * (34 - ta) + EPS_RAD * fcl * ((tcl + 273) ** 4 - (ta + 273) ** 4) + fcl * hc * (tcl - ta)
    return (0.303 * math.exp(-0.036 * m) + 0.028) * (m - loss)


def ppd(pmv):
    return 100 - 95 * math.exp(-0.03353 * pmv ** 4 - 0.2179 * pmv ** 2)


def rust_invert(rh, v, met, clo, limit, prefer_cool):
    """`annex-parameters-and-parts/🦀️.rs` invert_t_op_for_pmv: 40 bisection steps on [16, 30]."""
    lo, hi = 16.0, 30.0
    for _ in range(40):
        mid = 0.5 * (lo + hi)
        pmv = pmv_brent(mid, rh, v, met, clo, "iso7730")
        if prefer_cool:
            hi, lo = (mid, lo) if pmv > limit else (hi, mid)
        else:
            lo, hi = (mid, hi) if pmv < -limit else (lo, mid)
    return 0.5 * (lo + hi)


def brent_invert(rh, v, met, clo, target):
    g = lambda t: pmv_brent(t, rh, v, met, clo) - target
    if g(5.0) * g(45.0) > 0:
        return None
    return optimize.brentq(g, 5.0, 45.0, xtol=1e-10)


RUST_BANDS = {"winter": {1: (21.0, 23.0), 2: (20.0, 24.0), 3: (19.0, 25.0), 4: (18.0, 25.0)}, "summer": {1: (23.5, 25.5), 2: (23.0, 26.0), 3: (22.0, 27.0), 4: (21.0, 28.0)}}
PMV_LIMIT = {1: 0.2, 2: 0.5, 3: 0.7, 4: 1.0}


def co2_steady(occupants, volume, flow_m3h, outdoor_ppm=400.0, generation_l_s=0.005):
    g = occupants * generation_l_s / 1000.0
    q = flow_m3h / 3600.0
    c = lambda t, y: (g * 1e6 + q * (outdoor_ppm - y[0])) / volume
    sol = integrate.solve_ivp(c, (0.0, 60 * 3600.0), [outdoor_ppm], rtol=1e-10, atol=1e-8)
    return float(sol.y[0, -1]) - outdoor_ppm, g * 1e6 / q


def vectors():
    cases = [(24.5, 45, 0.1, 1.2, 0.5), (22.0, 60, 0.1, 1.2, 0.5), (27.0, 60, 0.1, 1.2, 0.5), (19.0, 40, 0.1, 1.2, 1.0), (23.5, 60, 0.1, 1.2, 0.5), (30.0, 80, 0.3, 1.0, 0.3), (16.0, 30, 0.05, 1.2, 1.0), (21.0, 50, 0.15, 1.6, 0.9)]
    pmv_rows = []
    for ta, rh, v, met, clo in cases:
        a, b = pmv_brent(ta, rh, v, met, clo, "iapws"), pmv_brent(ta, rh, v, met, clo, "iso7730")
        pmv_rows.append({"t_op": ta, "rh": rh, "v": v, "met": met, "clo": clo, "pmvIapwsVapour": a, "pmvIso7730Vapour": b, "ppd": ppd(b), "vapourModelDelta": a - b})
    inv = []
    for rh, v, met, clo in [(50, 0.1, 1.2, 0.5), (50, 0.1, 1.2, 1.0), (40, 0.1, 1.2, 1.5), (60, 0.2, 1.6, 0.5), (50, 0.05, 1.0, 0.3)]:
        for cat, lim in PMV_LIMIT.items():
            for cool in (True, False):
                ref = brent_invert(rh, v, met, clo, lim if cool else -lim)
                rust = rust_invert(rh, v, met, clo, lim, cool)
                inv.append({"rh": rh, "v": v, "met": met, "clo": clo, "category": cat, "side": "upper" if cool else "lower", "referenceC": ref, "rustTranscriptC": rust, "rustClampedToBracket": rust < 16.0001 or rust > 29.9999, "referenceInBracket": ref is not None and 16.0 <= ref <= 30.0})
    edges = []
    for season, clo in (("winter", 1.0), ("summer", 0.5)):
        for cat, (lo, hi) in RUST_BANDS[season].items():
            for edge, t in (("lower", lo), ("upper", hi)):
                edges.append({"season": season, "category": cat, "edge": edge, "t_op": t, "clo": clo, "rh": 50, "v": 0.1, "met": 1.2, "pmv": pmv_brent(t, 50, 0.1, 1.2, clo), "pmvLimit": PMV_LIMIT[cat]})
    co2 = []
    for occ, vol, flow in [(1, 30.0, 25.0), (20, 200.0, 180.0), (30, 150.0, 400.0)]:
        sim, closed = co2_steady(occ, vol, flow)
        co2.append({"occupants": occ, "volumeM3": vol, "flowM3H": flow, "outdoorPpm": 400.0, "generationLs": 0.005, "steadyAboveOutdoorPpm": sim, "closedFormPpm": closed, "rustMethod2OutdoorAirM3H": lambda_rust(occ, 1.2, closed)})
    sat = [{"tC": t, "iapwsPa": psat_iapws(t), "iso7730Pa": psat_iso7730(t), "iso7730Relative": psat_iso7730(t) / psat_iapws(t) - 1} for t in (10, 15, 18, 20, 22, 24, 26, 28, 30)]
    adaptive = [{"thetaRm": t, "comfort": 0.33 * t + 18.8, "rustLower": {c: 0.33 * t + 18.8 - w for c, w in {1: 2, 2: 3, 3: 4, 4: 5}.items()}, "rustUpper": {c: 0.33 * t + 18.8 + w for c, w in {1: 2, 2: 3, 3: 4, 4: 5}.items()}} for t in (10.0, 20.0, 30.0)]
    draught = [{"ta": ta, "v": v, "tu": tu, "dr": min(max((34 - ta) * max(v - 0.05, 0) ** 0.62 * (0.37 * max(v, 0.05) * tu + 3.14), 0), 100)} for ta, v, tu in [(20, 0.15, 40), (22, 0.3, 40), (24, 0.1, 20), (20, 0.05, 40)]]
    return {"oracle": "scipy brentq (PMV, inversion), scipy solve_ivp (CO2), IAPWS-IF97 vapour pressure", "pmv": pmv_rows, "inversion": inv, "bandEdges": edges, "co2": co2, "saturation": sat, "adaptive": adaptive, "draught": draught}


def lambda_rust(occ, met, delta):
    g = occ * 0.005 * max(met / 1.2, 0.5)
    return (1.0e6 * g / max(delta, 100.0)) * 3.6


def divergences(doc):
    out = [{"id": f"din16798.invert.rh{r['rh']}-v{r['v']}-met{r['met']}-clo{r['clo']}.cat{r['category']}.{r['side']}", "clause": "ISO 7730 Annex D inversion for EN 16798-1 Table B.2", "inputs": f"rh {r['rh']} %, v {r['v']} m/s, {r['met']} met, {r['clo']} clo, |PMV| <= {PMV_LIMIT[r['category']]}", "rust": r["rustTranscriptC"], "reference": r["referenceC"], "relative": r["rustTranscriptC"] / r["referenceC"] - 1, "note": "the bisection bracket [16, 30] C clamps the answer; the true temperature lies outside"} for r in doc["inversion"] if r["rustClampedToBracket"] and not r["referenceInBracket"]]
    out += [{"id": f"din16798.band-pmv.{e['season']}.cat{e['category']}.{e['edge']}", "clause": "EN 16798-1 Table B.2 band vs ISO 7730 PMV limit", "inputs": f"t_op {e['t_op']} C, 50 % rh, 0.1 m/s, 1.2 met, {e['clo']} clo", "rust": e["t_op"], "reference": e["pmv"], "relative": None, "note": f"informational: PMV at the tabulated band edge is {e['pmv']:.3f} against limit +-{e['pmvLimit']}"} for e in doc["bandEdges"] if abs(e["pmv"]) > e["pmvLimit"]]
    out.append({"id": "din16798.adaptive.lower-limits", "clause": "EN 16798-1 Annex B adaptive model (to verify against the clause text)", "inputs": "theta_rm 20 C, category II", "rust": "theta_c - 3.0 K", "reference": "theta_c - 4.0 K (EN 16798-1 Figure B.? asymmetric lower limits -3/-4/-5 K, unverified)", "relative": None, "note": "the Rust band is symmetric +-2/3/4/5 K as in EN 15251; verify before changing"})
    return out
