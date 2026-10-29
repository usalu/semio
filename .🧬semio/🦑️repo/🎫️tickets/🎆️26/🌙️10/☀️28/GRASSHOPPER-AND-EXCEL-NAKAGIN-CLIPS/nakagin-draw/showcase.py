"""📸 Showcase of the Nakagin drawings: renders still PNGs 1280x720 into 🗑️generated/nakagin-draw/.

Usage: SEMIO_TEACHING_LANGUAGE=en python showcase.py [name ...]
"""
import importlib
import os
import sys
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
TICKET = HERE.parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
language = os.environ.setdefault("SEMIO_TEACHING_LANGUAGE", "en")
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / "🗑️generated/fonts/nakagin-draw")
OUTPUT = TICKET / "🗑️generated/nakagin-draw"
OUTPUT.mkdir(parents=True, exist_ok=True)
only = sys.argv[1:]

manim = importlib.import_module("manim")
art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
bim = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🐍️")
draw = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")
model = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")
np = importlib.import_module("numpy")


class Showcase(bim.ClipScene, draw.NakaginDrawing):
    """🖼️ One scene that renders every showcase still."""

    def snap(self, name, *mobjects):
        self.add(*mobjects)
        self.renderer.static_image = None
        self.renderer.update_frame(self, mobjects=self.mobjects, ignore_skipping=True)
        self.renderer.get_frame()
        self.renderer.camera.get_image().save(OUTPUT / f"{name}-{language}.png")
        self.remove(*mobjects)

    def title(self, text):
        return self.text(text, (0, 3.35, 0), size=26)

    def construct(self):
        art.set_vo_language(self.language())
        fonts.apply_scene_style(self)
        self._caption = None
        cases = {k[5:]: getattr(self, k) for k in dir(self) if k.startswith("show_")}
        for name, fn in cases.items():
            if only and name not in only:
                continue
            start = time.perf_counter()
            fn(name)
            print(f"[DEBUG] {name} {time.perf_counter() - start:.2f}s", flush=True)

    def show_capsule(self, name):
        t = model.capsule_types()[0]
        cap = self.capsule_axon(t, at=(0.3, -0.1, 0), scale=0.95, dims=("width", "length", "height", "window"))
        self.snap(name, self.title(self.tr("One capsule with dimensions", "Eine Kapsel mit Massen")), cap)

    def show_capsule_back(self, name):
        t = model.capsule_types()[0]
        cap = self.capsule_axon(t, at=(0.3, -0.1, 0), scale=0.95, view="axon_back", bolts=True, hatch=("attached",), dims=("width",))
        self.snap(name, self.title(self.tr("Back face, bolts", "Rueckwand, Bolzen")), cap)

    def show_hatch(self, name):
        t = model.capsule_types()[0]
        cap = self.capsule_axon(t, at=(0.3, -0.1, 0), scale=0.95, hatch=("exposed",), highlight=("front",))
        self.snap(name, self.title(self.tr("Exposed envelope", "Aussenhuelle")), cap)

    def show_views(self, name):
        t = model.capsule_types()[0]
        views = self.capsule_views(t, at=(0, -0.15, 0), scale=0.5)
        self.snap(name, self.title(self.tr("Capsule plan, front, side", "Kapsel Grundriss, Front, Seite")), views)

    def show_types(self, name):
        group = manim.VGroup()
        for index, t in enumerate(model.capsule_types()):
            group.add(self.capsule_axon(t, at=(-5.1 + index * 3.4, -0.2, 0), scale=0.5, base=draw._blend(draw.CAPSULE, draw.type_color(t.key), 0.5), dims=("window",), dim_offset=0.3))
            group.add(self.text(f"{self.tr('Type', 'Typ')} {t.key}", (-5.1 + index * 3.4, -1.9, 0), size=22, color=draw.type_color(t.key)))
        self.snap(name, self.title(self.tr("Four capsule types", "Vier Kapseltypen")), group)

    def show_axes(self, name):
        slots = [s for s in model.slots() if s.tower == 0 and s.storey == 1 and s.facade == 1]
        scene = self.tower_axon(at=(-0.3, -0.15, 0), scale=0.3, towers=(0,), storeys=4, podium=False, capital=False,
                                where=lambda s: s.tower == 0 and s.storey in (0, 1) and s.facade in (0, 1))
        group = manim.VGroup(scene)
        for slot in slots:
            group.add(self.plane_axes(slot.plane, at=scene.at(*slot.plane.origin), scale=0.3, length=1.6))
        self.snap(name, self.title(self.tr("Planes on a core", "Ebenen auf einem Kern")), group)

    def show_tower(self, name):
        tower = self.tower_axon(at=(0.2, 0.3, 0), scale=0.125, lit=[(0, 1, 5, 0)])
        self.snap(name, self.title(self.tr("The Nakagin Capsule Tower, types and a lit facade", "Der Nakagin Capsule Tower, Typen und eine Fassade")), tower)

    def show_tower_close(self, name):
        tower = self.tower_axon(at=(0, 0.3, 0), scale=0.3, towers=(0,), podium=False, view="axon_left")
        self.snap(name, self.title("close"), tower)

    def show_tower_plain(self, name):
        tower = self.tower_axon(at=(0.2, 0.3, 0), scale=0.125, types=False)
        self.snap(name, self.title("plain"), tower)

    def show_elevation(self, name):
        tower = self.tower_elevation(at=(0.2, 0.25, 0), scale=0.125)
        self.snap(name, self.title(self.tr("Elevation", "Ansicht")), tower)


with manim.tempconfig({"pixel_width": 1280, "pixel_height": 720, "frame_rate": 1, "write_to_movie": False, "save_last_frame": False,
                       "disable_caching": True, "skip_animations": True, "media_dir": str(OUTPUT / "media"), "verbosity": "ERROR"}):
    Showcase().render()
