# /// script
# requires-python = ">=3.9"
# dependencies = ["pillow==11.3.0"]
# ///

import json
import math
from pathlib import Path

from PIL import Image

fixture = json.loads(
    (Path(__file__).parent / "🧫️fixtures" / "rotated-proxy.json").read_text(encoding="utf-8")
)
source = fixture["source"]
image = Image.frombytes("RGBA", (source["width"], source["height"]), bytes(source["rgba"]))
frame = fixture["frameBounds"]
rotation = -math.degrees(frame["rotation"])
rotated = image.rotate(rotation, expand=True)
assert {
    "width": rotated.width,
    "height": rotated.height,
    "rgba": list(rotated.tobytes()),
} == fixture["expected"]

frame_size = int(frame["w"]), int(frame["h"])
bounds = Image.new("1", frame_size, 1).rotate(rotation, expand=True)
assert {
    "x": frame["x"] + (frame["w"] - bounds.width) / 2,
    "y": frame["y"] + (frame["h"] - bounds.height) / 2,
    "width": bounds.width,
    "height": bounds.height,
} == fixture["expectedBounds"]
