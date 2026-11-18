import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/motor-scooter")
for sid, a, b in (("fork-l", 0.062, 0.094), ("fork-r", -0.094, -0.062)):
    s = d.shape(50, "plan", sid)
    sol = d.field(s, "solid"); d.set(sol, "z0", a); d.set(sol, "z1", b)
    d.set(s, "r0", 0.016); d.set(s, "r1", 0.016)
d.commit()
