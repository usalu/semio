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
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / f"🗑️generated/fonts/models-board-{language}")
OUTPUT = TICKET / "🗑️generated/models-board"
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
def plan_owner(s):
    return [s.villa_plan("ground", (-3.3, 0.1, 0), 0.2, layers=("structure", "architecture", "services"), colors="discipline", dims=("grid",)),
            s.villa_plan("first", (3.3, 0.1, 0), 0.2, layers=("structure", "architecture", "services"), colors="discipline", names=True)]


@board
def plan_three(s):
    return [s.villa_plan("ground", (-4.2, 0.4, 0), 0.17, layers=("architecture",), names=True, grid=False),
            s.villa_plan("first", (0.0, 0.4, 0), 0.17, layers=("architecture",), names=True, grid=False),
            s.villa_plan("roof", (4.2, 0.4, 0), 0.17, layers=("architecture",), names=True, grid=False)]


@board
def analysis_overlay(s):
    a = model.translate_structure()
    base = s.villa_plan("first", (-3.0, 0.1, 0), 0.2, layers=("structure",), labels=True, grid=True, dims=("grid", "overall"))
    top = s.analysis_view(a, (-3.0, 0.1, 0), 0.2, projection="plan")
    return [base, top, s.analysis_view(a, (3.6, 0.1, 0), 0.2, projection="plan", nodes=False, supports=False)]


@board
def wall_detail(s):
    return [s.wall_detail((-3.0, 0.0, 0), 0.9, tag="W-S"), s.villa_section((3.3, 0.1, 0), 0.2, cut="salon")]


@board
def sections(s):
    return [s.villa_section((-3.3, 0.1, 0), 0.2, cut="salon", layers=("structure", "architecture", "services"), colors="discipline", names=True),
            s.villa_elevation("south", (3.3, 0.1, 0), 0.2)]


@board
def spaces_hl(s):
    ids = ("SP-HALL", "SP-GARAGE", "SP-BOILER")
    return [s.villa_plan("ground", (-4.2, 0.4, 0), 0.17, layers=("architecture",), highlight=ids, grid=False),
            s.villa_plan("first", (0.0, 0.4, 0), 0.17, layers=("architecture",), highlight=("SP-SALON", "SP-TERRACE"), grid=False),
            s.villa_plan("roof", (4.2, 0.4, 0), 0.17, layers=("architecture",), highlight=("SP-SOLARIUM",), grid=False)]


@board
def section_row3(s):
    return [s.villa_section((-3.3, 0.4, 0), 0.3, cut=("x", 10.75), layers=("structure",), highlight=("C3",)),
            s.piloti_detail((3.3, 0.4, 0), 0.5, tag="C3")]


@board
def axon_parts(s):
    d = s.villa_axon((-3.3, 0.1, 0), 0.18, disciplines=("architecture",), colors="material")
    print("[DEBUG] parts", sorted(d.parts))
    e = s.element_axon("C3", (3.3, 0.1, 0), 0.4)
    return [d, e]


@board
def r3_plan(s):
    g = model.building_geometry()
    pl = s.villa_plan("ground", (-3.3, 0.05, 0), 0.2, layers=("structure", "architecture", "services"), dims=("grid",), names=True, scalebar=True, north=True)
    tags = [t for t in pl.parts if t[0].isupper()]
    return [pl, s.villa_plan("ground", (3.3, 0.05, 0), 0.2, layers=("structure", "architecture", "services"), names=True, scalebar=True, halo=0.05,
                             highlight=[t for t in pl.parts if t in model.partial_model("structure")["element_tags"]], highlight_colors={t: visuals.STRUCT for t in model.partial_model("structure")["element_tags"]})]


@board
def r3_axon(s):
    a = model.translate_structure()
    out = []
    for x, sc, ids in ((-3.4, 0.17, ("members",)), (3.4, 0.1, ("members",))):
        base = s.villa_axon((x, 0.3, 0), sc, disciplines=("structure",), grid=True, shell=0.25, ground=False)
        top = s.analysis_view(a, (x, 0.3, 0), sc, projection="axon", ids=ids)
        out += [base, top]
    return out


@board
def axon_disc(s):
    return [s.villa_axon((-3.3, 0.1, 0), 0.12, disciplines=("services",), ghost=("walls", "windows", "curtain", "ramp", "screens", "slabs", "spaces")),
            s.villa_axon((3.3, 0.1, 0), 0.12, disciplines=("structure",), ghost=("walls", "windows", "curtain", "ramp", "screens", "spaces"))]


def render():
    class Showcase(visuals.ClipScene):
        TITLE = {"en": "Models board", "de": "Modelle-Tafel"}
        NARRATION = [("board", "Board", "Tafel")]

        def construct(self):
            fonts.apply_scene_style(self)
            names = only or list(BOARDS)
            for name in names:
                started = time.time()
                title = self.text(name, (-5.5, 3.3, 0))
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
(OUTPUT / f"board-{language}-layout.md").write_text(f"# Layout board {language}\n\n" + "\n".join(report), encoding="utf-8")
