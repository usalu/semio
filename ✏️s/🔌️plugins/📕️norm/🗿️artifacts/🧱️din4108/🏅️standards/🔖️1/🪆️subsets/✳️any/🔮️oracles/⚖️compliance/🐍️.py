#!/usr/bin/env python3
"""Independent DIN 4108 envelope oracle (±0.5 % vs Rust evaluate JSON)."""
from __future__ import annotations

import json
import math
import sys
from pathlib import Path

R_SI_WALL = 0.13
R_SI_ROOF = 0.10
R_SI_FLOOR = 0.17
R_SE = 0.04
F_RSI_MIN = 0.70
GLASER_WINTER_T = -5.0
GLASER_WINTER_RH = 0.80
GLASER_SUMMER_T = 12.0
GLASER_SUMMER_ROOF_T = 20.0
GLASER_SUMMER_RH = 0.70


def layer_r(d, lam):
    return d / lam if lam > 0 else float("inf")


def total_resistance(layers, r_si, r_se):
    return r_si + r_se + sum(layer_r(L["thicknessM"], L["lambda"]) for L in layers)


def u_value(r):
    return 1.0 / r if r > 0 else float("inf")


def surface_resistances(kind, adjacent):
    r_si = {"roof": R_SI_ROOF, "floor": R_SI_FLOOR}.get(kind, R_SI_WALL)
    r_se = 0.0 if adjacent in {"ground", "unheated", "otherHeated"} else R_SE
    return r_si, r_se


def orientation_fw(orientation):
    return {"S": 1.0, "SE": 1.0, "SW": 1.0, "E": 0.90, "W": 0.90, "NE": 0.80, "NW": 0.80, "N": 0.70}.get(orientation, 1.0)


def inclination_fi(deg):
    a = max(0.0, min(90.0, float(deg)))
    return 0.70 + 0.30 * (a / 90.0)


def s_vorhanden(zone):
    ag = float(zone["floorAreaM2"])
    if ag <= 0:
        return float("inf")
    total = 0.0
    for w in zone.get("windows") or []:
        total += float(w["areaM2"]) * float(w["gValue"]) * float(w["shadingFc"]) * orientation_fw(w.get("orientation", "S")) * inclination_fi(w.get("inclinationDeg", 90.0))
    return total / ag


def n50_limit(mech):
    return 1.5 if mech else 3.0


def saturation_pa(t_c):
    return 610.5 * math.exp(17.269 * t_c / (237.3 + t_c))


def saturation_magnus(t_c):
    return 611.2 * math.exp(17.67 * t_c / (t_c + 243.5))


def interface_temperatures(layers, r_si, r_se, t_int, t_ext):
    import numpy as np

    resistances = [r_si] + [layer_r(L["thicknessM"], L["lambda"]) for L in layers] + [r_se]
    nodes = len(resistances) - 1
    conductance = np.zeros((nodes, nodes))
    source = np.zeros(nodes)
    for index, resistance in enumerate(resistances):
        g = 1.0 / resistance
        left, right = index - 1, index
        for node in (left, right):
            if 0 <= node < nodes:
                conductance[node, node] += g
        if left >= 0 and right < nodes:
            conductance[left, right] -= g
            conductance[right, left] -= g
        if left < 0:
            source[right] += g * t_int
        if right >= nodes:
            source[left] += g * t_ext
    return [float(t) for t in np.linalg.solve(conductance, source)]


def vapour_profile(layers, r_si, r_se, t_int, rh_int, t_ext, rh_ext, saturation):
    temps = interface_temperatures(layers, r_si, r_se, t_int, t_ext)
    temps = [temps[0]] + temps[1:]
    p_i = rh_int * saturation(t_int)
    p_e = rh_ext * saturation(t_ext)
    sd = [0.0]
    for L in layers:
        sd.append(sd[-1] + float(L["thicknessM"]) * float(L["mu"]))
    total = sd[-1]
    p = [p_i + (p_e - p_i) * s / total if total > 0 else p_i for s in sd]
    return sd, p, [saturation(t) for t in temps[: len(sd)]]


def glaser_masses(layers, r_si, r_se, t_int, rh_int, winter, summer, hours, saturation=saturation_magnus):
    """Winter condensed and summer evaporated mass in kg/m2 by the Glaser method: per interface with p > p_sat the flux
    delta0 * (dp_left / sd_left + dp_right / sd_right), sd bounded below by 0.1 m (condensate plane), delta0 = 2e-10 kg/(m s Pa)."""
    sd, p, p_sat = vapour_profile(layers, r_si, r_se, t_int, rh_int, winter[0], winter[1], saturation)
    total = sd[-1]
    if total <= 0:
        return 0.0, 0.0
    condensed = 0.0
    planes = []
    for i in range(len(sd)):
        if p[i] > p_sat[i]:
            planes.append(i)
            left = max(sd[i], 0.1)
            right = max(total - sd[i], 0.1)
            condensed += (p[i] - p_sat[i]) * (1.0 / left + 1.0 / right) * 2.0e-10 * hours
    sd_s, p_s, sat_s = vapour_profile(layers, r_si, r_se, t_int, rh_int, summer[0], summer[1], saturation)
    evaporated = 0.0
    for i in planes:
        left = max(sd_s[i], 0.1)
        right = max(total - sd_s[i], 0.1)
        flux = max(sat_s[i] - p_s[0], 0.0) / left + max(sat_s[i] - p_s[-1], 0.0) / right
        evaporated += flux * 2.0e-10 * hours
    return condensed, evaporated


def qty(obj, *keys):
    if not isinstance(obj, dict):
        return None
    for k in keys:
        if k in obj:
            v = obj[k]
            if isinstance(v, dict) and "value" in v:
                return float(v["value"])
            if isinstance(v, (int, float)):
                return float(v)
    return None


def within(a, b, tol=0.005):
    if a is None or b is None:
        return False
    if abs(b) < 1e-12:
        return abs(a - b) < 1e-9
    return abs(a - b) / abs(b) <= tol


def compare_report(snap, report):
    checks = {c["id"]: c for c in report.get("checks", [])}
    tol = 0.005
    mismatches = []
    checked = 0

    for el in snap.get("elements") or []:
        eid = el["id"]
        kind = el.get("kind", "wall")
        if kind in {"window", "door"} or not el.get("layers"):
            continue
        r_si, r_se = surface_resistances(kind, el.get("adjacent", "exterior"))
        r = total_resistance(el["layers"], r_si, r_se)
        u = u_value(r)
        f_rsi = 1.0 - r_si / r if r > 0 else 0.0
        table_id = f"din4108-2.table3.{eid}"
        frsi_id = f"din4108-2.frsi.{eid}"
        if table_id in checks and checks[table_id].get("status") != "notApplicable":
            computed = qty(checks[table_id], "computed")
            if computed is not None:
                checked += 1
                if not within(computed, r, tol):
                    mismatches.append(f"{table_id} R rust={computed} oracle={r}")
        if frsi_id in checks and checks[frsi_id].get("status") != "notApplicable":
            computed = qty(checks[frsi_id], "computed")
            if computed is not None:
                checked += 1
                if not within(computed, f_rsi, tol):
                    mismatches.append(f"{frsi_id} f_Rsi rust={computed} oracle={f_rsi}")
        gl_id = f"din4108-3.glaser.{eid}"
        dry_id = f"din4108-3.drying.{eid}"
        if gl_id in checks and checks[gl_id].get("status") != "notApplicable" and dry_id in checks:
            dry = checks[dry_id]
            if dry.get("status") != "notApplicable":
                condensed = qty(dry, "computed")
                evaporated = qty(dry, "limit")
                checked += 2
                if condensed is None or evaporated is None or condensed <= 0 or evaporated <= 0:
                    mismatches.append(f"{dry_id} masses missing: {condensed} {evaporated}")
                elif (condensed <= evaporated) != (dry.get("status") == "pass"):
                    mismatches.append(f"{dry_id} status {dry.get('status')} contradicts {condensed} <= {evaporated}")

        u_id = f"din4108-6.u.{eid}"
        status = (checks.get(u_id) or {}).get("status")
        if u_id in checks and status not in {"notApplicable", "NotApplicable", None}:
            computed = qty(checks[u_id], "computed")
            delta = float(el.get("deltaUG", 0) or 0) + float(el.get("deltaUF", 0) or 0) + float(el.get("deltaUR", 0) or 0)
            if computed is not None and computed > 0.0 and not any(L.get("segments") for L in el["layers"]):
                checked += 1
                if not within(computed, u + delta, tol):
                    mismatches.append(f"{u_id} U rust={computed} oracle={u + delta}")

    for zone in snap.get("zones") or []:
        zid = zone["id"]
        sid = f"din4108-2.summer.{zid}"
        if sid in checks and checks[sid].get("status") != "notApplicable":
            s = s_vorhanden(zone)
            computed = qty(checks[sid], "computed")
            if computed is not None:
                checked += 1
                if not within(computed, s, tol):
                    mismatches.append(f"{sid} S rust={computed} oracle={s}")

    n50_id = "din4108-7.n50"
    if n50_id in checks:
        computed = qty(checks[n50_id], "computed")
        n50 = float(snap.get("airtightnessN50", 0))
        if computed is not None:
            checked += 1
            if not within(computed, n50, tol):
                mismatches.append(f"{n50_id} n50 rust={computed} oracle={n50}")

    return {"ok": len(mismatches) == 0 and checked > 0, "checked": checked, "mismatches": mismatches}


def self_check():
    layers = [{"thicknessM": 0.24, "lambda": 0.81, "mu": 10}, {"thicknessM": 0.14, "lambda": 0.035, "mu": 40}]
    r = total_resistance(layers, R_SI_WALL, R_SE)
    u = u_value(r)
    assert abs(r - 4.466296296) < 1e-6, r
    assert abs(u - 0.223899) < 1e-4, u
    zone = {"floorAreaM2": 80.0, "windows": [
        {"areaM2": 8.0, "gValue": 0.5, "shadingFc": 0.5, "orientation": "S", "inclinationDeg": 90},
        {"areaM2": 4.0, "gValue": 0.5, "shadingFc": 0.7, "orientation": "E", "inclinationDeg": 90},
    ]}
    s = s_vorhanden(zone)
    assert abs(s - 0.04075) < 1e-9, s
    assert n50_limit(True) == 1.5
    print("oracle-ok", {"r": r, "u": u, "s": s})


def main() -> int:
    if len(sys.argv) >= 2 and sys.argv[1] == "--report":
        report = json.loads(sys.stdin.read())
        snap = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))
        body = compare_report(snap, report)
        print(json.dumps(body))
        return 0 if body["ok"] else 1
    self_check()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
