"""🪘️ Writes the repair ops of `used-djembe-and-cajon`: playing tilts the djembe 12 degrees onto the edge of its foot. The plan drew no shell, because the bowl covers the foot while the drum stands, so in the played pose the plan ended 59 mm behind the foot the side elevation shows ahead of the bowl. The plan of every band gets the shell as the body of revolution its side elevation and front draw, painted under the head collar: hidden while the drum stands, its foot comes out ahead when the drum is tilted."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, like, scales, shape, write
a = asset("used-djembe-and-cajon")
ops = []
for k in scales(a):
    collar = shape(a, k, "plan", "head-collar")
    ops.append({"op": "shape-add", "scale": k, "view": "plan", "shape": like(shape(a, k, "front", "shell"), z=round(collar["z"] - 0.5, 4))})
write("used-djembe-and-cajon", ops)
