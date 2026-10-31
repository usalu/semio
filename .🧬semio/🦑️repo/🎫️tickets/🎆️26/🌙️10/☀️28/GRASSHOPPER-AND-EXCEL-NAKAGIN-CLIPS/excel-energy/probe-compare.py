import importlib, sys
from pathlib import Path
root = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(root))
xl = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🧱️model.🐍️")
nk = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")
b = xl.workbook(language="en")
v = xl.evaluate(b)
EN = b.order()[-1]
print([v[(EN, f"F{r}")] for r in range(4, 8)], [v[(EN, f"B{r}")] for r in range(4, 8)])
print([(t.key, t.window_diameter, nk.heating(t)["annual_per_area"]) for t in nk.capsule_types()])
