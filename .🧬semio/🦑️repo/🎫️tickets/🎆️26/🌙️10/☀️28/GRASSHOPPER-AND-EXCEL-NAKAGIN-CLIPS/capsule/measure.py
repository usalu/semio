import importlib
import os
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
manim = importlib.import_module("manim")
v = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🎨️visuals.🐍️")
fonts = v._fonts


class S(v.ClipScene):
    NARRATION = [["a", "x", "y"]]

    def construct(self):
        fonts.apply_scene_style(self)
        for w in (2.3, 3.0):
            s = self.gh_slider("Width", 2.5, 1.5, 3.5, at=(0, 0, 0), width=w)
            print("[DEBUG] width", w, s.width, s.body.width)


with manim.tempconfig({"pixel_width": 1280, "pixel_height": 720, "frame_rate": 1, "write_to_movie": False, "disable_caching": True, "skip_animations": True, "verbosity": "ERROR"}):
    S().render()
