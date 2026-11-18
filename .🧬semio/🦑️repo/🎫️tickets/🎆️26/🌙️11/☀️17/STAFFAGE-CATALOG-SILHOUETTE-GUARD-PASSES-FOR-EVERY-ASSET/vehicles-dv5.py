import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/delivery-van")
for sc in (50, 100):
    s = d.solid(sc, "tail-pipe")
    d.set(s, "p0", [-2.63, 0.42, -0.31]); d.set(s, "p1", [-2.33, 0.42, -0.31])
    s = d.shape(sc, "elevation", "tail-pipe")
    d.set(s, "x0", -2.33); d.set(s, "x1", -2.63)
    s = d.shape(sc, "plan", "exhaust-tip")
    d.set(s, "x", -2.67); d.set(s, "w", 0.38)
s = d.solid(50, "bumper")
d.set(d.field(d.field(s, "sections"), "2"), "at", [2.66, 0.46, 0])
d.commit()
