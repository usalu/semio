import sys, importlib, json
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/cargo-bike")
sc, view, sid, dy, above = sys.argv[1], sys.argv[2], sys.argv[3], float(sys.argv[4]), float(sys.argv[5])
it = d.shape(int(sc), view, sid)
o = d.obj(it)
o["points"] = [[p[0], round(p[1] - dy, 4) if p[1] > above else p[1]] for p in o["points"]]
d.replace_shape(it, o)
d.commit()
