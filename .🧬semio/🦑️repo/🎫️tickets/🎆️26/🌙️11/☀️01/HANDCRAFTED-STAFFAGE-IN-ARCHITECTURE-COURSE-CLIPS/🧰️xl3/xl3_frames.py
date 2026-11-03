"""Quality-l frame comparison of bbox-padded vs hull-padded culling on a small tower variant. Usage: xl3_frames.py [storeys]"""
import importlib
import os
import sys
import time

import numpy as np
from manim import Camera, VGroup, tempconfig

sys.path.insert(0, os.getcwd())
V = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")
storeys = int(sys.argv[1]) if len(sys.argv) > 1 else 2
CONFIG = {"pixel_width": 854, "pixel_height": 480, "frame_width": 8 * 854 / 480, "frame_height": 8.0, "renderer": "cairo", "dry_run": True}
original = V._dilated


def boxed(points, padding):
    points = np.asarray(points, float)
    lower, upper = points.min(axis=0) - padding, points.max(axis=0) + padding
    return np.array([lower, [upper[0], lower[1]], upper, [lower[0], upper[1]]])


def render(dilated):
    V._dilated = dilated
    V._TOWERS.clear()
    V._OPAQUE_VISIBILITY.clear()
    V._OCCLUSION_ORDERS.clear()
    began = time.time()
    with tempconfig(CONFIG):
        fit = importlib.import_module("🎓️teaching.🏛️architecture.🧲️fit.🐍️").building_fit(5.4, 4.4, centre=(0, 0, 0), towers=(0,), podium=False, capital=False, core=False)
        drawing = V.NakaginDrawing().tower_axon(**fit, towers=(0,), where=lambda s: s.tower == 0 and s.storey < storeys, core=False, podium=False, capital=False)
        shapes = sum(1 for m in drawing.get_family() if len(m.points))
        camera = Camera(background_color=V.BACKGROUND)
        camera.capture_mobjects([drawing])
        pixels = camera.pixel_array.copy()
    print(f"[frames] shapes={shapes} t={time.time() - began:.0f}s", flush=True)
    return pixels, shapes


old, old_shapes = render(boxed)
new, new_shapes = render(original)
diff = np.abs(old.astype(int) - new.astype(int))
print(f"[frames] bbox shapes={old_shapes} hull shapes={new_shapes} differing pixels={int((diff.sum(axis=2) > 0).sum())} max={int(diff.max())} covered={int((old[..., :3] != np.array(old[0, 0, :3])).any(axis=2).sum())}", flush=True)
