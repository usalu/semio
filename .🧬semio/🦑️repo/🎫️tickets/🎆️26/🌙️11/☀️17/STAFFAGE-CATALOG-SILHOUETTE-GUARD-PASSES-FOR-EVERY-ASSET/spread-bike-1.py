import sys; sys.path.insert(0, "."); from importlib import import_module
lib = import_module("spread-lib")
sp = lib.sp
d = lib.mv.D("🚜️vehicles/city-bicycle")
for sid, z in (("handlebar-l", 0.31), ("handlebar-r", -0.31)):
    n = sp.child(sp.child(d.shape(200, "plan", sid), "to"), "z")
    d.setv(n, z)
d.commit()
