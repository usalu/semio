"""Probe: does Scene.remove drop a window after animations on it?"""
import importlib
import os
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
TICKET = HERE.parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.stdout.reconfigure(encoding="utf-8")
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / "🗑️generated/fonts/probe3")
manim = importlib.import_module("manim")
visuals = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🎨️visuals.🐍️")
model = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🧱️model.🐍️")


class Probe(visuals.ClipScene):
    NARRATION = [["a", "a", "a"]]

    def construct(self):
        visuals._fonts.apply_scene_style(self)
        wb = model.workbook()
        w1 = self.xl_window(wb, "Type A", cols="C:G", rows=(11, 16))
        self.add(w1)
        print("[DEBUG] after add", len(self.mobjects), flush=True)
        self.play(w1.select("G12"), run_time=0.01)
        print("[DEBUG] after select", len(self.mobjects), [type(m).__name__ for m in self.mobjects][:12], flush=True)
        self.remove(w1)
        print("[DEBUG] after remove", len(self.mobjects), [type(m).__name__ for m in self.mobjects][:12], flush=True)


with manim.tempconfig({"pixel_width": 640, "pixel_height": 360, "frame_rate": 1, "write_to_movie": False, "save_last_frame": False, "disable_caching": True, "skip_animations": True,
                       "media_dir": str(TICKET / "🗑️generated/probe3"), "verbosity": "ERROR"}):
    Probe().render()
