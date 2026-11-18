import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/vintage-car-1929")
d.add_shapes(100, "plan", "frame-r", "frame-r", [{"kind": "capsule", "x0": x0, "y0": y, "x1": x1, "y1": y, "r0": 0.012, "r1": 0.012, "id": i} for i, x0, x1, y in (("bracket-front-l", 2.2, 2.05, 0.3), ("bracket-front-r", 2.2, 2.05, -0.3), ("bracket-rear-l", -2.1, -1.95, 0.3), ("bracket-rear-r", -2.1, -1.95, -0.3))], role="detail")
d.commit()
