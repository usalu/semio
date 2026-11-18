"""Tower crane 1:50: the remaining pen strokes of the generated shapes (pendants in all views, tower head ties, jib and counter-jib of the front) become filled quadrilaterals as well (see machinery-tower-crane-29.py)."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
d = lib.D("🏗️machinery/tower-crane")
for view in ("elevation", "front", "plan"):
    ids = [x["id"] for x in d.val("asset", "representations", "50", "geometry", view) if x["id"].endswith("-stroke")]
    if not ids: continue
    solids = [i[:-len("-stroke")] for i in ids]
    res = {r["id"]: r for r in lib.strokes(50, view, "^(" + "|".join(solids) + ")$", "q", "quads")}
    print(view, len(ids), len(res))
    for i, sid in zip(ids, solids):
        n = d.shape(50, view, i); o = d.obj(n)
        out = {k: v for k, v in o.items() if k not in ("penMm", "d", "filled")}
        out["filled"] = True; out["d"] = res[sid]["d"]
        d.replace_obj(n, out)
d.commit()
