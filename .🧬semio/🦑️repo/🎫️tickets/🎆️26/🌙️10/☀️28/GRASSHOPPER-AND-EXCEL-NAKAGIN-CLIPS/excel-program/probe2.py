import importlib, sys
from pathlib import Path
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT)); sys.stdout.reconfigure(encoding="utf-8")
m = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🧱️model.🐍️")
nk = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")
wb = m.workbook(); v = m.evaluate(wb)
for t in nk.capsule_types():
    print(t.key, t.name, t.interior_width, t.interior_length, t.interior_height, t.occupants)
    for f in t.fittings: print("   ", f)
    s = f"Type {t.key}"
    print("   ", [(a, v.get((s, a))) for a in ("B9", "B10", "B16", "G16", "B17")])
    print("   ", [(a, m.shown(wb, v, s, a, "de")) for a in ("C12","D12","G12","B9","B10","B17")])
print(m.sheet_titles("de", wb))
