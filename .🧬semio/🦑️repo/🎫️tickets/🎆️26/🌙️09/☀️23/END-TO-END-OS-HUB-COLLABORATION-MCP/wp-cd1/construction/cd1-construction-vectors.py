#!/usr/bin/env python3
"""📐️ CD1: writes the language-agnostic `construction-from-2-points` vectors — an independent (Python) planner computes every
expected plan and its closed-form measures, so neither the TS nor the Rust planner is checked against itself.
Usage: cd1-construction-vectors.py <tree-root>"""
import json
import math
import sys

TARGET = "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🧬️typology/🧫️fixtures/📐️construction-from-2-points/🔣️.json"
EPS = 1e-6


def r(v):
    return [round(x, 12) + 0.0 for x in v]


def plan(c, a, b, h):
    if not (h > EPS):
        return None
    base_b = [b[0], b[1], a[2]]
    dx, dy = b[0] - a[0], b[1] - a[1]
    length = math.hypot(dx, dy)
    up = lambda p, dz: [p[0], p[1], p[2] + dz]
    kind = (c["primitive"], c["profile"])
    if kind == ("solid", "segment"):
        if not (length > EPS):
            return None
        x = [dx / length, dy / length, 0.0]
        y = [-x[1], x[0], 0.0]
        t = c["thickness"]
        shift = {"center": -t / 2, "left": 0.0, "right": -t}[c["alignment"]]
        origin = [a[0] + shift * y[0], a[1] + shift * y[1], a[2]]
        corners = [[origin[k] + i * length * x[k] + j * t * y[k] + (kk * h if k == 2 else 0) for k in range(3)] for i in (0, 1) for j in (0, 1) for kk in (0, 1)]
        return {"kind": "prism", "origin": r(origin), "xAxis": r(x), "yAxis": r(y), "extents": r([length, t, h]), "volume": round(length * t * h, 12), "bounds": bounds(corners)}
    if kind == ("solid", "rectangle"):
        if not (abs(dx) > EPS and abs(dy) > EPS):
            return None
        origin = [min(a[0], b[0]), min(a[1], b[1]), a[2]]
        extents = [abs(dx), abs(dy), h]
        return {"kind": "prism", "origin": r(origin), "xAxis": [1.0, 0.0, 0.0], "yAxis": [0.0, 1.0, 0.0], "extents": r(extents), "volume": round(abs(dx) * abs(dy) * h, 12), "bounds": {"min": r(origin), "max": r([origin[0] + extents[0], origin[1] + extents[1], origin[2] + h])}}
    if kind == ("solid", "circle"):
        if not (length > EPS):
            return None
        return {"kind": "cylinder", "base": r(a), "radius": round(length, 12), "height": round(h, 12), "volume": round(math.pi * length * length * h, 12), "bounds": {"min": r([a[0] - length, a[1] - length, a[2]]), "max": r([a[0] + length, a[1] + length, a[2] + h])}}
    if kind == ("surface", "segment"):
        if not (length > EPS):
            return None
        corners = [a, base_b, up(base_b, h), up(a, h)]
        return {"kind": "quad", "corners": [r(p) for p in corners], "normal": r([dy / length, -dx / length, 0.0]), "area": round(length * h, 12), "bounds": bounds(corners)}
    if kind == ("surface", "rectangle"):
        if not (abs(dx) > EPS and abs(dy) > EPS):
            return None
        x0, x1, y0, y1, z = min(a[0], b[0]), max(a[0], b[0]), min(a[1], b[1]), max(a[1], b[1]), a[2] + h
        corners = [[x0, y0, z], [x1, y0, z], [x1, y1, z], [x0, y1, z]]
        return {"kind": "quad", "corners": [r(p) for p in corners], "normal": [0.0, 0.0, 1.0], "area": round(abs(dx) * abs(dy), 12), "bounds": bounds(corners)}
    if kind == ("curve", "segment"):
        if not (length > EPS):
            return None
        return {"kind": "line", "start": r(up(a, h)), "end": r(up(base_b, h)), "length": round(length, 12), "bounds": bounds([up(a, h), up(base_b, h)])}
    if kind == ("curve", "point"):
        return {"kind": "line", "start": r(a), "end": r(up(a, h)), "length": round(h, 12), "bounds": bounds([a, up(a, h)])}
    raise SystemExit(f"unsupported construction {kind}")


def bounds(points):
    return {"min": r([min(p[k] for p in points) for k in range(3)]), "max": r([max(p[k] for p in points) for k in range(3)])}


WALL = {"primitive": "solid", "profile": "segment", "thickness": 0.3, "alignment": "center"}
CASES = [
    ("wall-along-x", "the bug vector: an axis-aligned wall gets the typology thickness, not a zero-depth bounding box", WALL, [0, 0, 0], [4, 0, 0], 2.7),
    ("wall-diagonal-left", "a 3-4-5 diagonal wall follows its baseline; left alignment puts the thickness on the left of A→B", {**WALL, "alignment": "left"}, [1, 1, 0], [4, 5, 0], 2.7),
    ("wall-diagonal-right", "right alignment puts the thickness on the right of A→B", {**WALL, "alignment": "right"}, [1, 1, 0], [4, 5, 0], 2.7),
    ("door-baseline-at-a", "the baseline is horizontal through A: B's elevation is not read", {"primitive": "solid", "profile": "segment", "thickness": 0.1, "alignment": "center"}, [0, 2, 1], [0, 3, 7], 2.1),
    ("slab-rectangle", "opposite footprint corners in any order, extruded by the height", {"primitive": "solid", "profile": "rectangle"}, [2, 3, 0], [-1, 1, 0], 0.25),
    ("column-circle", "centre A through B, extruded by the height", {"primitive": "solid", "profile": "circle"}, [1, 1, 0], [1.3, 1.4, 0], 3),
    ("external-wall-surface", "a surface wall is the vertical face over its baseline — area |AB|·h, no thickness", {"primitive": "surface", "profile": "segment"}, [0, 0, 0], [4, 0, 0], 2.7),
    ("roof-surface-rectangle", "a horizontal surface sits at the height above A, counter-clockwise from above", {"primitive": "surface", "profile": "rectangle"}, [5, 4, 0], [0, 0, 0], 3),
    ("internal-wall-curve", "a curve wall is its baseline lifted to the height", {"primitive": "curve", "profile": "segment"}, [0, 0, 0], [3, 4, 0], 3),
    ("column-curve-point", "a curve column is the vertical axis from A; B is not read", {"primitive": "curve", "profile": "point"}, [2, 2, 0], [9, 9, 9], 3),
]
REFUSALS = [
    ("coincident-segment", "A and B coincide in plan: a baseline has no direction", WALL, [1, 1, 0], [1, 1, 5], 2.7),
    ("zero-height", "nothing to extrude", WALL, [0, 0, 0], [4, 0, 0], 0),
    ("negative-height", "a height is a length", {"primitive": "surface", "profile": "segment"}, [0, 0, 0], [4, 0, 0], -1),
    ("flat-rectangle", "two corners on one axis span no footprint", {"primitive": "solid", "profile": "rectangle"}, [0, 0, 0], [5, 0, 0], 1),
    ("zero-radius", "a circle through its own centre", {"primitive": "solid", "profile": "circle"}, [1, 1, 0], [1, 1, 0], 3),
]
root = {
    "schema": "spatial.typology.construction-from-2-points/v1",
    "note": "How a typology's construction (`construction.from2PointsAndHeight` in its `spatial.typology` asset) reads two picked points A, B and a height h into a plan: `prism` (origin, unit xAxis/yAxis, extents along xAxis/yAxis/+z), `cylinder` (base, radius, height along +z), `quad` (4 corners, right-handed normal), `line` (start, end). Every implementation (TS planner, Rust planner, each kernel's realisation, OpenCascade) answers these vectors; refusals are `construction.degenerate-input`.",
    "units": {"length": "metre", "up": "z", "handedness": "right"},
    "tolerance": 1e-9,
    "cases": [{"id": i, "note": n, "construction": c, "pointA": a, "pointB": b, "height": h, "expect": plan(c, a, b, h)} for i, n, c, a, b, h in CASES],
    "refusals": [{"id": i, "note": n, "construction": c, "pointA": a, "pointB": b, "height": h, "code": "construction.degenerate-input"} for i, n, c, a, b, h in REFUSALS],
}
assert all(case["expect"] for case in root["cases"]) and not any(plan(c, a, b, h) for _, _, c, a, b, h in REFUSALS)
open(f"{sys.argv[1]}/{TARGET}", "w", encoding="utf-8").write(json.dumps(root, indent=2, ensure_ascii=False) + "\n")
print(f"{len(root['cases'])} cases, {len(root['refusals'])} refusals → {TARGET}")
