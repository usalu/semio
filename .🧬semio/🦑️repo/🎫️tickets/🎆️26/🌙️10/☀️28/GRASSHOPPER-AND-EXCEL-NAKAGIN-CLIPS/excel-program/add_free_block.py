"""Add the free-floor block (Program A20:D27) to the Excel workbook JSON; idempotent. Usage: add_free_block.py [--check]"""
import json
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
TARGET = ROOT / "🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🧱️model/🔣️.json"
text = TARGET.read_text(encoding="utf-8")
data = json.loads(text)
if "--check" in sys.argv:
    print("[DEBUG] roundtrip identical:", json.dumps(data, ensure_ascii=False, indent=1) + "\n" == text)
    raise SystemExit
M2 = '0.00 "m²"'
M2T = '#,##0.0 "m²"'


def label(en, de):
    return {"text": {"en": en, "de": de}}


cells = {
    "A20": label("Type", "Typ"), "B20": label("Count", "Anzahl"), "C20": label("Free", "Frei"), "D20": label("Total", "Gesamt"),
    "A25": label("Total", "Summe"), "B25": {"formula": "=SUM(B21:B24)", "format": "0"}, "D25": {"formula": "=SUM(D21:D24)", "format": M2T},
    "A26": label("Free m²", "Frei m²"), "B26": {"formula": "=SUMPRODUCT(B21:B24,C21:C24)", "format": M2T},
    "A27": label("Type C", "Typ C"),
    "B27": {"formula": "=XLOOKUP(A27,$A$21:$A$24,$B$21:$B$24)", "format": "0"},
    "C27": {"formula": "=XLOOKUP(A27,$A$21:$A$24,$C$21:$C$24)", "format": M2},
}
for index, key in enumerate("ABCD"):
    row = 21 + index
    cells[f"A{row}"] = {"formula": f"=A{4 + index}"}
    cells[f"B{row}"] = {"formula": f"=B{4 + index}", "format": "0"}
    cells[f"C{row}"] = {"formula": f"='Type {key}'!B16", "format": M2}
    cells[f"D{row}"] = {"formula": f"=B{row}*C{row}", "format": M2T}
program = next(sheet for sheet in data["sheets"] if sheet["name"] == "Program")
program["cells"].update(cells)
TARGET.write_text(json.dumps(data, ensure_ascii=False, indent=1) + "\n", encoding="utf-8", newline="\n")
print("[DEBUG] free block written", len(cells))
