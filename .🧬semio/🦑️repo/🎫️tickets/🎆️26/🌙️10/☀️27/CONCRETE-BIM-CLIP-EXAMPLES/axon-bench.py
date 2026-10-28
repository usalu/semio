import importlib, sys, time
from pathlib import Path
import numpy as np
TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
d = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🐍️")
pts = [(0,0),(1,0),(1,1),(0,1)]
t = time.time()
for _ in range(500):
    d.polygon(pts, d.INK, 1.0, d.MUTED, 1.0, 1.0)
print("polygon ms each", (time.time()-t)*2)
import manim
t = time.time()
g = manim.VGroup()
for _ in range(500):
    g.add(d.polygon(pts, d.INK, 1.0, d.MUTED, 1.0, 1.0))
print("polygon+add ms each", (time.time()-t)*2)
t = time.time()
for _ in range(500):
    m = manim.VMobject(); m.set_points_as_corners(np.array([[0,0,0],[1,0,0],[1,1,0],[0,1,0],[0,0,0]], float))
print("raw", (time.time()-t)*2)
