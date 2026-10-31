"""🧪 Probe the legend tables of the anatomy scene: do the old number discs leave when the data legend replaces the tool legend?"""
import importlib
import os
import sys
from pathlib import Path

import numpy as np

TICKET = Path(__file__).resolve().parents[1]
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / "🗑️generated/fonts/probe-legend")
manim = importlib.import_module("manim")
anatomy = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🦴️anatomy.🐍️")
Scene = anatomy.Beat1_Anatomy

with manim.tempconfig({"pixel_width": 320, "pixel_height": 180, "frame_rate": 1, "write_to_movie": False, "save_last_frame": False, "disable_caching": True, "skip_animations": True, "verbosity": "ERROR"}):
    scene = Scene()
    table = scene._table(Scene.TOOL_TERMS, np.array([2.55, -0.05, 0.0]))
    rest = scene._rest(table)
    marks = manim.VGroup(*[scene._num(i + 1, (-3 + 0.7 * i, 1.0, 0.0)) for i in range(8)])
    scene.add(marks, rest)
    scene.play(scene._fly_discs(marks, table, range(1, 9)))
    discs = [row[0] for row in table]
    print("[DEBUG] after fly", [d in scene.mobjects for d in discs], len(scene.mobjects))
    data = scene._table(Scene.DATA_TERMS, np.array([2.4, -0.05, 0.0]))
    scene.play(manim.FadeOut(marks), manim.FadeOut(table), manim.FadeOut(rest), manim.LaggedStart(*[manim.FadeIn(r) for r in scene._rest(data)], lag_ratio=0.15), run_time=1.2)
    print("[DEBUG] after fadeout", [d in scene.mobjects for d in discs], len(scene.mobjects))
    print("[DEBUG] visible", [(round(float(d.get_center()[0]), 2), round(float(d.get_fill_opacity()), 2)) for d in (m for m in scene.mobjects)][:12])
