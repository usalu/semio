"""🎞️ Verify Animate’s compiled demo with independent H.264 and PNG decoders."""

from fractions import Fraction
from pathlib import Path
import sys

import av
from PIL import Image


scene = Path(sys.argv[1])
size = (1280, 720)
window = (6, 26, 31, 255)
width, height = size
corners = [(0, 0), (width - 1, 0), (0, height - 1), (width - 1, height - 1)]

with Image.open(scene / "scene.png") as image:
    assert image.size == size
    pixels = image.convert("RGBA")
    assert all(pixels.getpixel(point) == window for point in corners)

with av.open(str(scene / "scene.mp4")) as movie:
    stream = movie.streams.video[0]
    assert stream.codec_context.name == "h264"
    assert (stream.width, stream.height) == size
    assert stream.duration * stream.time_base == Fraction(4, 15)
    times = []
    for frame in movie.decode(video=0):
        times.append(frame.pts * frame.time_base)
        pixels = frame.to_ndarray(format="rgba")
        assert all(tuple(pixels[y, x]) == window for x, y in corners)
    assert times == [Fraction(index, 15) for index in range(4)]

print("Verified four 1280 × 720 H.264 frames and the final PNG at the canonical window color.")
