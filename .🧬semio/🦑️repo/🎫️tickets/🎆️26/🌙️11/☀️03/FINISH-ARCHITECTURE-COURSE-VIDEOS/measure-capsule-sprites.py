"""⏱️ Benchmark 140 cropped sprites against equivalent vector facets."""
import importlib
import time
import sys
from pathlib import Path
sys.path.insert(0, str(next(p for p in Path(__file__).resolve().parents if (p / '🧰️framework').is_dir())))
import numpy as np
from manim import Camera, Square, VGroup

sprites = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🖼️sprites")
camera = Camera(pixel_width=426, pixel_height=240, frame_width=14.222222222222221, frame_height=8, background_opacity=1)
groups = []
for i in range(140):
    x, y = (i % 14) * .7 - 4.55, (i // 14) * .5 - 2.25
    parts = [Square(side_length=.025, stroke_width=0, fill_opacity=1, fill_color=["#4680a0", "#d88833"][j % 2]).shift([x + (j % 6) * .045, y + (j // 6) * .045, 0]) for j in range(30)]
    groups.append(VGroup(*parts))
start = time.perf_counter()
camera.capture_mobjects(groups)
vector = time.perf_counter() - start
expected = np.array(camera.get_image())
start = time.perf_counter()
images = [sprites.capture(group, camera) for group in groups]
capture = time.perf_counter() - start
start = time.perf_counter()
for _ in range(5):
    camera.reset()
    camera.capture_mobjects(images)
sprite = (time.perf_counter() - start) / 5
actual = np.array(camera.get_image())
error = np.abs(expected.astype(int) - actual.astype(int))
assert int(error.max()) <= 3
print({"capsules": len(groups), "facets": 4200, "vector_frame_seconds": vector, "sprite_capture_seconds": capture,
       "sprite_frame_seconds": sprite, "crop_bytes": sum(image.pixel_array.nbytes for image in images), "max_channel_error": int(error.max())})
