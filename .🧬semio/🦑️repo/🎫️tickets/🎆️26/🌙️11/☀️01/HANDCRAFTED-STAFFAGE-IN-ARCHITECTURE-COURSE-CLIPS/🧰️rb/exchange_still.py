"""🥞 rb: still frame of the exchange build-up beat (imposed-load people)."""
import importlib
import os
import sys
from pathlib import Path
from types import SimpleNamespace

HERE = Path(__file__).resolve().parent
ROOT = next(p for p in HERE.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_LANGUAGE"] = sys.argv[1] if len(sys.argv) > 1 else "en"
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(HERE.parent / "🗑️generated/fonts/rb")
import manim

ex = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🔌️exchange.🐍️")


class Still(ex.Beat5_Interfaces):
    def construct(self):
        ex._art.set_vo_language(self.language())
        ex._visuals._fonts.apply_scene_style(self)
        self._caption = None
        f = SimpleNamespace(analysis={"loads": {"self_weight": {"density": 25.0, "value": 5.0}, "finishes": {"value": 1.5}, "imposed": {"value": 2.0, "basis": "assumed"}}}, trib={"pilotis": [{"tag": ex.TAG, "area": 22.0}]}, slab={"top": 3.4, "thickness": 0.2}, b3={"x": 6.0, "y": 4.0, "radius": 0.15})
        self._s = SimpleNamespace(off=[])
        self._beat_buildup(f)


out = HERE.parent / "🗑️generated" / "rb" / "exchange"
with manim.tempconfig({"pixel_width": 1600, "pixel_height": 900, "media_dir": str(out), "verbosity": "WARNING", "disable_caching": True, "save_last_frame": True, "write_to_movie": False}):
    Still().render()
print("[DEBUG] done", list(out.rglob("*.png")))
