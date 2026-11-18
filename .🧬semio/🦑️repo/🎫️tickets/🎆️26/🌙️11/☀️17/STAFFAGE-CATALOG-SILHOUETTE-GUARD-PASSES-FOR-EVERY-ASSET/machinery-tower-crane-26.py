"""Tower crane 1:50: every drawn lattice member of the mast, jib and counter-jib (elevation, plan, front; one shape per volume member copy) takes its geometry and pen from the projected centre line of the matching volume member (machinery-strokes.ts copies mode); ids and other fields stay."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
d = lib.D("🏗️machinery/tower-crane")
JOBS = (("elevation", "jib", "^jib-"), ("plan", "jib", "^jib-"), ("elevation", "counter-jib", "^counter-"), ("plan", "counter-jib", "^counter-"), ("elevation", "mast", "^mast-"), ("front", "mast", "^mast-"))
for view, bone, regex in JOBS:
    old = [x["id"] for x in d.val("asset", "representations", "50", "geometry", view) if x.get("bone") == bone and x["kind"] == "member"]
    res = lib.strokes(50, view, regex, "cp", "copies")
    print(view, bone, len(old), len(res))
    lib.apply_copies(d, 50, view, res, old)
d.commit()
