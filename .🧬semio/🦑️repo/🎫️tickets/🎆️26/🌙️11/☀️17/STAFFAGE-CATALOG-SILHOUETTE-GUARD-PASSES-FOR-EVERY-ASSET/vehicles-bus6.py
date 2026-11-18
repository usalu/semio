import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-bus")
for sc in (50, 100, 200, 500):
    for sid in ("tyre-rl", "tyre-fl", "tyre-rr", "tyre-fr"):
        try: s = d.shape(sc, "front", sid)
        except KeyError: continue
        if d.obj(s).get("r", 0) > 0.05: d.set(s, "r", 0.035)
d.commit()
