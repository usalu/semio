import importlib
import os
import sys
from pathlib import Path

sys.stdout.reconfigure(encoding="utf-8")
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
HERE = Path(__file__).resolve().parent
ROOT = next(p for p in HERE.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
v = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🎨️visuals.🐍️")
manim = importlib.import_module("manim")
OUT = HERE.parent / "🗑️generated/gh-trees"
OUT.mkdir(parents=True, exist_ok=True)


class S(v.ClipScene):
    def construct(self):
        v._fonts.apply_scene_style(self)
        configs = [dict(storeys=8), dict(storeys=7, core=True), dict(core=False)]
        for number, cfg in enumerate(configs):
            g = self.tower_axon(at=(0, 0, 0), scale=0.3, view=(235, 24), lit=[(0, 1, 4, 2)], towers=(1,), where=lambda slot: 3 <= slot.storey <= 5, capital=False, podium=False, types=False, **cfg)
            self.add(g)
            self.renderer.update_frame(self, mobjects=self.mobjects, ignore_skipping=True)
            self.renderer.camera.get_image().save(OUT / f"view-{number}.png")
            self.remove(g)
            print("[DEBUG]", cfg, g.width, g.height)


with manim.tempconfig({"pixel_width": 1280, "pixel_height": 720, "frame_rate": 1, "write_to_movie": False, "disable_caching": True, "skip_animations": True, "verbosity": "ERROR"}):
    S().render()
