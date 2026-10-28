"""📸 Render the axonometric boards as 1280x720 stills with a layout report.

Usage: python axon-showcase.py <en|de> [board ...]
Output: 🗑️generated/axon/<board>-<language>.png and axon-<language>-layout.md
"""
import importlib
import os
import sys
import time
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
language, *only = sys.argv[1:]
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_LANGUAGE"] = language
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / f"🗑️generated/fonts/axon-{language}")
OUTPUT = TICKET / "🗑️generated/axon"
OUTPUT.mkdir(parents=True, exist_ok=True)
manim = importlib.import_module("manim")
art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
visuals = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🐍️")
axon = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧊️axon.🐍️")
report = []
BOARDS = {}
CAPTIONS = {}


def board(caption=("A caption of two lines at most stays inside the band", "Eine Bildunterschrift mit höchstens zwei Zeilen bleibt im Band")):
    def wrap(function):
        BOARDS[function.__name__.replace("_", "-")] = function
        CAPTIONS[function.__name__.replace("_", "-")] = caption
        return function
    return wrap


@board()
def plain(s):
    return [s.villa_axon((0, -0.1, 0), 0.17, model=model.load_model())]


@board()
def plain_large(s):
    return [s.villa_axon((0, -0.35, 0), 0.215)]


@board()
def plain_grid(s):
    return [s.villa_axon((0, -0.1, 0), 0.17, grid=True)]


@board()
def highlight_b3(s):
    return [s.villa_axon((-3.3, -0.1, 0), 0.17, highlight=("B3",), labels=True), s.villa_axon((3.5, -0.1, 0), 0.17, highlight=("IfcColumn",))]


@board()
def highlight_row_e(s):
    return [s.villa_axon((0, -0.1, 0), 0.17, highlight=("E1", "E2", "E3", "E4", "E5", "B3"), labels=True)]


@board()
def disciplines(s):
    return [s.villa_axon((-3.9, 0.5, 0), 0.115, disciplines=("architecture",)), s.villa_axon((0.2, 0.5, 0), 0.115, disciplines=("structure",)),
            s.villa_axon((4.3, 0.5, 0), 0.115, disciplines=("services",))]


@board()
def exploded(s):
    return [s.villa_axon((-1.0, 0.0, 0), 0.115, explode=1.0, grid=True)]


@board()
def exploded_half(s):
    return [s.villa_axon((-2.4, 0.0, 0), 0.12, explode=0.5), s.villa_axon((3.8, 0.0, 0), 0.085, explode=1.0)]


@board()
def ghosted(s):
    lost = ("RW-S", "RW-E", "RW-N", "RW-W", "W-S", "W-E", "W-N", "W-W")
    return [s.villa_axon((-3.7, 0.6, 0), 0.15, disciplines=("architecture", "structure"), highlight=lost), s.villa_axon((3.7, 0.6, 0), 0.15, disciplines=("architecture", "structure"), ghost=lost)]


@board()
def window_highlight(s):
    return [s.villa_axon((0, -0.1, 0), 0.17, highlight=("W-S", "RW-S"))]


@board()
def roof_slab(s):
    return [s.villa_axon((0, -0.1, 0), 0.17, highlight=("SL-R",)), ]


@board()
def element_wall(s):
    return [s.element_axon("W-S", (-3.3, 0.9, 0), 0.19, extrude=1.0, opening=1.0, window=1.0), s.element_axon("W-S", (3.3, 0.9, 0), 0.19, extrude=1.0, opening=0.6, window=0.4)]


@board()
def element_growth(s):
    return [s.element_axon("W-S", (-4.2, 0.2, 0), 0.12, extrude=0.5, opening=0.0, window=0.0), s.element_axon("W-S", (0.0, 0.2, 0), 0.12, extrude=1.0, opening=0.7, window=0.0),
            s.element_axon("W-S", (4.2, 0.2, 0), 0.12, extrude=1.0, opening=1.0, window=1.0)]


@board()
def element_piloti(s):
    return [s.element_axon("B3", (-3.0, 0.2, 0), 0.5, extrude=1.0), s.element_axon("B3", (2.0, 0.2, 0), 0.5, extrude=0.45)]


@board()
def element_roof(s):
    return [s.element_axon("SL-R", (-3.4, 0.1, 0), 0.15, opening=0.0), s.element_axon("SL-R", (3.4, 0.1, 0), 0.15, opening=1.0)]


@board()
def element_misc(s):
    return [s.element_axon("SL-1", (-4.4, 0.8, 0), 0.12), s.element_axon("RM-1", (0.0, 0.5, 0), 0.22), s.element_axon("CW-1", (4.2, 0.8, 0), 0.2)]


@board()
def views(s):
    return [s.villa_axon((-3.6, 1.1, 0), 0.085, view="se"), s.villa_axon((3.6, 1.1, 0), 0.085, view="ne"), s.villa_axon((-3.6, -1.4, 0), 0.085, view="nw"),
            s.villa_axon((3.6, -1.4, 0), 0.085, view="sw")]


@board()
def debug_floor(s):
    return [s.villa_axon((-3.4, 0.0, 0), 0.15, highlight=("SL-1",)), s.villa_axon((3.5, 0.0, 0), 0.15, highlight=("SP-TERRACE",))]


@board()
def shell(s):
    return [s.villa_axon((-3.4, -0.1, 0), 0.15, shell=0.25), s.villa_axon((3.5, -0.1, 0), 0.15, highlight=("D3",), shell=0.12)]


def render():
    class Showcase(visuals.ClipScene):
        TITLE = {"en": "Axonometric showcase", "de": "Axonometrie-Vorführung"}
        NARRATION = [("board", "Axonometric board", "Axonometrie-Tafel")]

        def construct(self):
            fonts.apply_scene_style(self)
            names = only or list(BOARDS)
            for name in names:
                started = time.time()
                title = fonts.scene_title(f"{self.TITLE[language]} · {name}")
                caption = art.caption_bar(self.tr(*CAPTIONS[name]))
                begun = time.time()
                mobs = BOARDS[name](self)
                built = time.time() - begun
                self.add(title, caption, *mobs)
                self.renderer.static_image = None
                self.renderer.update_frame(self, mobjects=self.mobjects, ignore_skipping=True)
                self.renderer.get_frame()
                self.renderer.camera.get_image().save(OUTPUT / f"{name}-{language}.png")
                conflicts = art.layout_conflicts(self)
                count = sum(len(m.get_family()) for m in mobs)
                extent = [(round(m.get_left()[0], 2), round(m.get_right()[0], 2), round(m.get_bottom()[1], 2), round(m.get_top()[1], 2)) for m in mobs]
                report.append(f"## {name}\n\n- build {built * 1000:.0f} ms, {count} mobjects, extents {extent}\n" + ("\n".join("- " + c for c in conflicts) if conflicts else "- layout: none") + "\n")
                print(f"[DEBUG] {name} {language} build {built * 1000:.0f} ms total {time.time() - started:.1f}s {count} mobjects {len(conflicts)} conflicts", flush=True)
                self.clear()

    with manim.tempconfig({"pixel_width": 1280, "pixel_height": 720, "frame_rate": 1, "write_to_movie": False, "save_last_frame": False,
                           "disable_caching": True, "skip_animations": True, "media_dir": str(OUTPUT / f"media-{language}"), "verbosity": "ERROR"}):
        Showcase().render()


render()
(OUTPUT / f"axon-{language}-layout.md").write_text(f"# Layout axon {language}\n\n" + "\n".join(report), encoding="utf-8")
