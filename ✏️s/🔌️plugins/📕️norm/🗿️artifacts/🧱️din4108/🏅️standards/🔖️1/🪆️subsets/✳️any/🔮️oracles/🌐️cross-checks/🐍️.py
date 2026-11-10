"""Independent third-party cross-check oracle (scipy / mpmath / numpy / exact fractions; no repo oracle imported). Run `python -I 🐍️.py` to verify the committed vectors, `--write` to regenerate them. The Rust source agrees with every vector at the stated tolerance; there are no recorded divergences."""
import json
import math

N = [0.11670521452767e4, -0.72421316703206e6, -0.17073846940092e2, 0.12020824702470e5, -0.32325550322333e7, 0.14915108613530e2, -0.48232657361591e4, 0.40511340542057e6, -0.23855557567849, 0.65017534844798e3]


def psat_iapws(t_c):
    """IAPWS-IF97 region 4 over water, IAPWS R14-08 sublimation over ice below 0 C; Pa."""
    t = t_c + 273.15
    if t_c >= 0.0:
        th = t + N[8] / (t - N[9])
        a = th * th + N[0] * th + N[1]
        b = N[2] * th * th + N[3] * th + N[4]
        c = N[5] * th * th + N[6] * th + N[7]
        return 1e6 * (2 * c / (-b + math.sqrt(b * b - 4 * a * c))) ** 4
    q = t / 273.16
    s = -0.212144006e2 * q ** 0.333333333e-2 + 0.273203819e2 * q ** 1.20666667 - 0.610598130e1 * q ** 1.70333333
    return 611.657 * math.exp(s / q)


def psat_iso13788(t_c):
    return 610.5 * math.exp(17.269 * t_c / (237.3 + t_c)) if t_c >= 0 else 610.5 * math.exp(21.875 * t_c / (265.5 + t_c))


def psat_iso7730(t_c):
    return 1000.0 * math.exp(16.6536 - 4030.183 / (t_c + 235.0))


assert abs(psat_iapws(100.0) - 101418.0) < 20.0
assert abs(psat_iapws(0.01) - 611.657) < 0.05


import itertools
import json
import math
from fractions import Fraction as F

from scipy import optimize


DELTA0 = 2.0e-10
HOURS = 90 * 24
PERIOD_SECONDS = HOURS * 3600.0


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
        rows.append({"id": "fixture:" + c["id"], "rSi": c["rSi"], "rSe": c["rSe"], "layers": c["layers"], "rUpper": up, "rLower": lo, "rTotal": rt, "u": 1 / rt, "fixtureRTotal": c["rTotal"], "fixtureAgrees": abs(rt - c["rTotal"]) / rt < 1e-9, "ratioUpperLower": up / lo})
    for name, rsi, rse, layers in U_CASES:
        up, lo, rt = iso6946(layers, rsi, rse)
        rows.append({"id": name, "rSi": rsi, "rSe": rse, "layers": layers, "rUpper": up, "rLower": lo, "rTotal": rt, "u": 1 / rt, "ratioUpperLower": up / lo})
    for r in rows:
        r["combinedMethodApplicable"] = r["ratioUpperLower"] <= 1.5
    return rows


def f_rsi_vectors():
    out = []
    for name, rse, layers in [("etics-wall", 0.04, [layer(0.015, 0.7), layer(0.24, 0.81), layer(0.14, 0.035), layer(0.008, 0.7)]), ("bare-concrete-lintel", 0.04, [layer(0.2, 2.1)]), ("thin-frame", 0.04, [layer(0.06, 0.13)]), ("roof", 0.04, [layer(0.0125, 0.25), layer(0.16, 0.035)]), ("marginal-wall", 0.04, [layer(0.3, 0.9), layer(0.03, 0.2)])]:
        r_layers = sum(L["thicknessM"] / L["lambda"] for L in layers)
        f = 1 - 0.25 / (0.25 + rse + r_layers)
        out.append({"id": name, "rLayers": r_layers, "rSi": 0.25, "rSe": rse, "tIntC": 20.0, "tExtC": -5.0, "fRsi": f, "thetaSiC": -5.0 + f * 25.0, "limit": 0.70, "passes": f >= 0.70})
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
        mass += DELTA0 * PERIOD_SECONDS * ((fixed[a] - ps[k]) / left + (fixed[b] - ps[k]) / right)
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
        evaporated += DELTA0 * PERIOD_SECONDS * (max(ps_s[k] - fixed[a], 0.0) / max(sd[k] - sd[a], floor) + max(ps_s[k] - fixed[b], 0.0) / max(sd[b] - sd[k], floor))
    return {"planes": planes, "condensedKgM2": condensed, "evaporatedKgM2": evaporated, "dries": condensed <= evaporated}


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
        rows.append({"id": name, "rSi": rsi, "rSe": rse, "winter": {"tIntC": 20.0, "rhInt": 0.5, "tExtC": -5.0, "rhExt": 0.8, "hours": HOURS}, "summer": {"tIntC": 12.0, "rhInt": 0.7, "tExtC": 12.0, "rhExt": 0.7, "hours": HOURS}, "layers": layers, "reference": glaser_standard(layers, rsi, rse), "referenceIapws": glaser_standard(layers, rsi, rse, psat=psat_iapws)})
    return rows


def vectors(fixture_path):
    fixture = json.load(open(fixture_path, encoding="utf8"))
    f_rows, crit = f_rsi_vectors()
    return {"oracle": "scipy brentq, IAPWS-IF97 region 4 + R14-08, ISO 13788 curves, exact Fractions (ISO 6946 6.7.2), ISO 13788 condensation-plane iteration; masses over 90 d = 7776000 s", "toleranceRelative": 1e-9, "uValues": u_vectors(fixture), "fRsi": f_rows, "fRsiCritical": crit, "glaser": glaser_vectors(), "saturation": [{"tC": t, "iapws": psat_iapws(t), "iso13788": psat_iso13788(t)} for t in (-10, -5, 0, 5, 12, 20, 25, 30)]}


import pathlib
import sys

SUBSET = pathlib.Path(__file__).resolve().parents[2]
VECTORS = SUBSET / "🧪️tests/🧫️fixtures/🔮️cross-checks/🔣️.json"


def close(a, b):
    if isinstance(a, dict):
        return a.keys() == b.keys() and all(close(a[k], b[k]) for k in a)
    if isinstance(a, list):
        return len(a) == len(b) and all(close(x, y) for x, y in zip(a, b))
    if isinstance(a, float) or isinstance(b, float):
        return a is not None and b is not None and abs(a - b) <= 1e-9 * max(abs(a), abs(b), 1e-300) + 1e-15
    return a == b


def main():
    fixture = SUBSET / "🧪️tests/🧫️fixtures/🧮️u-values/🔣️.json"
    doc = vectors(str(fixture))
    doc = json.loads(json.dumps(doc))
    if "--write" in sys.argv:
        VECTORS.parent.mkdir(parents=True, exist_ok=True)
        VECTORS.write_text(json.dumps(doc, indent=2, ensure_ascii=False) + "\n", encoding="utf8")
        return 0
    ok = close(doc, json.loads(VECTORS.read_text(encoding="utf8")))
    print(json.dumps({"ok": ok}))
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
