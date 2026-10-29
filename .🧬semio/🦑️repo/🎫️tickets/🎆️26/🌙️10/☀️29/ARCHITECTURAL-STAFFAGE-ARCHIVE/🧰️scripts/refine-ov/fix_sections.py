import sys; sys.path.insert(0, ".")
from lib import *
doc = load("components")
I = {("cantilever-balcony", "slab"): 1 * 0.2 ** 3 / 12, ("cantilever-balcony", "railing"): (0.05 ** 4 - 0.042 ** 4) / 12, ("steel-column", "column"): 5.696e-5, ("stair-flight", "stringer"): 1.673e-5,
     ("sliding-door", "track"): (0.05 * 0.1 ** 3 - 0.042 * 0.092 ** 3) / 12, ("tilt-window", "sash"): 0.09 ** 4 / 12, ("glass-balustrade", "panel"): 1 * 0.0215 ** 3 / 12, ("tied-canopy", "canopy"): 8.693e-6}
for (aid, bid), v in I.items():
    b = next(b for b in asset(doc, aid)["rig"]["bones"] if b["id"] == bid); b["section"]["secondMomentM4"] = float(f"{v:.6g}")
save("components", doc)
