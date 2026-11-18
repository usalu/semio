"""Tower crane 1:100 front: the four counter-jib diagonals (seen almost end-on as upright strips beside the chord dots) are 0.078 m wide like the chord dots, so the 7 mm slit between the jib diagonal and the counter-jib diagonal closes in the drawing and in the volume strips alike (the volume closed it with its tilted dot corner, the drawing left it open as a pocket edge)."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
sp = lib.sp
d = lib.D("🏗️machinery/tower-crane")
for part in ("down-r", "down-l", "up-r", "up-l"):
    n = d.shape(100, "front", f"counter-diag-{part}-1")
    d.setv(sp.child(sp.child(n, "section"), "d"), 0.078)
    d.setv(sp.child(n, "penMm"), 0.78)
d.commit()
