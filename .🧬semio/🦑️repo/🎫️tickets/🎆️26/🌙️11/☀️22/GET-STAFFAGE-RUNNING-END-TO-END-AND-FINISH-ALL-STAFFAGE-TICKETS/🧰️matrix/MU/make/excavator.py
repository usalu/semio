"""🚜️ Ops writer of `excavator`: the counterweight is curved about the slew axis; the plan draws it so, the side elevation and the front drew its box, whose corners stood 313 mm out when the upper carriage slews. The front of every scale and the side elevation of 1:100 to 1:500 take the body of the plan (the curved outline swept over the height); the side elevation of 1:50 keeps its chamfered outline over the middle metre and gets three narrower chamfered pieces under it that follow the curve to the sides, hidden at rest. The bucket link of 1:500 is 160 mm deep so that what the stick and the bucket leave visible prints: `python3 make/excavator.py` writes its ops."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *
a = asset("excavator")
ops = [{"op": "solid-from", "scale": str(scale), "view": view, "id": "counterweight-body", "from": "plan"} for scale in scales(a) for view in ("elevation", "front") if (str(scale), view) != ("50", "elevation")]
main = shape(a, "50", "elevation", "counterweight-body")
ops.append(op("50", "elevation", "counterweight-body", like(main, solid=extruded("w", -0.5, 0.5))))
for i, (x, z) in enumerate(((0.0829, 0.85), (0.1826, 1.15), (0.301, 1.37)), 1):
    d = f"M {x},0.08 L {round(x + 0.0199, 4)},0 L 0.5557,0 L 0.57,0.08 L 0.57,1.08 L 0.5557,1.16 L {round(x + 0.0199, 4)},1.16 L {x},1.08 Z"
    ops.append({"op": "shape-add", "scale": "50", "view": "elevation", "after": "counterweight-body", "shape": like(main, id=f"counterweight-flank-{i}", z=round(main["z"] - 0.1 * i, 2), d=d, solid=extruded("w", -z, z))})
for view in ("elevation", "front", "plan"):
    ops.append(op("500", view, "bucket-link", like(shape(a, "500", view, "bucket-link"), d="M 0,-0.08 L 0.66,-0.08 L 0.74,0 L 0.66,0.08 L 0,0.08 L -0.08,0 Z")))
write("excavator", ops)
