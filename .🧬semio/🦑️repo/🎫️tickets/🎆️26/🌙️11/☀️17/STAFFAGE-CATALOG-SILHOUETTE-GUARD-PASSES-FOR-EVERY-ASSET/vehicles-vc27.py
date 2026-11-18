import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/vintage-car-1929")
print(d.rects_from_volume(100, "plan", "frame-r", "frame-r", ["tail-lamp"], role="detail"))
d.commit()
