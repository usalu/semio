"""📸 Render orthographic drawing boards as 1280x720 stills with a layout report.

Usage: python ortho-showcase.py <en|de> [board ...]
Output: 🗑️generated/ortho/<board>-<language>.png and ortho-<language>-layout.md
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
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / f"🗑️generated/fonts/ortho-{language}")
OUTPUT = TICKET / "🗑️generated/ortho"
OUTPUT.mkdir(parents=True, exist_ok=True)
manim = importlib.import_module("manim")
art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
visuals = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🐍️")
report = []
BOARDS = {}


def board(function):
    BOARDS[function.__name__.replace("_", "-")] = function
    return function


@board
def plan_ground(s):
    return [s.villa_plan("ground", (0, -0.05, 0), 0.26, dims=("grid", "overall"), north=True, scalebar=True)]


@board
def plan_ground_lit(s):
    path = model.car_path()
    a, b = path["entry_pair"]
    return [s.villa_plan("ground", (-3.0, 0.0, 0), 0.23, highlight=(a, b), dims=("grid",)), s.villa_plan("ground", (3.6, 0.0, 0), 0.23, layers=("structure", "architecture", "services"), highlight=("B3",))]


@board
def plan_first(s):
    return [s.villa_plan("first", (0, -0.05, 0), 0.26, highlight=("W-S", "RW-S"), cuts=("salon",), layers=("structure", "architecture", "services"))]


@board
def plan_roof(s):
    return [s.villa_plan("roof", (0, -0.05, 0), 0.26, layers=("structure", "architecture", "services"))]


@board
def plan_thumbs(s):
    return [s.villa_plan(level, (x, 0.4, 0), 0.1, thumbnail=True) for level, x in (("ground", -4.5), ("first", 0.0), ("roof", 4.5))]


def render():
    class Showcase(visuals.ClipScene):
        TITLE = {"en": "Orthographic showcase", "de": "Orthografische Vorführung"}
        NARRATION = [("board", "Ortho board", "Ortho-Tafel")]

        def construct(self):
            fonts.apply_scene_style(self)
            names = only or list(BOARDS)
            for name in names:
                started = time.time()
                title = fonts.scene_title(f"{self.TITLE[language]} · {name}")
                caption = art.caption_bar(self.tr("A caption of two lines at most stays inside the band", "Eine Bildunterschrift mit höchstens zwei Zeilen bleibt im Band"))
                mobs = BOARDS[name](self)
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
(OUTPUT / f"ortho-{language}-layout.md").write_text(f"# Layout ortho {language}\n\n" + "\n".join(report), encoding="utf-8")
