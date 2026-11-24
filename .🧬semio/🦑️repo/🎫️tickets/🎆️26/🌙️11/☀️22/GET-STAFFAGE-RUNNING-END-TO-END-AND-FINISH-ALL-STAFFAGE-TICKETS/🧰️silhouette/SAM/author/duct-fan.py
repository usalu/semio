"""🌀️ Repair ops of `duct-fan`: the blade tips of the volume reach the nozzle ring (radius 88 mm), the drawn tips ended at 87 mm, so the pocket between a blade and the ring stayed open to the duct in the drawing and closed in the body (1:50 elevation 26.6 mm). In the default pose `high` only the pocket under blade 4 is closed in the body (measured blade by blade: carrying the tips of blade 1, 2, 3 or 5 out leaves 26.6 mm or makes 34.4 mm, blade 4 alone gives 3.1 mm), so the two tip corners of the drawn blade 4 are carried out to 88.5 mm, into the ring (`ONLY=1,2,…` chooses other blades)."""
import math, re
from oplib import load, shape, write

asset = load("duct-fan")
ops = []
import os
for k in [int(x) for x in os.environ.get("ONLY", "4").split(",")]:
    d = shape(asset, 50, "elevation", f"blade-{k}")["d"]
    def tip(m):
        x, y = float(m.group(1)), float(m.group(2))
        r = math.hypot(x, y)
        if 0.0862 <= r < 0.0884:
            x, y = x * 0.0885 / r, y * 0.0885 / r
            return f"{round(x, 5):g} {round(y, 5):g}"
        return m.group(0)
    ops.append({"op": "shape-set", "scale": "50", "view": "elevation", "id": f"blade-{k}", "set": {"d": re.sub(r"(-?\d+\.?\d*(?:e-?\d+)?) (-?\d+\.?\d*(?:e-?\d+)?)", tip, d)}})
write("duct-fan", ops)
