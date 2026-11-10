"""Independent third-party cross-check oracle (scipy / mpmath / numpy / exact fractions; no repo oracle imported). Run `python -I 🐍️.py` to verify the committed vectors, `--write` to regenerate them. Divergences between the Rust source and the standard are listed under `divergences` in the vectors and are not forced to agree."""
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


def psat_magnus_rust(t_c):
    return 611.2 * math.exp(17.67 * t_c / (t_c + 243.5))


def psat_iso7730(t_c):
    return 1000.0 * math.exp(16.6536 - 4030.183 / (t_c + 235.0))


assert abs(psat_iapws(100.0) - 101418.0) < 20.0
assert abs(psat_iapws(0.01) - 611.657) < 0.05


import calendar

import mpmath
import numpy as np

mpmath.mp.dps = 50
THETA_E = [-0.4, 0.6, 4.1, 8.4, 13.4, 16.6, 18.4, 17.9, 14.0, 9.2, 4.4, 1.0]
G_H = [25.0, 50.0, 95.0, 145.0, 185.0, 200.0, 195.0, 170.0, 120.0, 70.0, 35.0, 20.0]
RHO_CA = 0.34


def eta_heating(gamma, a):
    """EN ISO 13790 12.2.1.1 gain utilisation factor: (1 - g^a) / (1 - g^(a+1)), a/(a+1) at g = 1."""
    g, a = mpmath.mpf(gamma), mpmath.mpf(a)
    if abs(g - 1) < mpmath.mpf(10) ** -40:
        return float(a / (a + 1))
    return float((1 - g ** a) / (1 - g ** (a + 1)))


def eta_cooling(gamma, a):
    """EN ISO 13790 12.2.2 loss utilisation factor for cooling, gamma = Q_gain / Q_loss: (1 - g^-a) / (1 - g^-(a+1))."""
    g, a = mpmath.mpf(gamma), mpmath.mpf(a)
    if abs(g - 1) < mpmath.mpf(10) ** -40:
        return float(a / (a + 1))
    return float((1 - g ** (-a)) / (1 - g ** (-(a + 1))))


def eta_vectors():
    rows = []
    for a in (0.95, 1.0, 2.0, 4.0, 7.7):
        for gamma in (0.05, 0.3, 0.8, 1.0, 1.5, 3.0, 20.0):
            rows.append({"a": a, "gamma": gamma, "etaHeating": eta_heating(gamma, a), "etaCooling": eta_cooling(gamma, a)})
    limits = {"etaHeating(gamma->0)": eta_heating(1e-12, 4.0), "etaHeating(gamma=1, a=0.95)": 0.95 / 1.95, "etaHeating(gamma=1e6, a=4) * gamma": eta_heating(1e6, 4.0) * 1e6, "etaHeating(gamma=0.05, a=0.95)": eta_heating(0.05, 0.95), "etaCoolingEqualsGammaTimesEtaHeating": [eta_cooling(g, 3.0) / (g * eta_heating(g, 3.0)) for g in (0.2, 0.7, 1.7, 5.0)]}
    return rows, limits


def orientation(az, tilt):
    az %= 360.0
    fs = min(abs(az - 180.0), 360.0 - abs(az - 180.0))
    return (0.4 + 0.6 * (1 - fs / 180.0)) * (0.9 + 0.1 * min(max(tilt, 0.0), 90.0) / 90.0)


ZONE = {"id": "z1", "areaM2": 120.0, "volumeM3": 360.0, "thetaIHeatC": 20.0, "thetaICoolC": 26.0, "internalGainsWM2": 4.0, "outdoorAch": 0.5, "thermalMassClass": "Medium", "cWirkWhM2k": 90.0}
ELEMENTS = [
    {"id": "wall-s", "areaM2": 40.0, "u": 0.25, "adjacency": 1.0, "az": 180.0, "tilt": 90.0, "g": 0.0, "fc": 1.0},
    {"id": "wall-n", "areaM2": 40.0, "u": 0.25, "adjacency": 1.0, "az": 0.0, "tilt": 90.0, "g": 0.0, "fc": 1.0},
    {"id": "win-s", "areaM2": 18.0, "u": 1.0, "adjacency": 1.0, "az": 180.0, "tilt": 90.0, "g": 0.5, "fc": 0.7},
    {"id": "win-n", "areaM2": 6.0, "u": 1.0, "adjacency": 1.0, "az": 0.0, "tilt": 90.0, "g": 0.5, "fc": 0.9},
    {"id": "roof", "areaM2": 120.0, "u": 0.18, "adjacency": 1.0, "az": 0.0, "tilt": 0.0, "g": 0.0, "fc": 1.0},
    {"id": "floor", "areaM2": 120.0, "u": 0.3, "adjacency": 0.6, "az": 0.0, "tilt": 0.0, "g": 0.0, "fc": 1.0},
]
VENT = {"airflowM3H": 90.0, "heatRecoveryEta": 0.8, "heatedVolumeM3": 450.0}


def coefficients(zone=ZONE, elements=ELEMENTS, vent=VENT, delta_u_wb=0.05):
    a_env = sum(e["areaM2"] for e in elements)
    h_t = sum(e["adjacency"] * e["u"] * e["areaM2"] for e in elements) + delta_u_wb * a_env
    v = vent["airflowM3H"] + zone["outdoorAch"] * zone["volumeM3"]
    h_v = RHO_CA * (v * (1 - vent["heatRecoveryEta"]) + 0.2 * vent["heatedVolumeM3"])
    return h_t, h_v


def eta_ls(gamma, a):
    return 1.0 if abs(gamma) < 1e-9 else eta_cooling(gamma, a)


def eta_gn(gamma, a):
    return 1.0 if abs(gamma) < 1e-9 else eta_heating(gamma, a)


C_WIRK = {"Light": 50.0, "Medium": 90.0, "Heavy": 130.0}


def capacity_wh_m2k(zone):
    return zone["cWirkWhM2k"] if zone["thermalMassClass"] == "Custom" else C_WIRK[zone["thermalMassClass"]]


def months(zone_override=None):
    """EN ISO 13790 12.2 monthly method: a = a0 + tau/tau0 with tau = C/(H_T+H_V) and a0 = 1, tau0 = 16 h (DIN V 18599-2), calendar month hours."""
    h_t, h_v = coefficients()
    z = {**ZONE, **(zone_override or {})}
    tau = capacity_wh_m2k(z) * z["areaM2"] / (h_t + h_v)
    a = 1.0 + tau / 16.0
    out = []
    for m in range(12):
        hours = 24.0 * calendar.monthrange(2023, m + 1)[1]
        te = THETA_E[m]
        dt = max(z["thetaIHeatC"] - te, 0.0)
        q_t, q_v = h_t * dt * hours / 1000, h_v * dt * hours / 1000
        q_i = z["internalGainsWM2"] * z["areaM2"] * hours / 1000
        q_s = sum(G_H[m] * orientation(e["az"], e["tilt"]) * e["areaM2"] * e["g"] * min(max(e["fc"], 0), 1) * hours / 1000 for e in ELEMENTS if e["g"] > 0)
        loss, gain = q_t + q_v, q_i + q_s
        gamma = 1e9 if loss <= 1e-9 else gain / loss
        eta = eta_gn(gamma, a)
        loss_c = (h_t + h_v) * max(z["thetaICoolC"] - te, 0.0) * hours / 1000
        over = (h_t + h_v) * max(te - z["thetaICoolC"], 0.0) * hours / 1000
        gamma_c = 1e9 if loss_c <= 1e-9 else gain / loss_c
        q_c = max(gain - eta_ls(gamma_c, a) * loss_c, 0.0) + over
        out.append({"month": m + 1, "hours": hours, "qT": q_t, "qV": q_v, "qI": q_i, "qS": q_s, "gamma": gamma, "eta": eta, "qH": max(loss - eta * gain, 0.0), "qC": q_c})
    return out, {"hT": h_t, "hV": h_v, "tauH": tau, "a": a}


def vectors():
    rows, limits = eta_vectors()
    std, std_meta = months()
    heavy, heavy_meta = months({"thermalMassClass": "Heavy"})
    custom, custom_meta = months({"thermalMassClass": "Custom", "cWirkWhM2k": 60.0})
    capacities = [{"thermalMassClass": k, "cWirkWhM2k": c, "areaM2": 120.0, "capacityWhK": 120.0 * capacity_wh_m2k({"thermalMassClass": k, "cWirkWhM2k": c})} for k, c in (("Light", 33.0), ("Medium", 33.0), ("Heavy", 33.0), ("Custom", 33.0))]
    return {
        "oracle": "mpmath 50 digits (eta), closed forms, EN ISO 13790 monthly method with a = a0 + tau/tau0",
        "utilisationFactor": rows, "utilisationLimits": limits,
        "zone": ZONE, "elements": ELEMENTS, "ventilation": VENT, "deltaUWbWM2k": 0.05, "climate": {"thetaEC": THETA_E, "gHWM2": G_H},
        "monthsStandard": std, "standardMeta": std_meta, "monthsHeavy": heavy, "heavyMeta": heavy_meta, "monthsCustom": custom, "customMeta": custom_meta, "heatCapacity": capacities,
        "annual": {"standardQH": sum(r["qH"] for r in std), "standardQC": sum(r["qC"] for r in std), "standardQT": sum(r["qT"] for r in std), "standardHours": sum(r["hours"] for r in std)},
        "zeroGain": {"label": "zero gains: Q_H = Q_T + Q_V exactly", "qH": [r["qT"] + r["qV"] for r in std]},
    }


def divergences(doc):
    return []


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
    doc = vectors()
    doc["divergences"] = divergences(doc)
    doc = json.loads(json.dumps(doc))
    if "--write" in sys.argv:
        VECTORS.parent.mkdir(parents=True, exist_ok=True)
        VECTORS.write_text(json.dumps(doc, indent=2, ensure_ascii=False) + "\n", encoding="utf8")
        return 0
    ok = close(doc, json.loads(VECTORS.read_text(encoding="utf8")))
    print(json.dumps({"ok": ok, "divergences": len(doc["divergences"])}))
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
