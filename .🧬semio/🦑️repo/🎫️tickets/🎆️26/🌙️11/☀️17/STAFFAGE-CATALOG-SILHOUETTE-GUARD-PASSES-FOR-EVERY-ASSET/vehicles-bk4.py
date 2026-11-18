import sys, importlib, json
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-bicycle")
for v in ("front", "plan"):
    s = d.shape(200, v, "fork")
    o = d.obj(s)
    def leg(sid, a, b):
        n = json.loads(json.dumps(o)); n["id"] = sid; n["solid"] = {**o["solid"], "z0": a, "z1": b}; return n
    d.replace_shape(s, leg("fork-l", 0.038, 0.052))
    ind = d.indent(s)
    d.edits.append((s["e"], s["e"], ",\n" + ind + json.dumps(leg("fork-r", -0.052, -0.038), ensure_ascii=False, indent=1).replace("\n", "\n" + ind)))
d.commit()
