import importlib.util, sys
from pathlib import Path
spec = importlib.util.spec_from_file_location("m", Path("🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧮️model/🐍️.py"))
m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
st = m._frozen()[0]["algorithms"]
def reset():
    for f in (m._gradient_path, m._pattern_path, m._genetic): pass
mode = sys.argv[1]
if mode == "pareto":
    p = m.pareto()
    for s in p["steps"]: print(s["from"], s["to"], round(s["cost"]), round(s["saved"],1), round(s["ratio"],2), round(s["payback"],1), round(s["payback_hi"],1))
    print(p["knee"], p["refinement"], p["third_example"], p["front3"], p["picks"], p["service_life"])
    lk = {d["id"]: d for d in p["designs"]}
    for k in ("winner","loser"): d = lk[p["dominated_example"][k]]; print(k, d["id"], round(d["cost"]), round(d["electricity"]), round(d["heating"]), round(d["cooling"]))
elif mode == "ga":
    for seed in range(1, 400):
        st["genetic"]["lcg"]["seed"] = seed
        m._genetic.cache_clear() if hasattr(m._genetic, "cache_clear") else None
        g = m._genetic.__wrapped__ if False else None
