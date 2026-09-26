#!/usr/bin/env python3
"""🧪 Independent EN 1991 evaluate oracle (DIN EN NA snow/wind/imposed)."""
from __future__ import annotations
import json, math, sys
from pathlib import Path

def kn_m2_to_pa(x): return x * 1000.0

def ground_snow_pa(zone: str, altitude_m: float) -> float:
    a = max(altitude_m, 0.0)
    z = zone.lower()
    if z in ("1", "1a"):
        base = 0.65 if a <= 400 else 0.19 + 0.91 * ((a + 140) / 760) ** 2
        if z == "1a":
            base *= 1.25
    elif z in ("2", "2a"):
        base = 0.85 if a <= 285 else 0.25 + 1.91 * ((a + 140) / 760) ** 2
        if z == "2a":
            base *= 1.25
    else:
        base = 1.10 if a <= 255 else 0.31 + 2.91 * ((a + 140) / 760) ** 2
    return kn_m2_to_pa(base)

def shape_mu(pitch_deg, multi_span=False, exceptional=False):
    alpha = max(pitch_deg, 0.0)
    if alpha <= 30:
        mu = 0.8
    elif alpha < 60:
        mu = 0.8 * (60 - alpha) / 30
    else:
        mu = 0.0
    if multi_span:
        mu = max(mu, 1.6)
    if exceptional:
        mu = min(mu * 2.0, 2.0)
    return mu

def imposed_qk_pa_de(category: str) -> float:
    c = category.upper()
    table = {"A":1.5,"A1":1.5,"A2":3.0,"A3":4.0,"B":2.0,"B1":2.0,"B2":3.0,"C":3.0,"C1":3.0,"H":0.75}
    return kn_m2_to_pa(table.get(c, 2.0))

def self_weight_pa(material: str, thickness: float) -> float:
    dens = {"reinforced_concrete":25.0,"concrete":25.0,"steel":78.5}.get(material, 20.0)
    return dens * 1000.0 * thickness

def alpha_a(category, area):
    c = category.upper()[:1]
    if c not in "CDEIJK":
        return 1.0
    a = max(area, 1.0)
    return min(1.0, max(0.7, 0.5 + 10.0 / (a ** 0.5)))

def alpha_n(category, n):
    c = category.upper()[:1]
    if c not in "ABCD" or n < 2:
        return 1.0
    return min(1.0, max(0.7, (2.0 + (n - 2.0) * 0.3) / n))

def check_snapshot(doc: dict) -> list[dict]:
    out = []
    annex = doc.get("annex", "de")
    storeys = int(doc.get("storeyCount", 1) or 1)
    for i, floor in enumerate(doc.get("floors", [])):
        base = imposed_qk_pa_de(floor["category"]) if str(annex).lower() in ("de","De") else kn_m2_to_pa(3.0)
        req = base * alpha_a(floor["category"], floor.get("area", 1.0)) * alpha_n(floor["category"], storeys)
        out.append({"id": f"en1991.1-1.imposed.{floor['id']}", "required": req, "assumed": floor["assumedQk"], "ok": floor["assumedQk"] + 1e-9 >= req})
    for i, el in enumerate(doc.get("selfWeightElements", [])):
        req = self_weight_pa(el["material"], el["thickness"])
        out.append({"id": f"en1991.1-1.self-weight.{el['id']}", "required": req, "assumed": el["assumedGk"], "ok": el["assumedGk"] + 1e-9 >= req})
    sk = ground_snow_pa(doc.get("snowZone","2"), doc.get("altitude",150))
    for roof in doc.get("roofs", []):
        mu = shape_mu(roof.get("pitchDeg",0), roof.get("multiSpan", False), doc.get("exceptionalSnowNorthGermanLowlands", False))
        req = mu * roof.get("cE",1) * roof.get("cT",1) * sk
        out.append({"id": f"en1991.1-3.snow.{roof['id']}", "required": req, "assumed": roof["assumedSk"], "ok": roof["assumedSk"] + 1e-9 >= req})
    return out

def main():
    path = sys.argv[1]
    doc = json.loads(Path(path).read_text())
    print(json.dumps(check_snapshot(doc), indent=2))

if __name__ == "__main__":
    main()
