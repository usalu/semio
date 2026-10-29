import importlib.util
import sys
from pathlib import Path

ROOT = Path("C:/git/semio")
path = ROOT / "🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🧱️model/🐍️.py"
spec = importlib.util.spec_from_file_location("xlm", path)
m = importlib.util.module_from_spec(spec)
sys.modules["xlm"] = m
spec.loader.exec_module(m)
wb = m.workbook()
vals = m.evaluate(wb)
for name in sys.argv[1:]:
    sh = wb.sheet(name)
    print("==", name, sh.widths if hasattr(sh, "widths") else "")
    for addr, cell in sh.cells.items():
        v = vals.get((name, addr))
        print(addr, "|", cell.formula, "|", cell.value, "|", v, "|", cell.format, "|", cell.labels)
print(wb.names)
