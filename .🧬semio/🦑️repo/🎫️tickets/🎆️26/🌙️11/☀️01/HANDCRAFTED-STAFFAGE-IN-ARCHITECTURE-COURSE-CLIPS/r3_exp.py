"""R3: experiments room section."""
import importlib, os, sys
from pathlib import Path
HERE = Path(__file__).resolve().parent
ROOT = next(p for p in HERE.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(HERE / "🗑️generated/fonts/r3")
import manim
ex = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧪️experiments.🐍️")

class Exp(ex.Beat3_FactorialExperiments):
    def construct(self):
        self.camera.background_color = "#101418"
        self.add(self._merge(self._section({"overhang": 0.0}, (-3.5, -1.0, 0), 1.0, person=True, demand=True)[0]))
        self.add(self._merge(self._section({"overhang": 1.0}, (2.5, -1.0, 0), 1.0, person=True, demand=True)[0]))

out = HERE / "🗑️generated" / "r3exp"
with manim.tempconfig({"pixel_width": 1600, "pixel_height": 900, "media_dir": str(out), "verbosity": "WARNING", "disable_caching": True, "save_last_frame": True, "write_to_movie": False}):
    Exp().render()
print("[DEBUG] done", list(out.rglob("*.png")))
