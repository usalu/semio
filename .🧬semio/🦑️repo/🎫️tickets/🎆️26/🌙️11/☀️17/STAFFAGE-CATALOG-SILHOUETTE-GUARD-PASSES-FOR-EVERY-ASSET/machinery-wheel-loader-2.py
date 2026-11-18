"""Wheel loader: tyre polygons with the 20 mm lug gap, wheel bolt heads and the lift pivot at 1:50, tilt lugs and pin at 1:50, tilt cylinder and rod in the plan at 1:500 and 1:200."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
AR = [(-.25, .875), (-.01, .875), (-.01, .845), (.01, .845), (.01, .875), (.25, .875), (.26, .845), (.3, .74), (.3, -.74), (.26, -.845), (.25, -.875), (.01, -.875), (.01, -.845), (-.01, -.845), (-.01, -.875), (-.25, -.875), (-.26, -.845), (-.3, -.74), (-.3, .74), (-.26, .845)]
d = D("🏗️machinery/wheel-loader")
for sc in (100, 50):
    for sid, lat in (("tyre-f-l", 1.1), ("tyre-f-r", -1.1)):
        d.setv(sp.child(d.shape(sc, "front", sid), "points"), [[round(lat + a, 4), round(0.875 + r, 4)] for a, r in AR])
    for sid, lat, ctr in (("tyre-f-l", 1.1, 1.7), ("tyre-f-r", -1.1, 1.7), ("tyre-r-l", 1.1, -1.7), ("tyre-r-r", -1.1, -1.7)):
        d.setv(sp.child(d.shape(sc, "plan", sid), "points"), [[round(ctr + r, 4), round(lat + a, 4)] for a, r in AR])
def rect(x0, x1, y0, y1): return f"M {x0} {y0} L {x1} {y0} L {x1} {y1} L {x0} {y1} Z"
n = d.shape(50, "plan", "tyre-r-r")
for sid, side, ctr in (("bolts-f-l", 1, 1.7), ("bolts-f-r", -1, 1.7), ("bolts-r-l", 1, -1.7), ("bolts-r-r", -1, -1.7)):
    t = d.obj(d.shape(50, "plan", sid.replace("bolts", "tyre")))
    pa = " ".join(rect(round(ctr + c - 0.045, 4), round(ctr + c + 0.045, 4), *sorted((round(side * 1.4, 4), round(side * 1.433, 4)))) for c in (0.3, 0.15, -0.15, -0.3))
    o = {"id": sid, "bone": t["bone"], "role": "detail", "z": t["z"] + 0.1, "filled": True, "tone": 0.3, "kind": "path", "d": pa, "region": t["region"], "material": t["material"]}
    d.insert_after(n, o)
n = d.shape(50, "front", "lift-arm-r")
t = d.obj(n)
d.insert_after(n, {"id": "lift-pivot", "role": "body", "filled": True, "kind": "rect", "x": -1.25, "y": 1.93, "w": 2.5, "h": 0.14})
n = d.shape(50, "plan", "lift-arm-r")
d.insert_after(n, {"id": "lift-pivot", "role": "body", "filled": True, "kind": "rect", "x": 1.03, "y": -1.25, "w": 0.14, "h": 2.5})
tmpl = d.obj(d.shape(50, "plan", "bucket-edge"))
n = d.shape(50, "plan", "bucket-edge")
def bucket_shape(sid, pts, z0, z1):
    o = dict(tmpl); o["id"] = sid; o["role"] = "body"; o["points"] = pts; o["solid"] = {"kind": "extruded", "sweep": "w", "z0": z0, "z1": z1}
    return o
lug = [[-0.52, 0.4], [-0.36, 0.45], [-0.3, 0.6], [-0.52, 0.62]]
d.insert_after(n, bucket_shape("tilt-lug-1", lug, 0.05, 0.07))
d.insert_after(n, bucket_shape("tilt-lug-2", lug, -0.07, -0.05))
d.insert_after(n, bucket_shape("tilt-pin", [[-0.48, 0.47], [-0.42, 0.47], [-0.42, 0.53], [-0.48, 0.53]], -0.1, 0.1))
for sc, ids in ((500, ("tilt-cyl", "tilt-rod")), (200, ("tilt-rod",))):
    n = d.shape(sc, "plan", "tilt-cyl" if sc == 200 else "lift-arm-r")
    for sid in reversed(ids): d.insert_after(n, d.obj(d.shape(100, "plan", sid)))
d.commit()
