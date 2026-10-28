"""📸 Render toolkit boards as 1280x720 stills with a layout report.

Usage: python toolkit-showcase.py <en|de> [board ...]
Output: 🗑️generated/toolkit/<board>-<language>.png and toolkit-<language>-layout.md
"""
import importlib
import importlib.util
import os
import sys
import time

import numpy as np
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
language, *only = sys.argv[1:]
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_LANGUAGE"] = language
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / f"🗑️generated/fonts/toolkit-{language}")
OUTPUT = TICKET / "🗑️generated/toolkit"
OUTPUT.mkdir(parents=True, exist_ok=True)
manim = importlib.import_module("manim")
art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
spec = importlib.util.spec_from_file_location("toolkit_stub_model", TICKET / "toolkit-stub-model.py")
stub = importlib.util.module_from_spec(spec)
spec.loader.exec_module(stub)
stub.install(model, force=not (TICKET / "model-milestone-1.md").exists() or bool(os.environ.get("TOOLKIT_STUB")))
visuals = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🐍️")
report = []
BOARDS = {}


def board(function):
    BOARDS[function.__name__.replace("_", "-")] = function
    return function


@board
def axon_plain(s):
    return [s.villa_axon((0, -0.1, 0), 0.2)]


@board
def axon_highlight(s):
    return [s.villa_axon((-3.4, -0.1, 0), 0.15, highlight=("B3",)), s.villa_axon((3.4, -0.1, 0), 0.15, highlight=("IfcColumn",))]


@board
def axon_disciplines(s):
    return [s.villa_axon((0, -0.1, 0), 0.17, explode=1.0, grid=True)]


@board
def plan_ground(s):
    return [s.villa_plan("ground", (0, -0.1, 0), 0.25, north=True, scalebar=True)]


@board
def plan_first(s):
    return [s.villa_plan("first", (0, -0.1, 0), 0.25, north=True, scalebar=True)]


@board
def plan_roof(s):
    return [s.villa_plan("roof", (0, -0.1, 0), 0.25, north=True, scalebar=True)]


@board
def section(s):
    return [s.villa_section((0, -0.1, 0), 0.5)]


@board
def elevations(s):
    return [s.villa_elevation("south", (0, -0.1, 0), 0.5)]


@board
def piloti_detail(s):
    return [s.piloti_detail((-3.0, -0.2, 0), 0.62), s.piloti_detail((3.0, -0.2, 0), 0.62, grow=0.5)]


@board
def slab_strip(s):
    return [s.slab_strip((0, 0.0, 0), 0.62, supports="pinned"), s.slab_strip((0, -1.8, 0), 0.3, collapse=1.0)]


@board
def analysis_axon(s):
    return [s.analysis_view(model.translate_structure(), (-3.4, -0.1, 0), 0.15, projection="axon"), s.analysis_view(model.translate_structure(), (3.6, 0.0, 0), 0.22, projection="plan")]


@board
def tributary(s):
    return [s.tributary_view(model.tributary(), (-3.2, 0.0, 0), 0.22, projection="plan", reactions=True), s.tributary_view(model.tributary(), (3.4, -0.1, 0), 0.22, projection="axon")]


@board
def mesh(s):
    return [s.mesh_view(model.mesh("B3"), (-3.0, 0.0, 0), 0.75, view="axon"), s.mesh_view(model.mesh("B3"), (3.0, 0.0, 0), 0.75, view="top")]


@board
def load_arrows(s):
    points = [(-3.0 + 1.2 * i, -1.0, 0) for i in range(6)]
    return [s.load_arrows([40, 80, 120, 160, 200, 240], points, 0.012)]


@board
def class_tree(s):
    return [s.class_tree(model.class_chain("IfcColumn"), (0, 0.2, 0), highlight=("IfcColumn",), expand="IfcColumn")]


@board
def class_tree_leaves(s):
    chains = [model.class_chain(name) for name in ("IfcColumn", "IfcSlab", "IfcWall", "IfcWindow")]
    return [s.class_tree(chains, (-1.5, 0.3, 0), highlight=("IfcColumn",)), s.class_box("IfcColumn", ["GlobalId", "Name", "PredefinedType"], (4.7, 0.3, 0), grey=("GlobalId", "Name"))]


@board
def drafting(s):
    box = s.villa_axon if False else None
    frame = manim.Rectangle(width=3.0, height=1.6, stroke_color=art.P_WHITE, stroke_width=2).move_to((-3.4, 0.2, 0))
    items = [frame, s.dimension((-4.9, -0.6, 0), (-1.9, -0.6, 0), s.quantity(4.75, "m"), offset=-0.3),
             s.dimension((-4.9, -0.6, 0), (-4.9, 1.0, 0), s.quantity(3.5, "m"), offset=0.4),
             s.level_mark((-1.6, 1.0, 0), "+3.50"), s.level_mark((-1.6, -0.6, 0), "±0.00"),
             s.hatch([(-4.9, -0.6), (-1.9, -0.6), (-1.9, 1.0), (-4.9, 1.0)], art.P_WHITE, 0.12, 45),
             s.leader((-2.2, 1.35, 0), (-3.0, 0.5, 0), "B3", color=art.P_YELLOW), s.north_arrow((1.0, 1.7, 0)), s.scale_bar((0.5, 0.9, 0), 0.25, 10),
             s.badge(s.tr("assumed", "angenommen"), (2.5, -0.6, 0), color=art.P_ORANGE), s.badge("rev 3", (4.2, -0.6, 0), color=art.P_GREEN),
             s.label("Label", "Beschriftung", (4.5, 1.2, 0)), s.mono("#41=IFCCOLUMN(...);", (4.0, -0.1, 0))]
    return items


@board
def live_dimension(s):
    tracker = manim.ValueTracker(2.0)

    def build():
        width = tracker.get_value()
        start, end = np.array([-4.0, -0.4, 0]), np.array([-4.0 + width * 1.2, -0.4, 0])
        return manim.VGroup(manim.Rectangle(width=width * 1.2, height=1.2, stroke_color=art.P_WHITE).move_to((start + end) / 2 + (0, 0.6 + 0.4 - 0.4, 0)),
                            s.dimension(start, end, s.quantity(width, "m"), offset=-0.3))

    first = s.live(build, key=lambda: round(tracker.get_value(), 2))
    snapshot = manim.VGroup(*first.copy().submobjects).shift((0, 1.4, 0))
    tracker.set_value(3.4)
    first.refresh()
    return [snapshot, first, s.live_label(lambda: s.quantity(tracker.get_value(), "m", 2), (3.0, 0.5, 0), color=art.P_YELLOW)]


@board
def model_cards(s):
    return [s.model_card(name, (x, 0.0, 0), model=None) for name, x in (("architecture", -4.4), ("structure", 0.0), ("services", 4.4))]


@board
def property_table(s):
    rows = [("Pset_ColumnCommon.LoadBearing", "TRUE", "", "sourced"), ("Pset_ColumnCommon.IsExternal", "TRUE", "", "sourced"),
            ("NominalDiameter", 0.30, "m", "assumed"), ("Height", 3.25, "m", "assumed"), ("FireRating", "R60", "", "assumed")]
    return [s.property_table(rows, (0, 0.5, 0), title="Pset_ColumnCommon")]


@board
def step_panel(s):
    excerpt = model.step_excerpt("B3")
    wide = s.step_panel(excerpt, (0, 1.2, 0), width=12.4, only=[7, 10, 13, 14, 21, 23])
    narrow = s.step_panel(excerpt, (-3.0, -0.9, 0), width=6.6, only=[7, 14], highlight_roles=("guid", "class"))
    return [wide, narrow, s.guid_chip("B3", (3.6, -0.9, 0))]


def render():
    class Showcase(visuals.ClipScene):
        TITLE = {"en": "Toolkit showcase", "de": "Toolkit-Vorführung"}
        NARRATION = [("board", "Toolkit board", "Toolkit-Tafel")]

        def construct(self):
            fonts.apply_scene_style(self)
            names = only or list(BOARDS)
            for name in names:
                started = time.time()
                title = fonts.scene_title(f"{self.TITLE[language]} · {name}")
                caption = art.caption_bar(self.tr("A caption of two lines at most stays inside the band", "Eine Bildunterschrift mit höchstens zwei Zeilen bleibt im Band"))
                try:
                    mobs = BOARDS[name](self)
                except Exception as error:
                    report.append(f"## {name}\n\n- FAILED: {type(error).__name__}: {error}\n")
                    print(f"[DEBUG] {name} FAILED {type(error).__name__}: {error}", flush=True)
                    continue
                self.add(title, caption, *mobs)
                self.renderer.static_image = None
                self.renderer.update_frame(self, mobjects=self.mobjects, ignore_skipping=True)
                self.renderer.get_frame()
                self.renderer.camera.get_image().save(OUTPUT / f"{name}-{language}.png")
                conflicts = art.layout_conflicts(self)
                report.append(f"## {name}\n\n" + ("\n".join("- " + c for c in conflicts) if conflicts else "- none") + "\n")
                print(f"[DEBUG] {name} {language} {time.time() - started:.1f}s {len(conflicts)} conflicts", flush=True)
                self.clear()

    with manim.tempconfig({"pixel_width": 1280, "pixel_height": 720, "frame_rate": 1, "write_to_movie": False, "save_last_frame": False,
                           "disable_caching": True, "skip_animations": True, "media_dir": str(OUTPUT / f"media-{language}"), "verbosity": "ERROR"}):
        Showcase().render()



render()
(OUTPUT / f"toolkit-{language}-layout.md").write_text(f"# Layout toolkit {language}\n\n" + "\n".join(report), encoding="utf-8")
