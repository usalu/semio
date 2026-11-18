import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/vintage-car-1929")
print(d.rects_from_volume(200, "elevation", "frame", "tub", ["body-mount-l", "running-bracket-l", "bracket-front-l", "bracket-rear-l", "spare-mount"]))
d.commit()
