"""Tower crane 1:50 elevation: the chord and rail members end where the guard's volume strips end (the strip of the jib bars covers x 0.059 to 55.050 m, the strips of the counter-jib chords x -16.550 to 0.014 m; the flat member ends lie 2 mm inside their end points)."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
sp = lib.sp
d = lib.D("🏗️machinery/tower-crane")
for id_, a, b in (("jib-chord-1", 0.061, 55.052), ("jib-rail-1", 0.061, 55.052), ("counter-chord-top-1", 16.552, -0.016), ("counter-chord-bottom-1", 16.552, -0.016)):
    n = d.shape(50, "elevation", id_)
    d.setv(sp.child(sp.child(n, "from"), "x"), a)
    d.setv(sp.child(sp.child(n, "to"), "x"), b)
d.commit()
