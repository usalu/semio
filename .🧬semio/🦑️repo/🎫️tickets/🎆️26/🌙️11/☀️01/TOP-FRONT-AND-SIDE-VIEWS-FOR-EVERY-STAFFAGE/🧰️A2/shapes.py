import json, sys, os
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import lib

aid, label = sys.argv[1], sys.argv[2]
views = sys.argv[3].split(",") if len(sys.argv) > 3 else ["elevation"]
scale = int(sys.argv[4]) if len(sys.argv) > 4 else 50
A = lib.Asset(aid)
ev = {q["label"]: q for q in json.load(open(A.evalfile(scale), encoding="utf-8"))}
for v in views:
    print("==", v, label, "scale", scale)
    for s in ev[label]["views"][v]:
        xs = [p[0] for r in s["rings"] for p in r]
        ys = [p[1] for r in s["rings"] for p in r]
        print("  %-18s %-14s z%-5s x[% .3f,% .3f] y[% .3f,% .3f]" % (s["id"], s["bone"], s["z"], min(xs), max(xs), min(ys), max(ys)))
