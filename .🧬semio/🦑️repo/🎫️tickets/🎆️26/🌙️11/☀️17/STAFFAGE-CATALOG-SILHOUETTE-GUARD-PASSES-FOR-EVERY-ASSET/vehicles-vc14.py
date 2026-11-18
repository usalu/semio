import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/vintage-car-1929")
for sc in (50, 100):
    for sid in ("bracket-front", "bracket-rear"):
        s = d.shape(sc, "elevation", sid)
        d.set(s, "r0", 0.0225); d.set(s, "r1", 0.0225)
d.commit()
