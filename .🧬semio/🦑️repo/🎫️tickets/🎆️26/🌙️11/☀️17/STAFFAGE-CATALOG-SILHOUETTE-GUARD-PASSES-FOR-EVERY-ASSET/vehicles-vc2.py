import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/vintage-car-1929")
d.add_shapes(50, "elevation", "exhaust-pipe", "exhaust-pipe", [{"kind": "capsule", "x0": 0.95, "y0": 0.84, "x1": 1.5, "y1": 0.84, "r0": 0.03, "r1": 0.03, "id": "exhaust-manifold"}, {"kind": "capsule", "x0": 0.95, "y0": 0.84, "x1": 0.93, "y1": 0.28, "r0": 0.027, "r1": 0.027, "id": "exhaust-downpipe"}])
d.commit()
