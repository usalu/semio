"""🖍️ Writes the repair ops of `chalkboard`: at 1:50 the ruler is a 7.5 mm board as the elevation draws it and no slender strip (its stroke stood up to 22 mm outside the tray in the plan); at 1:100 the end view draws the leaf where its body is (3.5 mm towards the room, bottom edge at 68 mm)."""
import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import write
write("chalkboard", [
    {"op": "solid-set", "scale": "50", "id": "ruler", "set": {"profile": [[0.00125, -0.035], [0.00125, 0.005], [0.00875, 0.005], [0.00875, -0.027], [0.00475, -0.035]]}},
    {"op": "set", "path": ["volumes", "50", "body", {"id": "ruler"}, "slender"]},
    {"op": "shape-set", "scale": "100", "view": "front", "id": "end-leaf-frame", "set": {"x": -0.0155, "y": 0.068, "h": 1.154}},
])
