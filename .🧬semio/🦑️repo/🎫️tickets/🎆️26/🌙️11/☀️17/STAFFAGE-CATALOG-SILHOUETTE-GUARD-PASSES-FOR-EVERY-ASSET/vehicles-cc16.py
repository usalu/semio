import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-car")
for sid in ("tyre-fl", "tyre-fr", "tyre-rl", "tyre-rr"):
    d.set(d.shape(50, "front", sid), "r", 0.03)
d.commit()
