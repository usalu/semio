"""Tower crane 1:50: the tower head ties of the elevation and the front are drawn as the projected centre lines of the slender ties and the convex hulls of the plain cylinder ties of the volume (the old tie outline shape becomes the first stroke)."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
d = lib.D("🏗️machinery/tower-crane")
for view in ("elevation", "front"):
    st = lib.strokes(50, view, "^tower-head-tie", "tht")
    hl = lib.strokes(50, view, "^tower-head-tie", "thh", "hull")
    tmpl = d.obj(d.shape(50, view, "tower-head-ties"))
    n = d.shape(50, view, "tower-head-ties")
    shapes = [lib.shape_for(r, 50, tmpl) for r in st] + [lib.shape_for(r, 50, tmpl) for r in hl]
    d.replace_obj(n, shapes[0])
    for s in reversed(shapes[1:]): d.insert_after(n, s)
d.commit()
