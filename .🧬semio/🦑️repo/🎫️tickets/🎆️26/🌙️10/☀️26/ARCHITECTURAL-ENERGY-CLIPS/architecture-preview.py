"""🖼️ Render architectural primitives for visual inspection."""

import importlib
import json
from pathlib import Path

import numpy as np
from manim import Camera

visuals = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
root = Path(__file__).resolve().parent / "🗑️generated"
root.mkdir(parents=True, exist_ok=True)

house = visuals.house_section([-3.7, 1.0, 0])
room = visuals.room_section([2.8, 1.0, 0], w=5.0, h=2.5, window_side="both")
standing = visuals.person_glyph([-5, -2.65, 0], scale=3)
seated = visuals.seated_person_glyph([-2.5, -3.6, 0], scale=3.6)
heater = visuals.radiator([0.55, -2.9, 0]).scale(2.5)
casement = visuals.window_glyph(2.3, -3.5, -1.8, depth=0.9, wall=0.2)
casement["sash"].become(visuals.window_opening(casement))
tilt = visuals.window_glyph(4.5, -3.5, -1.8, depth=0.9, wall=0.2)
tilt["sash"].become(visuals.window_opening(tilt, tilt=True))
camera = Camera(pixel_width=1600, pixel_height=1000, frame_height=8.5, background_color=visuals.P_DEEP_DARK)
camera.capture_mobjects([house["group"], room["group"], standing, seated["group"], heater, casement["group"], tilt["group"]])
image = camera.get_image()
image.save(root / "architecture-primitives.png")
print(json.dumps({"preview": str(root / "architecture-primitives.png"), "size": image.size}))
