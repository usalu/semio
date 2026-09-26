#!/usr/bin/env python3
"""EN 1998 numeric oracle — spectrum / base shear utilizations (±0.5 % vs Rust)."""
from __future__ import annotations

import json
import sys
from typing import Any


def a_gr_de(zone: str) -> float:
    return {"zone0": 0.0, "zone1": 0.4, "zone2": 0.6, "zone3": 0.8}.get(zone, 0.6)


def spectrum_params(combo: str) -> tuple[float, float, float, float]:
    # DIN EN 1998-1/NA Table NA.4
    return {
        "A-R": (1.00, 0.05, 0.20, 2.0),
        "B-R": (1.25, 0.05, 0.25, 2.0),
        "C-R": (1.50, 0.05, 0.30, 2.0),
        "B-T": (1.25, 0.10, 0.30, 2.0),
        "C-T": (1.50, 0.10, 0.40, 2.0),
        "C-S": (0.75, 0.10, 0.50, 2.0),
    }.get(combo, (1.25, 0.05, 0.25, 2.0))


def elastic_se(a_g: float, s: float, tb: float, tc: float, td: float, t: float) -> float:
    eta = 1.0
    if t <= 0:
        return a_g * s
    if t <= tb:
        return a_g * s * (1.0 + t / tb * (2.5 * eta - 1.0))
    if t <= tc:
        return a_g * s * 2.5 * eta
    if t <= td:
        return a_g * s * 2.5 * eta * tc / t
    return a_g * s * 2.5 * eta * tc * td / (t * t)


def psi2(cat: str) -> float:
    c = cat.strip().lower()
    return {
        "a": 0.3,
        "residential": 0.3,
        "b": 0.3,
        "office": 0.3,
        "c": 0.6,
        "congregation": 0.6,
        "d": 0.6,
        "retail": 0.6,
        "e": 0.8,
        "storage": 0.8,
        "f": 0.6,
        "traffic": 0.6,
        "g": 0.3,
        "vehicles": 0.3,
        "h": 0.0,
        "roofs": 0.0,
        "snow": 0.2,
        "wind": 0.0,
    }.get(c, 0.3)


def storey_mass(st: dict, is_roof: bool) -> float:
    phi = 1.0 if is_roof else (0.8 if st.get("correlatedOccupancy", True) else 0.5)
    w = float(st["permanentGkN"])
    for v in st.get("variables", []):
        w += phi * psi2(str(v.get("category", "B"))) * float(v["qkN"])
    return w / 9.81


def evaluate(doc: dict[str, Any]) -> dict[str, Any]:
    site = doc["site"]
    annex = doc.get("annex", "de")
    if annex == "de":
        a_g = a_gr_de(site["seismicZone"])
        s, tb, tc, td = spectrum_params(site["deGroundCombo"])
    else:
        a_g = float(site["aGr"])
        s, tb, tc, td = 1.2, 0.15, 0.40, 2.0
    checks = []
    for b in doc.get("buildings", []):
        storeys = b["storeys"]
        h = sum(st["heightM"] for st in storeys)
        mass = sum(storey_mass(st, i + 1 == len(storeys)) for i, st in enumerate(storeys))
        ct = float(b.get("ct", 0.075))
        t1 = ct * (h**0.75)
        for sys in b.get("systems", []):
            q = max(1.0, float(sys["q0"]) * float(sys["alphaUOverAlpha1"]) * float(sys["kW"]))
            se = elastic_se(a_g, s, tb, tc, td, t1)
            sd = max(se / q, 0.2 * a_g)
            lam = 0.85 if t1 <= 2 * tc and len(storeys) > 2 else 1.0
            fb = sd * mass * lam
            vrd = float(sys["baseShearResistanceN"])
            checks.append(
                {
                    "id": f"en1998.1.{b['id']}.{sys['id']}.baseShear",
                    "utilization": (fb / vrd) if vrd > 0 else 0.0,
                }
            )
    return {"checks": checks, "spectrum": {"S": s, "TB": tb, "TC": tc, "TD": td, "aG": a_g}}


def main() -> None:
    if "--json" in sys.argv:
        doc = json.load(sys.stdin)
        json.dump(evaluate(doc), sys.stdout)
        return
    if "--combo-table" in sys.argv:
        out = {}
        for combo in ["A-R", "B-R", "C-R", "B-T", "C-T", "C-S"]:
            s, tb, tc, td = spectrum_params(combo)
            se = elastic_se(0.6, s, tb, tc, td, 0.25)
            out[combo] = {"S": s, "TB": tb, "TC": tc, "TD": td, "Se025": se}
        json.dump(out, sys.stdout)
        return
    se = elastic_se(0.6, 1.25, 0.05, 0.25, 2.0, 0.25)
    assert abs(se - 1.875) < 1e-9, se
    s_cs, _, _, _ = spectrum_params("C-S")
    assert abs(s_cs - 0.75) < 1e-12, s_cs
    print("ok", se)


if __name__ == "__main__":
    main()
