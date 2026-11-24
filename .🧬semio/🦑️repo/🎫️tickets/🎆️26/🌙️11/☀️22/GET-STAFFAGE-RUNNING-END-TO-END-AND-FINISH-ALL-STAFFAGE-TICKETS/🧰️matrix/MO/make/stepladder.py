"""🪜️ Writes the repair ops of `stepladder`: folding turns the platform up to 62 degrees about its hinge, and its front and plan were flat boards, so the front kept a board 26 mm high where the side elevation shows it standing 240 mm (117 mm apart in `folding-7`). The platform becomes one solid in the front and the plan of every band: its side elevation outline swept over its width of 0.29 m."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, extruded, op, scales, shape, solid_of, write
a = asset("stepladder")
ops = []
for k in scales(a):
    e = shape(a, k, "elevation", "platform")
    for view in ("front", "plan"):
        ops.append(op(k, view, "platform", solid_of(shape(a, k, view, "platform"), e, extruded("w", -0.145, 0.145))))
write("stepladder", ops)
