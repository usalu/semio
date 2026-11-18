import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-bicycle")
for sid in ("fender-f", "fender-r"):
    s = d.solid(50, sid)
    prof = d.obj(s)["profile"]
    for k, p in enumerate(prof):
        d.setv(d.field(d.field(d.field(s, "profile"), str(k)), "1"), round(p[1] - 0.013, 6))
s = d.solid(50, "stem-neck")
secs = d.obj(s)["sections"]
for k, sec in enumerate(secs):
    at = sec["at"]
    d.set(d.field(d.field(s, "sections"), str(k)), "at", [round(at[0] * 0.985, 6), round(at[1] * 0.985, 6), at[2]])
d.commit()
