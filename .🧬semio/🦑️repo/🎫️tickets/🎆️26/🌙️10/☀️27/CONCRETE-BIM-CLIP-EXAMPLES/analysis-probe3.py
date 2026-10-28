import importlib, os, sys
from pathlib import Path
import numpy as np
TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
manim = importlib.import_module("manim")
M = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
V = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🐍️")
class S(V.ClipScene):
    NARRATION = [("t", "t", "t")]
    def construct(self):
        a = M.translate_structure()
        ax = self.analysis_view(a, (0, 0, 0), 0.2, projection="axon")
        top = next(n["position"] for n in a["nodes"] if n["id"] == "N-B3"); foot = next(n["position"] for n in a["nodes"] if n["id"] == "F-B3")
        print("[DEBUG]", ax.at(*top), ax.at(*foot), top, foot, 0.2 * 3.375 * np.cos(np.radians(28)))
        t = M.tributary(level="roof"); f = M.tributary()
        print("[DEBUG] level", t["level"], f["level"])
        r = self.tributary_view(t, (0, 0, 0), 0.2, projection="axon"); q = self.tributary_view(f, (0, 0, 0), 0.2, projection="axon")
        print("[DEBUG]", r.at("C3"), q.at("C3"), 0.2 * (6.7 - 0.1 - 3.375) * np.cos(np.radians(28)))
with manim.tempconfig({"pixel_width": 640, "pixel_height": 360, "frame_rate": 1, "write_to_movie": False, "save_last_frame": False, "skip_animations": True, "verbosity": "ERROR", "media_dir": str(TICKET / "🗑️generated/analysis/media-probe")}):
    S().render()
