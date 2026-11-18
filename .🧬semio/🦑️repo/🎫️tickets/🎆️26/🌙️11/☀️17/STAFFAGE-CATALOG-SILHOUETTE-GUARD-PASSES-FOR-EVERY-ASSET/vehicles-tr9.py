import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/tram")
for sc in (50, 100, 200, 500):
    try: s = d.shape(sc, "elevation", "pan-head")
    except KeyError: continue
    o = d.obj(s)
    if o["kind"] == "capsule" and abs(o["x1"] - 0.25) < 1e-6:
        d.set(s, "x0", -0.28); d.set(s, "x1", 0.28)
d.commit()
