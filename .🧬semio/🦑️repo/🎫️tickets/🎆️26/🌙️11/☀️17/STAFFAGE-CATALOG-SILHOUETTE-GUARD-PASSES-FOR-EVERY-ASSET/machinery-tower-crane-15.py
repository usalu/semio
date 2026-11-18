"""Tower crane 1:200 and 1:100: the tower head (legs, diagonals, base frame, cross bars, ties) is drawn as the projected centre lines of its slender volume members in the three views; the old member and tie shapes of the head are replaced one for one."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
d = lib.D("🏗️machinery/tower-crane")
for scale in (200, 100):
    for view in ("elevation", "front", "plan"):
        old = [x["id"] for x in d.val("asset", "representations", str(scale), "geometry", view) if x["id"].startswith("tower-head")]
        res = lib.strokes(scale, view, "^tower-head-", "th")
        print(scale, view, len(old), len(res))
        if len(res) < len(old):
            res = res + [dict(res[0], id=res[0]["id"] + "-%d" % k) for k in range(len(old) - len(res))]
        lib.apply(d, scale, view, res, old, old[0])
d.commit()
