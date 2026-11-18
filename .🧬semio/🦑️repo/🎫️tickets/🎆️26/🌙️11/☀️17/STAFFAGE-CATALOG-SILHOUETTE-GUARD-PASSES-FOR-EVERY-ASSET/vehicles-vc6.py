import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/vintage-car-1929")
rects = []
for i, x in enumerate((-0.825, -0.025, 0.775), 1):
    rects += [(f"running-bracket-{i}-l", x, 0.41, 0.05, 0.27), (f"running-bracket-{i}-r", x, -0.68, 0.05, 0.27)]
for i, x in enumerate((0.9, 1.4), 1):
    rects += [(f"engine-bearer-{i}-l", x, 0.17, 0.1, 0.22), (f"engine-bearer-{i}-r", x, -0.39, 0.1, 0.22)]
rects += [("spare-mount", 0.84, -0.52, 0.04, 0.2)]
d.add_rects(50, "plan", "frame-r", "frame-r", rects)
d.commit()
