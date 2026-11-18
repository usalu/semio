"""Tower crane 1:100 front: the end-on chord members (a chord seen along its axis) take the section of the volume dot instead of the elevation pen width: the guard fills their end circle as a disc, and a 0.127 m disc against the 0.063 m tilted volume square closed the gap between the counter-jib chord dots and the winch house floor in the drawing only (counter-jib 0.078 m, jib 0.056 m)."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
sp = lib.sp
d = lib.D("🏗️machinery/tower-crane")
for pre, dd in (("counter-chord", 0.078), ("jib-chord", 0.056)):
    for part in ("top", "bottom"):
        for k in (1, 2):
            n = d.shape(100, "front", f"{pre}-{part}-{k}")
            d.setv(sp.child(sp.child(n, "section"), "d"), dd)
d.commit()
