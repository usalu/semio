import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-bus")
for sc, shift in ((50, 0.0126), (100, 0.0127)):
    for sid in ("leaf-m-aft", "leaf-m-fwd", "leaf-f-aft", "leaf-f-fwd", "leaf-m-aft-glass", "leaf-m-fwd-glass", "leaf-f-aft-glass", "leaf-f-fwd-glass"):
        try: s = d.solid(sc, sid)
        except KeyError: continue
        tr = d.obj(s)["transform"]["translation"]
        d.set(d.field(s, "transform"), "translation", [tr[0], tr[1], round(tr[2] + shift, 5)])
for sid in ("leaf-m-aft", "leaf-m-fwd", "leaf-f-aft", "leaf-f-fwd"):
    s = d.solid(200, sid)
    d.set(s, "size", [0.55, 2.12, 0.09])
for sid, z in (("glass-lower-l", 1.25), ("glass-lower-r", -1.25)):
    s = d.solid(200, sid)
    d.set(d.field(s, "transform"), "translation", [d.obj(s)["transform"]["translation"][0], 1.8, z])
s = d.solid(50, "hvac-louvre")
d.set(d.field(s, "transform"), "translation", [0.6, 0.158, 0])
d.commit()
