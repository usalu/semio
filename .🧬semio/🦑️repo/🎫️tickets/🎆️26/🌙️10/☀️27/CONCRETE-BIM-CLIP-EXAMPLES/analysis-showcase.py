"""📸 Render analysis-view boards as 1280x720 stills with a layout report.

Usage: python analysis-showcase.py <en|de> [board ...]
Output: 🗑️generated/analysis/<board>-<language>.png and analysis-<language>-layout.md
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
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / f"🗑️generated/fonts/analysis-{language}")
OUTPUT = TICKET / "🗑️generated/analysis"
OUTPUT.mkdir(parents=True, exist_ok=True)
manim = importlib.import_module("manim")
art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
visuals = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🐍️")
analysis_module = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🧮️analysis.🐍️")
report = []
BOARDS = {}
ROW_C = ("C1", "C2", "C3", "C4", "C5")


def board(function):
    BOARDS[function.__name__.replace("_", "-")] = function
    return function


@board
def analysis_plan(s):
    a = model.translate_structure()
    return [s.analysis_view(a, (-3.4, 0.1, 0), 0.26, highlight=("B3",), ids=("members",)),
            s.analysis_view(a, (3.4, 0.1, 0), 0.26, highlight=("SL-1",), ids=("shells",), nodes=False)]


@board
def analysis_axon(s):
    a = model.translate_structure()
    return [s.analysis_view(a, (-3.4, 0.1, 0), 0.17, projection="axon", highlight=("B3",), ids=("members",)),
            s.analysis_view(a, (3.4, 0.1, 0), 0.17, projection="axon", ids=("nodes",))]


@board
def analysis_reactions(s):
    a = model.translate_structure()
    t = model.tributary()
    row = {p["tag"]: p["reaction"] for p in t["pilotis"] if p["tag"] in ROW_C}
    return [s.analysis_view(a, (-3.4, 0.2, 0), 0.2, projection="axon", loads=row, highlight=ROW_C),
            s.analysis_view(a, (3.4, 0.2, 0), 0.2, projection="axon", loads=a["loads"]["area_load"])]


@board
def analysis_stale(s):
    a = model.translate_structure()
    t = model.tributary()
    row = {p["tag"]: p["reaction"] for p in t["pilotis"] if p["tag"] in ROW_C}
    new = {k: v * 1.3 for k, v in row.items()}
    return [s.analysis_view(a, (-3.4, 0.2, 0), 0.2, projection="axon", loads=new, highlight=ROW_C, revision=True),
            s.analysis_view(a, (3.4, 0.2, 0), 0.2, projection="axon", loads=row, stale=True, highlight=ROW_C)]


@board
def analysis_stale_plan(s):
    a = model.translate_structure()
    return [s.analysis_view(a, (-3.4, 0.2, 0), 0.26, loads={p["tag"]: p["reaction"] for p in model.tributary()["pilotis"][:6]}),
            s.analysis_view(a, (3.4, 0.2, 0), 0.26, stale=True, highlight=("B3",))]


@board
def loads_arrows(s):
    t = model.tributary()
    picks = [t["pilotis"][i] for i in (0, 1, 7, 8)]
    values = [p["reaction"] for p in picks]
    points = [(-4.5 + 2.2 * i, 1.0, 0) for i in range(4)]
    first = s.load_arrows(values, points, analysis_module.ARROW_KN, tags=[p["tag"] for p in picks])
    stacked = [s.load_arrows([v], [(4.0 + 1.1 * i, -0.3, 0)], 0.05, unit="kN", per="m²", anchor="tip", direction=(0, -1), colors=c)
               for i, (v, c) in enumerate(zip((3.5, 1.5, 4.75), (visuals.DATA, visuals.GOOD, visuals.WARN)))]
    caps = [s.text(p["tag"] + " · " + s.quantity(p["area"], "m²", 1), (-4.5 + 2.2 * i, 1.35, 0), size=16.2) for i, p in enumerate(picks)]
    return [first, *stacked, *caps]


@board
def tributary_plan(s):
    t = model.tributary()
    return [s.tributary_view(t, (-2.6, 0.3, 0), 0.27, highlight=("B3",)), s.tributary_legend((4.6, 2.2, 0), direction="column"),
            s.tributary_view(t, (4.6, -0.5, 0), 0.11, highlight=("corner",), labels=False)]


@board
def tributary_labels(s):
    t = model.tributary()
    return [s.tributary_view(t, (-2.0, 0.0, 0), 0.25, labels=("tag", "area", "reaction"), highlight=("B3", "A3", "A1")), s.tributary_legend((4.6, 1.2, 0), direction="column")]


@board
def tributary_axon(s):
    t = model.tributary()
    return [s.tributary_view(t, (-3.4, 0.3, 0), 0.2, projection="axon", highlight=ROW_C, reactions=ROW_C),
            s.tributary_view(t, (3.4, 0.3, 0), 0.2, projection="axon", reactions=True, labels=("tag",))]


@board
def tributary_roof(s):
    t = model.tributary(level="roof")
    return [s.tributary_view(t, (-3.4, 0.2, 0), 0.26, level="roof", labels=True), s.tributary_view(t, (3.4, 0.3, 0), 0.2, projection="axon", level="roof", highlight=("C3",), reactions=("C3",))]


@board
def mesh_views(s):
    m = model.mesh("B3", 8)
    n = model.mesh("B3", 16)
    return [s.mesh_view(m, (-4.6, 0.1, 0), 1.2, view="axon"), s.mesh_view(m, (-1.8, 0.1, 0), 1.2, view="side"), s.mesh_view(n, (1.2, 0.1, 0), 1.2, view="side"),
            s.mesh_view(m, (5.0, 1.0, 0), 6.0, view="top"), s.mesh_view(n, (5.0, -1.2, 0), 6.0, view="top")]


@board
def mesh_live(s):
    return [s.mesh_view("B3", (-4.6, 0.1, 0), 1.2, segments=6), s.mesh_view("B3", (-1.5, 0.1, 0), 1.2, segments=10), s.mesh_view("B3", (1.6, 0.1, 0), 1.2, segments=16, highlight=(0, 1, 2, 3)),
            s.mesh_view("B3", (5.0, 0.1, 0), 1.2, segments=24, view="side", vertices=False)]


def render():
    class Showcase(visuals.ClipScene):
        TITLE = {"en": "Analysis views", "de": "Analyse-Ansichten"}
        NARRATION = [("board", "Analysis board", "Analyse-Tafel")]

        def construct(self):
            fonts.apply_scene_style(self)
            names = only or list(BOARDS)
            for name in names:
                started = time.time()
                title = fonts.scene_title(f"{self.TITLE[language]} · {name}")
                caption = art.caption_bar(self.tr("A caption of two lines at most stays inside the band", "Eine Bildunterschrift mit höchstens zwei Zeilen bleibt im Band"))
                mobs = BOARDS[name](self)
                built = time.time() - started
                self.add(title, caption, *mobs)
                self.renderer.static_image = None
                self.renderer.update_frame(self, mobjects=self.mobjects, ignore_skipping=True)
                self.renderer.get_frame()
                self.renderer.camera.get_image().save(OUTPUT / f"{name}-{language}.png")
                conflicts = art.layout_conflicts(self)
                report.append(f"## {name}\n\n" + ("\n".join("- " + c for c in conflicts) if conflicts else "- none") + "\n")
                print(f"[DEBUG] {name} {language} build {built:.2f}s total {time.time() - started:.1f}s {len(conflicts)} conflicts", flush=True)
                self.clear()

    with manim.tempconfig({"pixel_width": 1280, "pixel_height": 720, "frame_rate": 1, "write_to_movie": False, "save_last_frame": False,
                           "disable_caching": True, "skip_animations": True, "media_dir": str(OUTPUT / f"media-{language}"), "verbosity": "ERROR"}):
        Showcase().render()


render()
(OUTPUT / f"analysis-{language}-layout.md").write_text(f"# Layout analysis {language}\n\n" + "\n".join(report), encoding="utf-8")
