"""📐 Round 3, vegetation/miscanthus-grass: estimates the cross-scale boundary distance of two scales from the strokes and outlines of the candidate drawings of the front and plan views (rest coordinates are world coordinates there): the 95th percentile of the distance from points along the lines of one scale to the lines of the other, both directions.

Usage: python3 r3-miscanthus-grass-cross.py 100 200
See: ./r3-vegetation-miscanthus-grass.md
"""
import json
import math
import sys
from pathlib import Path

DOC = Path(__file__).resolve().parent / "🗑️generated/r3-miscanthus-grass/🔣️.json"


def lines(shapes):
    out = []
    for s in shapes:
        pts = s["points"] + ([s["points"][0]] if s.get("closed") else [])
        out.append(pts)
    return out


def samples(polylines, step):
    out = []
    for pts in polylines:
        for a, b in zip(pts, pts[1:]):
            n = max(1, int(math.hypot(b[0] - a[0], b[1] - a[1]) / step))
            out.extend((a[0] + (b[0] - a[0]) * k / n, a[1] + (b[1] - a[1]) * k / n) for k in range(n + 1))
    return out


def distance(p, segments):
    best = 1e9
    for ax, ay, bx, by in segments:
        dx, dy = bx - ax, by - ay
        l2 = dx * dx + dy * dy
        f = 0 if l2 == 0 else max(0.0, min(1.0, ((p[0] - ax) * dx + (p[1] - ay) * dy) / l2))
        d = (p[0] - ax - f * dx) ** 2 + (p[1] - ay - f * dy) ** 2
        if d < best:
            best = d
    return math.sqrt(best)


def p95(a, b):
    segments = [(p[0], p[1], q[0], q[1]) for pts in b for p, q in zip(pts, pts[1:])]
    ds = sorted(distance(p, segments) for p in samples(a, 0.03))
    return ds[int(0.95 * (len(ds) - 1))]


if __name__ == "__main__":
    fine, coarse = sys.argv[1], sys.argv[2]
    reps = json.loads(DOC.read_text(encoding="utf-8"))["asset"]["representations"]
    for view in ("front", "plan"):
        a, b = lines(reps[fine]["geometry"][view]), lines(reps[coarse]["geometry"][view])
        print(f"{view}: p95 1:{fine} to 1:{coarse} {p95(a, b) * 1000:.0f} mm, 1:{coarse} to 1:{fine} {p95(b, a) * 1000:.0f} mm")
