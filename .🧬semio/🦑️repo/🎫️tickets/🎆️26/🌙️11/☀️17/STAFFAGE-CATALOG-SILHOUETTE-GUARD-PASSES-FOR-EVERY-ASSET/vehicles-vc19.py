import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/vintage-car-1929")
d.add_rects(200, "front", "frame", "frame", [("axle-f", -0.62, 0.34, 1.24, 0.06), ("running-board-l", 0.62, 0.4, 0.18, 0.06), ("running-board-r", -0.8, 0.4, 0.18, 0.06), ("body-mount", -0.415, 0.41, 0.83, 0.11)], role="detail")
d.commit()
