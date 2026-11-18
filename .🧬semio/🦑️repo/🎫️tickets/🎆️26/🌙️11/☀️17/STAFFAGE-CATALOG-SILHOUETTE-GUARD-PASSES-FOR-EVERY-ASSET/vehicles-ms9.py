import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/motor-scooter")
s = d.solid(100, "fender-f")
prof = d.obj(s)["profile"]
for k in range(13, 26):
    d.setv(d.field(d.field(d.field(s, "profile"), str(k)), "1"), round(prof[25 - k][1] - 0.03, 6))
d.commit()
