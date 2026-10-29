"""Probe: widths of the widgets of each casting view and the total layout width."""
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
casting = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🫗️casting.🐍️")
model = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🧱️model.🐍️")


class Probe(casting.Beat4_Casting):
    def construct(self):
        casting._visuals._fonts.apply_scene_style(self)
        self._d, self._cache, self._towers = model.canvas("casting"), {}, {}
        for view in self._d.views:
            sol = self._sol()
            group = self._view(view, sol)
            print("[DEBUG]", view, round(group.total, 2), {k: round(w.width, 2) for k, w in group.w.items()}, flush=True)


with manim.tempconfig({"pixel_width": 1280, "pixel_height": 720, "frame_rate": 1, "write_to_movie": False, "save_last_frame": False,
                       "disable_caching": True, "skip_animations": True, "media_dir": str(TICKET / "🗑️generated/casting-probe/media"), "verbosity": "ERROR"}):
    Probe().render()
