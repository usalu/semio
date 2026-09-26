#!/usr/bin/env python3
"""Independent VDI 3805 Part 1 + Blatt 2/3/5/6 (+ representative generic) oracle."""

from __future__ import annotations

import json
import sys
from pathlib import Path

VALVE_DN_SERIES = [10, 15, 20, 25, 32, 40, 50, 65, 80, 100, 125, 150, 200, 250, 300]
RADIATOR_N_MIN = 1.1
RADIATOR_N_MAX = 1.5

SHEET_MANDATORY = {
    4: ["dn", "pressure_class", "connection_type", "outer_diameter_m", "wall_thickness_m"],
    7: ["dn", "pressure_class", "connection_type", "volume_m3"],
    8: ["product_group", "type_code", "dn", "pressure_class"],
    16: ["product_group", "type_code", "airflow_m3_s", "pressure_drop_pa"],
    19: ["product_group", "type_code", "airflow_m3_s", "filter_class"],
    53: ["product_group", "type_code", "dn", "nominal_heat_output_w", "cop"],
    60: ["dn", "pressure_class", "connection_type", "axial_force_n"],
}


def valve_min_kvs_m3_h(dn: int) -> float:
    table = {
        10: 0.16,
        15: 0.25,
        20: 0.40,
        25: 0.63,
        32: 1.00,
        40: 1.60,
        50: 2.50,
        65: 4.00,
        80: 6.30,
        100: 10.0,
        125: 16.0,
        150: 25.0,
        200: 40.0,
        250: 63.0,
        300: 100.0,
    }
    return table.get(dn, 0.63)


def count_records(catalog: dict) -> int:
    return sum(len(p.get("records") or []) for p in catalog.get("products") or [])


def record_kv(product: dict) -> dict[str, str]:
    kv: dict[str, str] = {}
    for record in product.get("records") or []:
        family = str(record.get("family") or "")
        fields = list(record.get("fields") or [])
        if family == "210":
            it = iter(fields[1:])
            for k, v in zip(it, it):
                try:
                    float(k)
                except ValueError:
                    kv[str(k).lower()] = str(v)
        elif family == "100" and len(fields) > 2:
            kv.setdefault("product_group", str(fields[2]))
    return kv


def oracle_checks(doc: dict) -> list[dict]:
    checks = []
    catalog = doc["catalog"]
    file_ = catalog["file"]
    actual = count_records(catalog)
    record_count = file_.get("recordCount", file_.get("record_count"))
    structure_ok = (
        bool(file_.get("manufacturer"))
        and bool(file_.get("charset"))
        and bool(catalog.get("products"))
        and record_count == actual
    )
    checks.append({"id": "structure", "pass": structure_ok, "actual": actual, "declared": record_count})

    for product in catalog.get("products") or []:
        attrs = (product.get("configuration") or {}).get("attributes") or {}
        kind = attrs.get("kind")
        article = product.get("id") or product.get("identity", {}).get("articleNumber") or product.get("identity", {}).get("article_number")
        sheet = int(product.get("sheet") or 0)
        kv = record_kv(product)

        if sheet == 2 or kind == "valveHeating":
            dn = int(attrs.get("dn") or kv.get("dn") or 0)
            kvs_m3_s = float(attrs.get("kvsM3S", attrs.get("kvs_m3_s", 0.0)) or 0.0)
            if kvs_m3_s == 0.0 and "kvs" in kv:
                kvs_m3_s = float(kv["kvs"]) / 3600.0
            kvs_h = kvs_m3_s * 3600.0
            min_kvs = valve_min_kvs_m3_h(dn if dn in VALVE_DN_SERIES else 50)
            checks.append({"id": f"dn:{article}", "pass": dn in VALVE_DN_SERIES, "dn": dn})
            checks.append({"id": f"kvs:{article}", "pass": kvs_h + 1e-12 >= min_kvs, "kvs_m3_h": kvs_h, "min": min_kvs})
            pc = attrs.get("pressureClass") or attrs.get("pressure_class") or kv.get("pressure_class") or ""
            checks.append({"id": f"pn:{article}", "pass": bool(pc), "pressureClass": pc})
            amin = float(attrs.get("authorityMin", attrs.get("authority_min", kv.get("authority_min", 0))) or 0)
            amax = float(attrs.get("authorityMax", attrs.get("authority_max", kv.get("authority_max", 0))) or 0)
            checks.append({"id": f"authority:{article}", "pass": amin < amax and 0 <= amin and amax <= 1})

        if sheet == 3 or kind == "radiator":
            phi = float(attrs.get("standardOutputW", attrs.get("standard_output_w", kv.get("standard_output_w", 0))) or 0)
            n = float(attrs.get("heatExponentN", attrs.get("heat_exponent_n", kv.get("n", 0))) or 0)
            checks.append({"id": f"phi:{article}", "pass": phi >= 1.0, "phi": phi})
            checks.append({"id": f"n:{article}", "pass": RADIATOR_N_MIN <= n <= RADIATOR_N_MAX, "n": n})

        if sheet == 5 or kind == "pumpHeating":
            q = float(attrs.get("nominalFlowM3S", attrs.get("nominal_flow_m3_s", kv.get("nominal_flow_m3_s", 0))) or 0)
            eta = float(attrs.get("hydraulicEfficiency", attrs.get("hydraulic_efficiency", kv.get("hydraulic_efficiency", 0))) or 0)
            checks.append({"id": f"q:{article}", "pass": q >= 1e-6, "q": q})
            checks.append({"id": f"eta:{article}", "pass": 0.0 < eta <= 1.0, "eta": eta})

        if sheet == 6 or kind == "heatGenerator":
            qn = float(attrs.get("nominalHeatOutputW", attrs.get("nominal_heat_output_w", kv.get("nominal_heat_output_w", 0))) or 0)
            fuel = attrs.get("fuelType") or attrs.get("fuel_type") or kv.get("fuel_type") or ""
            checks.append({"id": f"qn:{article}", "pass": qn >= 1.0, "qn": qn})
            checks.append({"id": f"fuel:{article}", "pass": bool(fuel), "fuel": fuel})

        if sheet == 53:
            cop = float(attrs.get("cop") or kv.get("cop") or 0)
            entries = attrs.get("entries") or []
            if cop == 0.0:
                for e in entries:
                    if str(e.get("key", "")).lower() == "cop":
                        try:
                            cop = float(e.get("value") or 0)
                        except ValueError:
                            cop = 0.0
            checks.append({"id": f"cop:{article}", "pass": 1.0 <= cop <= 10.0, "cop": cop})

        if sheet in SHEET_MANDATORY and kind in (None, "generic"):
            missing = []
            for key in SHEET_MANDATORY[sheet]:
                if key == "product_group":
                    group = (product.get("identity") or {}).get("productGroup") or (product.get("identity") or {}).get("product_group") or kv.get("product_group")
                    if not group:
                        missing.append(key)
                elif not kv.get(key):
                    entries = attrs.get("entries") or []
                    if not any(str(e.get("key", "")).lower() == key and e.get("value") for e in entries):
                        missing.append(key)
            checks.append({"id": f"mandatory:{sheet}:{article}", "pass": not missing, "missing": missing})

        geom = (product.get("configuration") or {}).get("geometryRef") or (product.get("configuration") or {}).get("geometry_ref")
        if geom:
            checks.append({"id": f"geom:{article}", "pass": geom in (doc.get("geometry") or {}), "ref": geom})
    return checks


def compare_report(oracle: list[dict], report: dict | None) -> list[dict]:
    if not report:
        return [{"id": c["id"], "oracle_pass": c["pass"], "rust_pass": None} for c in oracle]
    rust = {c["id"]: c for c in report.get("checks") or []}
    out = []
    for c in oracle:
        rid = None
        for key in (c["id"], c["id"].replace(":", ".")):
            for rk, rv in rust.items():
                if key.split(":")[0] in rk or key.replace(":", ".") in rk:
                    rid = rk
                    break
            if rid:
                break
        rust_pass = None
        if rid:
            status = rust[rid].get("status") or rust[rid].get("Status")
            rust_pass = status in ("Pass", "pass", "Ok")
        out.append({"id": c["id"], "oracle_pass": c["pass"], "rust_pass": rust_pass, **{k: v for k, v in c.items() if k not in {"id", "pass"}}})
    return out


def main() -> int:
    if len(sys.argv) < 2:
        print("usage: 🐍️.py <snapshot.json> [report.json]", file=sys.stderr)
        return 2
    doc = json.loads(Path(sys.argv[1]).read_text())
    checks = oracle_checks(doc)
    report = json.loads(Path(sys.argv[2]).read_text()) if len(sys.argv) > 2 else None
    payload = {"checks": checks, "compare": compare_report(checks, report)}
    print(json.dumps(payload, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
