"""📺️ Writes the repair ops of `used-tv-sideboard`: the side elevation of 1:100, 1:200 and 1:500 gets the far flap door, which stands 0.38 m ahead of the carcass when it is open and was drawn only in the plan and the front (extent of the side elevation 0.425 m against 0.776 m in the plan). The door is covered by the near door in the rest pose; the plan of 1:200 and 1:500 stays one drawing (allowlisted, D47)."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, like, shape, write
a = asset("used-tv-sideboard")
ops = []
for scale in (100, 200, 500):
    ops.append({"op": "shape-add", "scale": str(scale), "view": "elevation", "after": "door-r", "shape": like(shape(a, scale, "elevation", "door-r"), id="door-l", bone="flap-l", z=3.9)})
write("used-tv-sideboard", ops)
