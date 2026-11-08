#!/usr/bin/env python3
"""Independent monthly balance oracle of DIN V 18599-2: numpy re-derivation of the zone heating and cooling balance from the snapshot JSON alone.

Usage: 🐍️.py <snapshot.json>; prints {"zones": [{"id", "qT", "qV", "qI", "qS", "eta", "qH", "qC"}]} with twelve kWh values per key.
"""
import json
import sys

import numpy as np

RHO_CA = 0.34
HOURS = 730.0
N_INF = 0.2
OUTDOOR_ACH = {"WFH": 0.5, "Office": 1.0, "School": 1.5}
ADJACENCY = {"Outdoor": 1.0, "Ground": 0.6, "Unheated": 0.5, "Heated": 0.0}
A = 0.95


def utilisation(gamma):
    gamma = np.asarray(gamma, dtype=float)
    safe = np.where(np.abs(gamma) < 1e-9, 2.0, gamma)
    general = (1.0 - safe ** (-A)) / (1.0 - safe ** (-(A + 1.0)))
    unity = A / (A + 1.0)
    return np.where(np.abs(gamma) < 1e-9, 1.0, np.where(np.abs(gamma - 1.0) < 1e-9, unity, general))


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
        heat = np.maximum(zone["thetaIHeatC"] - theta, 0.0)
        cool = np.maximum(zone["thetaICoolC"] - theta, 0.0)
        over = np.maximum(theta - zone["thetaICoolC"], 0.0)
        q_t = h_t * heat * HOURS / 1000.0
        q_v = h_v * heat * HOURS / 1000.0
        q_i = np.full(12, zone["internalGainsWM2"] * zone["areaM2"] * HOURS / 1000.0)
        q_s = sum((irradiance * orientation(e["orientationDeg"], e["tiltDeg"]) * e["areaM2"] * e["gValue"] * min(max(e["fc"], 0.0), 1.0) * HOURS / 1000.0 for e in elements if e["gValue"] > 0), np.zeros(12))
        loss, gain = q_t + q_v, q_i + q_s
        eta = utilisation(np.where(loss <= 1e-9, 1e9, gain / np.where(loss <= 1e-9, 1.0, loss)))
        loss_c = (h_t + h_v) * cool * HOURS / 1000.0
        eta_c = utilisation(np.where(loss_c <= 1e-9, 1e9, gain / np.where(loss_c <= 1e-9, 1.0, loss_c)))
        q_c = np.maximum(gain - eta_c * loss_c, 0.0) + (h_t + h_v) * over * HOURS / 1000.0
        out.append({"id": zone["id"], "qT": q_t.tolist(), "qV": q_v.tolist(), "qI": q_i.tolist(), "qS": q_s.tolist(), "eta": eta.tolist(), "qH": np.maximum(loss - eta * gain, 0.0).tolist(), "qC": q_c.tolist()})
    return out


if __name__ == "__main__":
    print(json.dumps({"zones": months(json.load(open(sys.argv[1])))}))
