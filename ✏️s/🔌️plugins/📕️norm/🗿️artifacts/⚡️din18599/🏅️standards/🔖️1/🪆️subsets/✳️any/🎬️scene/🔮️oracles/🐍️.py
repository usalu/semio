#!/usr/bin/env python3
"""Independent monthly balance oracle of DIN V 18599-2: numpy re-derivation of the zone heating and cooling balance from the snapshot JSON alone.

Usage: 🐍️.py <snapshot.json>; prints {"zones": [{"id", "qT", "qV", "qI", "qS", "eta", "qH", "qC"}]} with twelve kWh values per key.
"""
import calendar
import json
import sys

import mpmath
import numpy as np

mpmath.mp.dps = 50

RHO_CA = 0.34
DAY_HOURS = np.array([24.0 * calendar.monthrange(2023, m)[1] for m in range(1, 13)])
N_INF = 0.2
OUTDOOR_ACH = {"WFH": 0.5, "Office": 1.0, "School": 1.5}
ADJACENCY = {"Outdoor": 1.0, "Ground": 0.6, "Unheated": 0.5, "Heated": 0.0}
C_WIRK = {"Light": 50.0, "Medium": 90.0, "Heavy": 130.0}
TAU0 = 16.0


def eta_gain(gamma, a):
    """EN ISO 13790 12.2.1.1 heating: (1 - g^a) / (1 - g^(a+1)), a / (a + 1) at g = 1, 1 at g = 0."""
    def one(g):
        g = mpmath.mpf(float(g))
        if abs(g) < mpmath.mpf(10) ** -9:
            return 1.0
        if abs(g - 1) < mpmath.mpf(10) ** -9:
            return float(mpmath.mpf(a) / (mpmath.mpf(a) + 1))
        return float((1 - g ** a) / (1 - g ** (mpmath.mpf(a) + 1)))
    return np.array([one(g) for g in np.atleast_1d(gamma)])


def eta_loss(gamma, a):
    """EN ISO 13790 12.2.2 cooling: (1 - g^-a) / (1 - g^-(a+1)), a / (a + 1) at g = 1, 1 at g = 0."""
    def one(g):
        g = mpmath.mpf(float(g))
        if abs(g) < mpmath.mpf(10) ** -9:
            return 1.0
        if abs(g - 1) < mpmath.mpf(10) ** -9:
            return float(mpmath.mpf(a) / (mpmath.mpf(a) + 1))
        return float((1 - g ** -a) / (1 - g ** -(mpmath.mpf(a) + 1)))
    return np.array([one(g) for g in np.atleast_1d(gamma)])


def orientation(azimuth, tilt):
    azimuth = azimuth % 360.0
    from_south = min(abs(azimuth - 180.0), 360.0 - abs(azimuth - 180.0))
    horizontal = 0.4 + (1.0 - 0.4) * (1.0 - from_south / 180.0)
    steep = min(max(tilt, 0.0), 90.0) / 90.0
    return horizontal * (0.9 + (1.0 - 0.9) * steep)


def months(doc):
    theta = np.array(doc["climate"]["thetaEC"], dtype=float)
    irradiance = np.array(doc["climate"]["gHWM2"], dtype=float)
    envelope = sum(e["areaM2"] for e in doc["elements"]) or 1e-9
    total_volume = sum(z["volumeM3"] for z in doc["zones"]) or 1e-9
    eta_recovery = min(max(doc["ventilation"]["heatRecoveryEta"], 0.0), 1.0)
    out = []
    for zone in doc["zones"]:
        elements = [e for e in doc["elements"] if e["zoneId"] == zone["id"]]
        share = zone["volumeM3"] / total_volume
        h_t = sum(ADJACENCY[e["adjacency"]] * e["uValueWM2k"] * e["areaM2"] for e in elements) + doc["deltaUWbWM2k"] * envelope * share
        mechanical = doc["ventilation"]["airflowM3H"] * share + OUTDOOR_ACH[zone["usageProfile"]] * zone["volumeM3"]
        h_v = RHO_CA * (mechanical * (1.0 - eta_recovery) + N_INF * doc["heatedVolumeM3"] * share)
        capacity = zone["areaM2"] * (zone["cWirkWhM2k"] if zone["thermalMassClass"] == "Custom" else C_WIRK[zone["thermalMassClass"]])
        a = 1.0 + capacity / (h_t + h_v) / TAU0
        heat = np.maximum(zone["thetaIHeatC"] - theta, 0.0)
        cool = np.maximum(zone["thetaICoolC"] - theta, 0.0)
        over = np.maximum(theta - zone["thetaICoolC"], 0.0)
        q_t = h_t * heat * DAY_HOURS / 1000.0
        q_v = h_v * heat * DAY_HOURS / 1000.0
        q_i = zone["internalGainsWM2"] * zone["areaM2"] * DAY_HOURS / 1000.0
        q_s = sum((irradiance * orientation(e["orientationDeg"], e["tiltDeg"]) * e["areaM2"] * e["gValue"] * min(max(e["fc"], 0.0), 1.0) * DAY_HOURS / 1000.0 for e in elements if e["gValue"] > 0), np.zeros(12))
        loss, gain = q_t + q_v, q_i + q_s
        eta = eta_gain(np.where(loss <= 1e-9, 1e9, gain / np.where(loss <= 1e-9, 1.0, loss)), a)
        loss_c = (h_t + h_v) * cool * DAY_HOURS / 1000.0
        eta_c = eta_loss(np.where(loss_c <= 1e-9, 1e9, gain / np.where(loss_c <= 1e-9, 1.0, loss_c)), a)
        q_c = np.maximum(gain - eta_c * loss_c, 0.0) + (h_t + h_v) * over * DAY_HOURS / 1000.0
        out.append({"id": zone["id"], "qT": q_t.tolist(), "qV": q_v.tolist(), "qI": q_i.tolist(), "qS": q_s.tolist(), "eta": eta.tolist(), "qH": np.maximum(loss - eta * gain, 0.0).tolist(), "qC": q_c.tolist()})
    return out


if __name__ == "__main__":
    print(json.dumps({"zones": months(json.load(open(sys.argv[1])))}))
