"""Tower crane 1:50: the redundant tower head triangle and tie outlines of the elevation become open segments inside the legs and ties (a closed unfilled outline that encloses the volume counts as a filled shape and closed the lattice holes), the pendants (all views) and the jib and counter-jib of the front are drawn as the projected centre lines of the volume members."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
sp = lib.sp
d = lib.D("🏗️machinery/tower-crane")
for sid, pts in (("tower-head", [[-1.2, 1.2], [0, 8.6]]), ("tower-head-ties", [[-0.973, 2.6], [0.973, 2.6]])):
    n = d.shape(50, "elevation", sid); o = d.obj(n)
    out = {k: v for k, v in o.items() if k not in ("solid", "points", "closed")}
    out["points"] = pts; out["closed"] = False
    d.replace_obj(n, out)
order = {"pendant-fore-l": "pendant-fore-1", "pendant-fore-r": "pendant-fore-2", "pendant-back": "pendant-back"}
for view in ("elevation", "front", "plan"):
    res = lib.strokes(50, view, "^pendant-", "pd")
    res.sort(key=lambda r: list(order).index(r["id"]))
    lib.apply(d, 50, view, res, [order[r["id"]] for r in res], "pendant-back")
res = lib.strokes(50, "front", "^(jib|counter)-", "jc")
lib.apply(d, 50, "front", res, ["jib-frame", "counter-jib"], "jib-frame")
d.commit()
