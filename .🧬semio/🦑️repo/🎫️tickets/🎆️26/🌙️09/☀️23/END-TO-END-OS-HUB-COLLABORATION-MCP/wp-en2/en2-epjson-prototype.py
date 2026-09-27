"""⚡️ EN2 prototype of the three epJSON export fixes, applied to the codec's committed documents.

Usage: en2-epjson-prototype.py <repo-root> <out-dir>

For each case 600/600FF/900/900FF reads the committed `⚡️model.epJSON` (today's codec output) and the
committed `🔋️model.json`, then applies exactly what the patched Rust writer does:
1. infiltration: `ScheduledAch`/`PerExteriorArea` → EnergyPlus coefficients (A, B, C, D) = (1, 0, 0, 0);
2. glazing: an aperture whose fenestration names a `glazing_construction_id` references that layered
   `Construction`; its per-window simple-glazing construction is not written (the simple-glazing material
   stays, carrying the model's fallback optics);
3. vertices: every `BuildingSurface:Detailed` / `FenestrationSurface:Detailed` ring starts at its
   upper-left corner seen from outside (up = world z in-plane, or north for horizontal rings), keeping
   the counter-clockwise order — the `UpperLeftCorner` the document declares.
"""
import json
import math
import sys
from pathlib import Path

repo = Path(sys.argv[1])
out = Path(sys.argv[2])
out.mkdir(parents=True, exist_ok=True)
fixtures = repo / "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures"


def sub(a, b):
    return [a[0] - b[0], a[1] - b[1], a[2] - b[2]]


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def dot(a, b):
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def unit(a):
    length = math.sqrt(dot(a, a))
    return None if length <= 1e-12 else [a[0] / length, a[1] / length, a[2] / length]


def newell(points):
    n = [0.0, 0.0, 0.0]
    for i, c in enumerate(points):
        d = points[(i + 1) % len(points)]
        n[0] += (c[1] - d[1]) * (c[2] + d[2])
        n[1] += (c[2] - d[2]) * (c[0] + d[0])
        n[2] += (c[0] - d[0]) * (c[1] + d[1])
    return unit(n)


def upper_left_first(points):
    normal = newell(points)
    if normal is None:
        return points
    up = unit(sub([0.0, 0.0, 1.0], [normal[0] * normal[2], normal[1] * normal[2], normal[2] * normal[2]])) or unit(sub([0.0, 1.0, 0.0], [normal[0] * normal[1], normal[1] * normal[1], normal[2] * normal[1]]))
    right = cross(up, normal)
    heights = [dot(p, up) for p in points]
    top = max(heights)
    candidates = [i for i, h in enumerate(heights) if top - h <= 1e-9]
    start = min(candidates, key=lambda i: dot(points[i], right))
    return points[start:] + points[:start]


for case in ["600", "600FF", "900", "900FF"]:
    document = json.loads((fixtures / ("🏛️bestest-%s/⚡️model.epJSON" % case)).read_text(encoding="utf-8"))
    model = json.loads((fixtures / ("🏛️bestest-%s/🔋️model.json" % case)).read_text(encoding="utf-8"))
    for fields in document.get("ZoneInfiltration:DesignFlowRate", {}).values():
        if fields["design_flow_rate_calculation_method"] in ("AirChanges/Hour", "Flow/ExteriorArea"):
            fields.update(constant_term_coefficient=1.0, temperature_term_coefficient=0.0, velocity_term_coefficient=0.0, velocity_squared_term_coefficient=0.0)
    constructions = {entry["id"]: entry["name"] for entry in model["constructions"]}
    for window in model["fenestrations"]:
        layered = constructions.get(window.get("glazing_construction_id"))
        if layered is None:
            continue
        document["FenestrationSurface:Detailed"][window["name"]]["construction_name"] = layered
        document["Construction"].pop("%s Glazing Construction" % window["name"], None)
    for fields in document["BuildingSurface:Detailed"].values():
        points = [[v["vertex_x_coordinate"], v["vertex_y_coordinate"], v["vertex_z_coordinate"]] for v in fields["vertices"]]
        fields["vertices"] = [{"vertex_x_coordinate": p[0], "vertex_y_coordinate": p[1], "vertex_z_coordinate": p[2]} for p in upper_left_first(points)]
    for fields in document["FenestrationSurface:Detailed"].values():
        count = sum(1 for key in fields if key.endswith("_x_coordinate"))
        points = [[fields["vertex_%d_%s_coordinate" % (i, a)] for a in "xyz"] for i in range(1, count + 1)]
        for i, p in enumerate(upper_left_first(points), 1):
            for a, v in zip("xyz", p):
                fields["vertex_%d_%s_coordinate" % (i, a)] = v
    (out / ("%s.epJSON" % case)).write_text(json.dumps(document, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print("wrote", out / ("%s.epJSON" % case))
