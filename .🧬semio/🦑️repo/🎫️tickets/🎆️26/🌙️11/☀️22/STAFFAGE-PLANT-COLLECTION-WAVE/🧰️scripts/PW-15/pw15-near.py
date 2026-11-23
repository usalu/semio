"""🔎 PW-15 authoring aid: the drawn shapes of one scale and view whose outline passes near a view point (default pose, rest frames), with the distance and the paper width of the shape there, so a spot the gate reports is traced to the parts around it.

Usage: python3 pw15-near.py <asset document json> <scale> <elevation|front|plan> <x> <y> [reach in metres, default 0.012]
See: ./pw15-gate.ts
"""
import importlib.util
import json
import math
import sys
from pathlib import Path

spec = importlib.util.spec_from_file_location("kit", Path(__file__).resolve().parents[4] / "☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-miscanthus-grass-kit.py")
K = importlib.util.module_from_spec(spec)
spec.loader.exec_module(K)

asset = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))["asset"]
scale, view, px, py = sys.argv[2], sys.argv[3], float(sys.argv[4]), float(sys.argv[5])
reach = float(sys.argv[6]) if len(sys.argv) > 6 else 0.012
rig = K.Rig(asset["rig"]["bones"])
rows = []
for shape in asset["representations"][scale]["geometry"][view]:
    pts = shape["points"]
    if view == "elevation":
        u, v, _ = rig.axes(shape["bone"])
        o = rig.origin(shape["bone"])
        pts = [(o[0] + p[0] * u[0] + p[1] * v[0], o[1] + p[0] * u[1] + p[1] * v[1]) for p in pts]
    best = min(math.hypot(px - (a[0] + max(0, min(1, ((px - a[0]) * (b[0] - a[0]) + (py - a[1]) * (b[1] - a[1])) / ((b[0] - a[0]) ** 2 + (b[1] - a[1]) ** 2 or 1))) * (b[0] - a[0])), py - (a[1] + max(0, min(1, ((px - a[0]) * (b[0] - a[0]) + (py - a[1]) * (b[1] - a[1])) / ((b[0] - a[0]) ** 2 + (b[1] - a[1]) ** 2 or 1))) * (b[1] - a[1]))) for a, b in zip(pts, pts[1:] + pts[:1]))
    if best <= reach:
        area = abs(sum(a[0] * b[1] - b[0] * a[1] for a, b in zip(pts, pts[1:] + pts[:1]))) / 2
        length = sum(math.hypot(b[0] - a[0], b[1] - a[1]) for a, b in zip(pts, pts[1:] + pts[:1])) / 2
        rows.append((best, shape["id"], shape["bone"], area / max(length, 1e-9)))
for best, sid, bone, width in sorted(rows):
    print(f"[DEBUG] {sid} on {bone}: outline {best * 1000:.1f} mm away, mean width {width * 1000:.1f} mm")
