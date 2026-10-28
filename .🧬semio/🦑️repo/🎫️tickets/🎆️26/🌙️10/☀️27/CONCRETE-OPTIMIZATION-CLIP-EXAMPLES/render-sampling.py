"""🎞️ Render the sampling scene for real at low quality to check animation mode, timing and live drawings.

Usage: python render-sampling.py <en|de>
Output: 🗑️generated/sampling/render/…mp4 and the measured duration on stdout.
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
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / "🗑️generated/sampling/fonts")
OUTPUT = TICKET / "🗑️generated/sampling/render"
OUTPUT.mkdir(parents=True, exist_ok=True)
manim = importlib.import_module("manim")
scene_class = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🎲️sampling.🐍️").Beat2_GridRandomSampling
started = time.time()
with manim.tempconfig({"pixel_width": 854, "pixel_height": 480, "frame_rate": 15, "write_to_movie": True, "disable_caching": True,
                       "media_dir": str(OUTPUT), "verbosity": "WARNING", "output_file": f"sampling-{language}"}):
    scene = scene_class()
    scene.render()
print(f"[DEBUG] rendered {language}: {scene.renderer.time:.1f} s of video in {time.time() - started:.0f} s", flush=True)
