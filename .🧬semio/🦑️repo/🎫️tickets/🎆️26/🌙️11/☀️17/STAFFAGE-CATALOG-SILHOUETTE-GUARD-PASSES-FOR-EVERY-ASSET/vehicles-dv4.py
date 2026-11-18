import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/delivery-van")
for sc in (50, 100, 200, 500):
    for sid in ("tyre-rl", "tyre-fl", "tyre-rr", "tyre-fr"):
        try:
            s = d.shape(sc, "front", sid)
        except KeyError:
            continue
        o = d.obj(s)
        if o.get("r", 0) > 0.03: d.set(s, "r", 0.02)
d.commit()
