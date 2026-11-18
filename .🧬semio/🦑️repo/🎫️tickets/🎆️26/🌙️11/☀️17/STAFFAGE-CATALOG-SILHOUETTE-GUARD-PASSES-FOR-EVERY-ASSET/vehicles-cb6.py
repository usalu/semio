import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/cargo-bike")
sc = int(sys.argv[1])
def legs(z):
    for sid, sign in (("fork-leg-l", 1), ("fork-leg-r", -1)):
        s = d.solid(sc, sid)
        for k in (0, 1):
            d.setv(d.field(d.field(d.field(d.field(s, "sections"), str(k)), "at"), "2"), sign * z)
sp = importlib.import_module("sil-span")
import json
head = json.loads(sp.head_text("🚜️vehicles/cargo-bike"))["asset"]["volumes"][str(sc)]["body"]
hv = {s["id"]: s for s in head}
def shift(ids, dy):
    for sid in ids:
        if sid not in hv: continue
        s = d.solid(sc, sid)
        for k, p in enumerate(hv[sid]["profile"]):
            d.setv(d.field(d.field(d.field(s, "profile"), str(k)), "1"), round(p[1] - dy, 6))
def stand(dx):
    for sid in ("stand-leg-l", "stand-leg-r"):
        s = d.solid(sc, sid)
        d.setv(d.field(d.field(s, "p1"), "0"), round(0.3433 - dx, 4))
    s = d.solid(sc, "stand-foot")
    d.setv(d.field(d.field(d.field(s, "transform"), "translation"), "0"), round(0.3303 - dx, 4))
if sc == 50:
    stand(0.022)
    shift(["fender-r", "fender-f"], 0.008)
    legs(0.05)
def planks(x):
    s = d.shape(sc, "elevation", "box-planks")
    d.setv(d.field(s, "d"), f"M 0,0.2 L {x},0.2 M 0,0.4 L 1.0,0.4")
if sc in (100, 200):
    planks(0.73)
if sc == 100:
    shift(["fender-r", "fender-f"], 0.025)
d.commit()
