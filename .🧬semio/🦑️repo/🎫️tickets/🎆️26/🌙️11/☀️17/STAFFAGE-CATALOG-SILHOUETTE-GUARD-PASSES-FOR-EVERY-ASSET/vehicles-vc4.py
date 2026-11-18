import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/vintage-car-1929")
s = d.shape(50, "front", "axle-f")
d.set(s, "x", -0.62); d.set(s, "w", 1.24)
d.commit()
