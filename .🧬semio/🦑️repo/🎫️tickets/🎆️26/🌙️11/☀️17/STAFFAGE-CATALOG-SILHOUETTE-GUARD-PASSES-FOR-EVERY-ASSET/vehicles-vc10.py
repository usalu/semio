import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/vintage-car-1929")
d.drawn_array(d.shape(50, "plan", "tub"), "points", [[-1.991, -0.621], [0.601, -0.621], [0.601, 0.621], [-1.991, 0.621]])
d.set(d.shape(50, "plan", "hood"), "r", 0.005)
s = d.shape(50, "plan", "tail-pipe")
d.set(s, "x0", -1.75); d.set(s, "x1", -2.08); d.set(s, "r0", 0.032); d.set(s, "r1", 0.0275)
for sid, a, b in (("bracket-rear-l", -2.1, -1.95), ("bracket-rear-r", -2.1, -1.95), ("bracket-front-l", 2.05, 2.2), ("bracket-front-r", 2.05, 2.2)):
    s = d.shape(50, "plan", sid)
    d.set(s, "x0", a); d.set(s, "x1", b); d.set(s, "r0", 0.012); d.set(s, "r1", 0.012)
d.commit()
