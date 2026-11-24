"""⛱️ Writes the repair ops of `market-parasol`: at 1:50 the front draws the pin of the tilt knuckle that the volume carries (it stood 19 mm outside the drawing); at 1:100 and 1:200 the rib rods run to the end of the rib (0.03 … 1.3676 m, the tip cap merged into the rod) in the volume and in the two elevations, so the body is 2.591 m across like the declared 2.59 m instead of the 2.552 m of the canopy alone."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, shape, solid, write
a = asset("market-parasol")
ops = [{"op": "shape-add", "scale": "50", "view": "front", "after": "mast-knuckle", "shape": {"id": "mast-knuckle-pin", "bone": "mast-upper", "role": "detail", "z": 1.54, "filled": True, "tone": 0.3, "region": "frame-alu", "material": "aluminium-anodised", "kind": "capsule", "x0": -0.036, "y0": 1.8, "x1": 0.036, "y1": 1.8, "r0": 0.006, "r1": 0.006}}]
for scale in ("100", "200"):
    for rib in ("right", "b", "c", "d", "left", "f", "g", "h"):
        sid = f"rib-{rib}-rod"
        ops.append({"op": "solid-set", "scale": scale, "id": sid, "set": {"size": [1.3376, 0.016, 0.01], "transform": {"translation": [0.6988, 0, 0]}}})
        for view in ("elevation", "front"):
            ops.append({"op": "shape-set", "scale": scale, "view": view, "id": sid, "set": {"to": {"x": 1.3676, "y": 0.0, "z": 0.0}}})
write("market-parasol", ops)
