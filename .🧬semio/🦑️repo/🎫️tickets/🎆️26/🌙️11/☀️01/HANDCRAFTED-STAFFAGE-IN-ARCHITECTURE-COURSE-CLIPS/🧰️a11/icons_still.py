"""[DEBUG] A11: still of the new front icons (window, radiator, cooler) on the clip background. Run with r3_run.sh."""
import importlib
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = next(p for p in HERE.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
import manim
import numpy as np

extrema = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.📉️extrema.🐍️")
art = extrema._art


class Icons(manim.Scene):
    def construct(self):
        self.camera.background_color = "#101418"
        window = art.staffage("optimization-experiments-window", unit=1.4 / 1.25, anchor=np.array([-4.5, -1.0, 0.0]), color=extrema.INK)
        radiator = art.staffage("optimization-extrema-radiator", unit=1.2 / art.staffage_height("optimization-extrema-radiator"), anchor=np.array([-1.5, -1.0, 0.0]), color=extrema.HEAT)
        cooler = art.staffage("optimization-extrema-cooler", unit=1.6, anchor=np.array([1.8, 0.0, 0.0]), color=extrema.COOL)
        self.add(window, radiator, cooler)


out = HERE / "🗑️generated" / "icons"
with manim.tempconfig({"pixel_width": 1600, "pixel_height": 900, "media_dir": str(out), "verbosity": "WARNING", "disable_caching": True, "save_last_frame": True, "write_to_movie": False}):
    Icons().render()
print("[DEBUG] done", [str(p) for p in out.rglob("*.png")])
