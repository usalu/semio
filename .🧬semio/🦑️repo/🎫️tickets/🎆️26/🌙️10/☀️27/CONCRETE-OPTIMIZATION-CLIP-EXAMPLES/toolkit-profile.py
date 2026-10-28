"""⏱️ Per-frame cost of every live widget (rebuild) and of a full 320 × 180 frame with all of them on screen."""
import importlib, os, sys, time
from pathlib import Path
sys.stdout.reconfigure(encoding="utf-8")
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(Path(__file__).parent / "🗑️generated/fonts")
import manim
v = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🎨️visuals.🐍️")


class S(v.ClipScene):
    def construct(self):
        t = self.study_trackers({"orientation": 100.0})
        axes = self.axes(x_range=(90, 450, 90), y_range=(500, 1200, 100), point=(-3, 0, 0), width=4, height=2)
        widgets = {"section": self.room_section(t, (0, 0, 0), .4, "summer"), "bars": self.energy_bars(t, (4, -2, 0), 3, 800), "plan": self.plan(t, (-4, 1, 0), .2, arcs=True),
                   "facade": self.facade(t, (0, 2, 0), .4), "marker": self.live_marker(axes, t, "orientation", "electricity")}
        for name, w in widgets.items():
            for k in range(3):
                t["orientation"].set_value(100 + k)
                w.update(0)
            t0 = time.time()
            for k in range(10):
                t["orientation"].set_value(110 + k * 7)
                w.update(0)
            print("[DEBUG] rebuild", name, round((time.time() - t0) / 10 * 1000), "ms", flush=True)
        self.add(*widgets.values())
        t0 = time.time()
        for k in range(10):
            t["orientation"].set_value(200 + k * 5)
            self.update_mobjects(0)
        print("[DEBUG] all rebuilds", round((time.time() - t0) / 10 * 1000), "ms", flush=True)
        t0 = time.time()
        for k in range(10):
            self.renderer.update_frame(self, mobjects=self.mobjects, ignore_skipping=True)
        print("[DEBUG] render 320x180", round((time.time() - t0) / 10 * 1000), "ms", flush=True)


with manim.tempconfig({"pixel_width": 320, "pixel_height": 180, "frame_rate": 5, "write_to_movie": False, "save_last_frame": False, "disable_caching": True, "skip_animations": True, "media_dir": str(Path(__file__).parent / "🗑️generated/toolkit/media"), "verbosity": "ERROR", "progress_bar": "none"}):
    S().render()
