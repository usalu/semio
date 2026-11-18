import sys, importlib, json
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/cargo-bike")
sc = sys.argv[1]; view = sys.argv[2]
want = {tuple(x.split(":")) for x in sys.argv[3].split(",")}
for it in d.at("asset", "representations", sc, "geometry", view)["items"]:
    o = json.loads(d.t[it["s"]:it["e"]])
    if (o["id"], o["bone"]) in want:
        d.edits.append((d.t.rfind(",", 0, it["s"]), it["e"], ""))
        print("removed", view, o["id"], o["bone"])
d.commit()
