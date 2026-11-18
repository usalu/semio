import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/delivery-van")
spec = {50: (0.83, 0.88), 100: (0.83, 0.88), 200: (0.83, 0.88), 500: (0.8, 0.88)}
for sc, (a, b) in spec.items():
    for view in ("elevation", "plan"):
        for sid, lo, hi in (("rear-leaf-l", a, b), ("rear-leaf-r", -b, -a)):
            sol = d.field(d.shape(sc, view, sid), "solid")
            d.set(sol, "z0", lo); d.set(sol, "z1", hi)
for sid, zz in (("rear-hinge-r", 0.015), ("rear-hinge-l", -0.015)):
    s = d.solid(50, sid)
    d.set(s, "p0", [0, 0, zz]); d.set(s, "p1", [1.35, 0, zz])
d.commit()
