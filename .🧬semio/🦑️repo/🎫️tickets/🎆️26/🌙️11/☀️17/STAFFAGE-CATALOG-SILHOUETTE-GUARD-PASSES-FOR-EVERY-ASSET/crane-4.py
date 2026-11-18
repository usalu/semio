"""Tower crane 1:50 front: the tower head leg members end 18 mm further along their axis (the volume leg ends 17 to 21 mm above the flat member end)."""
import sys, math
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
sp = lib.sp
d = lib.D("🏗️machinery/tower-crane")
for id_ in ("tower-head-leg-1-1", "tower-head-leg-2-1"):
    x = next(s for s in d.val("asset", "representations", "50", "geometry", "front") if s["id"] == id_)
    a = [x["from"][k] for k in "xyz"]; b = [x["to"][k] for k in "xyz"]
    L = math.dist(a, b); u = [(b[i] - a[i]) / L for i in range(3)]
    n = sp.child(d.shape(50, "front", id_), "to")
    d.setv(n, {k: round(b[i] + 0.018 * u[i], 4) for i, k in enumerate("xyz")})
d.commit()
