import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-car")
s = d.shape(200, "elevation", "body-shell")
d.sub(d.field(s, "d"), "C 1.95,0.68 1.93,0.7 1.91,0.662 L 1.808,0.662 L 0.923,0.827", "C 1.95,0.68 1.945,0.68 1.93,0.678 L 1.878,0.678 L 1.878,0.78 L 0.923,0.827")
d.commit()
