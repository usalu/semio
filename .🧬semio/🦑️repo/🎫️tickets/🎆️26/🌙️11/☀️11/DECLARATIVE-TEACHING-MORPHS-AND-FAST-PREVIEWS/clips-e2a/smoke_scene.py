"""Run one ventilation scene with skipped animations under a profile: python smoke_scene.py <clip> <Class> <preview|final> [fps-sampled]."""

import importlib
import os
import sys
import time

clip, scene_name, profile = sys.argv[1:4]
sampled = len(sys.argv) > 4
os.environ["SEMIO_TEACHING_PROFILE"] = profile
manim = importlib.import_module("manim")
module = importlib.import_module(f"🎓️teaching.🏛️architecture.⚡️energy.🌬️ventilation.🎬️clip.{clip}.🐍️")
scene_class = getattr(module, scene_name)
options = {"write_to_movie": False, "disable_caching": True, "verbosity": "WARNING", "save_last_frame": False, "progress_bar": "none"}
if sampled:
    options.update({"frame_rate": 15, "pixel_width": 96, "pixel_height": 54})
else:
    options["skip_animations"] = True
started = time.time()
with manim.tempconfig(options):
    scene = scene_class(skip_animations=not sampled)
    scene.render()
print(f"OK {clip} {scene_name} {profile} {time.time() - started:.1f}s")
