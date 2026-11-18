import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/tram")
drop = ("kind", "points", "closed", "penMm", "filled")
for sc in (50,):
    for sid in ("pan-lower", "pan-upper"):
        s = d.shape(sc, "elevation", sid)
        o = d.obj(s)
        (x0, y0), (x1, y1) = o["points"]
        d.replace_shape(s, {"kind": "capsule", "x0": x0, "y0": y0, "x1": x1, "y1": y1, "r0": 0.03, "r1": 0.03, **{k: v for k, v in o.items() if k not in drop}, "filled": True})
d.commit()
