import importlib, os, sys
from pathlib import Path
TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"; os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / "🗑️generated/fonts/probe4")
manim = importlib.import_module("manim")
visuals = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🐍️")
class Probe(visuals.ClipScene):
    def construct(self):
        for s in (0.04, 0.055, 0.08, 0.1):
            for d in ("architecture", "structure", "services"):
                dr = self.villa_axon((0, 0, 0), s, disciplines=(d,), ground=False)
                print("[DEBUG]", s, d, round(dr.width, 2), round(dr.height, 2), len(dr.get_family()), [round(float(x), 2) for x in dr.get_center()])
with manim.tempconfig({"pixel_width": 320, "pixel_height": 180, "frame_rate": 1, "write_to_movie": False, "save_last_frame": False, "disable_caching": True, "skip_animations": True, "media_dir": str(TICKET / "🗑️generated/measure"), "verbosity": "ERROR"}):
    Probe().render()
