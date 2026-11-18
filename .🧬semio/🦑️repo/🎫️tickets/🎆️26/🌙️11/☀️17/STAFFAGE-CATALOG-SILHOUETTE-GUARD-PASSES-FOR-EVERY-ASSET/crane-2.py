"""Tower crane 1:50 elevation: the free ends of the chord and rail members reach half their diameter beyond the bar end (the volume strip of a slender bar ends a half pen beyond the bar end; the quarter reach left a 23 to 27 mm gap to the flat strip end)."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
sp = lib.sp
d = lib.D("🏗️machinery/tower-crane")
for id_, a, b in (("jib-chord-1", -0.05, 55.05), ("jib-rail-1", -0.05, 55.05), ("counter-chord-top-1", 16.55, -0.025), ("counter-chord-bottom-1", 16.55, -0.025)):
    n = d.shape(50, "elevation", id_)
    d.setv(sp.child(sp.child(n, "from"), "x"), a)
    d.setv(sp.child(sp.child(n, "to"), "x"), b)
d.commit()
