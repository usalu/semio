import importlib, os, sys
from pathlib import Path
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"; os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
manim = importlib.import_module("manim")
v = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🎨️visuals.🐍️")
m = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🧱️model.🐍️")
class S(v.ClipScene):
    def construct(self):
        w = self.xl_window(m.workbook(), "Type A", rows=(1, 6))
        self.add(w)
        self.play(w.select("B3:B4"), run_time=0.1)
        order = list(self.get_mobject_family_members())
        print("tops", [type(x).__name__ for x in self.mobjects])
        print("tint in top level:", any(x is w.tint for x in self.mobjects), "tint idx", [i for i, x in enumerate(order) if x is w.tint], "text idx", [i for i, x in enumerate(order) if x is w.cell("B3").text][:1])
with manim.tempconfig({"pixel_width": 480, "pixel_height": 270, "frame_rate": 4, "write_to_movie": False, "disable_caching": True, "dry_run": True, "media_dir": "x", "verbosity": "ERROR"}):
    S().render()
