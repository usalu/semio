"""🐴️ Writes the repair ops of `used-spring-rocker`: riding pitches the body 12 degrees on the spring. Its front and plan shapes were flat, so the pitched body kept its rest height in the front, 51 mm above the side elevation. Every part of the body is a board or a tube across the rocker: the two cut panels, the seat, bottom and spacer boards, the handle and footrest tubes with their grips and caps, the upper spring plate and its clamps. Each becomes its side elevation outline swept over the lateral span its front and plan drew; the spring coils of 1:200 become bodies of revolution about their bones."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, prism, scales, turned, twins, write
a = asset("used-spring-rocker")
ops = []
for k in scales(a):
    for sid in twins(a, k, {"body", "spring-1", "spring-3"}):
        if sid.startswith("bolt"):
            continue
        for view in ("front", "plan"):
            ops.append(turned(a, k, view, sid, f"spring-{sid[-1]}") if sid.startswith("coil-") else prism(a, k, view, sid))
    for view in ("front", "plan"):
        if "coil-2" in twins(a, k, {"spring-2"}):
            ops.append(turned(a, k, view, "coil-2", "spring-2"))
write("used-spring-rocker", ops)
