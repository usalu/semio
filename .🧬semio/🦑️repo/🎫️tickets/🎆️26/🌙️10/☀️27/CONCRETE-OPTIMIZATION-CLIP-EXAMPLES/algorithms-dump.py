import importlib
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
m = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
s = m.surface()
print("min", s["minimum"], "grid", len(s["wwr"]), len(s["overhang"]), "max", max(max(r) for r in s["electricity"]))
g = m.gradient_path()
for i, it in enumerate(g):
    d = it["design"]
    print("G", i, round(d["wwr"] * 100, 1), round(d["overhang"], 3), round(it["electricity"], 1), it["slope"], it["step"], it["evaluations"])
print("G probes0", g[0]["probes"])
p = m.pattern_path()
for i, it in enumerate(p):
    d = it["design"]
    print("P", i, round(d["wwr"] * 100, 1), round(d["overhang"], 3), round(it["electricity"], 1), it["step"], it["moved"], it["evaluations"],
          [(q["direction"], round(q["electricity"], 1)) for q in it["probes"]])
ga = m.genetic()
for k, gen in enumerate(ga["generations"]):
    print("GEN", k, gen["evaluations"], round(gen["best"]["electricity"], 1), gen["elites"], gen["survivors"])
    for ind in gen["population"]:
        print("   ", ind["id"], round(ind["design"]["wwr"] * 100, 1), round(ind["design"]["overhang"], 3), round(ind["electricity"], 1))
    print("   parents", gen["parents"])
    print("   tournaments", gen["tournaments"][:4])
    for ch in gen["children"]:
        print("   child", ch["id"], ch["from"], ch["genes"], {a: round(b, 4) for a, b in ch["mutation"].items()},
              round(ch["design"]["wwr"] * 100, 1), round(ch["design"]["overhang"], 3), round(ch["electricity"], 1))
c = m.convergence()
print({k: v for k, v in c.items() if k != "series"})
for k, v in c["series"].items():
    print(k, [round(b, 1) for _, b in v])
print(m.evaluate({"wwr": .75, "overhang": .1}))
