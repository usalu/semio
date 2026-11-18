import sys, importlib, json
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
slug, sc, view = sys.argv[1], sys.argv[2], sys.argv[3]
d = m.V("🚜️vehicles/" + slug)
pens = d.obj(d.at("asset", "representations", sc, "pens"))
n = 0
for it in d.at("asset", "representations", sc, "geometry", view)["items"]:
    o = json.loads(d.t[it["s"]:it["e"]])
    if o.get("role") == "detail" and pens.get("detail") is None:
        d.setv(d.field(it, "role"), "body"); n += 1; print("body", o["id"])
d.commit()
