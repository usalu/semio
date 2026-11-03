"""en1997 `piles[].concreteClass` SQLite contract: schemaBytes/tableWidths replayed with python sqlite3 from the DDL (old DDL = new DDL without the column), +5 value bytes (`c4050`) per pile row of the semantic and native-extent cases, ts test literals, fixture token (idempotent)."""
import json, re, sqlite3
Q = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot"
FX = Q + "/🧪️tests/🧫️fixtures"
rd = lambda p: open(p, encoding="utf8").read()


def wr(p, t):
    open(p, "w", encoding="utf8").write(t)


def measure(sql):
    db = sqlite3.connect(":memory:")
    db.executescript(sql)
    rows = db.execute("select name, sql from sqlite_schema where type='table' order by name").fetchall()
    return sum(len(n.encode()) + len(q.encode()) for n, q in rows), {n: len(db.execute(f"pragma table_info({n})").fetchall()) for n, _ in rows}


new_sql = rd(Q + "/🗄️.sql")
old_sql = re.sub(r" concrete_class TEXT NOT NULL CHECK\(concrete_class IN \([^)]*\)\),", "", new_sql)
assert old_sql != new_sql
old_b, old_w = measure(old_sql)
new_b, new_w = measure(new_sql)
sem_path = FX + "/🎛️semantic.json"
sem = json.loads(rd(sem_path))
if sem["schemaBytes"] == new_b:
    print("semantic contract already current")
else:
    assert sem["schemaBytes"] == old_b and sem["tableWidths"] == old_w, ("stale contract", sem["schemaBytes"], old_b)
    sem["schemaBytes"], sem["tableWidths"] = new_b, new_w
    for case in sem["cases"]:
        if case["id"] != "emptyCollections":
            case["valueBytes"] += 5
    wr(sem_path, json.dumps(sem, indent=2, ensure_ascii=False) + "\n")
    t = rd(Q + "/🧪️tests/🟦️.ts")
    old_lit, new_lit = json.dumps(old_w, separators=(",", ":")), json.dumps(new_w, separators=(",", ":"))
    assert f"toEqual({old_b})" in t and old_lit in t
    wr(Q + "/🧪️tests/🟦️.ts", t.replace(f"toEqual({old_b})", f"toEqual({new_b})").replace(old_lit, new_lit))
    print("semantic", old_b, "->", new_b)
ne_path = FX + "/📏️native-extent/🔣️.json"
ne = json.loads(rd(ne_path))
if "pileClassBytes" not in ne:
    for case in ne["cases"]:
        if case["id"] != "empty":
            case["valueBytes"] += 5
    ne["pileClassBytes"] = 5
    wr(ne_path, json.dumps(ne, indent=2, ensure_ascii=False) + "\n")
fx_path = FX + "/🔣️.json"
fx = rd(fx_path)
if '"concreteClass"' not in fx:
    assert fx.count('        "hammerEfficiency": 0.0,\n') == 1
    wr(fx_path, fx.replace('        "hammerEfficiency": 0.0,\n', '        "hammerEfficiency": 0.0,\n        "concreteClass": "c4050",\n'))
