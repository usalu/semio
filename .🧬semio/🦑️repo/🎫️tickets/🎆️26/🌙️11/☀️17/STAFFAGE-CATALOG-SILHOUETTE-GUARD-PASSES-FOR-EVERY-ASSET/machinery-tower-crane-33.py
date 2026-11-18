"""Tower crane 1:50: identical quadrilaterals inside one filled path cancel under the even-odd fill, so every subpath of the generated filled shapes appears once."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
sp = lib.sp
d = lib.D("🏗️machinery/tower-crane")
n_fixed = 0
for view in ("elevation", "front", "plan"):
    for x in d.val("asset", "representations", "50", "geometry", view):
        if x["kind"] != "path" or not x.get("filled") or "Z" not in x["d"]: continue
        parts = [p.strip() + " Z" for p in x["d"].split("Z") if p.strip()]
        uniq = list(dict.fromkeys(parts))
        if len(uniq) != len(parts):
            d.setv(sp.child(d.shape(50, view, x["id"]), "d"), " ".join(uniq)); n_fixed += 1
print(n_fixed)
d.commit()
