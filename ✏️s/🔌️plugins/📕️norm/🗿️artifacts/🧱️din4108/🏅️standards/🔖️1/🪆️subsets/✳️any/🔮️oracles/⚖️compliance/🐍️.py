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
WINTER = (20.0, 0.50, -5.0, 0.80, 2160.0)
SUMMER = (12.0, 0.70, 12.0, 0.70, 2160.0)
SD_FLOOR = 1.0e-3


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


def glaser_planes(layers, r_si, r_se, period):
    """Condensation planes of the Glaser iteration: the interface of largest excess becomes a plane at saturation, repeated until no excess is left."""
    t_i, rh_i, t_e, rh_e, _ = period
    temps = interface_temperatures(layers, r_si, r_se, t_i, t_e)
    t_c = [temps[0]] + [temps[i + 1] for i in range(len(layers))]
    sd = [0.0]
    for L in layers:
        sd.append(sd[-1] + float(L["thicknessM"]) * float(L["mu"]))
    ps = [saturation_pa(t) for t in t_c]
    last = len(sd) - 1
    planes = []

    def pressures():
        nodes = sorted([0, last] + planes)
        fixed = {0: rh_i * saturation_pa(t_i), last: rh_e * saturation_pa(t_e), **{k: ps[k] for k in planes}}
        p = [0.0] * len(sd)
        for a, b in zip(nodes, nodes[1:]):
            span = sd[b] - sd[a]
            for k in range(a, b + 1):
                p[k] = fixed[a] + (fixed[b] - fixed[a]) * ((sd[k] - sd[a]) / span if span > 0 else 0.0)
        return p, nodes, fixed

    while True:
        p, nodes, fixed = pressures()
        excess = [(p[k] - ps[k], k) for k in range(1, last) if k not in planes and p[k] > ps[k]]
        if not excess:
            return sd, ps, planes, nodes, fixed
        planes.append(max(excess)[1])


def glaser_masses(layers, r_si, r_se):
    """DIN 4108-3 Annex A masses in kg/m2: g = (p_i - p_c) / Z with Z = s_d / delta0, over the period in seconds; the condensate is the flux into minus the flux out of each plane."""
    sd, ps, planes, nodes, fixed = glaser_planes(layers, r_si, r_se, WINTER)
    seconds = WINTER[4] * 3600.0
    condensed = 0.0
    for idx, k in enumerate(nodes[1:-1], start=1):
        a, b = nodes[idx - 1], nodes[idx + 1]
        z_in, z_out = max(sd[k] - sd[a], SD_FLOOR) / DELTA_0, max(sd[b] - sd[k], SD_FLOOR) / DELTA_0
        condensed += ((fixed[a] - ps[k]) / z_in + (fixed[b] - ps[k]) / z_out) * seconds
    condensed = max(condensed, 0.0)
    t_i, rh_i, t_e, rh_e, hours = SUMMER
    ts = interface_temperatures(layers, r_si, r_se, t_i, t_e)
    ps_s = [saturation_pa(ts[0])] + [saturation_pa(ts[i + 1]) for i in range(len(layers))]
    p_b = {0: rh_i * saturation_pa(t_i), len(sd) - 1: rh_e * saturation_pa(t_e)}
    value = lambda n: p_b[n] if n in p_b else ps_s[n]
    evaporated = 0.0
    for idx, k in enumerate(nodes[1:-1], start=1):
        a, b = nodes[idx - 1], nodes[idx + 1]
        z_in, z_out = max(sd[k] - sd[a], SD_FLOOR) / DELTA_0, max(sd[b] - sd[k], SD_FLOOR) / DELTA_0
        evaporated += (max(ps_s[k] - value(a), 0.0) / z_in + max(ps_s[k] - value(b), 0.0) / z_out) * hours * 3600.0
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
        gl_id = f"din4108-3.glaser.{eid}"
        dry_id = f"din4108-3.drying.{eid}"
        if gl_id in checks and checks[gl_id].get("status") != "notApplicable":
            want_condensed, want_evaporated = glaser_masses(el["layers"], r_si, r_se)
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
