import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/vintage-car-1929")
d.add_rects(200, "front", "frame", "frame", [("hub-l", 0.685, 0.3, 0.082, 0.14), ("hub-r", -0.767, 0.3, 0.082, 0.14)], role="detail")
d.commit()
