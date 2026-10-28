import importlib.util
from pathlib import Path
spec = importlib.util.spec_from_file_location("m", Path("🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧮️model/🐍️.py"))
m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
g = m.gradient_path(); print([(round(x["design"]["wwr"],2), round(x["design"]["overhang"],2), round(x["electricity"],1), x["evaluations"]) for x in g]); print(g[0]["slope"], g[0]["probes"][0]["direction"])
p = m.pattern_path(); print([(round(x["design"]["wwr"],3), round(x["design"]["overhang"],3), round(x["electricity"],1), x["moved"], x["evaluations"]) for x in p])
c = m.convergence(); print(c["budget"], c["map_minimum"], c["runs_map"], {k: (len(v), round(v[-1][1],1), [round(v[i][1]) for i in (0,4,9,19,29,39,len(v)-1)]) for k, v in c["series"].items()})
f = m.fractional(); print(f["alias_contrast"], f["true"], [r["response"] for r in f["runs"]])
d = m.sampling_diagnostics(); print(d)
print(m.lhs_strata()["index"][:2], m.samples("lhs")[0]["strata_index"], m.levels())
print(min(x["electricity"] for x in m.samples("grid")), m.surface()["minimum"]["electricity"])
o = m.orientation_study(); print(o["sun"], o["start"], o["descent"][-1]["theta"], len(o["samples"]))
