import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-car")
for sid in ("headlamp-l", "headlamp-r"):
    d.set(d.solid(50, sid), "radii", [0.12, 0.06, 0.1])
d.commit()
