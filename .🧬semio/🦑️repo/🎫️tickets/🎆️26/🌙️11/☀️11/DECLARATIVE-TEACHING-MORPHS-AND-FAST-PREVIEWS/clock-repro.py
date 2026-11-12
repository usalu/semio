"""Reproduce per-play frame drift in stock Manim (run with the teaching venv; pass `clock` to import the canonical clock first)."""
import importlib
import json
import sys

from manim import Mobject, Scene, tempconfig

if "clock" in sys.argv:
    importlib.import_module("🎓️teaching.🧰️interop.🕐️clock.🐍️")

plays = [("play", 0.5, False), ("wait", 0.5, False), ("wait", 0.5, True), ("play", 0.6, False), ("wait", 0.2, True)]
written = []
with tempconfig({"dry_run": True, "pixel_width": 320, "pixel_height": 180, "frame_rate": 15}):
    scene = Scene()
    mob = Mobject()
    mob.add_updater(lambda m: None)
    scene.add(mob)
    counter = [0]

    def write_frame(frame, repeat=1):
        counter[0] += repeat

    scene.renderer.file_writer.write_frame = write_frame
    for kind, duration, frozen in plays:
        before = counter[0]
        if kind == "play":
            scene.play(mob.animate.shift([1, 0, 0]), run_time=duration)
        else:
            scene.wait(duration, frozen_frame=frozen)
        written.append({"kind": kind, "duration": duration, "frozen": frozen, "written": counter[0] - before, "time": scene.renderer.time})
print(json.dumps(written))
