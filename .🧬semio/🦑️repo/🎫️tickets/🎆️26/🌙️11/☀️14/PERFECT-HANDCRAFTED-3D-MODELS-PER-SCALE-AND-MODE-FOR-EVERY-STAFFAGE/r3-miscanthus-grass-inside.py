"""🔎 Round 3, vegetation/miscanthus-grass: lists the structure solids of the candidate that reach farthest outside the drawn body in front and plan (rest coordinates are world coordinates there): for every member rod, joint and mass the distance of its surface to the nearest stroke (half pen) or outline of the same scale.

Usage: python3 r3-miscanthus-grass-inside.py 50
See: ./r3-vegetation-miscanthus-grass.md
"""
import importlib.util
import json
import math
import sys
from pathlib import Path

TICKET = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("kit", TICKET / "r3-miscanthus-grass-kit.py")
K = importlib.util.module_from_spec(spec)
spec.loader.exec_module(K)
asset = json.loads((TICKET / "🗑️generated/r3-miscanthus-grass/🔣️.json").read_text(encoding="utf-8"))["asset"]
scale = sys.argv[1]
rig = K.Rig(asset["rig"]["bones"])


def world(bone, q):
    u, v, w = rig.axes(bone)
    o = rig.origin(bone)
    return tuple(o[k] + q[0] * u[k] + q[1] * v[k] + q[2] * w[k] for k in range(3))


def inside(p, ring):
    n, c = len(ring), False
    for i in range(n):
        a, b = ring[i], ring[(i + 1) % n]
        if (a[1] > p[1]) != (b[1] > p[1]) and p[0] < a[0] + (b[0] - a[0]) * (p[1] - a[1]) / (b[1] - a[1]):
            c = not c
    return c


def outside(p, r, shapes):
    """📏 How far a disc of radius r about p reaches outside the nearest shape: a stroke covers half its pen about its line, an outline its interior."""
    best = 1e9
    for s in shapes:
        pts = s["points"]
        ring = pts + [pts[0]] if s.get("closed") else pts
        near = 1e9
        for a, b in zip(ring, ring[1:]):
            dx, dy = b[0] - a[0], b[1] - a[1]
            l2 = dx * dx + dy * dy
            f = 0 if l2 == 0 else max(0.0, min(1.0, ((p[0] - a[0]) * dx + (p[1] - a[1]) * dy) / l2))
            near = min(near, math.hypot(p[0] - a[0] - f * dx, p[1] - a[1] - f * dy))
        if s["filled"]:
            reach = r - near if inside(p, pts) else r + near
        else:
            reach = near + r - s.get("penMm", 0.1) * int(scale) / 2000
        best = min(best, max(0.0, reach))
    return best


for view, pick in (("front", lambda p: (p[2], p[1])), ("plan", lambda p: (p[0], p[2]))):
    shapes = asset["representations"][scale]["geometry"][view]
    rows = []
    for solid in asset["volumes"][scale]["structure"]:
        if solid["kind"] == "loft":
            points = [(world(solid["bone"], s["at"]), max(s["rx"], s["ry"])) for s in solid["sections"]]
        else:
            t = solid.get("transform", {}).get("translation", [0, 0, 0])
            points = [(world(solid["bone"], t), max(solid["radii"]))]
        worst = max(outside(pick(p), r, shapes) for p, r in points)
        rows.append((worst, solid["id"]))
    rows.sort(reverse=True)
    print(view, " ".join(f"{name} {d * 1000:.1f}" for d, name in rows[:6]))
