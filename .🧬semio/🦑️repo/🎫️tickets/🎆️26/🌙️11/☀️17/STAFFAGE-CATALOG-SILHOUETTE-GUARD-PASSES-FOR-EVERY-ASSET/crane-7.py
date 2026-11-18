"""Tower crane 1:50 front: the chord dot members take the section of the followed pen (70 mm): their end circle then lies over more than half of the volume dot, so the guard fills it as a disc (a 4 mm ring was filled as a point and drew nothing), which covers the corners of the tilted volume square within 12 mm; the 2 mm tilt of the last try is undone."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
sp = lib.sp
d = lib.D("🏗️machinery/tower-crane")
for id_, y, z in (("jib-chord-1", 0.74, 0.74), ("jib-chord-2", 0.74, -0.74)):
    n = d.shape(50, "front", id_)
    d.setv(sp.child(n, "to"), {"x": 55.0, "y": y, "z": z})
    d.setv(sp.child(sp.child(n, "section"), "d"), 0.07)
d.commit()
