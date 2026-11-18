"""Tower crane 1:50: the winch vent stack of the volume is drawn in the three views like at 1:200 (role body)."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
d = lib.D("🏗️machinery/tower-crane")
src = lib.D("🏗️machinery/tower-crane")
for view in ("elevation", "front", "plan"):
    c = src.obj(src.shape(200, view, "winch-vent-stack")); c["role"] = "body"
    d.insert_after(d.shape(50, view, "winch-house"), c)
d.commit()
