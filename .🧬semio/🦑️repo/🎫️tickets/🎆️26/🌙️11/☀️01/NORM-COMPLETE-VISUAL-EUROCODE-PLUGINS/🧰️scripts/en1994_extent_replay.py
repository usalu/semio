"""Replays the independent SQLite extent (rows, value bytes, schema bytes, table widths) of the EN 1994 fixture from the schema SQL and the fixture JSON, the way `framework/.../sqlite-snapshot/tests/semantic-extent` measures an exported database (method of `catalogue-bridge-v3/en1990_extent_replay.py`). Usage: python3 -I en1994_extent_replay.py [--write]  (--write updates schemaBytes, tableWidths and the case valueBytes in the contract)"""
import json, sqlite3, sys

ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/"
SQL, FX, SEM = ROOT + "🗄️.sql", ROOT + "🧪️tests/🧫️fixtures/🔣️.json", ROOT + "🧪️tests/🧫️fixtures/🎛️semantic.json"


def camel(s):
    p = s.split("_")
    return p[0] + "".join(x.title() for x in p[1:])


def value(col, rec, ordinal, parents):
    if col.endswith("_ieee754_bits"):
        return 1
    if col.endswith("_ieee754_class"):
        return "finite"
    if col == "id":
        return parents.get("id")
    if col in parents:
        return parents[col]
    if col == "ordinal":
        return ordinal
    if col == "logical_id":
        return rec["id"]
    v = rec[camel(col)]
    return int(v) if isinstance(v, bool) else v


def run(empty):
    fx = json.load(open(FX, encoding="utf8"))
    if empty:
        for key in [k for k, v in fx.items() if isinstance(v, list)]:
            fx[key] = []
    db = sqlite3.connect(":memory:")
    db.executescript(open(SQL, encoding="utf8").read())
    cols = {t: [r[1] for r in db.execute(f"pragma table_info({t})")] for (t,) in db.execute("select name from sqlite_schema where type='table'").fetchall()}

    def insert(table, rec, ordinal, parents):
        values = [value(c, rec, ordinal, parents) for c in cols[table]]
        db.execute(f"insert into {table} values ({','.join('?' * len(values))})", values)
        return db.execute("select last_insert_rowid()").fetchone()[0]

    root = dict(fx)
    insert("en1994_document", root, 0, {"id": 1})
    for i, beam in enumerate(fx["beams"]):
        bid = insert("en1994_beam", beam, i, {"document_id": 1})
        for part, table in (("steel", "en1994_beam_steel"), ("sheeting", "en1994_beam_sheeting"), ("studs", "en1994_beam_studs")):
            insert(table, beam[part], 0, {"id": bid, "beam_id": bid})
        for j, a in enumerate(beam["actions"]):
            insert("en1994_beam_action", a, j, {"beam_id": bid})
    for i, column in enumerate(fx["columns"]):
        cid = insert("en1994_column", column, i, {"document_id": 1})
        for j, a in enumerate(column["actions"]):
            insert("en1994_column_action", a, j, {"column_id": cid})
    for i, slab in enumerate(fx["slabs"]):
        sid = insert("en1994_slab", slab, i, {"document_id": 1})
        insert("en1994_slab_sheeting", slab["sheeting"], 0, {"id": sid, "slab_id": sid})
        for j, a in enumerate(slab["actions"]):
            insert("en1994_slab_action", a, j, {"slab_id": sid})
    for i, p in enumerate(fx["placements"]):
        insert("en1994_placement", p, i, {"document_id": 1})
    rows = value_bytes = schema_bytes = 0
    widths = {}
    for name, sql in db.execute("select name, sql from sqlite_schema where type='table' order by name").fetchall():
        widths[name] = len(cols[name])
        schema_bytes += len(name.encode()) + len(sql.encode())
        rows += db.execute(f"select count(*) from {name}").fetchone()[0]
        for col in cols[name]:
            value_bytes += db.execute(f'select coalesce(sum(case typeof("{col}") when \'null\' then 0 when \'integer\' then 8 when \'real\' then 8 when \'text\' then length(cast("{col}" as blob)) when \'blob\' then length("{col}") end),0) from {name}').fetchone()[0]
    return {"rows": rows, "valueBytes": value_bytes, "schemaBytes": schema_bytes, "tables": len(widths), "widths": widths}


if __name__ == "__main__":
    contract = json.load(open(SEM, encoding="utf8"))
    results = {case["id"]: run(case["id"] == "emptyCollections") for case in contract["cases"]}
    for case in contract["cases"]:
        got = results[case["id"]]
        print(case["id"], {k: got[k] for k in ("rows", "valueBytes", "schemaBytes", "tables")}, "contract", {k: case[k] for k in ("rows", "valueBytes")}, contract["schemaBytes"], got["widths"] == contract["tableWidths"])
    if "--write" in sys.argv:
        any_result = next(iter(results.values()))
        contract["schemaBytes"] = any_result["schemaBytes"]
        contract["tableWidths"] = dict(sorted(any_result["widths"].items()))
        for case in contract["cases"]:
            case["rows"], case["valueBytes"] = results[case["id"]]["rows"], results[case["id"]]["valueBytes"]
        open(SEM, "w", encoding="utf8").write(json.dumps(contract, indent=2, ensure_ascii=False) + "\n")
        print("contract written")
