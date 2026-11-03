import json, sys, os
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import lib

aid = sys.argv[1]
labels = sys.argv[2].split(",")
A = lib.Asset(aid)
ev = {q["label"]: q for q in json.load(open(A.evalfile(50), encoding="utf-8"))}
print("%-14s %s" % ("bone", "  ".join("%-27s" % l for l in ["rest"] + labels)))
for b in A.rest:
    cells = ["(%.3f,%.3f)>(%.3f,%.3f)" % A.rest[b]]
    for l in labels:
        cells.append("(%.3f,%.3f)>(%.3f,%.3f)" % tuple(ev[l]["bones"][b]))
    print("%-14s %s" % (b, "  ".join(cells)))
