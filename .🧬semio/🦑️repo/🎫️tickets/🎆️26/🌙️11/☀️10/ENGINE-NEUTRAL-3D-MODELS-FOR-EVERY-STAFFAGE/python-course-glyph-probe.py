import importlib, sys
import numpy as np
sys.path.insert(0, "/Users/ueli/Documents/semio")
from manim import Camera, VGroup, ValueTracker, UpdateFromAlphaFunc, tempconfig
energy = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.⚡️energy.🐍️")
with tempconfig({"dry_run": True, "pixel_width": 320, "pixel_height": 180, "frame_rate": 6, "background_color": "#000000"}):
    scene = energy.Beat4_Energy(); scene.language = lambda: "en"
    tracker = ValueTracker(1)
    key = lambda: "1.0 m²" if tracker.get_value() < 2 else "123.45 kWh"
    display = scene._readout(key, (0, 0), size=20)
    scene.add(display)
    oracle = Camera(pixel_width=320, pixel_height=180, frame_width=scene.camera.frame_width, frame_height=scene.camera.frame_height, background_color="#000000", background_opacity=1)
    original = scene.renderer.update_frame
    def update(*args, **kwargs):
        result = original(*args, **kwargs)
        actual = np.asarray(scene.camera.get_image(), dtype=int).copy()
        oracle.reset(); oracle.capture_mobjects([VGroup(scene._say(key(), (0, 0), size=20, color=energy.DATA, anchor="c"))])
        expected = np.asarray(oracle.get_image(), dtype=int)
        difference = np.abs(actual - expected).max(axis=2)
        ys, xs = np.nonzero(difference > 2)
        print("[DEBUG]", key(), "max", difference.max(), "pixels>2", len(ys), "coverage pixels", int((expected.max(axis=2) > 0).sum()), "mean signed (actual-expected) at those", float((actual - expected).sum(axis=2)[ys, xs].mean()) if len(ys) else 0)
        return result
    scene.renderer.update_frame = update
    scene.play(UpdateFromAlphaFunc(tracker, lambda mob, alpha: mob.set_value(1 + 2 * alpha)), run_time=1)
