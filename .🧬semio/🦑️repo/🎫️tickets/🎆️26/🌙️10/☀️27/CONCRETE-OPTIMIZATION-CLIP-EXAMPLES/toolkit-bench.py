"""[DEBUG] Measure heat map build and frame cost."""
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
        axes, labels = self.plot_axes(x_range=(10, 80, 10), y_range=(0, 1.2, .3), x_label="x", y_label="y", point=(-3, 0, 0), width=6, height=3.4)
        t0 = time.time()
        heat = self.heat_map(axes, colour=self.ramp(), levels=4, legend=True, columns=False)
        print("[DEBUG] build", round(time.time() - t0, 2), "s; mobjects", len(heat["image"].get_family()))
        self.add(axes, labels, heat["image"], heat["contours"], heat["star"], heat["legend"])
        for k in range(3):
            self.renderer.update_frame(self, mobjects=self.mobjects, ignore_skipping=True)
        t0 = time.time()
        for k in range(5):
            self.renderer.update_frame(self, mobjects=self.mobjects, ignore_skipping=True)
        print("[DEBUG] frame", round((time.time() - t0) / 5 * 1000), "ms")
        self.renderer.camera.get_image().save(Path(__file__).parent / "🗑️generated/toolkit/bench.png")


with manim.tempconfig({"pixel_width": 1920, "pixel_height": 1080, "frame_rate": 1, "write_to_movie": False, "save_last_frame": False, "disable_caching": True, "skip_animations": True, "media_dir": str(Path(__file__).parent / "🗑️generated/toolkit/media"), "verbosity": "ERROR"}):
    S().render()
