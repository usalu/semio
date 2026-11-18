"""Tower crane 1:100: the five tower head ties that are plain (not slender) cylinders in the volume are drawn as the convex hulls of their parts (machinery-strokes.ts hull mode) in the three views."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
d = lib.D("🏗️machinery/tower-crane")
for view in ("elevation", "front", "plan"):
    last = lib.strokes(100, view, "^tower-head-", "th")[-1]["id"] + "-stroke"
    res = lib.strokes(100, view, "^tower-head-tie", "thh", "hull")
    tmpl = d.obj(d.shape(100, view, last))
    for r in reversed(res): d.insert_after(d.shape(100, view, last), lib.shape_for(r, 100, tmpl))
d.commit()
