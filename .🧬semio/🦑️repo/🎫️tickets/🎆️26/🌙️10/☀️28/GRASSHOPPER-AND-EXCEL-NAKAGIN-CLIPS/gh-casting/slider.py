"""Probe: slider widths for several width arguments."""
import importlib
import os
import sys
from pathlib import Path

TICKET = Path(__file__).resolve().parents[1]
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / "🗑️generated/fonts/casting-widths")
manim = importlib.import_module("manim")
visuals = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🎨️visuals.🐍️")


class Probe(visuals.ClipScene):
    NARRATION = [["a", "a", "a"]]

    def construct(self):
        visuals._fonts.apply_scene_style(self)
        for width in (None, 1.7, 2.6, 4.0):
            slider = self.gh_slider("Storeys", 10.6, 0, 14, decimals=1, width=width)
            print("[DEBUG]", width, round(slider.width, 2), round(slider.body.width, 2), flush=True)


with manim.tempconfig({"pixel_width": 1280, "pixel_height": 720, "frame_rate": 1, "write_to_movie": False, "save_last_frame": False,
                       "disable_caching": True, "skip_animations": True, "media_dir": str(TICKET / "🗑️generated/casting-probe/media"), "verbosity": "ERROR"}):
    Probe().render()
