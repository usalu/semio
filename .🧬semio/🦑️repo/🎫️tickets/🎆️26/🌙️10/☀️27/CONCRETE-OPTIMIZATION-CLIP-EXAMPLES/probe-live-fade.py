"""🔬 Probe which animations accept the toolkit's live widgets: FadeIn alone, in a group, FadeOut after clear_updaters."""
import importlib
import os
import sys
import traceback
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / "🗑️generated/fonts")
manim = importlib.import_module("manim")
visuals = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🎨️visuals.🐍️")


class Probe(visuals.ClipScene):
    def construct(self):
        visuals._art.set_vo_language("en")
        visuals._fonts.apply_scene_style(self)
        dials = self.study_trackers({"orientation": 90.0})
        cases = {
            "fadein-alone": lambda: self.play(manim.FadeIn(self.plan(dials, (-3, 0, 0), .2, arcs=True)), run_time=.5),
            "fadein-in-group": lambda: self.play(manim.AnimationGroup(manim.FadeIn(self.plan(dials, (3, 0, 0), .2)), dials["orientation"].animate.set_value(180)), run_time=.5),
            "fadein-plus-tracker": lambda: self.play(manim.FadeIn(self.energy_bars(dials, (0, -2, 0), 3)), dials["orientation"].animate.set_value(270), run_time=.5),
            "fadein-with-run-time-kw": lambda: self.play(manim.AnimationGroup(manim.FadeIn(self.plan(dials, (0, 2, 0), .2), run_time=.3), dials["orientation"].animate.set_value(300).set_run_time(.6)), run_time=.6),
        }
        for name, run in cases.items():
            try:
                run()
                print("[DEBUG] ok  ", name, flush=True)
            except Exception as error:
                print("[DEBUG] FAIL", name, type(error).__name__, error, flush=True)
                traceback.print_exc(limit=-3)
                return


with manim.tempconfig({"pixel_width": 320, "pixel_height": 180, "frame_rate": 5, "write_to_movie": False, "disable_caching": True,
                       "media_dir": str(TICKET / "🗑️generated/extrema/probe"), "verbosity": "ERROR", "progress_bar": "none"}):
    Probe().render()
