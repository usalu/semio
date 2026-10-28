"""[DEBUG] Real low-res playback of fade, transform and indicate on live widgets while trackers move."""
import importlib, os, sys, time, traceback
from pathlib import Path
sys.stdout.reconfigure(encoding="utf-8")
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(Path(__file__).parent / "🗑️generated/fonts")
import manim
v = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🎨️visuals.🐍️")
only = sys.argv[1:]


class S(v.ClipScene):
    def construct(self):
        t = self.study_trackers({"orientation": 90.0})
        axes = self.axes(point=(-3, 0, 0), width=4, height=2)
        makers = {
            "section-winter": lambda: self.room_section(t, (0, 0, 0), .4, "winter"),
            "section-summer": lambda: self.room_section(t, (0, 0, 0), .4, "summer"),
            "bars": lambda: self.energy_bars(t, (0, -2, 0), 3, 800),
            "plan-arcs": lambda: self.plan(t, (0, 0, 0), .2, arcs=True),
            "facade": lambda: self.facade(t, (0, 0, 0), .4),
            "marker": lambda: self.live_marker(axes, t, "orientation", "electricity"),
        }
        for name, make in makers.items():
            if only and name not in only:
                continue
            w = make()
            second = make()
            steps = [
                ("fadein+move", lambda: self.play(manim.FadeIn(w), self.set_design(t, {"orientation": 180, "wwr": .7, "overhang": 1.0}, 1.0))),
                ("indicate+move", lambda: self.play(manim.Indicate(w), self.set_design(t, {"orientation": 270, "wwr": .3, "overhang": 0.2}, 1.0))),
                ("fadeout+move", lambda: self.play(manim.FadeOut(w), self.set_design(t, {"orientation": 360, "wwr": .5, "overhang": 0.6}, 1.0))),
                ("fadein-again+move", lambda: self.play(manim.FadeIn(w), self.set_design(t, {"orientation": 90, "wwr": .2, "overhang": 0.0}, 1.0))),
                ("transform+move", lambda: self.play(manim.Transform(w, make()), self.set_design(t, {"orientation": 200, "wwr": .6, "overhang": .9}, 1.0))),
                ("replace+move", lambda: self.play(manim.ReplacementTransform(w, second), self.set_design(t, {"orientation": 120, "wwr": .4, "overhang": .5}, 1.0))),
                ("fadeout-end", lambda: self.play(manim.FadeOut(second), run_time=.5)),
            ]
            for label, run in steps:
                t0 = time.time()
                try:
                    run()
                    print("[DEBUG] ok  ", name, label, round(time.time() - t0, 1), "s", flush=True)
                except Exception as error:
                    print("[DEBUG] FAIL", name, label, type(error).__name__, str(error)[:70], flush=True)
                    traceback.print_exc(limit=-2)
                    return


with manim.tempconfig({"pixel_width": 320, "pixel_height": 180, "frame_rate": 5, "write_to_movie": False, "save_last_frame": False, "disable_caching": True, "skip_animations": False, "media_dir": str(Path(__file__).parent / "🗑️generated/toolkit/media"), "verbosity": "ERROR", "progress_bar": "none"}):
    S().render()
