import sys; sys.path.insert(0, "."); from importlib import import_module
lib = import_module("spread-lib")
sp = lib.sp
sc, z = int(sys.argv[1]), float(sys.argv[2])
d = lib.mv.D("🐕️animals/horse")
s = sp.child(d.shape(sc, "plan", "head"), "solid")
d.setv(sp.child(s, "z0"), -z); d.setv(sp.child(s, "z1"), z)
d.commit()
