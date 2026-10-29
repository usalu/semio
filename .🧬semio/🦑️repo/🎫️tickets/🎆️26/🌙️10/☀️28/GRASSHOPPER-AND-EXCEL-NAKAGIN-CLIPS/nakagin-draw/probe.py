"""🔬 Probe the drawings: build times, live redraw with trackers, German strings."""
import importlib
import os
import sys
import time
from pathlib import Path

TICKET = Path(__file__).resolve().parent.parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ.setdefault("SEMIO_TEACHING_LANGUAGE", "de")
bim = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🐍️")
draw = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")
model = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")
manim = importlib.import_module("manim")


class S(bim.ClipScene, draw.NakaginDrawing):
    pass


s = S()
width, length, height, window = (manim.ValueTracker(v) for v in (2.5, 4.0, 2.5, 1.3))
live = s.live(lambda: s.capsule_axon({"key": "A", "width": width.get_value(), "length": length.get_value(), "height": height.get_value(), "window_diameter": window.get_value()},
                                     dims=("width", "length", "height", "window"), hatch=("exposed",), bolts=True))
start = time.perf_counter()
for i in range(20):
    width.set_value(2.0 + i * 0.05)
    live.refresh()
print("[DEBUG] capsule live redraw ms", (time.perf_counter() - start) / 20 * 1000)
for kw in ({}, {"hatch": ("exposed",)}, {"bolts": True}, {"dims": ("width",)}, {"dims": ("width", "length", "height", "window")}):
    start = time.perf_counter()
    for i in range(10):
        s.capsule_axon({"key": "A", "width": 2.0 + i * 0.05}, **kw)
    print("[DEBUG] variant", list(kw), (time.perf_counter() - start) / 10 * 1000, "ms")
for _ in range(3):
    start = time.perf_counter()
    t = s.tower_axon(at=(0, 0, 0), scale=0.13, lit=[(0, 1, 5, 0)], tint={(0, 0, 3, 1, 0): "#ffffff"})
    print("[DEBUG] tower build s", time.perf_counter() - start, len(t.capsules), len(t.cores), len(t.capitals), len(t.submobjects))
start = time.perf_counter()
s.tower_elevation(at=(0, 0, 0), scale=0.13)
print("[DEBUG] elevation s", time.perf_counter() - start)
core = s.core_axon(model.towers()[0], storeys=7, side=6.0, storey_height=3.0)
print("[DEBUG] core ok", core.get_width(), s.capital_axon(1).get_width(), s.podium_axon().get_width())
v = s.capsule_views({"key": "D"})
print("[DEBUG] views dims", {k: list(g.dims) for k, g in v.parts.items()})
