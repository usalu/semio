"""🔤 Render the special glyphs the Pareto scene wants to use, with the clip's own label engines, into one PNG."""

import importlib
import os
import sys
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / "🗑️generated/fonts/glyph-probe")
OUTPUT = TICKET / "🗑️generated/pareto-r2"
OUTPUT.mkdir(parents=True, exist_ok=True)
manim = importlib.import_module("manim")
visuals = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🎨️visuals.🐍️")
room = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🎨️visuals.🏠️room.🐍️")
SAMPLES = ["64.9 kWh/a × 0.25 CHF/kWh = 16.2 CHF/a", "192 CHF ÷ 16.2 CHF/a ≈ 12 a", "12 a ≤ 30 a Δ 6.4 % → ≥ ·", "τ · A_w · θ / (A_s · (1 − ρ²)) = 2.4 %", "1 kWh/a = 18.2 CHF"]


class Probe(visuals.ClipScene):
    def construct(self):
        visuals._fonts.apply_scene_style(self)
        for index, sample in enumerate(SAMPLES):
            self.add(visuals._text(sample, (-3.2, 3 - index * .6, 0), size=20, width=6))
            self.add(room.text([sample], (3.2, 3 - index * .6, 0), size=20))


with manim.tempconfig({"pixel_width": 1280, "pixel_height": 720, "frame_rate": 1, "write_to_movie": False, "save_last_frame": False,
                       "disable_caching": True, "skip_animations": True, "media_dir": str(OUTPUT / "media"), "verbosity": "ERROR"}):
    scene = Probe()
    scene.render()
    scene.renderer.update_frame(scene, mobjects=scene.mobjects, ignore_skipping=True)
    scene.renderer.get_frame()
    scene.renderer.camera.get_image().save(OUTPUT / "glyph-probe.png")
print("[DEBUG] glyph probe saved")
