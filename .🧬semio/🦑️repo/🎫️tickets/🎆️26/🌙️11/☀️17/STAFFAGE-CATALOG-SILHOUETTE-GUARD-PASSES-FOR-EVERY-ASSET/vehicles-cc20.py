import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-car")
for sid, z in (("headlamp-l", 0.6), ("headlamp-r", -0.6)):
    s = d.solid(50, sid)
    d.set(s, "radii", [0.12, 0.07, 0.1])
    d.set(d.field(s, "transform"), "translation", [1.82, 0.715, z])
d.commit()
