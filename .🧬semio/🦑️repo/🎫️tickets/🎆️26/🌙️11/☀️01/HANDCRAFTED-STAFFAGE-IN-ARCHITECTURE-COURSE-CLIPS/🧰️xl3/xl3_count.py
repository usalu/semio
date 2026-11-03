"""Count retained faces of a small tower variant at different drawing scales. Usage: xl3_count.py"""
import importlib
import os
import sys
import time

sys.path.insert(0, os.getcwd())
V = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")
fit = importlib.import_module("🎓️teaching.🏛️architecture.🧲️fit.🐍️").building_fit(5.4, 4.4, centre=(0, 0, 0))
print("[count] fit scale", fit["scale"], "pixel", V._raster_footprint(1, 0), flush=True)
draw = V.NakaginDrawing()
view = V.make_view("axon", (0, 0, 0), 1, (0, 0, 0))
for scale in (1.0, fit["scale"]):
    padding = V._raster_footprint(scale, .45)
    began = time.time()
    V._OPAQUE_VISIBILITY.clear()
    items = draw._tower_items(lit=None, tint=None, types=False, capital=False, podium=False, where=lambda s: s.tower == 0 and s.storey < 4, core=False, ground=False, segments=2,
                              capsules={}, cores={}, capitals={}, base=V.CAPSULE, towers=(0,), storeys=None, view=view, coverage_padding=padding)
    print(f"[count] scale={scale:.4f} padding={padding:.4f} kept={len(items)} t={time.time() - began:.0f}s", flush=True)
