import importlib.util, itertools, sys
from pathlib import Path
spec = importlib.util.spec_from_file_location("m", Path("🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧮️model/🐍️.py"))
m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
st = m._frozen()[0]["algorithms"]
opt = m.surface()["minimum"]["electricity"]
found = []
for seed in range(1, 1500):
    st["genetic"]["lcg"]["seed"] = seed
    g = m._genetic.__wrapped__()["generations"]
    bests = [x["best"]["electricity"] for x in g]
    if bests[0] < 1.12 * opt or bests[-1] > opt * 1.015: continue
    improves = sum(1 for a, b in itertools.pairwise(bests) if b < a - 0.05)
    if improves < 3: continue
    better = worse = 0
    for x in g:
        for c in x["children"]:
            if any(abs(v) > 1e-12 for v in c["mutation"].values()):
                inh = m.evaluate({**c["design"], "wwr": c["design"]["wwr"] - c["mutation"]["wwr"], "overhang": c["design"]["overhang"] - c["mutation"]["overhang"]})["electricity"]
                better += c["electricity"] < inh - 1e-9; worse += c["electricity"] > inh + 1e-9
    crossover = sum(1 for x in g for c in x["children"] if c["genes"]["wwr"] != c["genes"]["overhang"])
    found.append((seed, round(bests[0]), [round(b) for b in bests], improves, better, worse))
print(len(found))
for f in found[:15]: print(f)
