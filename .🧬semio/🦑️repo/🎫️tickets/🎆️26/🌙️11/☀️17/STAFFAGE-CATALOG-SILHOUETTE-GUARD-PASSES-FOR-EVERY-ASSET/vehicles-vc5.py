import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/vintage-car-1929")
d.to_poly(50, "front", "silencer", "silencer,tail-pipe,exhaust-pipe,exhaust-hanger,exhaust-downpipe,exhaust-manifold")
d.commit()
