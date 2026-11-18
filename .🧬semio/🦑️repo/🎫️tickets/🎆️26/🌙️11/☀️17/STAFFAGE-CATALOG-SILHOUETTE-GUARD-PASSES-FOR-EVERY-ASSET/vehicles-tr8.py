import sys, importlib, math
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/tram")
for sc in (50, 100, 200, 500):
    try: s = d.solid(sc, "windscreen-slope")
    except KeyError: continue
    o = d.obj(s)
    q = o["transform"]["rotation"]; th = 2 * math.atan2(q[2], q[3])
    extra = 0.05
    tr = o["transform"]["translation"]
    d.set(s, "size", [round(o["size"][0] + extra, 4), o["size"][1], o["size"][2]])
    d.set(d.field(s, "transform"), "translation", [round(tr[0] - extra / 2 * math.cos(th), 5), round(tr[1] - extra / 2 * math.sin(th), 5), tr[2]])
    print(sc, o["size"], th)
d.commit()
