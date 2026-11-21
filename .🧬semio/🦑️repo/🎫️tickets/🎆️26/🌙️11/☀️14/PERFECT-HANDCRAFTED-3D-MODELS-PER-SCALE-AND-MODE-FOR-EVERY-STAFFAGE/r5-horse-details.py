"""🖍️ Round 5 horse (corrected rig and body): the hand-drawn detail and texture strokes of the side elevation (creases, coronet bands, coat strokes, tail hair), stated in the rest world and written in the frame of their bone for r3-horse-draw2d.ts.

Usage: python3 r5-horse-details.py   (writes 🗑️generated/r5-horse/details.json)
"""
import importlib.util
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "r3-horse-kit.py"))
K = importlib.util.module_from_spec(spec)
spec.loader.exec_module(K)


def line(bone, pts):
    return {"points": [[round(c, 4) for c in K.local(bone, x, y)] for x, y in pts], "closed": False}


def own(pts):
    return {"points": [[round(x, 4), round(y, 4)] for x, y in pts], "closed": False}


def ring(cx, cy, rx, ry, n=12):
    return {"points": [[round(cx + rx * math.cos(2 * math.pi * i / n), 4), round(cy + ry * math.sin(2 * math.pi * i / n), 4)] for i in range(n)], "closed": True}


HIP = line("lumbar", [(0.23, 1.48), (0.21, 1.32), (0.24, 1.12)])
SHOULDER = line("thorax", [(1.22, 1.53), (1.37, 1.36), (1.43, 1.15)])
E50 = {
    "hip-line": HIP, "shoulder-line": SHOULDER,
    "nostril": own([(0.535, -0.008), (0.553, 0.006), (0.571, -0.004)]),
    "brow": own([(0.162, 0.03), (0.2, 0.037), (0.235, 0.022)]),
    "mouth-line": own([(0.592, -0.076), (0.52, -0.088), (0.44, -0.1), (0.4, -0.114)]),
    "stifle-line": line("thigh-hn", [(0.19, 1.04), (0.235, 0.97), (0.215, 0.89)]),
    "knee-line": line("radius-fn", [(1.12, 0.53), (1.21, 0.53)]),
    "hock-line": line("shank-hn", [(-0.11, 0.6), (0.02, 0.53)]),
    "hoof-line-h": line("hoof-hn", [(-0.021, 0.045), (0.057, 0.087)]),
    "hoof-line-f": line("hoof-fn", [(1.17, 0.043), (1.257, 0.084)]),
    "coat-line-1": line("thorax", [(0.75, 1.2), (0.82, 1.0)]),
    "coat-line-2": line("thorax", [(0.95, 1.22), (1.0, 1.02)]),
    "coat-line-3": line("lumbar", [(0.42, 1.25), (0.5, 1.08)]),
    "tail-hair-1": own([(0.02, 0.02), (0.2, 0.025)]),
    "tail-hair-2": own([(0.03, -0.005), (0.22, -0.002)]),
    "tail-hair-3": own([(0.02, -0.03), (0.2, -0.032)]),
}
OUT = {"50": {"elevation": E50}, "100": {"elevation": {"hip-line": HIP, "shoulder-line": SHOULDER}}, "200": {"elevation": {"hip-line": HIP, "shoulder-line": SHOULDER, "eye": ring(0.2, 0.0, 0.02, 0.016)}}}
json.dump(OUT, open(os.path.join(HERE, "🗑️generated", "r5-horse", "details.json"), "w", encoding="utf-8"), ensure_ascii=False)
print("details written")
