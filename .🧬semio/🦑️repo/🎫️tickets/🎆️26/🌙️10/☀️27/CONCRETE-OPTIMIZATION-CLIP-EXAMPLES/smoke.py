"""🚬 Play one optimization scene for real (no skipped animations) at a tiny size to prove every transition runs.

Usage: python smoke.py <topic-folder> <SceneClass> <en|de>
Exit code 1 and a traceback when any animation fails; prints wall time and scene duration.
"""
import importlib
import os
import sys
import time
import traceback
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
topic, name, language = sys.argv[1:4]
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_LANGUAGE"] = language
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / "🗑️generated/fonts")
OUTPUT = TICKET / "🗑️generated/smoke" / f"{name}-{language}"
OUTPUT.mkdir(parents=True, exist_ok=True)
manim = importlib.import_module("manim")
scene_class = getattr(importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip." + topic + ".🐍️"), name)
started = time.time()
with manim.tempconfig({"pixel_width": 320, "pixel_height": 180, "frame_rate": 3, "write_to_movie": False, "save_last_frame": False,
                       "disable_caching": True, "media_dir": str(OUTPUT / "media"), "verbosity": "ERROR", "progress_bar": "none"}):
    scene = scene_class()
    try:
        scene.render()
        failed = False
    except Exception:
        traceback.print_exc()
        failed = True
print(f"[DEBUG] smoke {name} {language} {'FAILED' if failed else 'ok'} in {time.time() - started:.0f}s, scene time {scene.renderer.time:.1f}s", flush=True)
sys.exit(1 if failed else 0)
