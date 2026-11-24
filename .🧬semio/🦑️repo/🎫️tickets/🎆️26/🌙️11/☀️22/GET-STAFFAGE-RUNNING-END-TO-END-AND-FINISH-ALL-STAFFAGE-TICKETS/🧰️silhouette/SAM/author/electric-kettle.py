"""🫖️ Repair ops of `electric-kettle`: the lid of the volume has a plug that sits 12 mm deep in the neck of the jug; the drawn lid lacked it, so the 1.4° the lid stands open in `boiling` showed an empty wedge over the rim that the body does not have (1:50 elevation 15.2 mm). The plug is drawn in the side and front elevation of 1:50 exactly as the profile of the solid."""
from oplib import load, shape, write

asset = load("electric-kettle")
ops = []
plug = [[0.066, 0.012], [0.1265, 0.012], [0.129, 0.0095], [0.129, 0.004]]
for view in ("elevation", "front"):
    points = shape(asset, 50, view, "lid")["points"]
    rest = [p for p in points if p[1] < 0.004 or p[0] > 0.129]
    ops.append({"op": "shape-set", "scale": "50", "view": view, "id": "lid", "set": {"points": plug + rest}})
write("electric-kettle", ops)
