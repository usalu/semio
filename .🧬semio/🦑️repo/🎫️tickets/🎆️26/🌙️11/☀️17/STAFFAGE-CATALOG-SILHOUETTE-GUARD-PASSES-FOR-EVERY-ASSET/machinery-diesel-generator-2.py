"""Diesel generator: the roof silencer has the 0.285 m radius of its flange and clamps (it overlaps the roof and the stack base by 10 mm, so no hairline crack runs along the roof), drawn as the matching stadium; the stack hinge bracket and rain cap knuckle are drawn at 1:100 and 1:50."""
import sys, math
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/diesel-generator")
R, CY, X0, X1 = 0.285, 2.865, -1.725, -0.075
pts = []
for k in range(0, 13): t = -math.pi / 2 + math.pi * k / 12; pts.append([round(X1 + R * math.cos(t), 4), round(CY + R * math.sin(t), 4)])
for k in range(0, 13): t = math.pi / 2 + math.pi * k / 12; pts.append([round(X0 + R * math.cos(t), 4), round(CY + R * math.sin(t), 4)])
for sc in (200, 100, 50):
    n = d.solid(sc, "silencer"); d.setv(sp.child(n, "r0"), R); d.setv(sp.child(n, "r1"), R)
    d.setv(sp.child(d.shape(sc, "elevation", "silencer"), "points"), pts)
    n = d.shape(sc, "front", "silencer"); d.setv(sp.child(n, "rx"), R); d.setv(sp.child(n, "ry"), R)
    n = d.shape(sc, "plan", "silencer"); d.setv(sp.child(n, "x"), -2.01); d.setv(sp.child(n, "y"), -1.185); d.setv(sp.child(n, "w"), 2.22); d.setv(sp.child(n, "h"), 0.57)
    for sid in ("silencer-clamp", "silencer-clamp-2", "silencer-flange"):
        try: n = d.shape(sc, "elevation", sid)
        except KeyError: continue
        p = d.obj(n)["points"]
        d.setv(sp.child(n, "points"), [[x, 2.58 if y == 2.59 else 3.15 if y == 3.14 else y] for x, y in p])
for sc in (100, 50):
    tmpl = d.obj(d.shape(sc, "elevation", "exhaust-stack"))
    tmpl.pop("penMm", None)
    br = dict(tmpl); br.update({"id": "stack-hinge-bracket", "role": "detail", "points": [[0.25, -0.165], [0.33, -0.165], [0.33, -0.095], [0.25, -0.095]]})
    kn = {"kind": "ellipse", "cx": 0, "cy": 0, "rx": 0.025 if sc == 50 else 0.04, "ry": 0.025 if sc == 50 else 0.04, "id": "rain-cap-knuckle", "bone": "cap-flap", "role": "detail", "z": 2.75, "filled": True, "tone": 0.3, "region": "rain-cap", "material": "galvanised-steel"}
    d.insert_after(d.shape(sc, "elevation", "exhaust-stack"), br)
    d.insert_after(d.shape(sc, "elevation", "rain-cap"), kn)
d.commit()
