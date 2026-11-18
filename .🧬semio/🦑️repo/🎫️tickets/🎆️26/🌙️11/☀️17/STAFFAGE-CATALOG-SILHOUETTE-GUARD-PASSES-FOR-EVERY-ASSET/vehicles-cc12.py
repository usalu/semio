import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-car")
d.set(d.shape(50, "front", "silencer"), "r", 0.075)
d.commit()
