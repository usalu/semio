import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/cargo-bike")
sc = int(sys.argv[1]); hw = float(sys.argv[2])
for view in sys.argv[3].split(","):
    s = d.shape(sc, view, "hub-f")
    sol = d.field(s, "solid")
    d.setv(d.field(sol, "z0"), -hw)
    d.setv(d.field(sol, "z1"), hw)
d.commit()
