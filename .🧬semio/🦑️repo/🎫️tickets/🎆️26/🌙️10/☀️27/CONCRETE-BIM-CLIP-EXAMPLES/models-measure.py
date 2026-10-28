import importlib
import os
import sys
import time
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / "🗑️generated/fonts/measure")
manim = importlib.import_module("manim")
visuals = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🐍️")
M = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")


class Probe(visuals.ClipScene):
    def construct(self):
        for scale in (0.16, 0.2):
            t = time.time()
            d = self.villa_axon((0, 0, 0), scale, colors="discipline")
            print("[DEBUG] scale", scale, "w", round(d.width, 2), "h", round(d.height, 2), "secs", round(time.time() - t, 2), "n", len(d.get_family()))
        for disc in ("architecture", "structure", "services"):
            d = self.villa_axon((0, 0, 0), 0.16, disciplines=(disc,))
            print("[DEBUG]", disc, round(d.width, 2), round(d.height, 2), sorted(d.parts)[:80])
        d = self.villa_axon((0, 0, 0), 0.11, disciplines=("architecture",), explode=1.0)
        print("[DEBUG] explode arch", round(d.width, 2), round(d.height, 2), d.get_center())
        for name in ("architecture", "structure", "services"):
            info = M.partial_model(name)
            print("[DEBUG]", name, info["element_tags"])
        print("[DEBUG] models exist", [n for n in ("translate_structure", "clash_check", "tributary") if hasattr(M, n)])


with manim.tempconfig({"pixel_width": 320, "pixel_height": 180, "frame_rate": 1, "write_to_movie": False, "save_last_frame": False,
                       "disable_caching": True, "skip_animations": True, "media_dir": str(TICKET / "🗑️generated/measure"), "verbosity": "ERROR"}):
    Probe().render()
