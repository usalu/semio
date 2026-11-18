import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
sp = importlib.import_module("sil-span")
d = m.V("🚜️vehicles/tram")
h = sp.parse(sp.head_text("🚜️vehicles/tram"))
import json
ht = sp.head_text("🚜️vehicles/tram")
for sid in ("pan-lower", "pan-upper"):
    node = d.shape(50, "elevation", sid)
    hnode = None
    for it in sp.walk(h, ["asset", "representations", "50", "geometry", "elevation"])["items"]:
        for k, v in it["pairs"]:
            if k == "id" and json.loads(ht[v["s"]:v["e"]]) == sid: hnode = it
    d.edits.append((node["s"], node["e"], ht[hnode["s"]:hnode["e"]]))
d.commit()
d = m.V("🚜️vehicles/tram")
for sid in ("pan-lower", "pan-upper"):
    s = d.shape(50, "elevation", sid)
    d.set(s, "penMm", float(sys.argv[1]))
d.commit()
