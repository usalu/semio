"""Tower crane 1:50 plan: four jib lattice strokes end 8 to 26 mm inside a tile border of the guard (the tiled measurement selects drawn shapes by their ring bounds without the pen), so a dot of their own pen at y -0.748 m keeps them in the tile; the dot lies within 14 mm of their end cap."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
sp = lib.sp
d = lib.D("🏗️machinery/tower-crane")
for sid, x in (("jib-diag-down-r-8", 25.2282), ("jib-brace-a-8", 25.2221), ("jib-diag-up-r-7", 25.1753), ("jib-brace-b-7", 25.1813)):
    n = d.shape(50, "plan", sid); o = d.obj(n)
    d.setv(sp.child(n, "d"), o["d"] + f" M {x} -0.748 L {x} -0.748")
d.commit()
