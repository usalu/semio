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
R_SI_MOULD = 0.25
DELTA_0 = 2.0e-10


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
    if t_c >= 0.0:
        return 610.5 * math.exp(17.269 * t_c / (237.3 + t_c))
    return 610.5 * math.exp(21.875 * t_c / (265.5 + t_c))


def interface_temperatures(layers, r_si, r_se, t_int, t_ext):
    import numpy as np

    resistances = [r_si] + [layer_r(L["thicknessM"], L["lambda"]) for L in layers] + [r_se]
    nodes = len(resistances) - 1
    conductance = np.zeros((nodes, nodes))
    source = np.zeros(nodes)
    for index, resistance in enumerate(resistances):
        g = 1.0 / max(resistance, 1e-9)
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


def annex_a_masses(layers):
    temperatures = interface_temperatures(layers, 0.25, 0.04, 20.0, -5.0)
    sd = [0.0]
    for index, layer in enumerate(layers):
        thickness = layer["thicknessM"] * layer["mu"]
        sd.append(sd[-1] + (max(thickness, 0.1) if index + 1 == len(layers) else thickness))
    p = [1168.0, *(saturation_pa(t) for t in temperatures[1:-1]), 321.0]
    hull = []
    for point in range(len(sd)):
        while len(hull) > 1:
            a, b = hull[-2:]
            if (p[b] - p[a]) * (sd[point] - sd[b]) < (p[point] - p[b]) * (sd[b] - sd[a]):
                break
            hull.pop()
        hull.append(point)
    duration = 7776000.0
    wet = []
    stored = 0.0
    for left, point, right in zip(hull, hull[1:], hull[2:]):
        incoming = (p[left] - p[point]) / (sd[point] - sd[left])
        outgoing = (p[point] - p[right]) / (sd[right] - sd[point])
        water = DELTA_0 * (incoming - outgoing) * duration
        if water > 0.0:
            wet.append([sd[point], water])
            stored += water
    capacity = 0.0
    while wet and duration > 0.0:
        inward = 1e-7 / wet[0][0]
        outward = 1e-7 / (sd[-1] - wet[-1][0])
        if len(wet) == 1:
            capacity += duration * (inward + outward)
            break
        stage = min(duration, wet[0][1] / inward, wet[-1][1] / outward)
        capacity += stage * (inward + outward)
        wet[0][1] -= stage * inward
        wet[-1][1] -= stage * outward
        wet = [entry for entry in wet if entry[1] > 1e-14]
        duration -= stage
    return stored, capacity


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


def f_rsi_minimum(t_int, rh_int):
    """DIN 4108-2 6.2: theta_si,min is where the room air reaches 80 % at the surface (bisection on the Magnus curve), rounded to 0.1 K; f = (theta_si,min - theta_e)/(theta_i - theta_e) rounded to 0.01."""
    target = rh_int * saturation_pa(t_int) / 0.8
    lo, hi = 0.0, t_int
    for _ in range(200):
        mid = (lo + hi) / 2
        lo, hi = (mid, hi) if saturation_pa(mid) < target else (lo, mid)
    theta = math.floor((lo + hi) / 2 * 10 + 0.5) / 10
    return math.floor((theta + 5.0) / (t_int + 5.0) * 100 + 0.5) / 100


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
        r_mould = total_resistance(el["layers"], R_SI_MOULD, r_se)
        f_rsi = 1.0 - R_SI_MOULD / r_mould if r_mould > 0 else 0.0
        inhomogeneous = any(L.get("segments") for L in el["layers"])
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
            if computed is not None and not inhomogeneous:
                checked += 1
                if not within(computed, f_rsi, tol):
                    mismatches.append(f"{frsi_id} f_Rsi rust={computed} oracle={f_rsi}")
            limit = qty(checks[frsi_id], "limit")
            f_min = f_rsi_minimum(float(snap.get("tIntC", 20.0)), float(snap.get("rhInt", 0.5)))
            if limit is not None:
                checked += 1
                if not within(limit, f_min, 1e-9):
                    mismatches.append(f"{frsi_id} f_Rsi limit rust={limit} oracle={f_min}")
        gl_id = f"din4108-3.glaser.{eid}"
        dry_id = f"din4108-3.drying.{eid}"
        if gl_id in checks and checks[gl_id].get("status") != "notApplicable":
            want_condensed, want_evaporated = annex_a_masses(el["layers"])
            got = qty(checks[gl_id], "computed")
            checked += 1
            if got is None or not within(got, want_condensed, tol):
                mismatches.append(f"{gl_id} condensed rust={got} oracle={want_condensed}")
            if dry_id in checks and checks[dry_id].get("status") != "notApplicable":
                condensed = qty(checks[dry_id], "computed")
                evaporated = qty(checks[dry_id], "limit")
                checked += 2
                if condensed is None or evaporated is None:
                    mismatches.append(f"{dry_id} masses missing: {condensed} {evaporated}")
                else:
                    if not within(condensed, want_condensed, tol):
                        mismatches.append(f"{dry_id} condensed rust={condensed} oracle={want_condensed}")
                    if not within(evaporated, max(want_evaporated, 1e-9), tol):
                        mismatches.append(f"{dry_id} evaporated rust={evaporated} oracle={want_evaporated}")

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
