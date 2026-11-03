"""Measure paint runs of a small ordered tower variant at the real fit scale. Usage: xl3_runs.py [storeys]"""
import importlib
import os
import sys
import time

import numpy as np

sys.path.insert(0, os.getcwd())
V = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")
fit = importlib.import_module("🎓️teaching.🏛️architecture.🧲️fit.🐍️").building_fit(5.4, 4.4, centre=(0, 0, 0))
storeys = int(sys.argv[1]) if len(sys.argv) > 1 else 4
draw = V.NakaginDrawing()
began = time.time()
drawing = draw.tower_axon(at=fit["at"], scale=fit["scale"], towers=(0,), where=lambda s: s.tower == 0 and s.storey < storeys, core=False, podium=False, capital=False)
shapes = [m for m in drawing.get_family() if len(m.points)]
print(f"[runs] shapes={len(shapes)} t={time.time() - began:.0f}s", flush=True)


def paint(shape):
    return (tuple(np.round(shape.get_fill_rgbas().ravel(), 5)), tuple(np.round(shape.get_stroke_rgbas().ravel(), 5)), round(float(shape.get_stroke_width()), 5), len(shape.submobjects) > 0)


runs = [1]
for a, b in zip(shapes, shapes[1:]):
    runs.append(runs[-1] + 1 if paint(a) == paint(b) else 1)
count = sum(1 for a, b in zip(runs, runs[1:] + [1]) if b == 1)
print(f"[runs] same-paint runs={count} of {len(shapes)} children={sum(1 for s in shapes if s.submobjects)}", flush=True)
pixel = V._raster_footprint(1, 0) / 2


def box(shape):
    pts = shape.points
    return pts[:, :2].min(axis=0) - 2 * pixel - shape.get_stroke_width() * .005, pts[:, :2].max(axis=0) + 2 * pixel + shape.get_stroke_width() * .005


merged, group, key = 0, [], None
for shape in shapes:
    k = paint(shape)
    b = box(shape)
    if k != key or any(not ((b[1] < o[0]).any() or (o[1] < b[0]).any()) for o in group):
        merged += 1
        group, key = [], k
    group.append(b)
print(f"[runs] disjoint-merged shapes={merged}", flush=True)
