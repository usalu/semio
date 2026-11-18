"""Tower crane 1:50: the generated lattice strokes (mast, jib, counter-jib members, pendants, tower head ties, front jib) become filled quadrilaterals of the exact stroke area. The tiled guard picks the drawn shapes of a tile by their ring bounds without the pen, so a pen stroke whose axis lies up to half a pen outside a tile box is dropped from that tile; a filled quad carries its width in its ring and no pen is needed (followed width = strip width)."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
sp = lib.sp
d = lib.D("🏗️machinery/tower-crane")
JOBS = (("elevation", "jib", "^jib-"), ("plan", "jib", "^jib-"), ("elevation", "counter-jib", "^counter-"), ("plan", "counter-jib", "^counter-"), ("elevation", "mast", "^mast-"), ("front", "mast", "^mast-"))
for view, bone, regex in JOBS:
    old = [x["id"] for x in d.val("asset", "representations", "50", "geometry", view) if x.get("bone") == bone and x["kind"] == "path" and x["id"] not in ("mast-ladder",) and not x["id"].startswith(("jib-tip", "head"))]
    if view == "front" and bone == "mast": old = [x["id"] for x in d.val("asset", "representations", "50", "geometry", view) if x.get("bone") == bone]
    res = lib.strokes(50, view, regex, "qc", "quadcopies")
    print(view, bone, len(old), len(res))
    lib.apply_copies(d, 50, view, res, old)
d.commit()
