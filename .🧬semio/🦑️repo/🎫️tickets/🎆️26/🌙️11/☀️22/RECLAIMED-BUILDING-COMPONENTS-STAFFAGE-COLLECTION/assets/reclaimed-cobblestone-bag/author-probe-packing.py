"""🧪️ Probes the layer order and in-bag shift at the one interface where the right-hand patches give way to the left-hand ones."""
import importlib.util, sys
from shapely.geometry import Polygon
from shapely.ops import unary_union
spec = importlib.util.spec_from_file_location("g", "author-geometry.py"); g = importlib.util.module_from_spec(spec); spec.loader.exec_module(g)
g.LAYER_ORDER[:] = ["r1", "r2", "r3", "r4", "l1", "l2", "l3", "l4"]
fc = {p: g.field_centre(p) for p in g.PATCH_IDS}
g.field_centre = lambda p: fc[p]
under = unary_union([Polygon(g.square(x, z, y)) for _, _, x, z, y in g.rest_squares("r4")])
best = []
for sx in [i * 0.01 for i in range(-6, 7)]:
    for sz in [i * 0.01 for i in range(-6, 7)]:
        g.REST_SHIFT["l1"] = (sx, sz)
        polys = [Polygon(g.square(x, z, y)) for _, _, x, z, y in g.rest_squares("l1")]
        sh = sorted(p.intersection(under).area / p.area for p in polys)
        x0, z0, x1, z1 = unary_union(polys).bounds
        inner = 0.4746
        m = min(x0 - (0.6 - inner), (0.6 + inner) - x1, z0 + inner, inner - z1)
        if m > 0.01: best.append((sh[0], sh[1], sx, sz, round(m, 3)))
best.sort(reverse=True); print(best[:8])
