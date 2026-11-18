"""Wheel loader 1:50: mirrored right hub plate, bolt heads standing 13 mm off the tyre as in the volume, tilt cylinder in the plan with the elliptical end caps of its 31.5 degree tilt, bucket ears in the plan."""
import sys, math
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/wheel-loader")
for sid in ("hub-r", "hub-f"):
    n = d.solid(50, sid, "body")
    d.setv(sp.child(sp.child(n, "repeat"), "translation"), [0, 0, -2.816])
def rect(x0, x1, y0, y1): return f"M {x0} {y0} L {x1} {y0} L {x1} {y1} L {x0} {y1} Z"
for sid, ctr in (("bolts-f-l", 1.7), ("bolts-f-r", 1.7), ("bolts-r-l", -1.7), ("bolts-r-r", -1.7)):
    side = 1 if sid.endswith("-l") else -1
    n = d.shape(50, "plan", sid)
    parts = [rect(round(ctr + c - 0.045, 4), round(ctr + c + 0.045, 4), *sorted((round(side * 1.413, 4), round(side * 1.433, 4)))) for c in (0.3, 0.15, -0.15, -0.3)]
    parts.append(rect(round(ctr - 0.14, 4), round(ctr + 0.14, 4), *sorted((round(side * 1.4, 4), round(side * 1.413, 4)))))
    d.setv(sp.child(n, "d"), " ".join(parts))
L, r, x0, x1 = 0.9, 0.1, 2.021, 2.893
lo, hi = 0.0, math.pi / 2
for _ in range(60):
    th = (lo + hi) / 2
    if L * math.cos(th) + 2 * r * math.sin(th) > x1 - x0: lo = th
    else: hi = th
a = r * math.sin(th)
xa, xb = x0 + a, x1 - a
pts = [[round(xb + a * math.cos(t), 4), round(r * math.sin(t), 4)] for t in [-math.pi / 2 + k * math.pi / 12 for k in range(13)]]
pts += [[round(xa - a * math.cos(t), 4), round(r * math.sin(t), 4)] for t in [math.pi / 2 - k * math.pi / 12 for k in range(13)]]
n = d.shape(50, "plan", "tilt-cyl"); o = d.obj(n)
o2 = {k: v for k, v in o.items() if k != "solid"}; o2["points"] = pts
d.replace_obj(n, o2)
tmpl = d.obj(d.shape(50, "plan", "bucket-edge"))
n = d.shape(50, "plan", "bucket-edge")
ear = [[-0.52, -0.2], [0.05, -0.2], [0.2, -0.05], [0.2, 0.1], [0.05, 0.25], [-0.52, 0.3]]
for sid, z0, z1 in (("bucket-ear-1", 1.2, 1.225), ("bucket-ear-2", -1.225, -1.2)):
    o = dict(tmpl); o["id"] = sid; o["role"] = "body"; o["points"] = ear; o["solid"] = {"kind": "extruded", "sweep": "w", "z0": z0, "z1": z1}
    d.insert_after(n, o)
d.commit()
print(math.degrees(th))
