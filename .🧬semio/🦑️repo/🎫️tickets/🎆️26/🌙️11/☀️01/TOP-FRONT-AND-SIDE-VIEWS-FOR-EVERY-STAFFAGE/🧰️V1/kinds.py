import collections, sys
from lib import *

for a in sys.argv[1:]:
    d = load(a)
    r, bones = rest(d)
    print("==", a)
    for sc, rep in d["asset"]["representations"].items():
        if rep.get("status") != "authored":
            continue
        c = collections.Counter(s["kind"] for s in rep["geometry"]["elevation"])
        print(" ", sc, dict(c), "roles", dict(collections.Counter(s["role"] for s in rep["geometry"]["elevation"])), "hatches", [h.get("id") for h in rep.get("hatches", [])])
        for s in rep["geometry"]["elevation"]:
            if s["kind"] == "path":
                print("    path", s["id"], s["bone"], s["d"][:90])
                break
