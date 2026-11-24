"""🗄️ Writes the repair ops of `used-wall-unit`: the centre cabinet is tilted 25 degrees on the trolley in the pose `carried`, and its plan was drawn with flat shapes, which foreshorten with the tilt (plan 1.774 m ahead against 1.885 m in the side elevation, the plinth contact 0.22 to 0.44 m off the drawn body). Every plan shape of the bone `centre-body` becomes a solid: at 1:100, 1:200 and 1:500 the top and the cornice are the side elevation profiles swept over the 0.90 m width, at 1:50 each cut part of the plan section is its own outline swept over its real height (heights of the 1:50 body). The rest pose draws as before."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, like, shape, write, GEOMETRY
a = asset("used-wall-unit")
WIDTH = {"kind": "extruded", "sweep": "w", "z0": -0.45, "z1": 0.45}
HEIGHTS = {"centre-bottom": (0.08, 0.099), "centre-bar-bottom": (0.44, 0.459), "centre-mirror": (0.459, 0.86), "centre-back": (0.08, 1.94), "centre-side-p": (0.0, 1.94), "centre-side-m": (0.0, 1.94)}


def corners(s):
    """🔲️ The corner points of a rect or the points of a poly."""
    return [[s["x"], s["y"]], [s["x"] + s["w"], s["y"]], [s["x"] + s["w"], s["y"] + s["h"]], [s["x"], s["y"] + s["h"]]] if s["kind"] == "rect" else s["points"]


ops = []
for scale in (100, 200, 500):
    for sid, twin in (("centre-top", "centre-side"), ("centre-cornice", "centre-cornice")):
        ops.append({"op": "shape", "scale": str(scale), "view": "plan", "id": sid, "shape": like(shape(a, scale, "plan", sid), drop=GEOMETRY, kind="poly", points=corners(shape(a, scale, "elevation", twin)), closed=True, solid=WIDTH)})
for sid, (low, high) in HEIGHTS.items():
    flat = shape(a, 50, "plan", sid)
    if "solid" not in flat:
        ops.append({"op": "shape", "scale": "50", "view": "plan", "id": sid, "shape": like(flat, drop=GEOMETRY, kind="poly", points=corners(flat), closed=True, solid={"kind": "extruded", "sweep": "v", "z0": low, "z1": high})})
    else:
        ops.append({"op": "shape", "scale": "50", "view": "plan", "id": sid, "shape": flat})
write("used-wall-unit", ops)
