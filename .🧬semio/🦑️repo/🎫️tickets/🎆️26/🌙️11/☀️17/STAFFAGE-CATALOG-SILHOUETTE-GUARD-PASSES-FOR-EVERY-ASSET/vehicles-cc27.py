import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-car")
s = d.shape(100, "elevation", "body-shell")
d.sub(d.field(s, "d"), "C 1.95,0.68 1.93,0.7 1.91,0.667 L 1.809,0.667 L 0.924,0.832", "C 1.95,0.68 1.945,0.7 1.93,0.701 L 1.878,0.701 L 1.878,0.78 L 0.924,0.832")
d.commit()
