"""Where the bytes of a built tower template go. Usage: xl4_budget.py [storeys]"""
import gc
import sys

import numpy as np

import xl4_common as C

visuals = C.load()
storeys = int(sys.argv[1]) if len(sys.argv) > 1 else 2
fit = C.program_fit()
gc.collect()
base = C.rss()
draw = visuals.NakaginDrawing()
drawing = draw.tower_axon(**fit, types=False, towers=(0,), where=lambda s, n=storeys: s.tower == 0 and s.storey < n, core=False, podium=False, capital=False)
template = next(iter(visuals._TOWERS.values()))
members = [m for m in template.get_family() if len(m.points)]
cover = [m for m in members if getattr(m, "semantic_role", None) == "material_coverage"]
lines = [m for m in members if not hasattr(m, "paint") and hasattr(m, "world_vertices") and not hasattr(m, "semantic_role")]
frag = [m for m in members if hasattr(m, "source_polygon")]


def nbytes(shapes, attr):
    total = 0
    for shape in shapes:
        value = getattr(shape, attr, None)
        if value is None:
            continue
        if isinstance(value, np.ndarray):
            total += value.nbytes + 112
        elif isinstance(value, tuple):
            total += sum(a.nbytes + 112 + 8 for facet in value for a in facet if isinstance(a, np.ndarray))
    return total


print(f"[budget] members={len(members)} coverage={len(cover)} fragments={len(frag)} lines={len(lines)} groups={len(template.get_family()) - len(members)}")
print(f"[budget] points MB all={sum(m.points.nbytes + 112 for m in members) / 1e6:.1f} coverage={sum(m.points.nbytes for m in cover) / 1e6:.1f}")
print(f"[budget] world_vertices MB={nbytes(members, 'world_vertices') / 1e6:.1f} world_facets MB={nbytes(members, 'world_facets') / 1e6:.1f} world_normal MB={nbytes(members, 'world_normal') / 1e6:.1f}")
rows = np.array([len(m.points) for m in members])
print(f"[budget] point rows mean={rows.mean():.1f} median={np.median(rows):.0f} p90={np.percentile(rows, 90):.0f} max={rows.max()}")
total = C.rss() - base
print(f"[budget] rss delta={total / 1e6:.0f}MB per member={total / len(members):.0f}B")
orders = visuals._OCCLUSION_ORDERS
print(f"[budget] cached orders={len(orders)} rows={sum(len(v) for v in orders.values())} opaque cache={len(visuals._OPAQUE_VISIBILITY)}")
