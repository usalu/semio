import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/vintage-car-1929")
d.add_rects(50, "elevation", "frame", "frame", [("body-mount-a", -1.5, 0.41, 0.1, 0.11), ("body-mount-b", -0.55, 0.41, 0.1, 0.11), ("body-mount-c", 0.35, 0.41, 0.1, 0.11), ("exhaust-hanger-1", -1.515, 0.285, 0.03, 0.08), ("exhaust-hanger-2", -0.615, 0.285, 0.03, 0.08), ("exhaust-hanger-3", 0.485, 0.285, 0.03, 0.08), ("running-bracket-1", -0.825, 0.33, 0.05, 0.08), ("running-bracket-2", -0.025, 0.33, 0.05, 0.08), ("running-bracket-3", 0.775, 0.33, 0.05, 0.08)])
d.commit()
