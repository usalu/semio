"""📋 Print the factorial and fractional numbers the experiments scene uses."""
import importlib, sys
from pathlib import Path
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
m = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
f = m.factorial()
for r in f["runs"]:
    print(r["id"], r["coded"], r["design"], round(r["response"], 2), round(r["shading_summer"], 2), round(r["solar_summer"]))
print(f["effects"], f["coefficients"], f["centre"], f["centre_savings"], f["factors"], f["response"])
fr = m.fractional()
print(fr["generator"], fr["factors"])
for r in fr["runs"]:
    print(r["coded"], r["design"], round(r["response"], 2))
print(fr["alias_contrast"], fr["true"])
print(m.load_study()["sun"])
