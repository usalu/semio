"""🔬 Render widget probes of the objects scene into the ticket: python scene-objects-probe.py <name>."""
import importlib
import os
import sys
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
name = sys.argv[1]
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / "🗑️generated/fonts/probe")
OUT = TICKET / "🗑️generated/objects-probe"
OUT.mkdir(parents=True, exist_ok=True)
manim = importlib.import_module("manim")
visuals = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🐍️")
fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")


class Probe(visuals.ClipScene):
    def construct(self):
        fonts.apply_scene_style(self)
        m = model.load_model()
        if name == "elevation":
            d = self.villa_elevation((2.6, 0.5, 0), 0.34, model=m, highlight=("W-S",))
            self.add(d)
        elif name == "plan":
            d = self.villa_plan("ground", (2.35, -0.1, 0), 0.19, model=m, highlight=("B3",), layers=("structure", "architecture"))
            self.add(d)
        elif name == "axon":
            d = self.villa_axon((2.35, 0.1, 0), 0.18, model=m, highlight=("B3",), ghost=("walls", "windows", "screens", "slabs"), disciplines=("architecture", "structure"))
            self.add(d)
        elif name == "axon-structure":
            d = self.villa_axon((2.35, 0.1, 0), 0.18, model=m, disciplines=("structure",), shell=0.22)
            self.add(d)
        elif name == "axon-a":
            self.add(self.villa_axon((2.35, 0.1, 0), 0.18, model=m, highlight=("B3",), shell=0.4, disciplines=("architecture", "structure")))
        elif name == "axon-b":
            self.add(self.villa_axon((2.35, 0.1, 0), 0.18, model=m, highlight=("B3",), ghost=("walls", "windows", "screens"), shell=0.5, disciplines=("architecture", "structure")))
        elif name == "piloti":
            d = self.piloti_detail((0, -0.1, 0), 0.8, model=m)
            self.add(d)
        self.wait(0.1)


with manim.tempconfig({"pixel_width": 1280, "pixel_height": 720, "frame_rate": 1, "write_to_movie": False, "save_last_frame": True,
                       "disable_caching": True, "media_dir": str(OUT / "media"), "verbosity": "ERROR", "output_file": name}):
    scene = Probe()
    scene.render()
    scene.renderer.update_frame(scene, mobjects=scene.mobjects, ignore_skipping=True)
    scene.renderer.camera.get_image().save(OUT / f"{name}.png")
print("[DEBUG] probe", name, flush=True)
