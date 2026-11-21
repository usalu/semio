#!/usr/bin/env python3
"""Independent DIN EN 16798 oracle for key formulas (±0.5 % vs Rust report)."""
from __future__ import annotations

import json
import sqlite3
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


def design_flow(zone: dict, annex: str, delta_ppm: float):
    cat = zone["comfortCategory"]
    usage = zone["usageType"]
    people = float(zone["occupants"])
    area = float(zone["floorAreaM2"])
    pollution = zone["pollutionClass"]
    method = zone["ventMethod"]
    if annex.lower() == "de" and (usage == "residential" or "method_3" in method):
        return None
    if annex.lower() == "de" and "method_1" in method and pollution == "very_low":
        return None
    if "method_2" in method:
        return people * (20 / 1000) / (delta_ppm / 1_000_000)
    if usage == "residential" and cat != "IV":
        if "method_3" in method:
            return {"I": 10, "II": 7, "III": 4}[cat] * people * 3.6
        person, building = {"I": (3.5, 0.25), "II": (2.5, 0.15), "III": (1.5, 0.1)}[cat]
        return (person * people + building * area) * 3.6
    if usage == "residential":
        return None
    if "method_3" not in method:
        return required_outdoor_air_m3_h(people, area, cat, pollution)
    if usage == "office" and pollution == "low":
        person, building = {"I": (20, 2), "II": (14, 1.4), "III": (8, 0.8), "IV": (5.5, 0.55)}[cat]
        return max(person * people, building * area) * 3.6
    return None


def draught_rate_percent(t_a: float, v: float, tu: float) -> float:
    v = max(v, 0.05)
    dr = (34.0 - t_a) * max(v - 0.05, 0.0) ** 0.62 * (0.37 * v * tu + 3.14)
    return max(0.0, min(100.0, dr))


def hygienic_minimum(zone: dict) -> float:
    return 0.0 if zone.get("usageType") == "residential" else 14.4 * zone["occupants"]


def capacity_served(snap: dict, vent_id: str) -> float:
    return sum(
        max(float(z["outdoorAirSuppliedM3H"]), hygienic_minimum(z))
        for z in (snap.get("zones") or []) if z.get("ventSystemId") == vent_id
    )


def sfp_bound(cls: int) -> float:
    return [300.0, 500.0, 750.0, 1250.0, 2000.0, 3000.0, 4500.0, 6500.0][int(cls)]


def co2_limit_above(cat: str, annex: str) -> float:
    values = (350, 550, 900, 1350) if annex.lower() == "de" else (550, 800, 1350, 1350)
    return dict(zip(("I", "II", "III", "IV"), values))[cat]


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
    return st in {"notApplicable", "NotApplicable", "unverifiedBasis", "UnverifiedBasis"}


def main() -> int:
    if len(sys.argv) == 3 and sys.argv[1] == "--hygienic-fixture":
        fixture = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))
        with sqlite3.connect(":memory:") as db:
            for case in fixture["cases"]:
                floor, required = db.execute("""
                    WITH basis(floor, q) AS (
                        SELECT CASE :usage WHEN 'residential' THEN 0 ELSE 14.4 * :occupants END,
                               CASE :method
                                   WHEN 'method_2_limit_concentration' THEN :occupants * 18000.0 / 1350
                                   WHEN 'method_3_predefined_rates' THEN 9 * :occupants + 1.44 * :areaM2
                                   ELSE CASE :category
                                       WHEN 'II' THEN 25.2 * :occupants + 2.52 * :areaM2
                                       WHEN 'IV' THEN 9 * :occupants + 1.08 * :areaM2
                                   END
                               END
                    )
                    SELECT floor, max(floor, q) FROM basis
                """, case).fetchone()
                assert abs(floor - case["healthMinimumM3H"]) < 1e-9, case["name"]
                assert abs(required - case["designMinimumM3H"]) < 1e-9, case["name"]
                assert (case["supplyM3H"] + 1e-9 >= required) == case["passes"], case["name"]
        print(json.dumps({"ok": True, "cases": len(fixture["cases"])}))
        return 0
    if len(sys.argv) >= 2 and sys.argv[1] == "--report":
        report = json.loads(sys.stdin.read())
        snap = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))
        zones = snap.get("zones") or []
        vents = snap.get("ventSystems") or []
        annex = snap.get("annex", "de")
        tol = 0.005
        checked = 0
        checks = {c["id"]: c for c in report.get("checks", [])}
        for z in zones:
            cat = z.get("comfortCategory", "II")
            method = z.get("ventMethod", "method_1_perceived_air_quality")
            delta = co2_limit_above(cat, annex)
            q_exp = max(design_flow(z, annex, delta) or 0.0, hygienic_minimum(z))
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
