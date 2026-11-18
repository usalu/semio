"""Tower crane 1:50 elevation: the jib chord and rail bars of the volume (d 0.14 and 0.12 m) had no drawn member (the drawn lattice stopped at the braces and diagonals), so the volume rail strip stood 1.1 m from the nearest drawn boundary; both get a 2 mm pen member (0.1 m) reaching a quarter diameter beyond the bar ends."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
d = lib.D("🏗️machinery/tower-crane")
tmpl = d.obj(d.shape(50, "elevation", "counter-chord-top-1"))
anchor = d.shape(50, "elevation", "slew-platform")
new = []
for id_, y, z in (("jib-chord-1", 0.74, 0.74), ("jib-rail-1", -0.74, 0.45)):
    o = dict(tmpl); o["id"] = id_; o["bone"] = "jib"
    o["from"] = {"x": -0.025, "y": y, "z": z}; o["to"] = {"x": 55.025, "y": y, "z": z}
    o["section"] = {"kind": "circle", "d": 0.1}; o["penMm"] = 2.0
    new.append(o)
for o in reversed(new): d.insert_after(anchor, o)
d.commit()
