import importlib, sys
from pathlib import Path
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
m = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🧱️model.🐍️")
wb = m.workbook()
v = m.evaluate(wb)
for t in "ABCD":
    print(t, [round(v[(f"Type {t}", c)], 2) for c in ("B9", "B10", "B13", "B14", "B15")])
