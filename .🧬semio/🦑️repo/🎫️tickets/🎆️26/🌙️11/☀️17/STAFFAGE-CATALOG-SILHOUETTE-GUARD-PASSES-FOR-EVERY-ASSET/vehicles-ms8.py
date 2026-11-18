import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/motor-scooter")
for sc in (100,):
    s = d.solid(sc, "fender-f")
    prof = d.obj(s)["profile"]
    print(sc, len(prof), prof[12:15])
    for k in range(13, len(prof) - 1):
        d.setv(d.field(d.field(d.field(s, "profile"), str(k)), "1"), round(prof[k][1] - 0.022, 6))
d.commit()
