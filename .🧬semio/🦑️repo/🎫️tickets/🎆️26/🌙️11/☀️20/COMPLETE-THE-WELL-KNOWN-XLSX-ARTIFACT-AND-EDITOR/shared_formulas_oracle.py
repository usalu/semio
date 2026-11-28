import json, sys, zipfile, tempfile, os
import openpyxl, formulas
from openpyxl.formula.translate import Translator

out_dir = sys.argv[1]
ct = '<?xml version="1.0" encoding="UTF-8" standalone="yes"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/><Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/><Override PartName="/xl/worksheets/sheet2.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/></Types>'
rels = '<?xml version="1.0" encoding="UTF-8" standalone="yes"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="xl/workbook.xml"/></Relationships>'
wb = '<?xml version="1.0" encoding="UTF-8" standalone="yes"?><workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets><sheet name="Sheet1" sheetId="1" r:id="rId1"/><sheet name="Other" sheetId="2" r:id="rId2"/></sheets></workbook>'
wbrels = '<?xml version="1.0" encoding="UTF-8" standalone="yes"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/><Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet2.xml"/></Relationships>'
S = 'xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"'
sheet1 = f'<?xml version="1.0" encoding="UTF-8" standalone="yes"?><worksheet {S}><sheetData>' \
 '<row r="1"><c r="A1"><v>10</v></c><c r="C1"><f t="shared" ref="C1:D1" si="1">A1+Other!$A$1</f></c><c r="D1"><f t="shared" si="1"/></c><c r="F1"><f t="array" ref="F1">SUM(A2:A5)</f></c></row>' \
 '<row r="2"><c r="A2"><v>1</v></c><c r="B2"><f t="shared" ref="B2:B5" si="0">A2*2+$A$1</f></c></row>' \
 '<row r="3"><c r="A3"><v>2</v></c><c r="B3"><f t="shared" si="0"/></c></row>' \
 '<row r="4"><c r="A4"><v>3</v></c><c r="B4"><f t="shared" si="0"/></c></row>' \
 '<row r="5"><c r="A5"><v>4</v></c><c r="B5"><f t="shared" si="0"/></c></row>' \
 '</sheetData></worksheet>'
sheet2 = f'<?xml version="1.0" encoding="UTF-8" standalone="yes"?><worksheet {S}><sheetData><row r="1"><c r="A1"><v>100</v></c></row></sheetData></worksheet>'
xlsx = os.path.join(out_dir, "workbook.xlsx")
with zipfile.ZipFile(xlsx, "w", zipfile.ZIP_DEFLATED) as z:
    for name, body in [("[Content_Types].xml", ct), ("_rels/.rels", rels), ("xl/workbook.xml", wb), ("xl/_rels/workbook.xml.rels", wbrels), ("xl/worksheets/sheet1.xml", sheet1), ("xl/worksheets/sheet2.xml", sheet2)]:
        z.writestr(name, body)

def sheet_formulas(path):
    book = openpyxl.load_workbook(path)
    result = {}
    for ws in book.worksheets:
        for row in ws.iter_rows():
            for cell in row:
                v = cell.value
                if hasattr(v, "text"):
                    v = "=" + v.text.lstrip("=")
                if isinstance(v, str) and v.startswith("="):
                    result[f"{ws.title}!{cell.coordinate}"] = v[1:]
    return result

def values(path):
    model = formulas.ExcelModel().loads(path).finish()
    solution = model.calculate()
    book = openpyxl.load_workbook(path)
    out = {}
    for key, rng in solution.items():
        k = key.replace("'", "").upper()
        out[k] = rng.value.tolist()
    result = {}
    for ref in sheet_formulas(path):
        sheet, coord = ref.split("!")
        lookup = f"[{os.path.basename(path)}]{sheet.upper()}!{coord}".upper()
        val = out[lookup][0][0]
        result[ref] = float(val) if not isinstance(val, str) else val
    return result

base = sheet_formulas(xlsx)
translations = []
for si_master, targets in [("Sheet1!B2", ["B3", "B4", "B5"]), ("Sheet1!C1", ["D1"])]:
    master = base[si_master]
    anchor = si_master.split("!")[1]
    for t in targets:
        translations.append({"master": master, "from": anchor, "to": t, "expected": Translator("=" + master, origin=anchor).translate_formula(t)[1:]})
extra = [("A1+B2*$C3-D$4", "B2", "E9"), ("SUM($A2:A2)", "C2", "D5"), ("Sheet1!A1+'My Sheet'!B2+\"A1\"", "A1", "B2"), ("LOG10(A1)+A:A", "A1", "C3"), ("IF(A1>0,\"x\",B$2)", "A1", "A1")]
for f, o, t in extra:
    translations.append({"master": f, "from": o, "to": t, "expected": Translator("=" + f, origin=o).translate_formula(t)[1:]})

cases = [{"id": "open", "edit": None, "formulas": base, "values": values(xlsx)}]
edits = [
    ("edit-master-to-formula", "Sheet1!B2", "=A2*3"),
    ("edit-dependent-to-formula", "Sheet1!B4", "=A4+1000"),
    ("edit-master-to-number", "Sheet1!B2", 7),
    ("edit-horizontal-master", "Sheet1!C1", "=A1*2"),
]
for cid, ref, value in edits:
    book = openpyxl.load_workbook(xlsx)
    sheet, coord = ref.split("!")
    book[sheet][coord].value = value
    tmp = os.path.join(tempfile.mkdtemp(), "edited.xlsx")
    book.save(tmp)
    cases.append({"id": cid, "edit": {"cell": ref, "formula": value if isinstance(value, str) else None, "number": value if not isinstance(value, str) else None}, "formulas": sheet_formulas(tmp), "values": values(tmp)})
with open(os.path.join(out_dir, "fixture.json"), "w") as f:
    json.dump({"generator": f"openpyxl {openpyxl.__version__} Translator + formulas {formulas.__version__}", "translations": translations, "cases": cases}, f, indent=1, ensure_ascii=False)
