import importlib.util, sys
from pathlib import Path
spec = importlib.util.spec_from_file_location("m", Path("🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧮️model/🐍️.py"))
m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
b = m.evaluate()
print({k: round(v, 2) for k, v in b.items()})
o = m.orientation_study()
print("min", o["local_minima"], [round(x) for x in o["local_minima_electricity"]], "max", o["local_maxima"], [round(x) for x in o["local_maxima_electricity"]])
print("glob", o["global_minimum"], "feas", o["feasible_minimum"], "ref", round(o["reference_electricity"]))
print("descent", [(d["theta"], round(d["electricity"])) for d in o["descent"]])
for t in (90,135,180,225,270,315,360,405,450):
    e = m.evaluate({**o["design"], "orientation": t}); print(t, round(e["heating"]), round(e["cooling"]), round(e["lighting"]), round(e["electricity"]))
s = m.surface()
print("surface min", s["minimum"]["design"]["wwr"], s["minimum"]["design"]["overhang"], round(s["minimum"]["electricity"]))
for g in (m.samples("grid"), m.samples("random"), m.samples("lhs")):
    print([round(x["electricity"]) for x in g], "best", round(min(x["electricity"] for x in g)))
f = m.factorial(); print([round(r["response"]) for r in f["runs"]], {k: round(v,1) for k,v in f["effects"].items()}, f["centre"])
for v in m.variants(): print(v["id"], round(v["intensity"],1), round(v["cost"]), round(v["daylight"],2), round(v["carbon"]))
for w in ({"electricity":.6,"cost":.1,"daylight":.2,"carbon":.1},{"electricity":.1,"cost":.5,"daylight":.1,"carbon":.3}):
    print([(r["id"], round(r["score"],2), r["rank"]) for r in m.index(w)])
p = m.pareto(); print(len(p["designs"]), p["front"], p["step"])
for d in sorted(p["designs"], key=lambda d: d["cost"]): print(d["id"], round(d["cost"]), round(d["electricity"]), "*" if d["id"] in p["front"] else "")
g = m.gradient_path(); print(len(g), [(round(x["design"]["wwr"],3), round(x["design"]["overhang"],3), round(x["electricity"],1)) for x in g])
q = m.pattern_path(); print(len(q), [(round(x["design"]["wwr"],3), round(x["design"]["overhang"],3), round(x["electricity"],1), x["step"]) for x in q])
ge = m.genetic(); print([round(x["best"]["electricity"],1) for x in ge["generations"]])
