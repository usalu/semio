import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-bus")
for sid in ("leaf-handrail-m-aft", "leaf-handrail-m-fwd", "leaf-handrail-f-aft", "leaf-handrail-f-fwd"):
    s = d.solid(50, sid)
    d.set(s, "p0", [0.04, 0.34, -1.26]); d.set(s, "p1", [0.51, 0.34, -1.26])
for sid in ("door-opening-m", "door-opening-f"):
    s = d.solid(50, sid)
    tr = d.obj(s)["transform"]["translation"]
    d.set(d.field(s, "transform"), "translation", [tr[0], tr[1], -1.2559])
d.commit()
