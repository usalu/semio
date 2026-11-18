"""Tower crane 1:200 front: the jib and counter-jib are drawn as the projected centre lines of their slender volume members (the filled frame rectangles hid the see-through interior of the lattice, whose hole edges the volume shows)."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
d = lib.D("🏗️machinery/tower-crane")
res = lib.strokes(200, "front", "^(jib|counter)-", "jc")
lib.apply(d, 200, "front", res, ["jib-frame", "counter-jib"], "jib-frame")
d.commit()
