"""Forklift 1:100 and 1:50: tyres of the front and plan drawn as the revolve profile they are (chamfered tread corners), plan chassis and counterweight with the square corners of the extrusions, seat back to its true height of 1.66 m in the front, pallet runners in the front."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/forklift")
def retype(o, pts):
    out = {}
    for k, v in o.items():
        if k == "kind": out[k] = "poly"; out["points"] = pts; out["closed"] = True
        elif k not in ("x", "y", "w", "h", "r"): out[k] = v
    return out
FRONT = [(-.05, .29), (.05, .29), (.085, .2697), (.1, .232), (.1, -.232), (.085, -.2697), (.05, -.29), (-.05, -.29), (-.085, -.2697), (-.1, -.232), (-.1, .232), (-.085, .2697)]
REAR = [(-.04, .23), (.04, .23), (.068, .2139), (.08, .184), (.08, -.184), (.068, -.2139), (.04, -.23), (-.04, -.23), (-.068, -.2139), (-.08, -.184), (-.08, .184), (-.068, .2139)]
for sc in (100, 50):
    for sid, lat, cy, prof in (("tyre-f-left", .5, .29, FRONT), ("tyre-f-right", -.5, .29, FRONT), ("tyre-r-left", .24, .23, REAR), ("tyre-r-right", -.24, .23, REAR)):
        n = d.shape(sc, "front", sid)
        d.replace_obj(n, retype(d.obj(n), [[round(lat + a, 4), round(cy + r, 4)] for a, r in prof]))
    for sid, lat in (("tyre-f-left", .5), ("tyre-f-right", -.5)):
        n = d.shape(sc, "plan", sid)
        d.replace_obj(n, retype(d.obj(n), [[round(r, 4), round(lat + a, 4)] for a, r in FRONT]))
    d.setv(sp.child(d.shape(sc, "plan", "chassis-body"), "r"), 0)
    d.setv(sp.child(d.shape(sc, "plan", "counterweight-body"), "r"), 0)
    s = d.shape(sc, "front", "seat-back")
    d.setv(sp.child(s, "y"), 1.03); d.setv(sp.child(s, "h"), 0.63)
    n = d.shape(sc, "front", "pallet-deck"); o = d.obj(n)
    o["id"] = "pallet-runner"; o["y"] = 0; o["h"] = 0.1
    d.insert_after(n, o)
d.commit()
