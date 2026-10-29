import importlib, os, sys
from pathlib import Path
ROOT = Path("C:/git/semio"); sys.path.insert(0, str(ROOT))
T = ROOT / ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS"
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"; os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(T / "🗑️generated/excel-energy/fonts")
manim = importlib.import_module("manim")
v = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🎨️visuals.🐍️")
n = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")
OUT = T / "🗑️generated/excel-energy"; OUT.mkdir(parents=True, exist_ok=True)

class P(v.ClipScene):
    def construct(self):
        v._fonts.apply_scene_style(self)
        t = n.capsule_type("A")
        a = self.capsule_axon(t, at=(-3.2, 0, 0), scale=0.7, view="axon", dims=("width", "length", "height"), highlight={"top": v.WARN, "right": v.GOOD})
        b = self.capsule_axon(t, at=(3.2, 0, 0), scale=0.7, view="axon_back", bolts=True, hatch=("attached",), highlight={"left": v.WARN, "top": v.WARN})
        self.add(a, b)
        for d, face in ((a, (0, 2, 1.25)), (a, (0, 4, 1.25)), (b, (0, 0, 1.25)), (b, (-1.25, 2, 1.25))):
            self.add(manim.Dot(d.at(*face), radius=0.06, color=v.SERV))
        self.renderer.static_image = None
        self.renderer.update_frame(self, mobjects=self.mobjects, ignore_skipping=True)
        self.renderer.get_frame()
        self.renderer.camera.get_image().save(OUT / "probe2.png")

with manim.tempconfig({"pixel_width": 1280, "pixel_height": 720, "frame_rate": 1, "write_to_movie": False, "save_last_frame": False, "disable_caching": True, "skip_animations": True, "media_dir": str(OUT / "media"), "verbosity": "ERROR"}):
    P().render()
print("ok")
