import importlib.util, itertools
from pathlib import Path
spec = importlib.util.spec_from_file_location("m", Path("🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧮️model/🐍️.py"))
m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
st = m._frozen()[0]["pareto"]
def front(ws, ts, ds):
    pts = [(m.evaluate({"wwr": w, "insulation": t, "overhang": d}), (w,t,d)) for w, t, d in itertools.product(ws, ts, ds)]
    f = [(p["cost"], p["electricity"], k) for p, k in pts if not any((q["cost"] <= p["cost"] and q["electricity"] <= p["electricity"] and (q["cost"] < p["cost"] or q["electricity"] < p["electricity"])) for q, _ in pts)]
    return sorted(f)
for ws in ([.2,.35,.5,.65,.8], [.2,.3,.4,.5,.6,.7,.8]):
  for ts in ([.12,.2,.3], [.1,.2,.3]):
    for ds in ([0,.8], [0,.6], [0,.6,1.2]):
        f = front(ws, ts, ds); print(len(ws)*len(ts)*len(ds), len(f), ws, ts, ds)
