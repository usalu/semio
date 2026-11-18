import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-car")
for sid in ("tail-lamp-l", "tail-lamp-r"):
    s = d.shape(50, "plan", sid)
    d.set(s, "x", -1.95); d.set(s, "w", 0.08)
d.commit()
