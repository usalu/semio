"""🏠 M7: still frames of the optimization room section (winter, summer) and the icon rows, straight from the clip code."""
import importlib
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = next(p for p in HERE.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
import manim

room = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🎨️visuals.🏠️room.🐍️")
extrema = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.📉️extrema.🐍️")


class Winter(room.RoomVisuals, manim.Scene):
    def language(self):
        return "en"

    def tr(self, en, de):
        return en

    def construct(self):
        self.camera.background_color = "#101418"
        trackers = self.study_trackers()
        self.add(self.room_section(trackers, (-3.2, 0.2, 0), 0.9, "winter", labels=False, demand=True, person=True))
        self.add(self.room_section(trackers, (3.2, 0.2, 0), 0.9, "summer", labels=False, demand=True, person=True))


class Icons(extrema.Beat1_Basics if hasattr(extrema, "Beat1_Basics") else manim.Scene):
    pass


out = HERE / "🗑️generated" / "m7opt"
with manim.tempconfig({"pixel_width": 1600, "pixel_height": 900, "media_dir": str(out), "verbosity": "WARNING", "disable_caching": True, "save_last_frame": True, "write_to_movie": False}):
    Winter().render()
print("[DEBUG] done", list(out.rglob("*.png")))
