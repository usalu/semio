import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/vintage-car-1929")
s = d.solid(50, "radiator-shell")
d.set(s, "size", [0.12, 0.7, 0.44])
d.set(d.field(s, "transform"), "translation", [1.82, 0.91, 0.0])
for v in ("elevation", "plan"):
    s = d.shape(50, v, "radiator-shell")
    d.set(s, "x", 1.76); d.set(s, "w", 0.12)
d.commit()
