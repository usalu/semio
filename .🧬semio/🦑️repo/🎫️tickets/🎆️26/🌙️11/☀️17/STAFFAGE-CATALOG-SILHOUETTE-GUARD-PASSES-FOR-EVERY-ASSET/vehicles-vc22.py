import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/vintage-car-1929")
s = d.shape(100, "front", "axle-f")
d.set(s, "x", -0.62); d.set(s, "w", 1.24)
d.to_poly(100, "front", "silencer", "silencer,tail-pipe,exhaust-pipe,exhaust-hanger,exhaust-downpipe,exhaust-manifold", "0.006")
d.set(d.shape(100, "front", "cabin"), "r", 0.005)
d.commit()
