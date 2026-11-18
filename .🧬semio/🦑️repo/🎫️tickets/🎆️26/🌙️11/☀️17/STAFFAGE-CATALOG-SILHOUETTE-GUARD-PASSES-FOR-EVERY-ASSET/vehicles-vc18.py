import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/vintage-car-1929")
d.add_poly(200, "front", "frame", "frame", "exhaust", "silencer,tail-pipe,exhaust-pipe,exhaust-downpipe,exhaust-manifold,exhaust-hanger", "0.01", role="detail")
d.commit()
