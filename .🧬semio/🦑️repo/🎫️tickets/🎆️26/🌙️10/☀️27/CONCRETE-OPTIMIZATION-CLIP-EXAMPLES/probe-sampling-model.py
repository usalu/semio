"""🔬 Print the sampling data of the model."""
import importlib, sys, json
from pathlib import Path
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
m = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
print(m.levels())
s = m.lhs_strata(); print(s)
for k in ("grid", "random", "lhs"):
    print(k)
    for p in m.samples(k):
        print("  ", round(p["wwr"]*100, 1), round(p["overhang"], 3), round(p["electricity"], 1), p.get("strata_index"), round(p["heating"]), round(p["cooling"]), round(p["lighting"]))
    print(m.sampling_diagnostics(k))
su = m.surface()
print(su["minimum"], len(su["wwr"]), min(min(r) for r in su["electricity"]), max(max(r) for r in su["electricity"]))
print(m.load_study()["baseline"], m.load_study()["sun"])
