import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/vintage-car-1929")
d.add_rects(50, "plan", "frame-r", "frame-r", [("crossmember-front", 1.725, -0.35, 0.05, 0.7), ("crossmember-mid", 0.175, -0.35, 0.05, 0.7), ("crossmember-rear", -1.925, -0.35, 0.05, 0.7)])
d.commit()
