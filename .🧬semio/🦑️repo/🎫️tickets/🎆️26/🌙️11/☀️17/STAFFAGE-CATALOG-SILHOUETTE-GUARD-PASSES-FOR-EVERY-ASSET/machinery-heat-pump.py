"""Heat pump outdoor unit: lid exactly as deep as the unit (1.1 m), vane 5 mm longer (extent 0.72 m), 1:200 casing standing on the two feet like the 1:100, 1:50 fan guard drawn as the cage rectangle plus a thin spoke stroke."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/heat-pump-outdoor-unit")
for sc in (500, 200, 100, 50):
    n = d.solid(sc, "lid"); s = d.obj(n)["size"]; d.set(sp.child(n, "size"), None, s) if False else d.setv(sp.child(n, "size"), [s[0], s[1], 1.1])
    n = d.solid(sc, "vane"); o = d.obj(n); s = o["size"]; t = o["transform"]["translation"]
    d.setv(sp.child(n, "size"), [0.245, s[1], s[2]])
    d.setv(sp.child(sp.child(n, "transform"), "translation"), [0.1025, t[1], t[2]])
    for v in ("elevation", "front", "plan"):
        for sid in ("lid", "vane"):
            try: n = d.shape(sc, v, sid)
            except KeyError: continue
            o = d.obj(n)
            if "solid" in o and o["solid"].get("z0") == -0.56:
                d.setv(sp.child(sp.child(n, "solid"), "z0"), -0.55); d.setv(sp.child(sp.child(n, "solid"), "z1"), 0.55)
            if sid == "vane": d.setv(sp.child(n, "points"), [[p[0] if p[0] != 0.22 else 0.225, p[1]] for p in o["points"]])
n = d.shape(200, "front", "casing"); o = d.obj(n)
o2 = {"id": "casing", "bone": "unit", "role": "body", "z": 2, "filled": True, "tone": 0.55, "kind": "rect", "x": -0.55, "y": 0.08, "w": 1.1, "h": 1.33, "region": "casing", "material": "powder-coat"}
d.replace_obj(n, o2)
for sid in ("foot-left", "foot-right"): d.insert_after(n, d.obj(d.shape(100, "front", sid)))
n = d.shape(200, "elevation", "casing"); o = d.obj(n)
o2 = {"kind": "poly", "points": [[-0.225, 0.08], [0.225, 0.08], [0.225, 1.41], [-0.225, 1.41]], "closed": True, "id": "casing", "bone": "unit", "role": "body", "z": 2, "filled": True, "tone": 0.55, "region": "casing", "material": "powder-coat"}
d.replace_obj(n, o2)
d.insert_after(n, d.obj(d.shape(100, "elevation", "feet")))
n = d.shape(50, "elevation", "fan-guard")
d.setv(sp.child(n, "points"), [[0.214, 0.447], [0.26, 0.447], [0.26, 1.153], [0.214, 1.153]])
spoke = {"id": "guard-spokes", "bone": "unit", "role": "detail", "z": 2.3, "filled": False, "tone": 0.3, "kind": "path", "d": "M 0.26 0.5 L 0.26 1.1", "penMm": 0.12, "region": "fan-guard", "material": "galvanised-steel"}
d.insert_after(n, spoke)
d.commit()
