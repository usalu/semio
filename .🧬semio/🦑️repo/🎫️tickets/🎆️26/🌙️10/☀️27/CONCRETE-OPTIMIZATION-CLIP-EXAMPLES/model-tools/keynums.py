import importlib.util
from pathlib import Path
spec = importlib.util.spec_from_file_location("m", Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧮️model/🐍️.py"))
m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
b = m.evaluate(); print({k: round(v, 2) for k, v in b.items() if k in ("heating","cooling","lighting","electricity","intensity","shading_winter","shading_summer","solar_winter","solar_summer","daylight","cost")})
for d in (.3, .6, 1.0, 1.2): e = m.evaluate({"overhang": d}); print(d, round(e["shading_winter"], 3), round(e["shading_summer"], 3))
p = m.pareto(); print(p["front"], p["knee"])
print([round(s["payback"], 1) for s in p["steps"]])
c = m.convergence(); print({k: (round(v[14][1]), round(v[29][1]), round(v[-1][1])) for k, v in c["series"].items()}, round(c["map_minimum"], 1))
print(m.sampling_diagnostics())
