"""🔥️ Writes the repair ops of `patio-heater`: at 1:50 the side elevation draws the wheel bracket of the volume between the wheel and the housing (the notch it left stood 26 mm from the body) and the two visible spacer studs of the reflector on the side where the volume has them (they were mirrored about the pole axis)."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, solid, write
a = asset("patio-heater")
write("patio-heater", [
    {"op": "shape-add", "scale": "50", "view": "elevation", "after": "base", "shape": {"id": "wheel-bracket", "bone": "base", "role": "detail", "z": 3.05, "filled": True, "tone": 0.51, "kind": "poly", "points": solid(a, 50, "bracket-left")["profile"], "closed": True, "region": "base-housing", "material": "powder-steel"}},
    {"op": "shape-set", "scale": "50", "view": "elevation", "id": "spacer-0", "set": {"points": [[0, -0.095], [0.041, -0.095]]}},
    {"op": "shape-set", "scale": "50", "view": "elevation", "id": "spacer-1", "set": {"points": [[0, 0.0475], [0.041, 0.0475]]}},
])
