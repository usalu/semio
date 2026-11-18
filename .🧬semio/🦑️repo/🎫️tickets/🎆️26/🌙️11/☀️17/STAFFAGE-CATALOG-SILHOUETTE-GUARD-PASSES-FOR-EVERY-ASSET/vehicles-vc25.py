import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/vintage-car-1929")
s = d.solid(100, "hood-top")
sec = d.field(d.field(s, "sections"), "0")
d.set(sec, "at", [0.61, 1.157748, 0.0]); d.set(sec, "rx", 0.34)
s = d.solid(100, "radiator-shell")
d.set(s, "size", [0.12, 0.7, 0.44]); d.set(d.field(s, "transform"), "translation", [1.82, 0.91, 0.0])
for v in ("elevation", "plan"):
    s = d.shape(100, v, "radiator-shell"); d.set(s, "x", 1.76); d.set(s, "w", 0.12)
s = d.shape(100, "plan", "tail-pipe")
d.set(s, "x0", -1.75); d.set(s, "x1", -2.08); d.set(s, "r0", 0.032); d.set(s, "r1", 0.0275)
for sid, a, b in (("bracket-rear-l", -2.1, -1.95), ("bracket-rear-r", -2.1, -1.95), ("bracket-front-l", 2.05, 2.2), ("bracket-front-r", 2.05, 2.2)):
    try:
        s = d.shape(100, "plan", sid)
    except KeyError:
        continue
    d.set(s, "x0", a); d.set(s, "x1", b); d.set(s, "r0", 0.012); d.set(s, "r1", 0.012)
d.commit()
