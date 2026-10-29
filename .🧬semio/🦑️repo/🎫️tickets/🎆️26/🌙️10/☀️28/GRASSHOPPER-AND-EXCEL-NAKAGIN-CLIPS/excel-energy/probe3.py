import importlib.util, importlib, sys
from pathlib import Path
ROOT = Path("C:/git/semio"); sys.path.insert(0, str(ROOT))
m = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🧱️model.🐍️")
n = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")
d = n.load()
print(list(d["energy"].keys()))
for k, v in d["energy"].items(): print(k, v if not isinstance(v, dict) or "value" in v else list(v)[:4])
wb = m.workbook(); print(wb.order()); print(wb.names)
ex = m.dependents(wb, (wb.order()[-1], "B14")); print(len(ex), ex[:12])
t = n.capsule_type("A"); print([f for f in dir(t) if not f.startswith("_")][:30])
print(m.calculate("=SUMPRODUCT(A1:A3,B1:B3)", {"A1": 1, "A2": 2, "A3": 3, "B1": 4, "B2": 5, "B3": 6}))
print(m.display(1.1, '0.00 "W/(m²·K)"', "de"))
