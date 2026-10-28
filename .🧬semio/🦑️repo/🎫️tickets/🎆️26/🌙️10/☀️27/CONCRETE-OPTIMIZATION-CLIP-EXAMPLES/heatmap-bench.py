"""[DEBUG] Benchmark the optimization heat map: build time and ms/frame at 1280x720 (columns=True, as the scenes use it).

Usage: python heatmap-bench.py [frames] [width height]; writes 🗑️generated/toolkit/heatmap-bench.png
"""
import importlib, os, sys, time
from pathlib import Path
sys.stdout.reconfigure(encoding="utf-8")
HERE = Path(__file__).parent
ROOT = next(p for p in HERE.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(HERE / "🗑️generated/toolkit/fonts")
import manim
v = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🎨️visuals.🐍️")
FRAMES = int(sys.argv[1]) if len(sys.argv) > 1 else 20
WIDTH, HEIGHT = (int(sys.argv[2]), int(sys.argv[3])) if len(sys.argv) > 3 else (1280, 720)


class S(v.ClipScene):
    def construct(self):
        axes, labels = self.plot_axes(x_range=(10, 80, 10), y_range=(0, 1.2, .3), x_label="x", y_label="y", point=(-3, 0, 0), width=6.2, height=3.4)
        started = time.time()
        heat = self.heat_map(axes, colour=self.ramp(), opacity=.9, levels=4, legend=True)
        print(f"[DEBUG] build {(time.time() - started) * 1000:.0f} ms; family {len(heat['image'].get_family())}", flush=True)
        self.add(axes, labels, heat["image"], heat["contours"], heat["star"], heat["legend"])
        for _ in range(3):
            self.renderer.update_frame(self, mobjects=self.mobjects, ignore_skipping=True)
        started = time.time()
        for _ in range(FRAMES):
            self.renderer.update_frame(self, mobjects=self.mobjects, ignore_skipping=True)
        print(f"[DEBUG] {(time.time() - started) / FRAMES * 1000:.0f} ms/frame at {WIDTH}x{HEIGHT} ({FRAMES} frames)", flush=True)
        self.renderer.camera.get_image().save(HERE / "🗑️generated/toolkit/heatmap-bench.png")


with manim.tempconfig({"pixel_width": WIDTH, "pixel_height": HEIGHT, "frame_rate": 1, "write_to_movie": False, "save_last_frame": False, "disable_caching": True,
                       "skip_animations": True, "media_dir": str(HERE / "🗑️generated/toolkit/media"), "verbosity": "ERROR"}):
    S().render()
