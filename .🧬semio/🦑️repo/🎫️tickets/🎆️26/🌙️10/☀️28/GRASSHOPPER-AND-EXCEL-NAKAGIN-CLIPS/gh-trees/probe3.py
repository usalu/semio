import importlib
import os
import sys
import time
from pathlib import Path

sys.stdout.reconfigure(encoding="utf-8")
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
v = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🎨️visuals.🐍️")
manim = importlib.import_module("manim")


class S(v.ClipScene):
    def construct(self):
        t = time.time()
        for sc in (0.1, 0.14):
            g = self.tower_axon(at=(0, 0, 0), scale=sc, view="axon")
            print("[DEBUG]", sc, g.width, g.height, g.get_center(), time.time() - t)
        g2 = self.tower_axon(at=(0, 0, 0), scale=0.14, view="north")
        print("[DEBUG] north", g2.width, g2.height)
        print("[DEBUG] keys", len(g.capsules), list(g.capsules)[:3], g.at(0, 0, 0))
        print("[DEBUG] frame", manim.config.frame_width, manim.config.frame_height)


with manim.tempconfig({"pixel_width": 1280, "pixel_height": 720, "frame_rate": 1, "write_to_movie": False, "disable_caching": True, "skip_animations": True, "verbosity": "ERROR"}):
    S().render()
