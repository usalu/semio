"""Tower crane: the four sling members of the 1:100 drawing are the slings at every scale; the duplicate sling members generated at 1:100 and 1:50 are removed and 1:200 gets the 1:100 members (elevation and front)."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
d = lib.D("🏗️machinery/tower-crane")
src = lib.D("🏗️machinery/tower-crane")
import re
for scale in (200, 100, 50):
    for view in ("elevation", "front"):
        for x in d.val("asset", "representations", str(scale), "geometry", view):
            if re.fullmatch(r"sling-\d+-\d+x*", x["id"]): d.delete_item(d.shape(scale, view, x["id"]))
for view in ("elevation", "front"):
    anchor = d.shape(200, view, "slew-platform")
    for i in (4, 3, 2, 1): d.insert_after(anchor, src.obj(src.shape(100, view, f"sling-{i}")))
d.commit()
