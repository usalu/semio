"""Tower crane 1:100 elevation: the free ends of the jib chord members and the counter-jib chord members end where the volume strips of the bars end (jib strips x -0.057 to 54.949 m, counter-jib strips x -16.564 to 0.031 m)."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
sp = lib.sp
d = lib.D("🏗️machinery/tower-crane")
for id_, a, b in (("jib-chord-top-1", -0.055, 54.951), ("jib-chord-bottom-1", -0.055, 54.951), ("counter-chord-top-1", 16.567, -0.0285), ("counter-chord-bottom-1", 16.567, -0.0285)):
    n = d.shape(100, "elevation", id_)
    d.setv(sp.child(sp.child(n, "from"), "x"), a)
    d.setv(sp.child(sp.child(n, "to"), "x"), b)
d.commit()
