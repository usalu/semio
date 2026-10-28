import importlib.util, itertools
from pathlib import Path
spec = importlib.util.spec_from_file_location("m", Path("🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧮️model/🐍️.py"))
m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
p = m.pareto(); lk = {d["id"]: d for d in p["designs"]}
front = [lk[i] for i in p["front"]]
costs = [d["cost"] for d in p["designs"]]; els = [d["electricity"] for d in p["designs"]]
print("ranges", min(costs), max(costs), min(els), max(els))
l = lk["w70-t30-d0"]; w = lk["w30-t12-d0"]
print("sep", (l["cost"]-w["cost"])/(max(costs)-min(costs)), (l["electricity"]-w["electricity"])/(max(els)-min(els)))
for member in front:
    for wwr, d, t in itertools.product((.25,.35,.45,.55), (.3,.45,.75,.9,1.0), (.12,.15,.2,.25)):
        e = m.evaluate({"wwr": wwr, "overhang": d, "insulation": t})
        if e["cost"] < member["cost"] and e["electricity"] < member["electricity"]:
            print(member["id"], round(member["cost"]), round(member["electricity"],1), "<-", wwr, d, t, round(e["cost"]), round(e["electricity"],1))
