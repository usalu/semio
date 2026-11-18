import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/vintage-car-1929")
s = d.solid(50, "hood-top")
d.set(d.field(d.field(s, "sections"), "0"), "at", [0.61, 1.157748, 0.0])
d.commit()
