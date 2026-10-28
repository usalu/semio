import importlib
import os
import sys
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / "🗑️generated/fonts/board")
manim = importlib.import_module("manim")
visuals = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🐍️")
fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
OUT = TICKET / "🗑️generated/models"
OUT.mkdir(parents=True, exist_ok=True)


class Board(visuals.ClipScene):
    def construct(self):
        fonts.apply_scene_style(self)
        left = (-3.3, 0.2, 0)
        right = (3.3, 0.2, 0)
        a = self.villa_axon(left, 0.19, disciplines=("architecture",), colors="material")
        a.set_stroke(color=visuals.ARCH, width=1.0)
        s = self.villa_axon(left, 0.19, disciplines=("structure",))
        v = self.villa_axon(left, 0.19, disciplines=("services",))
        self.add(v, s, a)
        for name in ("architecture", "structure", "services"):
            pass
        b = self.villa_axon(right, 0.19, colors="material")
        self.add(b)
        self.renderer.static_image = None
        self.renderer.update_frame(self, mobjects=self.mobjects, ignore_skipping=True)
        self.renderer.get_frame()
        self.renderer.camera.get_image().save(OUT / "board.png")


with manim.tempconfig({"pixel_width": 1280, "pixel_height": 720, "frame_rate": 1, "write_to_movie": False, "save_last_frame": False,
                       "disable_caching": True, "skip_animations": True, "media_dir": str(OUT / "media"), "verbosity": "ERROR"}):
    Board().render()
