"""Tower crane 1:500: the filled tower head triangle (a 2.4 m strip that widened every slender volume leg and pendant to 2.4 m) is drawn as the 24 thin members of the volume (4 legs, 4 base frame bars, 8 x-ties, 8 z-ties) in the three views, like the 1:200 drawing."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/tower-crane")
src = D("🏗️machinery/tower-crane")
def member(tmpl, sid, a, b, dia, pen):
    o = dict(tmpl); o["id"] = sid
    o["from"] = {"x": a[0], "y": a[1], "z": a[2]}; o["to"] = {"x": b[0], "y": b[1], "z": b[2]}
    o["section"] = {"kind": "circle", "d": dia}; o["penMm"] = pen
    return {k: o[k] for k in ("kind", "from", "to", "section", "id", "bone", "role", "z", "filled", "tone", "region", "material", "penMm")}
tmpl = src.obj(src.shape(200, "elevation", "tower-head-leg-1"))
M = []
for i, (sx, sz) in enumerate(((1, 1), (1, -1), (-1, 1), (-1, -1)), 1):
    M.append(member(tmpl, f"tower-head-leg-{i}", (1.2 * sx, 1.2, 0.9 * sz), (0, 8.6, 0.12 * sz), 0.02, 0.2))
for i, (a, b) in enumerate((((-1.2, 1.2, -0.9), (1.2, 1.2, -0.9)), ((1.2, 1.2, -0.9), (1.2, 1.2, 0.9)), ((-1.2, 1.2, 0.9), (1.2, 1.2, 0.9)), ((-1.2, 1.2, -0.9), (-1.2, 1.2, 0.9))), 1):
    M.append(member(tmpl, f"tower-head-frame-{i}", a, b, 0.02, 0.16))
for k, (y, xh, zh) in enumerate(((2.6, 0.97297, 0.75243), (4.0, 0.74595, 0.60486), (5.4, 0.51892, 0.4573), (6.8, 0.29189, 0.30973)), 1):
    for j, s in enumerate((1, -1), 1):
        M.append(member(tmpl, f"tower-head-tie-x-{k}-{j}", (-xh, y, s * zh), (xh, y, s * zh), 0.02, 0.15))
        M.append(member(tmpl, f"tower-head-tie-z-{k}-{j}", (s * xh, y, -zh), (s * xh, y, zh), 0.02, 0.15))
for v in ("elevation", "front", "plan"):
    n = d.shape(500, v, "tower-head")
    d.replace_obj(n, M[0])
    for m in reversed(M[1:]): d.insert_after(n, m)
d.commit()
