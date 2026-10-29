"""Probe the Nakagin workbook for the program scene."""
import importlib
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.stdout.reconfigure(encoding="utf-8")
m = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🧱️model.🐍️")
wb = m.workbook()
v = m.evaluate(wb)
for a in ["B4", "B5", "B6", "B7", "B8", "C4", "D4", "D8", "B11", "C11", "B12", "F11", "F12", "F13", "B15", "C15", "B16", "B17", "B18", "M2", "N2", "M3", "I2", "J2", "K2"]:
    print(a, repr(v.get(("Program", a))), wb.cell("Program", a).formula if wb.cell("Program", a) else None)
print([ (c, m.tab_name(wb, "Program", c)) for c in ("en", "de")])
rows = [(i, v[("Program", f"M{i}")]) for i in range(2, 142)]
c_rows = [i for i, t in rows if t == "Type C"]
print("first C rows", c_rows[:5], "tower of", [v[("Program", f"{c}{c_rows[0]}")] for c in "HIJKLM"])
w2 = m.apply(wb, [{"type": "set-cell", "sheet": "Program", "cell": f"M{c_rows[0]}", "value": "Type A"}])
v2 = m.evaluate(w2)
print([v2[("Program", f"B{r}")] for r in range(4, 9)], v2[("Program", "B18")], v2[("Program", "D8")])
w3 = m.apply(wb, [{"type": "set-cell", "sheet": "Program", "cell": f"M{c_rows[0]}", "value": "Type A "}])
v3 = m.evaluate(w3)
print([v3[("Program", f"B{r}")] for r in range(4, 9)], v3[("Program", "B18")], v3[("Program", f"N{c_rows[0]}")], v3[("Program", "F11")], v3[("Program", "F12")])
print(m.localize("=COUNTIFS($I$2:$I$141,$A11,$M$2:$M$141,B$10)", "de", wb))
print(m.formula_text(wb, "Program", "B4", "de"), m.formula_text(wb, "Program", "B15", "de"))
