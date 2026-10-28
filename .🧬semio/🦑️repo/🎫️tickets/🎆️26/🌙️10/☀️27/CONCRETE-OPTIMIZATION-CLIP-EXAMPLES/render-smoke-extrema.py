"""🚬 Play the extrema scene for real (no skipped animations) at a tiny size to prove every transition runs.

Usage: python render-smoke-extrema.py <en|de>
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
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / "🗑️generated/fonts")
OUTPUT = TICKET / "🗑️generated/extrema/smoke"
OUTPUT.mkdir(parents=True, exist_ok=True)
manim = importlib.import_module("manim")
scene_class = getattr(importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.📉️extrema.🐍️"), "Beat1_LocalGlobalExtrema")
started = time.time()
with manim.tempconfig({"pixel_width": 320, "pixel_height": 180, "frame_rate": 3, "write_to_movie": False, "save_last_frame": False,
                       "disable_caching": True, "media_dir": str(OUTPUT / "media"), "verbosity": "ERROR", "progress_bar": "none"}):
    scene = scene_class()
    try:
        scene.render()
    except Exception as error:
        print(f"[DEBUG] render ended with {type(error).__name__}: {str(error)[:120]}", flush=True)
print(f"[DEBUG] smoke render {language} finished in {time.time() - started:.0f}s, scene time {scene.renderer.time:.1f}s", flush=True)
