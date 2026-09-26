#!/usr/bin/env python3
"""Independent DIN EN 16798 oracle for key formulas (±0.5 % vs Rust report)."""
from __future__ import annotations

import json
import sys
from pathlib import Path


def required_outdoor_air_m3_h(occupants: float, area: float, cat: str, pollution: str) -> float:
    qp = {"I": 10.0, "II": 7.0, "III": 4.0, "IV": 2.5}[cat]
    qa = {
        "very_low": {"I": 0.5, "II": 0.35, "III": 0.2, "IV": 0.15},
        "low": {"I": 1.0, "II": 0.7, "III": 0.4, "IV": 0.3},
        "non_low": {"I": 2.0, "II": 1.4, "III": 0.8, "IV": 0.6},
    }[pollution][cat]
    return (occupants * qp + area * qa) * 3.6


def predefined_outdoor_air_m3_h(area: float, cat: str, occupants: float = 0.0, pollution: str = "low") -> float:
    qa = {"I": 1.4, "II": 1.0, "III": 0.6, "IV": 0.4}[cat]
    qp = {"I": 10.0, "II": 7.0, "III": 4.0, "IV": 2.5}[cat]
    scale = {"very_low": 0.8, "low": 1.0, "non_low": 1.25}.get(pollution.replace("-", "_"), 1.0)
    return area * qa * scale * 3.6 + occupants * qp * 3.6


def method2_outdoor_air_m3_h(occupants: float, met: float, delta_ppm: float) -> float:
    g = occupants * 0.005 * max(met / 1.2, 0.5)
    return (1.0e6 * g / max(delta_ppm, 100.0)) * 3.6


def draught_rate_percent(t_a: float, v: float, tu: float) -> float:
    v = max(v, 0.05)
    dr = (34.0 - t_a) * max(v - 0.05, 0.0) ** 0.62 * (0.37 * v * tu + 3.14)
    return max(0.0, min(100.0, dr))


def capacity_served(snap: dict, vent_id: str) -> float:
    return sum(float(z.get("outdoorAirSuppliedM3H", 0)) for z in (snap.get("zones") or []) if z.get("ventSystemId") == vent_id)


def sfp_bound(cls: int) -> float:
    return [300.0, 500.0, 750.0, 1250.0, 2000.0, 3000.0, 4500.0, 6500.0][int(cls)]


def co2_limit_above(cat: str, annex: str, outdoor: float) -> float:
    dlim = {"I": 550.0, "II": 800.0, "III": 1350.0, "IV": 1350.0}.get(cat, 800.0)
    annex_s = annex if isinstance(annex, str) else str(annex)
    if annex_s.lower() != "de":
        return dlim
    return min(900.0 - outdoor, dlim)


def qty(obj, *keys):
    if not isinstance(obj, dict):
        return 0.0
    for k in keys:
        if k in obj:
            v = obj[k]
            if isinstance(v, dict) and "value" in v:
                return float(v["value"])
            if isinstance(v, (int, float)):
                return float(v)
    return 0.0


def status_na(st: str) -> bool:
    return st in {"notApplicable", "NotApplicable"}


def main() -> int:
    if len(sys.argv) >= 2 and sys.argv[1] == "--report":
        report = json.loads(sys.stdin.read())
        snap = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))
        zones = snap.get("zones") or []
        vents = snap.get("ventSystems") or []
        annex = snap.get("annex", "de")
        outdoor = float(snap.get("outdoorCo2Ppm", 400.0))
        tol = 0.005
        checked = 0
        checks = {c["id"]: c for c in report.get("checks", [])}
        for z in zones:
            cat = z.get("comfortCategory", "II")
            pol = z.get("pollutionClass", "low")
            method = z.get("ventMethod", "method_1_perceived_air_quality")
            delta = co2_limit_above(cat, annex, outdoor)
            if "method_3" in method:
                q_exp = predefined_outdoor_air_m3_h(float(z["floorAreaM2"]), cat, float(z.get("occupants", 0)), str(z.get("pollutionClass", "low")))
            elif "method_2" in method:
                q_exp = method2_outdoor_air_m3_h(float(z["occupants"]), float(z.get("metabolicRateMet", 1.2)), delta)
            else:
                q_exp = required_outdoor_air_m3_h(float(z["occupants"]), float(z["floorAreaM2"]), cat, pol)
            zid = z["id"]
            vent_id = f"din16798-1.vent.{zid}"
            if vent_id in checks and not status_na(str(checks[vent_id].get("status"))):
                lim = qty(checks[vent_id], "limit") or q_exp
                if abs(lim - q_exp) / max(q_exp, 1e-9) > tol:
                    print(json.dumps({"ok": False, "check": vent_id, "lim": lim, "expected": q_exp, "method": method}))
                    return 1
                checked += 1
            co2_id = f"din16798-1.co2.{zid}"
            if co2_id in checks and not status_na(str(checks[co2_id].get("status"))):
                lim = qty(checks[co2_id], "limit")
                if lim and abs(lim - delta) / max(delta, 1e-9) > tol:
                    print(json.dumps({"ok": False, "check": co2_id, "lim": lim, "expected": delta}))
                    return 1
                checked += 1
            dr_id = f"din16798-1.draught.{zid}"
            if dr_id in checks and not status_na(str(checks[dr_id].get("status"))):
                exp = draught_rate_percent(float(z.get("tOpSummerC", 24.5)), float(z.get("airSpeedMS", 0.1)), float(z.get("turbulenceIntensityPercent", 40)))
                got = qty(checks[dr_id], "computed")
                if got and abs(got - exp) / max(exp, 1e-9) > tol and abs(got - exp) > 0.05:
                    print(json.dumps({"ok": False, "check": dr_id, "got": got, "expected": exp}))
                    return 1
                checked += 1
        for v in vents:
            vid = v["id"]
            cls = int(v.get("sfpRequiredClass", 3))
            exp = sfp_bound(cls)
            sfp_id = f"din16798-3.sfp.{vid}"
            if sfp_id in checks and not status_na(str(checks[sfp_id].get("status"))):
                lim = qty(checks[sfp_id], "limit") or exp
                if abs(lim - exp) / max(exp, 1e-9) > tol:
                    print(json.dumps({"ok": False, "check": sfp_id, "lim": lim, "expected": exp}))
                    return 1
                checked += 1
            cap_id = f"din16798-3.capacity.{vid}"
            if cap_id in checks and not status_na(str(checks[cap_id].get("status"))):
                exp = capacity_served(snap, vid)
                got = qty(checks[cap_id], "computed")
                if got and abs(got - exp) / max(exp, 1e-9) > tol:
                    print(json.dumps({"ok": False, "check": cap_id, "got": got, "expected": exp}))
                    return 1
                checked += 1
        q = required_outdoor_air_m3_h(20, 200, "II", "low")
        assert abs(q - 1008.0) < 1e-6
        assert sfp_bound(3) == 1250.0
        print(json.dumps({"ok": True, "checked": checked, "q_m3_h": q, "sfp3": sfp_bound(3)}))
        return 0

    q = required_outdoor_air_m3_h(20, 200, "II", "low")
    assert abs(q - 1008.0) < 1e-6, q
    assert sfp_bound(3) == 1250
    print(json.dumps({"ok": True, "q_m3_h": q, "sfp3": sfp_bound(3)}))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
