import importlib.util, itertools
from pathlib import Path
spec = importlib.util.spec_from_file_location("m", Path("🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧮️model/🐍️.py"))
m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
st = m.load_study()
W = st["weights"]
for name, w in W.items():
    print(name, [(r["id"], round(r["score"],3), r["rank"]) for r in m.index(w)])
    print(name, "6", [(r["id"], round(r["score"],3), r["rank"]) for r in m.index(w, ids=[v["id"] for v in m.variants(True)])])
print(m.reweigh(W["balanced"], "cost", .6))
for v in m.variants(True): print(v["id"], round(v["intensity"],2), round(v["cost"]), round(v["daylight"],2), round(v["carbon"]))
# naive sum
for scale in (1, 1000):
    rows = []
    for v in m.variants():
        rows.append((v["intensity"] + v["cost"]/scale + v["carbon"]/scale + (-v["daylight"]), v["id"]))
    print(scale, [i for _, i in sorted(rows)])
# winners per criterion
for c in m.criteria():
    key = c["key"]; vs = m.variants()
    best = (min if c["direction"]=="min" else max)(vs, key=lambda v: v[key]); print(c["id"], best["id"])
