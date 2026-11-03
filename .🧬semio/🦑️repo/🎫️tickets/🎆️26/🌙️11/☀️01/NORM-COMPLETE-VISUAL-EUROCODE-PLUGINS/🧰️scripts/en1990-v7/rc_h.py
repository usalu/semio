"""Task 1 layer H: SQLite semantic contract (schemaBytes, tableWidths, case extents) recomputed by the python sqlite3 replay and the independent extent model."""
from lib import *
import sqlite3, math
SCRIPTS = SEMIO + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️scripts/en1990"
src = rd(SCRIPTS + "/sqlite_extent.py")
src = src.replace('row(8 + t(d["annex"].capitalize() if d["annex"] in ("En", "De") else d["annex"]) + t(d["projectId"]) + t(d["structureKind"]) + 8 + 8*3 + 8 + 8 + t(d["supervisionLevel"]) + t(d["inspectionLevel"]) + 8 + 8 + 0)', 'row(8 + t(d["annex"].capitalize() if d["annex"] in ("En", "De") else d["annex"]) + t(d["projectId"]) + t(d["structureKind"]) + 8 + 8*2 + t(d["reliabilityClass"]) + 8 + 8 + t(d["supervisionLevel"]) + t(d["inspectionLevel"]) + 8 + 0)')
src = src.replace('"referencePeriodYears", "kFiDeclared", "betaComputed")', '"referencePeriodYears", "betaComputed")')
assert 'reliabilityClass' in src and 'kFiDeclared' not in src.split("def extent")[1].split("actions =")[0]
ns = {}
exec(compile(src.split('if __name__')[0], "extent", "exec"), ns)
extent = ns["extent"]
old = json.loads(rd(SQ + "/🧪️tests/🧫️fixtures/🎛️semantic.json"))
fx = json.loads(rd(SQ + "/🧪️tests/🧫️fixtures/🔣️.json"))
full = extent(fx)
e = dict(fx)
for k in ("permanents","variables","accidentals","seismics","members","bridgeSls","effects","fatigueSpectra","reliabilityAnalyses","testSeries","factorOverrides"):
    e[k] = []
empty = extent(e)
print("full", full, "empty", empty, "old", [(c["rows"], c["valueBytes"]) for c in old["cases"]])
db = sqlite3.connect(":memory:"); db.executescript(rd(SQ + "/🗄️.sql"))
rows = db.execute("SELECT name, sql FROM sqlite_schema WHERE type='table' ORDER BY name").fetchall()
widths = {n: len(db.execute('PRAGMA table_info("%s")' % n).fetchall()) for n, _ in rows}
size = sum(len(n.encode()) + len(s.encode()) for n, s in rows)
old["schemaBytes"] = size
old["tableWidths"] = dict(sorted(widths.items()))
for c in old["cases"]:
    c["rows"], c["valueBytes"] = full if c["id"] == "full" else empty
wr(SQ + "/🧪️tests/🧫️fixtures/🎛️semantic.json", json.dumps(old, indent=2, ensure_ascii=False) + "\n")
def ts(t):
    t = re.sub(r'(normSemanticContract\["schemaBytes"\]\)\.toEqual\()\d+\)', lambda m: m.group(1) + str(size) + ")", t)
    return re.sub(r'(normSemanticContract\["tableWidths"\]\)\.toEqual\()\{[^}]*\}\)', lambda m: m.group(1) + json.dumps(old["tableWidths"], separators=(",", ":")) + ")", t)
edit(SQ + "/🧪️tests/🟦️.ts", ts)
print("schemaBytes", size, "document width", widths["en1990_document"])
