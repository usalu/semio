"""🔬 Probe ray counts, blocked sun and electricity per run for the experiments scene."""
import importlib, sys
from pathlib import Path
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
m = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
f = m.factorial()
r = f["runs"]
for a, b in ((r[0], r[1]), (r[2], r[3])):
    blocked = a["solar_summer"] - b["solar_summer"]
    print("blocked sun", round(blocked, 2), "demand saved", round(a["cooling_demand"] - b["cooling_demand"], 2), "electricity saved", round(a["response"] - b["response"], 2), "blocked/3", round(blocked / 3, 2))
print([round(x["response"], 2) for x in r], [round(x["solar_summer"], 2) for x in r])
print(m.load_study()["parameters"]["room"]["width"]["value"], m.load_study()["parameters"]["systems"]["eer"]["value"])
