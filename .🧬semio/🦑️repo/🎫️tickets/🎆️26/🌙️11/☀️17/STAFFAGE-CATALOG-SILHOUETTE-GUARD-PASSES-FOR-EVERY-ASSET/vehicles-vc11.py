import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/vintage-car-1929")
s = d.solid(50, "hood-top")
d.set(d.field(d.field(s, "sections"), "0"), "rx", 0.34)
d.set(d.shape(50, "plan", "hood"), "r", 0.015)
d.commit()
