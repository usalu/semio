"""Potential of post-order single-cover culling on a small tower variant. Usage: xl4_cull.py [storeys]"""
import sys
import time

import numpy as np

import xl4_common as C

visuals = C.load()
storeys = int(sys.argv[1]) if len(sys.argv) > 1 else 2
fit = C.program_fit()
drawing = visuals.NakaginDrawing().tower_axon(**fit, types=False, towers=(0,), where=lambda s: s.tower == 0 and s.storey < storeys, core=False, podium=False, capital=False)
top = [m for m in drawing.submobjects]
shapes = [m for m in drawing.get_family() if len(m.points) and getattr(m, "world_vertices", None) is not None]
members = [m for m in drawing.get_family() if len(m.points)]
print(f"[cull] family={len(members)} world-shapes={len(shapes)}", flush=True)
config = visuals._engine.config
pixel = max(config.frame_width / config.pixel_width, config.frame_height / config.pixel_height)
pad = 2 * pixel + 0.45 * .005
grid = {}
size = 20 * pixel
rows = []
for index, shape in enumerate(shapes):
    anchors = shape.points[::4]
    if len(anchors) < 3 or float(shape.get_fill_opacity()) < 1:
        rows.append(None)
        continue
    xy = anchors[:, :2]
    rows.append((xy.min(axis=0), xy.max(axis=0), xy))
began = time.time()
hidden = 0
wide = 0
for index in range(len(shapes) - 1, -1, -1):
    row = rows[index]
    if row is None:
        continue
    lo, hi, xy = row
    lo, hi = lo - pad, hi + pad
    box = np.array([lo, [hi[0], lo[1]], hi, [lo[0], hi[1]]])
    covered = False
    for gx in range(int(lo[0] // size), int(hi[0] // size) + 1):
        for gy in range(int(lo[1] // size), int(hi[1] // size) + 1):
            for later in grid.get((gx, gy), ()):
                tlo, thi, txy, tconvex = later
                if tlo[0] > lo[0] or tlo[1] > lo[1] or thi[0] < hi[0] or thi[1] < hi[1] or not tconvex:
                    continue
                edge = np.roll(txy, -1, axis=0) - txy
                side = box[:, None, :] - txy[None, :, :]
                cross = edge[None, :, 0] * side[:, :, 1] - edge[None, :, 1] * side[:, :, 0]
                if (cross >= 0).all() or (cross <= 0).all():
                    covered = True
                    break
            if covered:
                break
        if covered:
            break
    if covered:
        hidden += 1
        continue
    edge = np.roll(xy, -1, axis=0) - xy
    turn = edge[:, 0] * np.roll(edge[:, 1], -1) - edge[:, 1] * np.roll(edge[:, 0], -1)
    convex = bool((turn >= -1e-12).all() or (turn <= 1e-12).all())
    item = (row[0], row[1], xy, convex)
    for gx in range(int(row[0][0] // size), int(row[1][0] // size) + 1):
        for gy in range(int(row[0][1] // size), int(row[1][1] // size) + 1):
            grid.setdefault((gx, gy), []).append(item)
print(f"[cull] shapes={len(shapes)} hideable={hidden} ({100 * hidden / len(shapes):.1f}%) t={time.time() - began:.1f}s pixel={pixel:.5f}", flush=True)
