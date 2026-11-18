"""Tower crane 1:50 front: the volume chord dots are squares of the followed pen (65 mm) tilted by 20 degrees; the chord members end 2 mm along that tilt from their start (hull ring of 2 mm length), so their pen strokes form the tilted square."""
import sys, math
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
sp = lib.sp
d = lib.D("🏗️machinery/tower-crane")
t = math.radians(19.7)
for id_, y, z in (("jib-chord-1", 0.74, 0.74), ("jib-chord-2", 0.74, -0.74)):
    n = sp.child(d.shape(50, "front", id_), "to")
    d.setv(n, {"x": 55.0, "y": round(y + 0.002 * math.sin(t), 5), "z": round(z + 0.002 * math.cos(t), 5)})
d.commit()
