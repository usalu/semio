"""Tower crane 1:50 front: the drawn diagonal and brace quadrilaterals of the jib and counter-jib stop 45 mm before the chord points where they end (the 0.12 m to 0.14 m chord strokes cover that part): the centre lines of the diagonal strips then no longer pass through the chord points, so the slender volume chords follow their own chord pen instead of a diagonal that ties with them at distance zero."""
import sys, math
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET")
from importlib import import_module
lib = import_module("machinery-tc-lib")
sp = lib.sp
d = lib.D("🏗️machinery/tower-crane")
S = 0.045
def shrink(q):
    pts = [(q[i], q[i + 1]) for i in range(0, 8, 2)]
    a, b = math.dist(pts[0], pts[1]), math.dist(pts[1], pts[2])
    if a < b: pts = pts[1:] + pts[:1]; a = b
    u = ((pts[1][0] - pts[0][0]) / a, (pts[1][1] - pts[0][1]) / a)
    p0 = (pts[0][0] + u[0] * S, pts[0][1] + u[1] * S); p3 = (pts[3][0] + u[0] * S, pts[3][1] + u[1] * S)
    p1 = (pts[1][0] - u[0] * S, pts[1][1] - u[1] * S); p2 = (pts[2][0] - u[0] * S, pts[2][1] - u[1] * S)
    return "M " + " L ".join(f"{round(p[0], 4)} {round(p[1], 4)}" for p in (p0, p1, p2, p3)) + " Z"
for x in d.val("asset", "representations", "50", "geometry", "front"):
    if x["kind"] != "path" or not x.get("filled") or not x["id"].startswith(("jib-diag", "jib-brace", "counter-diag", "counter-brace")): continue
    parts = [p.strip() for p in x["d"].split("Z") if p.strip()]
    out = []
    for p in parts:
        v = [float(t) for t in p.replace("M", " ").replace("L", " ").split()]
        out.append(shrink(v))
    d.setv(sp.child(d.shape(50, "front", x["id"]), "d"), " ".join(out))
d.commit()
