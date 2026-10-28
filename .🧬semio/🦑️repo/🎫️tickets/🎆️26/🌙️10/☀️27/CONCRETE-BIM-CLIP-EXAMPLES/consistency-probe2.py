"""[DEBUG] probe the structure of the arrow groups of tributary_view and analysis_view."""
import importlib
import os
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
manim = importlib.import_module("manim")
visuals = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🐍️")
m = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")


class Probe(visuals.ClipScene):
    def construct(self):
        base = m.load_model()
        trib = m.tributary(base, "roof")
        view = self.tributary_view(trib, (0, 0, 0), 0.15, projection="axon", highlight=("A3",), level="roof", reactions={"A3", "C3"}, arrow_scale=0.003, model=base)
        print("[DEBUG] arrows", type(view.arrows), len(view.arrows))
        for arrow in view.arrows:
            print("[DEBUG]  group", type(arrow).__name__, [(type(x).__name__, getattr(x, "text", None)) for x in arrow.submobjects])
        an = m.translate_structure(base)
        av = self.analysis_view(an, (0, 0, 0), 0.15, projection="axon", loads={"A3": 105.0}, stale=True, highlight=("A3",), load_scale=0.003, revision=False, model=base)
        print("[DEBUG] loads", type(av.loads), len(av.loads))
        for arrow in av.loads:
            print("[DEBUG]  group", type(arrow).__name__, [(type(x).__name__, getattr(x, "text", None)) for x in arrow.submobjects])


with manim.tempconfig({"pixel_width": 320, "pixel_height": 180, "write_to_movie": False, "save_last_frame": False, "disable_caching": True, "skip_animations": True, "verbosity": "ERROR"}):
    Probe().render()
