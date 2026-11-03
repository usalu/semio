import sys, os
A2 = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "🧰️A2")
sys.path.insert(0, A2)
import lib, jsonfmt

# usage: elevsolid.py <asset> <shape-id=halfwidth> ...   gives the elevation shape of every authored scale an extruded solid z0=-half..z1=half (one 3D definition: the side view is the projection of the carried profile)
aid = sys.argv[1]
spec = {k: float(v) for k, v in (a.split("=") for a in sys.argv[2:])}
path = os.path.join(lib.CAT, aid, "🔣️.json")
doc = jsonfmt.read(path)
n = 0
for rep in doc["asset"]["representations"].values():
    for s in rep.get("geometry", {}).get("elevation", []):
        if s["id"] in spec:
            s["solid"] = {"kind": "extruded", "z0": -spec[s["id"]], "z1": spec[s["id"]]}
            n += 1
jsonfmt.write(path, doc)
print("solids", n)
