"""Skipped-animation dry run of Beat2_References under a profile, then work counters: python smoke.py <preview|final> [sampled]."""

import importlib
import json
import os
import sys
import time
import faulthandler
faulthandler.dump_traceback_later(300, repeat=True, file=open(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"] + "/stack.log", "a"))

profile = sys.argv[1]
sampled = len(sys.argv) > 2
os.environ["SEMIO_TEACHING_PROFILE"] = profile
manim = importlib.import_module("manim")
instrument = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.📊️instrument.🐍️")
module = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.📍️references.🐍️")
options = {"write_to_movie": False, "disable_caching": True, "verbosity": "WARNING", "save_last_frame": False, "progress_bar": "none"}
if sampled:
    options.update({"frame_rate": 15, "pixel_width": 96, "pixel_height": 54})
else:
    options["skip_animations"] = True
started = time.time()
with manim.tempconfig(options):
    scene = module.Beat2_References(skip_animations=not sampled)
    scene.render()
snapshot = instrument.snapshot()
print(f"OK {profile} {time.time() - started:.1f}s transitions={len(scene.morphs.played)}")
print(json.dumps(snapshot))
