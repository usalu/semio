"""Probe: facade detail of Tower B west facade in the viewport, wrong capsule and plane axes."""
import importlib
import os
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
TICKET = HERE.parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / "🗑️generated/fonts/casting-probe")
OUT = TICKET / "🗑️generated/casting-probe"
OUT.mkdir(parents=True, exist_ok=True)
manim = importlib.import_module("manim")
visuals = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🎨️visuals.🐍️")
nk = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")
model = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")
np = importlib.import_module("numpy")
SCALE = float(os.environ.get("PROBE_SCALE", "0.5"))
VIEW = os.environ.get("PROBE_VIEW", "plan")


class Probe(visuals.ClipScene):
    NARRATION = [["a", "a", "a"]]

    def construct(self):
        visuals._fonts.apply_scene_style(self)
        self._caption = None
        port = self.gh_viewport(at=(3.5, 0.05), width=6.5, height=5.0)
        slot = next(s for s in model.slots() if (s.building, s.tower, s.storey, s.facade, s.item) == (0, 1, 3, 3, 0))
        tower = self.tower_axon(at=port.get_center(), scale=SCALE, view=VIEW, towers=(1,), podium=False, capital=False, storeys=1,
                                where=lambda s: (s.storey, s.facade) == (3, 3), lit=[(0, 1, 3, 3, 0)])
        v = tower.view
        axes = self.plane_axes(slot.plane, at=tower.at(*slot.plane.origin), scale=SCALE, view=v, length=2.0)
        wrong = nk._capsule_lite(nk._cap(model.capsule_type(slot.type)), model.Plane(origin=slot.plane.origin, x=(1, 0, 0), y=(0, 1, 0), z=(0, 0, 1)), v, visuals.WARN)
        wall = nk._shape([nk._screen(v, np.array([(-13.85, -2.75, 11.5), (-13.85, 2.75, 11.5), (-13.85, 2.75, 17.5), (-13.85, -2.75, 17.5)]))], nk.CAPITAL, 0.9, nk.INK, 1.0)
        wall.set_z_index(-1)
        self.add(port, wall, tower, wrong, axes)
        self.renderer.static_image = None
        self.renderer.update_frame(self, mobjects=self.mobjects, ignore_skipping=True)
        self.renderer.get_frame()
        self.renderer.camera.get_image().save(OUT / f"detail-{VIEW}-{SCALE}.png")
        print("[DEBUG] size", tower.width, tower.height)


with manim.tempconfig({"pixel_width": 1280, "pixel_height": 720, "frame_rate": 1, "write_to_movie": False, "save_last_frame": False,
                       "disable_caching": True, "skip_animations": True, "media_dir": str(OUT / "media"), "verbosity": "ERROR"}):
    Probe().render()
