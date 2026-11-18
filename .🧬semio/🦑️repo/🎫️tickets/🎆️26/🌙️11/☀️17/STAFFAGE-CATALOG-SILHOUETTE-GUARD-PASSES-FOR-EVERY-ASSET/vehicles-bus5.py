import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-bus")
for sc in (50, 100):
    for sid in ("battery-pack", "hvac-unit"):
        s = d.solid(sc, sid)
        secs = d.obj(s)["sections"]
        for k in (1, 2):
            sec = d.field(d.field(s, "sections"), str(k))
            at = secs[k]["at"]; ry = secs[k]["ry"]
            d.set(sec, "at", [at[0], round(at[1] - 0.01, 4), at[2]]); d.set(sec, "ry", round(ry + 0.01, 4))
    for sid, y, h in (("battery-pack", 2.965, 0.155), ("hvac-unit", 2.965, 0.175)):
        s = d.shape(sc, "front", sid); d.set(s, "y", y); d.set(s, "h", h)
d.commit()
