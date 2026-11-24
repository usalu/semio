"""🎻️ Writes the repair ops of `used-viola`: at 1:50 the plan draws the case handle as the volume has it, the grip and its two end tabs with the finger slot of 9.5 mm open between grip and case, instead of one filled block over the slot (28 mm from the slot edges); the last 18 mm of the wedge-shaped slit between the raised lid and the case, where it is narrower than 2 mm, are drawn closed as the body closes them (15.1 mm)."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, shape, write
a = asset("used-viola")
base = {k: v for k, v in shape(a, 50, "plan", "case-base").items() if k != "points"}
handle = {k: v for k, v in shape(a, 50, "plan", "case-handle").items() if k != "points"}
rect = lambda x0, x1, y0, y1: [[x0, y0], [x1, y0], [x1, y1], [x0, y1]]
write("used-viola", [
    {"op": "shape", "scale": "50", "view": "plan", "id": "case-handle", "shape": {**handle, "points": rect(0.3365, 0.4635, -0.1103, -0.1313)}},
    {"op": "shape-add", "scale": "50", "view": "plan", "after": "case-handle", "shape": {**handle, "id": "case-handle-tab-a", "points": rect(0.334, 0.352, -0.1008, -0.1208)}},
    {"op": "shape-add", "scale": "50", "view": "plan", "after": "case-handle-tab-a", "shape": {**handle, "id": "case-handle-tab-b", "points": rect(0.448, 0.466, -0.1008, -0.1208)}},
    {"op": "shape-add", "scale": "50", "view": "plan", "after": "case-base", "shape": {**base, "id": "case-lid-seam", "role": "detail", "points": rect(0.119, 0.137, 0.134, 0.145)}},
])
