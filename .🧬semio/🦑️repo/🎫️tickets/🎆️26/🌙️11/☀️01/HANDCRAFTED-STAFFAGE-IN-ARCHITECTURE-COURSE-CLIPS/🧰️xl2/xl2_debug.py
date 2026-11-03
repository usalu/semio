"""Debug one-capsule recolour differences."""
import importlib
import os
import sys

sys.path.insert(0, os.getcwd())
V = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")
draw = V.NakaginDrawing()
slot = V._model.slots()[0]
opts = dict(towers=(slot.tower,), core=False, podium=False, capital=False, where=lambda s: s is slot or s == slot)
opts["lit"] = None
fresh = draw._tower_drawing(at=(0, 0, 0), scale=1, view="axon", coverage_padding=V._raster_footprint(1, .45), tint=lambda k: "#AA3377", **opts)
got = draw.tower_axon(at=(0, 0, 0), scale=1, tint=lambda k: "#AA3377", **opts)
a = [m for m in fresh.get_family() if len(m.points)]
b = [m for m in got.get_family() if len(m.points)]
print(len(a), len(b))
for i, (x, y) in enumerate(zip(a, b)):
    sx, sy = x.get_stroke_width(), y.get_stroke_width()
    fx, fy = x.get_fill_rgbas().tolist(), y.get_fill_rgbas().tolist()
    if sx != sy or fx != fy:
        print(i, getattr(x, "semantic_role", None), getattr(y, "paint", None), sx, sy, fx == fy, getattr(x, "paint", None))
        if i > 200:
            break
