import importlib, os, sys
from pathlib import Path
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_LANGUAGE"] = "de"
b = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🎨️visuals.📗️workbook.🐍️")
full = '=SUMMENPRODUKT((Kapseln!$D$2:$D$400=B2)*(Kapseln!$E$2:$E$400="A")*Kapseln!$F$2:$F$400)+ZÄHLENWENNS(Kapseln!$A$2:$A$400;B2;Kapseln!$B$2:$B$400;"Turm 1")|'
for n in range(20, len(full) + 1, 10):
    try:
        b._label(full[:n], 16.2, "#000")
    except Exception as e:
        print("FAIL at", n, repr(full[n-10:n])); break
else:
    print("all ok")
