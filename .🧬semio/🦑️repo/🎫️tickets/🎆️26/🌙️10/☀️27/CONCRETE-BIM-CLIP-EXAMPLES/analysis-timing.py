"""⏱️ Time analysis widgets and smoke-test animations (always_redraw, Live, transforms).

Usage: python analysis-timing.py <en|de>
"""
import importlib
import os
import sys
import time
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
language = sys.argv[1]
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_LANGUAGE"] = language
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / f"🗑️generated/fonts/analysis-{language}")
OUTPUT = TICKET / "🗑️generated/analysis"
manim = importlib.import_module("manim")
fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
visuals = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🐍️")
ROW_C = ("C1", "C2", "C3", "C4", "C5")


def clock(label, function, repeat=5):
    function()
    started = time.time()
    for _ in range(repeat):
        function()
    print(f"[DEBUG] {label}: {(time.time() - started) / repeat * 1000:.0f} ms per build", flush=True)


def render():
    class Timing(visuals.ClipScene):
        TITLE = {"en": "Timing", "de": "Zeitmessung"}
        NARRATION = [("t", "t", "t")]

        def construct(self):
            fonts.apply_scene_style(self)
            a = model.translate_structure()
            t = model.tributary()
            row = {p["tag"]: p["reaction"] for p in t["pilotis"] if p["tag"] in ROW_C}
            clock("analysis_view plan", lambda: self.analysis_view(a, (0, 0, 0), 0.26))
            clock("analysis_view axon", lambda: self.analysis_view(a, (0, 0, 0), 0.2, projection="axon"))
            clock("analysis_view axon + row loads", lambda: self.analysis_view(a, (0, 0, 0), 0.2, projection="axon", loads=row, highlight=ROW_C))
            clock("analysis_view axon + q", lambda: self.analysis_view(a, (0, 0, 0), 0.2, projection="axon", loads=9.75))
            clock("tributary_view plan", lambda: self.tributary_view(t, (0, 0, 0), 0.26))
            clock("tributary_view axon", lambda: self.tributary_view(t, (0, 0, 0), 0.2, projection="axon", highlight=ROW_C, reactions=ROW_C))
            clock("load_arrows 5 (labels)", lambda: self.load_arrows(list(row.values()), [(i, 0, 0) for i in range(5)]))
            clock("load_arrows 5 (no labels)", lambda: self.load_arrows(list(row.values()), [(i, 0, 0) for i in range(5)], labels=False))
            clock("mesh_view axon 16", lambda: self.mesh_view("B3", (0, 0, 0), 1.2, segments=16))
            clock("mesh_view top 16", lambda: self.mesh_view("B3", (0, 0, 0), 6.0, segments=16, view="top"))

            q = manim.ValueTracker(0.0)
            seg = manim.ValueTracker(6.0)
            arrows = manim.always_redraw(lambda: self.load_arrows([q.get_value() * v for v in row.values()], [(-5 + 1.4 * i, 1.5, 0) for i in range(5)], labels=False))
            counts = manim.always_redraw(lambda: self.mesh_view("B3", (3.0, 0.0, 0), 1.2, segments=int(round(seg.get_value()))))
            frames = self.live(lambda: self.analysis_view(a, (-2.5, -0.3, 0), 0.17, projection="axon", loads={k: v * q.get_value() for k, v in row.items()}, highlight=ROW_C),
                               key=lambda: round(q.get_value(), 2))
            self.add(arrows, counts, frames)
            started = time.time()
            self.play(q.animate.set_value(1.0), seg.animate.set_value(16.0), run_time=2.0, rate_func=manim.linear)
            print(f"[DEBUG] 2 s animation with three live widgets: {time.time() - started:.1f} s wall", flush=True)
            plan = self.analysis_view(a, (0, 0, 0), 0.26)
            axon = self.analysis_view(a, (0, 0, 0), 0.26, projection="axon")
            self.clear()
            self.add(plan)
            started = time.time()
            self.play(manim.ReplacementTransform(plan, axon), run_time=1.0)
            self.play(manim.FadeOut(axon), run_time=0.5)
            tview = self.tributary_view(t, (0, 0, 0), 0.26)
            self.play(manim.FadeIn(tview), run_time=0.5)
            self.play(tview.parts["B3"].animate.scale(0.5, about_point=tview.at("B3")), run_time=0.5)
            print(f"[DEBUG] transform + fade + scale: {time.time() - started:.1f} s wall", flush=True)

    with manim.tempconfig({"pixel_width": 640, "pixel_height": 360, "frame_rate": 10, "write_to_movie": False, "save_last_frame": False,
                           "disable_caching": True, "media_dir": str(OUTPUT / f"media-timing-{language}"), "verbosity": "ERROR"}):
        Timing().render()


render()
