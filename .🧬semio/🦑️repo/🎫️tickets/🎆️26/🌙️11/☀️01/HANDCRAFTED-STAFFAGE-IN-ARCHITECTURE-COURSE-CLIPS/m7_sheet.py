"""🧍 M7: contact sheet of the M7 casts at metre scale on a dark ground (one PNG)."""
import importlib
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = next(p for p in HERE.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
import manim

st = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")
IDS = ["bim-information-car", "optimization-room-cooler", "optimization-room-lamp-on", "optimization-room-lamp-off"]


class Sheet(manim.Scene):
    def construct(self):
        self.camera.background_color = "#101418"
        for index, cast in enumerate(IDS):
            x = -3.0 + 2.2 * index
            self.add(st.staffage(cast, unit=1.4, anchor=(x, -1.5, 0), color="#FFB000" if index in (3, 6) else None))
            self.add(manim.Text(cast.replace("-", "\n", 2), font_size=10, color="#CCCCCC").move_to((x, -2.2, 0)))
        self.add(manim.Line((-7, -1.5, 0), (7, -1.5, 0), color="#555555"))


out = HERE / "🗑️generated" / "m7"
with manim.tempconfig({"pixel_width": 1600, "pixel_height": 600, "frame_height": 6.0, "frame_width": 16.0, "media_dir": str(out), "verbosity": "WARNING", "disable_caching": True, "save_last_frame": True, "write_to_movie": False}):
    Sheet().render()
print("[DEBUG] done", list(out.rglob("*.png")))
