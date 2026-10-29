import importlib, sys
from pathlib import Path
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
m = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🧱️model.🐍️")
wb = m.workbook(); best = []
for s in wb.sheets:
    for k, c in s.cells.items():
        if c.formula: best.append((len(c.formula), s.name, k, c.formula))
best.sort(reverse=True); print(best[:3])
