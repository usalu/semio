import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/vintage-car-1929")
rects = []
for n, x in (("f", 1.47), ("r", -1.53)):
    rects += [(f"axle-{n}-l", x, 0.41, 0.06, 0.19), (f"axle-{n}-r", x, -0.6, 0.06, 0.19), (f"axle-{n}-mid", x, -0.35, 0.06, 0.7)]
d.add_rects(50, "plan", "frame-r", "frame-r", rects, role="detail")
d.commit()
