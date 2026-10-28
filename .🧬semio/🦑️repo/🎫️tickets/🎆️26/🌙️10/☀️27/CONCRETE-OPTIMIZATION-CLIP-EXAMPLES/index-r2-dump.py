import importlib, sys
from pathlib import Path
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
m = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
for v in m.variants(True):
    e = m.evaluate(v["design"])
    print(v["id"], {k: round(e[k], 3) for k in ("window_area", "wall_area", "heating_demand", "cooling_demand", "heating", "cooling", "lighting", "electricity", "intensity", "daylight", "autonomy", "cost", "carbon", "shading_summer")})
    d = v["design"]
    print("   lighting base", 9 * 48 * 1500 / 1000, "x(1-aut)=", 648 * (1 - e["autonomy"]))
    print("   sky", 1 - 0.5 * d["overhang"] ** 2 / (d["overhang"] ** 2 + 0.36), "theta", 75 * (1 - 0.5 * d["overhang"] ** 2 / (d["overhang"] ** 2 + 0.36)))
