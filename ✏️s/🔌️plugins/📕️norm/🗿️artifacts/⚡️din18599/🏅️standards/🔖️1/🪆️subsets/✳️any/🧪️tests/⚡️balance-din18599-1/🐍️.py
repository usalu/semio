#!/usr/bin/env python3
"""Independent DIN V 18599 / GEG oracle — zone-aggregated balance vs Rust evaluate() JSON."""
from __future__ import annotations

import json
import sys
from pathlib import Path

RHO_CA = 0.34
HOURS = 730.0
N_INF = 0.2
LIGHT_HOURS = {"WFH": 1700.0, "Office": 2500.0, "School": 1800.0}
OUTDOOR_ACH = {"WFH": 0.5, "Office": 1.0, "School": 1.5}
FAN_HOURS = {"WFH": 5000.0, "Office": 2500.0, "School": 2000.0, "Residential": 5000.0}


def fp(carrier: str) -> float:
    return {
        "natural_gas": 1.1,
        "heating_oil": 1.1,
        "electricity": 1.8,
        "district_heating": 0.7,
        "biomass": 0.2,
    }.get(carrier, 1.1)


def adjacency(a: str) -> float:
    return {"Outdoor": 1.0, "Ground": 0.6, "Unheated": 0.5, "Heated": 0.0}[a]


def util(gamma: float) -> float:
    if abs(gamma) < 1e-9:
        return 1.0
    if abs(gamma - 1.0) < 1e-9:
        return 0.95 / 1.95
    return (1.0 - gamma ** (-0.95)) / (1.0 - gamma ** (-1.95))


def orient_factor(az: float, tilt: float) -> float:
    az = az % 360.0
    from_south = min(abs(az - 180.0), 360.0 - abs(az - 180.0))
    horiz = 0.4 + (1.0 - 0.4) * (1.0 - from_south / 180.0)
    t = max(0.0, min(90.0, tilt)) / 90.0
    tilt_f = 0.9 + (1.0 - 0.9) * t
    return horiz * tilt_f


def reference_u(kind: str) -> float:
    return {"Wall": 0.28, "Roof": 0.20, "Floor": 0.35, "Window": 1.3, "Door": 1.8}.get(kind, 0.28)


def ht_limit(attachment: str, a_n: float) -> float:
    small = a_n <= 350.0
    if attachment == "Detached":
        return 0.40 if small else 0.50
    if attachment in ("SemiDetached", "EndTerrace"):
        return 0.45 if small else 0.50
    return 0.50


def cooling_installed(cool: dict) -> bool:
    if "plant" in cool:
        return cool["plant"] is not None
    if "kind" in cool:
        return cool["kind"] == "system"
    return bool(cool.get("present"))


def cooling_eer(cool: dict) -> float:
    plant = cool.get("plant")
    if isinstance(plant, dict):
        return float(plant.get("eer", 1.0))
    return float(cool.get("eer", 1.0))


def cooling_carrier(cool: dict) -> str:
    plant = cool.get("plant")
    if isinstance(plant, dict):
        return plant.get("energyCarrier", "electricity")
    return cool.get("energyCarrier", "electricity")


def zone_balances(doc: dict, elems: list, delta: float, climate: dict) -> list:
    theta = climate["thetaEC"]
    gh = climate["gHWM2"]
    a_env = sum(e["areaM2"] for e in elems) or 1e-9
    v_net = sum(z["volumeM3"] for z in doc["zones"]) or 1e-9
    v_e = doc["heatedVolumeM3"]
    eta_hrv = max(0.0, min(1.0, doc["ventilation"]["heatRecoveryEta"]))
    control = doc["lighting"]["controlFactor"]
    out = []
    for zone in doc["zones"]:
        z_elems = [e for e in elems if e["zoneId"] == zone["id"]]
        share = zone["volumeM3"] / v_net
        h_t = sum(adjacency(e["adjacency"]) * e["uValueWM2k"] * e["areaM2"] for e in z_elems) + delta * a_env * share
        profile = zone["usageProfile"]
        v_mech = doc["ventilation"]["airflowM3H"] * share + OUTDOOR_ACH.get(profile, 0.5) * zone["volumeM3"]
        v_inf = N_INF * v_e * share
        h_v = RHO_CA * (v_mech * (1.0 - eta_hrv) + v_inf)
        theta_i = zone["thetaIHeatC"]
        theta_c = zone["thetaICoolC"]
        qi_w = zone["internalGainsWM2"]
        q_h = 0.0
        q_c = 0.0
        for m in range(12):
            te = theta[m]
            qt = h_t * max(theta_i - te, 0.0) * HOURS / 1000.0
            qv = h_v * max(theta_i - te, 0.0) * HOURS / 1000.0
            qi = qi_w * zone["areaM2"] * HOURS / 1000.0
            qs = 0.0
            for e in z_elems:
                if e["gValue"] > 0:
                    qs += gh[m] * orient_factor(e["orientationDeg"], e["tiltDeg"]) * e["areaM2"] * e["gValue"] * max(0.0, min(1.0, e["fc"])) * HOURS / 1000.0
            loss = qt + qv
            gain = qi + qs
            gamma = 1e9 if loss <= 1e-9 else gain / loss
            eta = util(gamma)
            q_h += max(loss - eta * gain, 0.0)
            loss_c = (h_t + h_v) * max(theta_c - te, 0.0) * HOURS / 1000.0
            qt_c = h_t * max(te - theta_c, 0.0) * HOURS / 1000.0
            qv_c = h_v * max(te - theta_c, 0.0) * HOURS / 1000.0
            gamma_c = 1e9 if loss_c <= 1e-9 else (qi + qs) / loss_c
            eta_c = util(gamma_c)
            q_c += max((qi + qs) - eta_c * loss_c, 0.0) + qt_c + qv_c
        q_l = zone["lightingPowerWM2"] * zone["areaM2"] * LIGHT_HOURS.get(profile, 1700.0) * control / 1000.0
        out.append({"hT": h_t, "hV": h_v, "qH": q_h, "qC": q_c, "qL": q_l, "vol": zone["volumeM3"]})
    return out


def balance(doc: dict, climate: dict, *, elements=None, delta_u=None, heat_eff=None, carrier=None, pv_area=None) -> dict:
    elems = elements if elements is not None else doc["elements"]
    delta = doc["deltaUWbWM2k"] if delta_u is None else delta_u
    heat = doc["heating"]
    eff = heat_eff if heat_eff is not None else (
        heat["generationEfficiency"] * heat["distributionEfficiency"] * heat["storageEfficiency"] * heat["transferEfficiency"]
    )
    carrier = heat["energyCarrier"] if carrier is None else carrier
    pv_area = doc["renewables"]["pvAreaM2"] if pv_area is None else pv_area
    zones = zone_balances(doc, elems, delta, climate)
    h_t = sum(z["hT"] for z in zones)
    h_v = sum(z["hV"] for z in zones)
    q_h = sum(z["qH"] for z in zones)
    q_c = sum(z["qC"] for z in zones)
    q_l = sum(z["qL"] for z in zones)
    auto = {"A": 0.88, "B": 0.93, "C": 0.97, "D": 1.0}[doc["automationClass"]]
    occupants = sum(z["occupants"] for z in doc["zones"])
    dhw = doc["dhw"]
    q_w = occupants * dhw["specificDemandKwhPersonA"] + dhw["storageLossKwhA"] + dhw["distributionLossKwhA"]
    q_f_heat = q_h / max(eff, 0.05) * auto
    q_f_dhw = q_w / max(eff, 0.5) * auto
    cool = doc["cooling"]
    q_f_cool = (q_c / max(cooling_eer(cool), 0.5) * auto) if cooling_installed(cool) else 0.0
    q_f_light = q_l * auto
    v_net = sum(z["volumeM3"] for z in doc["zones"]) or 1e-9
    fan_h = sum(FAN_HOURS.get(z["usageProfile"], 5000.0) * z["volumeM3"] / v_net for z in doc["zones"])
    q_f_aux = doc["ventilation"]["fanPowerW"] * fan_h / 1000.0
    ren = doc["renewables"]
    gh = climate["gHWM2"]
    annual_g = sum(gh) / 12.0 * 8760.0 / 1000.0
    q_pv = pv_area * ren["pvEfficiency"] * annual_g + ren["solarThermalKwhA"]
    q_p = (
        q_f_heat * fp(carrier)
        + q_f_dhw * fp(dhw["energyCarrier"])
        + q_f_cool * fp(cooling_carrier(cool))
        + (q_f_light + q_f_aux) * fp("electricity")
        - q_pv * fp("electricity")
    )
    a_env = sum(e["areaM2"] for e in elems) or 1e-9
    return {
        "hT": h_t,
        "hV": h_v,
        "hTPrime": h_t / a_env,
        "qH": q_h,
        "qP": max(q_p, 0.0),
    }


def within(a: float, b: float, tol: float = 0.005) -> bool:
    if abs(b) < 1e-9:
        return abs(a - b) < 1e-6
    return abs(a - b) / abs(b) <= tol


def compare_to_report(doc: dict, climate: dict, report: dict) -> dict:
    actual = balance(doc, climate)
    ref_elems = []
    for e in doc["elements"]:
        r = dict(e)
        r["uValueWM2k"] = reference_u(e["kind"])
        if e["kind"] == "Window":
            r["gValue"] = 0.60
            r["fc"] = 1.0
        ref_elems.append(r)
    reference = balance(doc, climate, elements=ref_elems, delta_u=0.05, heat_eff=0.95 * 0.95 * 0.98 * 0.95, carrier="natural_gas", pv_area=0.0)
    q_p_ref = reference["qP"]
    q_p_limit = doc["gegQpFactor"] * q_p_ref
    checks = {c["id"]: c for c in report.get("checks", [])}
    mismatches = []
    ht_c = checks.get("din18599.geg.ht-prime")
    if ht_c and "computed" in ht_c:
        if not within(actual["hTPrime"], ht_c["computed"]["value"]):
            mismatches.append(("hTPrime", actual["hTPrime"], ht_c["computed"]["value"]))
    qp_c = checks.get("din18599.geg.qp")
    if qp_c and "computed" in qp_c:
        if not within(actual["qP"], qp_c["computed"]["value"]):
            mismatches.append(("qP", actual["qP"], qp_c["computed"]["value"]))
        if "limit" in qp_c and not within(q_p_limit, qp_c["limit"]["value"]):
            mismatches.append(("qPLimit", q_p_limit, qp_c["limit"]["value"]))
    return {"ok": len(mismatches) == 0, "mismatches": mismatches, "actual": actual, "qPRef": q_p_ref, "qPLimit": q_p_limit}


def main(argv: list[str]) -> int:
    if len(argv) == 1:
        assert abs(0.34 * 120.0 - 40.8) < 1e-9
        assert ht_limit("Detached", 140.0) == 0.40
        print("oracle_self_checks_ok")
        return 0
    snap_path = Path(argv[1])
    report_path = Path(argv[2])
    climate_path = Path(argv[3]) if len(argv) > 3 else None
    doc = json.loads(snap_path.read_text())
    report = json.loads(report_path.read_text())
    if climate_path is not None:
        climate = json.loads(climate_path.read_text())
    else:
        climate = doc.get("_climate")
        if climate is None:
            raise SystemExit("climate required")
    result = compare_to_report(doc, climate, report)
    print(json.dumps(result))
    return 0 if result["ok"] else 1


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
