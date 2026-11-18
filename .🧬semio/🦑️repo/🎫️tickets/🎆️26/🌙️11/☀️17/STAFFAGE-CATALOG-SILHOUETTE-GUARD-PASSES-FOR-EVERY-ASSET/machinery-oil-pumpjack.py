"""Oil pumpjack: skid cross members fill the skid height (no 10 mm slits above and below), motor feet overlap the motor by 20 mm (no hairline crack), crank shaft drawn in the front and plan, polished rod drawn as a pen stroke in the plan so the slender rod no longer follows the 2.4 m wide slab strip."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/oil-pumpjack")
CROSS = {200: (0.22, 0.11), 100: (0.235, 0.1175), 50: (0.24, 0.12)}
for sc, (h, y) in CROSS.items():
    n = d.solid(sc, "skid-cross"); o = d.obj(n)
    d.setv(sp.child(n, "size"), [o["size"][0], h, o["size"][2]])
    d.setv(sp.child(sp.child(n, "transform"), "translation"), [o["transform"]["translation"][0], y, 0.0])
for sc in (500, 200, 100, 50):
    try: n = d.solid(sc, "motor-feet")
    except KeyError: continue
    o = d.obj(n)
    if o["size"] == [1.0, 0.1, 0.9]:
        d.setv(sp.child(n, "size"), [1.0, 0.12, 0.9]); d.setv(sp.child(sp.child(n, "transform"), "translation"), [0.0, -0.29, 0.0])
for sc in (500, 200, 100, 50):
    for v, rect in (("front", (-0.8, 1.61, 1.6, 0.28)), ("plan", (2.56, -0.8, 0.28, 1.6))):
        try: tmpl = d.obj(d.shape(sc, v, "crank-arm-l")); n = d.shape(sc, v, "crank-arm-l")
        except KeyError: print("no crank-arm-l", sc, v); continue
        o = {k: x for k, x in tmpl.items() if k != "solid"}
        o["id"] = "crank-hub"; o["x"], o["y"], o["w"], o["h"] = rect
        d.insert_after(n, o)
    n = d.shape(sc, "plan", "polished-rod"); t = d.obj(n)
    o = {"id": "polished-rod-axis", "bone": t["bone"], "role": t["role"], "z": t["z"] + 0.1, "filled": False, "tone": t["tone"], "kind": "path", "d": "M -3 -0.001 L -3 0.001", "penMm": round(50.0 / sc, 4), "region": t["region"], "material": t["material"]}
    d.insert_after(n, o)
d.commit()
