import importlib, sys
from pathlib import Path
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT)); sys.stdout.reconfigure(encoding="utf-8")
m = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🧱️model.🐍️")
for lang in ("en", "de"):
    wb = m.workbook(language=lang); v = m.evaluate(wb)
    print(lang, [(a, m.formula_text(wb, "Program", a, lang), m.shown(wb, v, "Program", a, lang)) for a in ("B4","B11","F11","B15","B16","B18","A15","H2","I2","M2","N2","K4","M1")])
    print(m.sheet_titles(lang, wb))
print(m.reference_spans("=COUNTIF(Capsules[Type],A4)"))
w = m.workbook(); print(type(w).__name__, [n for n in dir(w) if not n.startswith('_')])
