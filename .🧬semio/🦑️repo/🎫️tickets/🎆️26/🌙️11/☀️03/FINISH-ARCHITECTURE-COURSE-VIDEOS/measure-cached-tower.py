"""\U0001f52c Measure cached painter-ordered vector facets against Cairo and Pillow."""
import time
import numpy as np
from PIL import Image
from manim import Camera, Square

def camera():
    return Camera(pixel_width=426, pixel_height=240, frame_width=14.222222, frame_height=8, background_opacity=0)

facets = []
for i in range(5000):
    shape = Square(side_length=.035, stroke_width=0, fill_opacity=1, fill_color=["#336699", "#77aa55", "#cc8866"][i % 3])
    shape.shift(np.array([(i % 100) * .045 - 2.25, (i // 100) * .045 - 1.1, 0]))
    facets.append(shape)

start = time.perf_counter()
first = camera()
first.capture_mobjects(facets)
capture = time.perf_counter() - start
rgba = np.array(first.get_image())
start = time.perf_counter()
for _ in range(21):
    Image.alpha_composite(Image.new("RGBA", (426, 240), (25, 30, 35, 255)), Image.fromarray(rgba))
cached = time.perf_counter() - start
start = time.perf_counter()
second = camera()
second.capture_mobjects(facets)
repeat = time.perf_counter() - start
assert np.array_equal(rgba, np.array(second.get_image()))
print({"facets": len(facets), "capture_seconds": capture, "repeat_seconds": repeat, "cached_21_frames_seconds": cached})
