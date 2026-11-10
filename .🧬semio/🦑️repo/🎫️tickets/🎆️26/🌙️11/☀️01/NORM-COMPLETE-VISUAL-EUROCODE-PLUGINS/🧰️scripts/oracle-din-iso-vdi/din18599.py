"""DIN V 18599-2 independent cross-checks: the utilisation factor at 50 digits (mpmath) with its limits, the monthly balance against the EN ISO 13790 / DIN V 18599-2 method (time-constant dependent a, calendar month hours), and a transcription of the Rust monthly model for the divergence."""
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
            rows.append({"a": a, "gamma": gamma, "etaHeating": eta_heating(gamma, a), "etaHeatingRustFormula": eta_cooling(gamma, a), "etaCooling": eta_cooling(gamma, a)})
    limits = {"etaHeating(gamma->0)": eta_heating(1e-12, 4.0), "etaHeating(gamma=1, a=0.95)": 0.95 / 1.95, "etaHeating(gamma=1e6, a=4) * gamma": eta_heating(1e6, 4.0) * 1e6, "etaHeating(gamma=0.05, a=0.95)": eta_heating(0.05, 0.95), "rustFormulaEquals_gamma_times_etaHeating": [eta_cooling(g, 3.0) / (g * eta_heating(g, 3.0)) for g in (0.2, 0.7, 1.7, 5.0)]}
    return rows, limits


def orientation(az, tilt):
    az %= 360.0
    fs = min(abs(az - 180.0), 360.0 - abs(az - 180.0))
    return (0.4 + 0.6 * (1 - fs / 180.0)) * (0.9 + 0.1 * min(max(tilt, 0.0), 90.0) / 90.0)


ZONE = {"id": "z1", "areaM2": 120.0, "volumeM3": 360.0, "thetaIHeatC": 20.0, "thetaICoolC": 26.0, "internalGainsWM2": 4.0, "outdoorAch": 0.5, "capacityWhM2K": 90.0}
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


def months(model, a_override=None):
    """model 'rust': a = 0.95, 730 h (`balance-model/🦀️.rs` heating_months); 'standard': a = 1 + tau/16 h with tau = C/(H_T+H_V), calendar hours (EN ISO 13790 12.2.1.1 / DIN V 18599-2 eq. 36-38)."""
    h_t, h_v = coefficients()
    z = ZONE
    c = z["capacityWhM2K"] * z["areaM2"]
    tau = c / (h_t + h_v)
    a_std = 1.0 + tau / 16.0
    out = []
    for m in range(12):
        hours = 730.0 if model in ("rust", "rust-fixed-eta") else 24.0 * calendar.monthrange(2023, m + 1)[1]
        a = 0.95 if model in ("rust", "rust-fixed-eta") else a_std
        eta_fn = eta_ls if model == "rust" else eta_gn
        te = THETA_E[m]
        dt = max(z["thetaIHeatC"] - te, 0.0)
        q_t, q_v = h_t * dt * hours / 1000, h_v * dt * hours / 1000
        q_i = z["internalGainsWM2"] * z["areaM2"] * hours / 1000
        q_s = sum(G_H[m] * orientation(e["az"], e["tilt"]) * e["areaM2"] * e["g"] * min(max(e["fc"], 0), 1) * hours / 1000 for e in ELEMENTS if e["g"] > 0)
        loss, gain = q_t + q_v, q_i + q_s
        gamma = 1e9 if loss <= 1e-9 else gain / loss
        eta = eta_fn(gamma, a)
        loss_c = (h_t + h_v) * max(z["thetaICoolC"] - te, 0.0) * hours / 1000
        over = (h_t + h_v) * max(te - z["thetaICoolC"], 0.0) * hours / 1000
        gamma_c = 1e9 if loss_c <= 1e-9 else gain / loss_c
        q_c = max(gain - eta_ls(gamma_c, a) * loss_c, 0.0) + over
        out.append({"month": m + 1, "hours": hours, "qT": q_t, "qV": q_v, "qI": q_i, "qS": q_s, "gamma": gamma, "eta": eta, "qH": max(loss - eta * gain, 0.0), "qC": q_c})
    return out, {"hT": h_t, "hV": h_v, "tauH": tau, "a": a_std if model == "standard" else 0.95}


def vectors():
    rows, limits = eta_vectors()
    rust, rust_meta = months("rust")
    fixed, _ = months("rust-fixed-eta")
    std, std_meta = months("standard")
    sums = lambda ms, k: sum(r[k] for r in ms)
    no_gain = {"label": "zero gains: Q_H = Q_T + Q_V exactly (agree)", "qH": [r["qT"] + r["qV"] for r in rust]}
    return {
        "oracle": "mpmath 50 digits (eta), numpy-free closed forms, EN ISO 13790 monthly method with a = a0 + tau/tau0",
        "utilisationFactor": rows, "utilisationLimits": limits,
        "zone": ZONE, "elements": ELEMENTS, "ventilation": VENT, "deltaUWbWM2k": 0.05, "climate": {"thetaEC": THETA_E, "gHWM2": G_H},
        "coefficients": rust_meta, "monthsRustModel": rust, "monthsRustModelWithStandardEta": fixed, "monthsStandard": std, "standardMeta": std_meta,
        "annual": {"rustQH": sums(rust, "qH"), "rustWithStandardEtaQH": sums(fixed, "qH"), "standardQH": sums(std, "qH"), "rustQC": sums(rust, "qC"), "standardQC": sums(std, "qC"), "rustQT": sums(rust, "qT"), "standardQT": sums(std, "qT"), "rustHours": sums(rust, "hours"), "standardHours": sums(std, "hours")},
        "zeroGain": no_gain,
    }


def divergences(doc):
    a = doc["annual"]
    return [
        {"id": "din18599.eta-heating-formula", "clause": "DIN V 18599-2 / EN ISO 13790 12.2.1.1 gain utilisation factor", "inputs": "gamma 0.3, a 0.95", "rust": eta_cooling(0.3, 0.95), "reference": eta_heating(0.3, 0.95), "relative": eta_cooling(0.3, 0.95) / eta_heating(0.3, 0.95) - 1, "note": "Rust utilization_factor uses (1 - g^-a)/(1 - g^-(a+1)) (the cooling loss form) for the heating gains; it equals gamma times the standard eta for every gamma"},
        {"id": "din18599.annual-qh-eta-only", "clause": "DIN V 18599-2 heating demand", "inputs": "reference zone of the vectors, Rust a = 0.95 and 730 h", "rust": a["rustQH"], "reference": a["rustWithStandardEtaQH"], "relative": a["rustQH"] / a["rustWithStandardEtaQH"] - 1, "note": "effect of the eta formula alone"},
        {"id": "din18599.annual-qh-full", "clause": "DIN V 18599-2 heating demand (a = 1 + tau/16 h, calendar hours)", "inputs": "reference zone, C = 90 Wh/(m2 K)", "rust": a["rustQH"], "reference": a["standardQH"], "relative": a["rustQH"] / a["standardQH"] - 1, "note": "eta formula, constant a = 0.95 (no time constant) and 730 h together"},
        {"id": "din18599.annual-qc-a", "clause": "DIN V 18599-2 cooling demand", "inputs": "reference zone", "rust": a["rustQC"], "reference": a["standardQC"], "relative": a["rustQC"] / a["standardQC"] - 1, "note": "cooling uses the correct eta form but the constant a = 0.95"},
        {"id": "din18599.hours-per-month", "clause": "DIN V 18599-2 monthly balance", "inputs": "annual Q_T", "rust": a["rustQT"], "reference": a["standardQT"], "relative": a["rustQT"] / a["standardQT"] - 1, "note": "730 h every month against 24 h times the calendar days"},
    ]
