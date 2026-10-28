import importlib.util, json
from pathlib import Path
clip = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip")
spec = importlib.util.spec_from_file_location("model", clip / "🧮️model/🐍️.py")
m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
s = m.surface()["minimum"]; print(s)
r = m.evaluate(s["design"]); print(r["cost"], r["electricity"])
for d in m.pareto()["designs"]:
    if d["cost"] > r["cost"] and d["electricity"] > r["electricity"]: print(d["id"], d["cost"], d["electricity"])
print(m.pareto()["front"])
