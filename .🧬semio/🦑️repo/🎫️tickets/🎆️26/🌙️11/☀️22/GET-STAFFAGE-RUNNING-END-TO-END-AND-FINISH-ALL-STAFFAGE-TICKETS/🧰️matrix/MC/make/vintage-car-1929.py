"""🚙️ Ops writer of `vintage-car-1929`: the tyres, brake drums and hubs of the front view were flat sheets on the spinning wheel bones and turned over with the wheels (front 230 to 333 mm above the ground while driving, the tyre contacts up to 351 mm off them); they are bound to the axle bones, which carry the wheels and do not spin. The plan of 1:50 draws the tie rod on the front axle but no wheel, so the front contacts lay 192 mm off it: the four tyres are added to that plan under their fenders, where the view hides them: `python3 make/vintage-car-1929.py` writes `ops/vintage-car-1929.json`."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from mc import *
a = asset("vintage-car-1929")
ops = rebind(a, {"tyre-fl", "tyre-fr", "drum-fl", "drum-fr", "hub-fl", "hub-fr"}, "axle-f") + rebind(a, {"tyre-rl", "tyre-rr"}, "axle-r")
frames = rest(a)
for name, bone in (("tyre-fl", "axle-f"), ("tyre-fr", "axle-f"), ("tyre-rl", "axle-r"), ("tyre-rr", "axle-r")):
    front = shape(a, "50", "front", name)
    b = box(front)
    x = frames[bone][0]
    radius = (b[3] - b[2]) / 2
    new = like(front, drop=GEOMETRY, bone=bone, z=0.9, kind="rect", x=round(x - radius, 6), y=round(b[0], 6), w=round(2 * radius, 6), h=round(b[1] - b[0], 6), r=0.05)
    ops.append({"op": "shape-add", "scale": "50", "view": "plan", "shape": new})
write("vintage-car-1929", ops)
