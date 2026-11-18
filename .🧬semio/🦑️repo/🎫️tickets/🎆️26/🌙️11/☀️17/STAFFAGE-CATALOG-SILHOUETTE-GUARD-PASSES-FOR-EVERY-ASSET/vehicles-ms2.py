import sys, importlib, json
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/motor-scooter")
s = d.solid(50, "fender-f")
prof = d.val("asset", "volumes", "50", "body")
prof = d.obj(s)["profile"]
n = len(prof)
for k in range(13, 26):
    d.setv(d.field(d.field(d.field(s, "profile"), str(k)), "1"), round(prof[k][1] - 0.022, 6))
d.commit()
