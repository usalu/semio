"""🔬 Render toolkit widgets used by the objects clip as PNG probes: python objects-r2-widgets.py <name> [<name> ...]."""
import importlib
import os
import sys
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
names = sys.argv[1:]
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / "🗑️generated/fonts/widgets")
OUT = TICKET / "🗑️generated/objects-r2/widgets"
OUT.mkdir(parents=True, exist_ok=True)
manim = importlib.import_module("manim")
visuals = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🐍️")
fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")


class Probe(visuals.ClipScene):
    def snap(self, name, *mobjects):
        for mob in mobjects:
            self.add(mob)
        self.renderer.static_image = None
        self.renderer.update_frame(self, mobjects=self.mobjects, ignore_skipping=True)
        self.renderer.get_frame()
        self.renderer.camera.get_image().save(OUT / f"{name}.png")
        for mob in mobjects:
            self.remove(mob)

    def construct(self):
        fonts.apply_scene_style(self)
        m = model.load_model()
        for name in names:
            if name == "elev":
                self.snap(name, self.villa_elevation("south", (2.35, -0.1, 0), 0.3, model=m, highlight=("RW-S",), levels=False))
            elif name == "elev-b3":
                self.snap(name, self.villa_elevation("south", (2.35, -0.1, 0), 0.3, model=m, highlight=("B3", "B1"), levels=True))
            elif name == "wall0":
                self.snap(name, self.element_axon("W-S", (2.35, 0.0, 0), 0.3, model=m, extrude=1, opening=0, window=0))
            elif name == "wall1":
                self.snap(name, self.element_axon("W-S", (2.35, 0.0, 0), 0.3, model=m, extrude=1, opening=1, window=0))
            elif name == "wall2":
                self.snap(name, self.element_axon("W-S", (2.35, 0.0, 0), 0.3, model=m, extrude=1, opening=1, window=1))
            elif name == "wallhalf":
                self.snap(name, self.element_axon("W-S", (2.35, 0.0, 0), 0.3, model=m, extrude=0.5, opening=0.5, window=0.5))
            elif name == "col":
                self.snap(name, self.element_axon("B3", (2.35, 0.0, 0), 1.0, model=m, extrude=1))
            elif name == "col-half":
                self.snap(name, self.element_axon("B3", (2.35, 0.0, 0), 1.0, model=m, extrude=0.5))
            elif name == "axon-xray":
                self.snap(name, self.villa_axon((2.35, 0.1, 0), 0.18, model=m, highlight=("B3",), ghost=("walls", "windows", "screens"), shell=0.5, disciplines=("architecture", "structure")))
            elif name == "axon-struct":
                self.snap(name, self.villa_axon((2.35, 0.1, 0), 0.2, model=m, disciplines=("structure",), highlight=("IfcColumn",)))
            elif name == "axon-class":
                self.snap(name, self.villa_axon((2.35, 0.1, 0), 0.2, model=m, highlight=("IfcSlab",), colors="material", disciplines=("architecture", "structure")))
            elif name == "plan":
                self.snap(name, self.villa_plan("ground", (2.35, -0.1, 0), 0.19, model=m, highlight=("B3",), names=False, layers=("structure", "architecture"), dims=("grid",)))
            elif name == "section":
                self.snap(name, self.villa_section((2.35, -0.1, 0), 0.3, model=m, cut="row:3"))
            elif name == "piloti-detail":
                self.snap(name, self.piloti_detail((0, -0.1, 0), 0.8, model=m))


with manim.tempconfig({"pixel_width": 1280, "pixel_height": 720, "frame_rate": 1, "write_to_movie": False, "save_last_frame": False,
                       "disable_caching": True, "skip_animations": True, "media_dir": str(OUT / "media"), "verbosity": "ERROR"}):
    Probe().render()
print("[DEBUG] done", names)
