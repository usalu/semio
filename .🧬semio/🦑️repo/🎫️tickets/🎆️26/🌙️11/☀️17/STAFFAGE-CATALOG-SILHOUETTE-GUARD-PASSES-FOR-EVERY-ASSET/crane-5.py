"""Tower crane 1:50 front: the chord and rail bars of the volume (named jib-chord and jib-rail, without the trailing hyphen the generator pattern asked for) get the end-on member dots every other jib bar has (4 mm section, 1.3 mm pen)."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
d = lib.D("🏗️machinery/tower-crane")
tmpl = d.obj(d.shape(50, "front", "jib-brace-a-1"))
anchor = d.shape(50, "front", "jib-brace-a-1")
new = []
for id_, y, z in (("jib-chord-1", 0.74, 0.74), ("jib-chord-2", 0.74, -0.74), ("jib-rail-1", -0.74, 0.45), ("jib-rail-2", -0.74, -0.45)):
    o = dict(tmpl); o["id"] = id_
    o["from"] = {"x": 0.0, "y": y, "z": z}; o["to"] = {"x": 55.0, "y": y, "z": z}
    o["section"] = {"kind": "circle", "d": 0.004}; o["penMm"] = 1.3
    new.append(o)
for o in reversed(new): d.insert_after(anchor, o)
d.commit()
