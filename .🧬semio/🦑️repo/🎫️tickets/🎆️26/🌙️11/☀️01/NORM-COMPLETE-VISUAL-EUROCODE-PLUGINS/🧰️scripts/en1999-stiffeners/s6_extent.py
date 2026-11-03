"""Stage 6: recomputes the EN 1999 SQLite semantic contract (schemaBytes, tableWidths, rows and valueBytes of both cases) independently of the Rust code: the SQL schema is executed with python sqlite3 and the value bytes are counted per cell exactly as `independentSqliteExtent` does (integer/real 8, text UTF-8 length, IEEE companion bits 8 + class "finite" 6). Usage: python -I s6_extent.py [--write]"""
import json, sqlite3, sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent.parent / 'en1999-wave2'))
from spec import *

T = SQL / "🧪️tests/🧫️fixtures"
db = sqlite3.connect(":memory:")
db.executescript((SQL / "🗄️.sql").read_text(encoding="utf8"))
tables = db.execute("SELECT name, sql FROM sqlite_schema WHERE type='table' ORDER BY name").fetchall()
widths = {n: len(db.execute(f'PRAGMA table_info("{n}")').fetchall()) for n, _ in tables}
schema_bytes = sum(len(n.encode()) + len(s.encode()) for n, s in tables)
I = ("i", 0)
tx = lambda s: ("t", len(s.encode()))
R = ("r", 0)


def size(cells, table):
    n = 0
    for k, v in cells:
        if k == "i":
            n += 8
        elif k == "t":
            n += v
        else:
            n += 8 + 8 + 6
            n -= 8 + 8 + 6
            n += 8 + 8 + 6
    return n


def row(table, base_cells):
    reals = sum(1 for k, _ in base_cells if k == "r")
    base_width = len(base_cells)
    assert base_width + 2 * reals == widths[table], (table, base_width + 2 * reals, widths[table])
    return sum(8 if k == "i" else v if k == "t" else 8 for k, v in base_cells) + reals * (8 + 6)


def extent(s):
    rows, vb = 1, 2 + 8 if False else 0
    rows = 1
    vb = 8 + len(s["annex"].encode())
    assert widths["en1999_document"] == 2

    def add(table, cells):
        nonlocal rows, vb
        rows += 1
        vb += row(table, cells)

    def head(r, parent=True):
        return [I, I, I, tx(r["id"])]

    def actions(table, a):
        for x in a:
            add(table, [I, I, I, tx(x["id"]), tx(x["kind"]), tx(x["category"]), tx(x["source"])] + [R] * 9)

    for m in s["materials"]:
        add("en1999_material", head(m) + [tx(m["designation"]), tx(m["productForm"])])
    for x in s["sections"]:
        add("en1999_section", head(x) + [tx(x["kind"])] + [R] * 5)
        for p in x["elements"]:
            add("en1999_plate_element", head(p) + [R, R, I, I, R, tx(p["weldProcess"]), I, tx(p["weldDirection"]), R, R])
    for m in s["members"]:
        add("en1999_member", head(m) + [tx(m["sectionId"]), tx(m["materialId"]), R, tx(m["support"]), R, R, R, R, R, I, R, I, R])
        actions("en1999_member_action", m["actions"])
    for c in s["connections"]:
        add("en1999_connection", head(c) + [tx(c["memberId"]), tx(c["materialId"]), tx(c["kind"])])
        actions("en1999_connection_action", c["actions"])
        add("en1999_bolt_group", [I, tx(c["bolts"]["material"]), R, I, I, R, R, R, R])
        add("en1999_weld_group", [I, tx(c["welds"]["fillerAlloy"]), R, R, R, R])
    for f in s["fireScenarios"]:
        add("en1999_fire_scenario", head(f) + [tx(f["memberId"]), R, R])
    for f in s["fatigueDetails"]:
        add("en1999_fatigue_detail", head(f) + [tx(f["memberId"]), tx(f["detailCategory"]), R, R, R, R, R])
    for c in s["coldFormed"]:
        add("en1999_cold_formed_sheet", head(c) + [tx(c["materialId"]), R, R, R, I])
        actions("en1999_sheet_action", c["actions"])
    for c in s["shells"]:
        add("en1999_shell", head(c) + [tx(c["materialId"]), R, R, R, I, tx(c["fabricationClass"])])
        actions("en1999_shell_action", c["actions"])
    for p in s["placements"]:
        add("en1999_placement", head(p) + [R, R, R, R])
    for w in s.get("weldRuns", []):
        add("en1999_weld_run", head(w) + [tx(w["memberId"]), tx(w["elementId"]), R, R, R, R, tx(w["process"]), I, tx(w["direction"]), R])
    for v in s.get("outlineVertices", []):
        add("en1999_section_vertex", head(v) + [tx(v["sectionId"]), tx(v["ring"]), tx(v["elementId"]), R, R])
    for x in s.get("fatigueCategories", []):
        add("en1999_fatigue_category", head(x) + [tx(x["memberId"]), tx(x["elementId"]), tx(x["detailCategory"]), tx(x["detailType"]), R, R, R])
    for x in s.get("patchLoads", []):
        add("en1999_patch_load", head(x) + [tx(x["subjectId"]), tx(x["position"]), R, R, R, R, R, R])
    for x in s.get("ringStiffeners", []):
        add("en1999_ring_stiffener", head(x) + [tx(x["shellId"]), R, R, R, I])
    for x in s.get("transverseStiffeners", []):
        add("en1999_transverse_stiffener", head(x) + [tx(x["girderId"]), R, R, R, tx(x["sides"])])
    return rows, vb


f = json.loads((T / "🔣️.json").read_text(encoding="utf8"))["snapshot"]
empty = {k: ([] if isinstance(v, list) else v) for k, v in f.items()}
full, none = extent(f), extent(empty)
contract = {"schemaBytes": schema_bytes, "tableWidths": dict(sorted(widths.items())), "cases": [{"id": "full", "rows": full[0], "valueBytes": full[1]}, {"id": "emptyCollections", "rows": none[0], "valueBytes": none[1]}]}
print(json.dumps(contract, ensure_ascii=False)[:400], full, none)
if "--write" in sys.argv:
    (T / "🎛️semantic.json").write_text(json.dumps(contract, indent=2, ensure_ascii=False) + "\n", encoding="utf8")
    p = SQL / "🧪️tests/🟦️.ts"
    t = p.read_text(encoding="utf8")
    import re
    t = re.sub(r'expect\(normSemanticContract\["schemaBytes"\]\)\.toEqual\(\d+\)', f'expect(normSemanticContract["schemaBytes"]).toEqual({schema_bytes})', t)
    t = re.sub(r'expect\(normSemanticContract\["cases"\]\)\.toEqual\(\[.*?\]\);', 'expect(normSemanticContract["cases"]).toEqual(' + json.dumps(contract["cases"], separators=(",", ":")) + ");", t, count=1)
    t = re.sub(r'expect\(normSemanticContract\["tableWidths"\]\)\.toEqual\(\{.*?\}\)', 'expect(normSemanticContract["tableWidths"]).toEqual(' + json.dumps(contract["tableWidths"], separators=(",", ":")) + ")", t, count=1)
    p.write_text(t, encoding="utf8")
