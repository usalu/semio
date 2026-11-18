"""Tower crane 1:200, 1:100, 1:50: the generated members of jib, counter-jib and slewing reach a quarter of their diameter beyond the bar ends (the flat square end of the volume bar stroke lies between the bar end and the end of a round hull of the bar end circle)."""
import sys, math
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
sp = lib.sp
d = lib.D("🏗️machinery/tower-crane")
for sc in (200, 100, 50):
    for view in ("elevation", "front"):
        for x in d.val("asset", "representations", str(sc), "geometry", view):
            if x["kind"] != "member" or x["bone"] not in ("jib", "counter-jib", "slewing") or x["id"].startswith("sling"): continue
            a = [x["from"][k] for k in "xyz"]; b = [x["to"][k] for k in "xyz"]
            L = math.dist(a, b)
            if L < 1e-6: continue
            e = x["section"]["d"] / 4
            u = [(b[i] - a[i]) / L for i in range(3)]
            n = d.shape(sc, view, x["id"])
            for key, p, sgn in (("from", a, -1), ("to", b, 1)):
                node = sp.child(n, key)
                d.setv(node, {k: round(p[i] + sgn * u[i] * e, 4) for i, k in enumerate("xyz")})
d.commit()
