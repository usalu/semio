import importlib, sys
from pathlib import Path

root = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(root))
model = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
data = model.pareto()
by = {d["id"]: d for d in data["designs"]}
print("[DEBUG] front", data["front"])
for i in data["front"]:
    d = by[i]
    print("[DEBUG]", i, d["design"], round(d["cost"]), round(d["electricity"]), "heat", round(d["heating"]), "cool", round(d["cooling"]), "light", round(d["lighting"]), "dl", round(d["daylight"], 2))
for s in data["steps"]:
    print("[DEBUG] step", s["from"], s["to"], round(s["cost"]), round(s["saved"]), round(s["ratio"], 2), "payback", round(s["payback"], 1))
print("[DEBUG] third", data["third_example"], "front3", len(data["front3"]))
for key in ("dominated_example", "refinement", "knee", "picks"):
    print("[DEBUG]", key, data[key])
t = data["third_example"]
for k in ("dominated_2d", "dominator"):
    d = by[t[k]]
    print("[DEBUG]", k, d["id"], d["design"], round(d["cost"]), round(d["electricity"]), "dl", round(d["daylight"], 1), "light", round(d["lighting"]))
ev = model.evaluate(by[t["dominator"]]["design"])
print("[DEBUG] autonomy", {k: round(model.evaluate(by[k]["design"])["autonomy"], 2) for k in by if k in (t["dominator"], t["dominated_2d"])})
print("[DEBUG] daylight range", min(d["daylight"] for d in by.values()), max(d["daylight"] for d in by.values()))
