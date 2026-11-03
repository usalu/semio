"""Replays the independent SQLite extent (rows, value bytes, schema bytes, table widths) of the EN 1990 fixture from the schema SQL and the fixture JSON, the way `framework/.../sqlite-snapshot/tests/semantic-extent` measures an exported database. Usage: python3 -I en1990_extent_replay.py"""
import json, sqlite3
ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/"
SQL, FX, SEM = ROOT + "🗄️.sql", ROOT + "🧪️tests/🧫️fixtures/🔣️.json", ROOT + "🧪️tests/🧫️fixtures/🎛️semantic.json"
ACTIONS = [("permanents", "permanent"), ("variables", "variable"), ("accidentals", "accidental"), ("seismics", "seismic")]
REFERENCED = {"members": "member", "bridgeSls": "bridge_sls", "effects": "effect", "fatigueSpectra": "fatigue_spectrum", "reliabilityAnalyses": "reliability_analysis", "testSeries": "test_series", "placements": "placement", "factorOverrides": "factor_override", "cases": "case"}

def camel(s):
    p = s.split("_")
    return p[0] + "".join(x.title() if not x.isdigit() else x for x in p[1:])

def value(col, rec, ordinal, resolved):
    if col.endswith("_ieee754_bits"): return 1
    if col.endswith("_ieee754_class"): return "finite"
    if col == "id": return None
    if col == "document_id": return 1
    if col == "ordinal": return ordinal
    if col == "logical_id": return rec["id"]
    if col.startswith("resolved_"): return resolved.get(col)
    if col.endswith("_reference"): return rec[camel(col[: -len("_reference")]) + "Id"]
    if col == "annex": return rec[col]
    return rec["group"] if col == "exclusive_group" else rec[camel(col)]

def run(empty):
    fx = json.load(open(FX, encoding="utf8"))
    if empty:
        for key in [k for k, v in fx.items() if isinstance(v, list)]: fx[key] = []
    db = sqlite3.connect(":memory:")
    db.executescript(open(SQL, encoding="utf8").read())
    cols = {t: [r[1] for r in db.execute(f"pragma table_info({t})")] for (t,) in db.execute("select name from sqlite_schema where type='table'").fetchall()}
    def insert(table, values):
        db.execute(f"insert into {table} values ({','.join('?' * len(values))})", values)
        return db.execute("select last_insert_rowid()").fetchone()[0]
    def row(table, rec, ordinal, fixed=None, resolved=None):
        fixed = fixed or {}
        return insert(table, [fixed[c] if c in fixed else value(c, rec, ordinal, resolved or {}) for c in cols[table]])
    root = dict(fx); root["annex"] = "En" if fx["annex"].lower() == "en" else "De"
    row("en1990_document", root, 0, {"id": 1})
    ids = {}
    for key, kind in ACTIONS:
        for i, rec in enumerate(fx[key]):
            fixed = {"kind": kind}
            action_id = row("en1990_action", rec, i, fixed)
            ids.setdefault(rec["id"], []).append(action_id)
            sub = f"en1990_{kind}_action"
            insert(sub, [action_id if c == "id" else value(c, rec, i, {}) for c in cols[sub]])
    members = {}
    for i, rec in enumerate(fx["members"]): members.setdefault(rec["id"], []).append(row("en1990_member", rec, i))
    unique = lambda table, name: table[name][0] if len(table.get(name, [])) == 1 else None
    for key in ["bridgeSls", "effects", "fatigueSpectra", "reliabilityAnalyses", "testSeries", "placements", "factorOverrides"]:
        table = "en1990_" + REFERENCED[key]
        for i, rec in enumerate(fx[key]):
            row(table, rec, i, resolved={"resolved_member_id": unique(members, rec.get("memberId")), "resolved_action_id": unique(ids, rec.get("actionId"))})
    rows = valueBytes = schemaBytes = 0; widths = {}
    for name, sql in db.execute("select name, sql from sqlite_schema where type='table' order by name").fetchall():
        widths[name] = len(cols[name]); schemaBytes += len(name.encode()) + len(sql.encode())
        rows += db.execute(f"select count(*) from {name}").fetchone()[0]
        for col in cols[name]:
            valueBytes += db.execute(f'select coalesce(sum(case typeof("{col}") when \'null\' then 0 when \'integer\' then 8 when \'real\' then 8 when \'text\' then length(cast("{col}" as blob)) when \'blob\' then length("{col}") end),0) from {name}').fetchone()[0]
    return {"rows": rows, "valueBytes": valueBytes, "schemaBytes": schemaBytes, "tables": len(widths), "widths": widths}

if __name__ == "__main__":
    contract = json.load(open(SEM, encoding="utf8"))
    for case in contract["cases"]:
        got = run(case["id"] == "emptyCollections")
        print(case["id"], {k: got[k] for k in ("rows", "valueBytes", "schemaBytes", "tables")}, "contract", {k: case[k] for k in ("rows", "valueBytes")}, contract["schemaBytes"], got["widths"] == contract["tableWidths"])
