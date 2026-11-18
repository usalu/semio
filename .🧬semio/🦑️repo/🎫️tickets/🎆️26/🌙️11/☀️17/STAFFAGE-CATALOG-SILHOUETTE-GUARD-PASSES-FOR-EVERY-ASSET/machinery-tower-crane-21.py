"""Tower crane 1:100: the counter-jib end diagonals (up-l, up-r, brace-b) stop 11 mm before the chord end, so their tilted end caps no longer stand 20 mm beyond the 71.8 m extent; the drawn strokes of those members are regenerated."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
sp = lib.sp
d = lib.D("🏗️machinery/tower-crane")
for sid in ("counter-diag-up-l", "counter-diag-up-r", "counter-brace-b"):
    n = d.solid(100, sid); p = d.obj(n)["profile"]
    d.setv(sp.child(n, "profile"), [p[0], [2.15, p[1][1]]])
d.commit()
d = lib.D("🏗️machinery/tower-crane")
for view, dx, dy in (("elevation", 0, 0), ("front", 0, 0), ("plan", 0, 0)):
    for r in lib.strokes(100, view, "^counter-(diag-up-l|diag-up-r|brace-b)$", "cj"):
        d.setv(sp.child(d.shape(100, view, r["id"] + "-stroke"), "d"), r["d"])
d.commit()
