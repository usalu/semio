"""Tower crane 1:50: the open tower head leg segment of the elevation carries the 0.1 m leg pen (2 mm), because the slender volume legs follow the nearest drawn centre line and took its 12.5 mm role pen."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
d = lib.D("🏗️machinery/tower-crane")
n = d.shape(50, "elevation", "tower-head"); o = d.obj(n)
o["penMm"] = 2.0
d.replace_obj(n, o)
d.commit()
