import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/cargo-bike")
sc = int(sys.argv[1]); shift = float(sys.argv[2])
for sid in sys.argv[3].split(","):
    s = d.solid(sc, sid)
    prof = d.obj(s)["profile"]
    for k, p in enumerate(prof):
        d.setv(d.field(d.field(d.field(s, "profile"), str(k)), "1"), round(p[1] - shift, 6))
d.commit()
