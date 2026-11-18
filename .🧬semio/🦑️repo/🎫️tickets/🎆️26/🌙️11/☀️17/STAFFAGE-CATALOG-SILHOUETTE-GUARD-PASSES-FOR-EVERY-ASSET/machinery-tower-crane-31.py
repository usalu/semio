"""Tower crane 1:50: the four ballast blocks round their edges by 7 mm instead of 20 mm, so the notches between the blocks and the rounded plan corners stay within the 15 mm limit of the drawn slab outline."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
sp = lib.sp
d = lib.D("🏗️machinery/tower-crane")
for i in range(1, 5): d.setv(sp.child(d.solid(50, f"ballast-block-{i}"), "cornerRadius"), 0.007)
d.commit()
