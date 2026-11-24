"""💦️ Writes the repair ops of `used-water-play-table`: at 1:50 the plan draws the strap of the chute across the head of the trough (30 mm wide, 192 mm long in the volume; its ends stood 47 mm inside the two open corners of the drawing); at 1:100 the two sluice gates are plates of the 15 mm feature instead of 3 mm sheets, the thickness their 0.18 mm strokes draw (a 3 mm sheet is thinner than the raster of the guard at 1:100, so the raised gate stood 41 mm above the body)."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import write
write("used-water-play-table", [
    {"op": "shape-add", "scale": "50", "view": "plan", "after": "head-board", "shape": {"id": "chute-strap", "bone": "frame", "role": "body", "z": 4.5, "filled": True, "tone": 0.5, "kind": "rect", "x": 0.835, "y": -0.096, "w": 0.03, "h": 0.192, "region": "screw-steel", "material": "stainless-scaled"}},
    {"op": "solid-set", "scale": "100", "id": "gate-gate-1", "set": {"from": -0.0075, "to": 0.0075}},
    {"op": "solid-set", "scale": "100", "id": "gate-gate-2", "set": {"from": -0.0075, "to": 0.0075}},
    {"op": "set", "path": ["volumes", "100", "body", {"id": "gate-gate-1"}, "sheet"]},
    {"op": "set", "path": ["volumes", "100", "body", {"id": "gate-gate-2"}, "sheet"]},
])
