import importlib, sys
from pathlib import Path
sys.dont_write_bytecode = True
root = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(root))
m = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
r = m.factorial()["runs"]
for a, b in ((0, 1), (2, 3)):
    print(r[a]["cooling_demand"], r[b]["cooling_demand"], r[a]["cooling_demand"] - r[b]["cooling_demand"], r[a]["solar_summer"] - r[b]["solar_summer"])
