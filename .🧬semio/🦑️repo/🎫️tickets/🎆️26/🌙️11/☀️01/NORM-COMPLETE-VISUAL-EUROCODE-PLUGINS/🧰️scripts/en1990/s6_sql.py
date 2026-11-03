from spec import *
import sqlite3
SQ = SUB + "/🚪️io/🪶️sqlite/📸️snapshot"
def extent(sql):
    db = sqlite3.connect(":memory:"); db.executescript(sql)
    rows = db.execute("SELECT name, sql FROM sqlite_schema WHERE type='table' ORDER BY name").fetchall()
    w = {n: len(db.execute(f'PRAGMA table_info("{n}")').fetchall()) for n, _ in rows}
    return sum(len(n.encode()) + len(s.encode()) for n, s in rows), w
p = SQ + "/🗄️.sql"
t = open(p, encoding="utf8").read()
print("old", extent(t)[0])
if "en1990_fatigue_spectrum" not in t:
    t = t.replace("category TEXT NOT NULL, qk REAL, qk_ieee754_bits", "category TEXT NOT NULL, qk REAL, exclusive_group TEXT NOT NULL, qk_ieee754_bits")
    def floats(cols): return "".join(f", {c}_ieee754_bits INTEGER NOT NULL, {c}_ieee754_class TEXT NOT NULL" for c in cols)
    base = "id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1990_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL"
    t = t.rstrip("\n") + "\n" + \
      f"CREATE TABLE en1990_fatigue_spectrum({base}, member_reference TEXT NOT NULL, resolved_member_id INTEGER REFERENCES en1990_member(id), detail_category REAL, stress_range REAL, cycles REAL{floats(['detail_category','stress_range','cycles'])});\n" + \
      f"CREATE TABLE en1990_reliability_analysis({base}, resistance_distribution TEXT NOT NULL, resistance_mean REAL, resistance_sd REAL, action_distribution TEXT NOT NULL, action_mean REAL, action_sd REAL{floats(['resistance_mean','resistance_sd','action_mean','action_sd'])});\n" + \
      f"CREATE TABLE en1990_test_series({base}, quantity TEXT NOT NULL, distribution TEXT NOT NULL, sample_size INTEGER NOT NULL CHECK(sample_size>=0 AND sample_size<=4294967295), mean REAL, std_dev REAL, variation_known REAL, conversion_factor REAL, required REAL{floats(['mean','std_dev','variation_known','conversion_factor','required'])});\n"
    open(p, "w", encoding="utf8").write(t)
b, w = extent(t); print("new", b, w)
import json
open(SQ + "/_extent.json", "w").write(json.dumps({"schemaBytes": b, "tableWidths": w}))
