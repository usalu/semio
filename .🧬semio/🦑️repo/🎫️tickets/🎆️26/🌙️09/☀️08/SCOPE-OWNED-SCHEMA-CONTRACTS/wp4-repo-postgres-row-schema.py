import json, re, sys, collections
src = open("🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🧬️schema/🐘️postgres/🗄️.sql", encoding="utf-8").read()
# strip line comments
lines = [l for l in src.split("\n") if not l.strip().startswith("--")]
text = "\n".join(lines)
blocks = re.findall(r"CREATE TABLE IF NOT EXISTS\s+(\w+)\s*\((.*?)\n\);", text, re.S)
TABLE_CONSTRAINTS = ("PRIMARY KEY", "UNIQUE", "FOREIGN KEY", "CHECK", "CONSTRAINT", "EXCLUDE")

def split_top(body):
    out, depth, cur = [], 0, ""
    for ch in body:
        if ch == "(": depth += 1
        if ch == ")": depth -= 1
        if ch == "," and depth == 0:
            out.append(cur); cur = ""
        else:
            cur += ch
    if cur.strip(): out.append(cur)
    return [c.strip() for c in out if c.strip()]

TYPE = [
    (r"^TIMESTAMPTZ\b", "timestamptz"),
    (r"^DATE\b", "date"),
    (r"^JSONB\b", "jsonb"),
    (r"^BOOLEAN\b", "boolean"),
    (r"^BIGINT\b", "integer"),
    (r"^SMALLINT\b", "integer"),
    (r"^INTEGER\b", "integer"),
    (r"^INT\b", "integer"),
    (r"^DOUBLE PRECISION\b", "number"),
    (r"^REAL\b", "number"),
    (r"^NUMERIC\b", "number"),
    (r"^BYTEA\b", "string"),
    (r"^TEXT\b", "string"),
    (r"^UUID\b", "string"),
]

def pascal(name):
    return "".join(part.capitalize() for part in name.split("_"))

defs = collections.OrderedDict()
tables = collections.OrderedDict()
for table, body in blocks:
    cols = collections.OrderedDict()
    for clause in split_top(body):
        head = clause.upper()
        if any(head.startswith(c) for c in TABLE_CONSTRAINTS):
            continue
        m = re.match(r"^(\w+)\s+(.*)$", clause, re.S)
        if not m:
            print("SKIP", table, clause[:60], file=sys.stderr); continue
        col, rest = m.group(1), " ".join(m.group(2).split())
        kind = None
        for pattern, mapped in TYPE:
            if re.match(pattern, rest, re.I):
                kind = mapped; break
        if kind is None:
            print("UNKNOWN TYPE", table, col, rest[:60], file=sys.stderr); sys.exit(1)
        notnull = bool(re.search(r"\bNOT NULL\b", rest, re.I)) or bool(re.search(r"\bPRIMARY KEY\b", rest, re.I))
        enum = None
        check = re.search(r"CHECK\s*\(\s*\w+\s+IN\s*\((.*?)\)\s*\)", rest, re.I)
        if check:
            enum = [v.strip().strip("'") for v in check.group(1).split(",")]
        cols[col] = {"kind": kind, "notnull": notnull, "enum": enum}
    tables[table] = cols

def shape(spec):
    kind, enum = spec["kind"], spec["enum"]
    if kind == "timestamptz": base = {"$ref": "#/$defs/Timestamp"}
    elif kind == "date": base = {"$ref": "#/$defs/Date"}
    elif kind == "jsonb": base = {"$ref": "#/$defs/JsonDocument"}
    elif kind == "boolean": base = {"type": "boolean"}
    elif kind == "integer": base = {"type": "integer"}
    elif kind == "number": base = {"type": "number"}
    else: base = {"type": "string"}
    if enum: base = {"type": "string", "enum": enum}
    if not spec["notnull"]:
        if "$ref" in base: base = {"anyOf": [base, {"type": "null"}]}
        elif "enum" in base: base = {"anyOf": [base, {"type": "null"}]}
        else: base = {"type": [base["type"], "null"]}
    return base

for table, cols in tables.items():
    defs[pascal(table) + "Row"] = {
        "$comment": f"Row of the {table} table in 🐘️postgres/🗄️.sql.",
        "type": "object",
        "additionalProperties": False,
        "required": list(cols),
        "properties": {c: shape(s) for c, s in cols.items()},
    }

document = {
    "$schema": "http://json-schema.org/draft-07/schema#",
    "$id": "https://semio.tech/schema/repo/server/schema.json",
    "title": "RepoServerPersistence",
    "description": "Documents persisted by the repo server. The native implementation of this contract is the sibling 🐘️postgres/🗄️.sql; every table there has exactly one <Table>Row export here.",
    "$defs": collections.OrderedDict([
        ("Timestamp", {"type": "string", "pattern": "^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}(?:\\.[0-9]+)?(?:Z|[+-][0-9]{2}:[0-9]{2})$"}),
        ("Date", {"type": "string", "pattern": "^[0-9]{4}-[0-9]{2}-[0-9]{2}$"}),
        ("JsonDocument", {}),
    ] + list(defs.items())),
}
out = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🧬️schema/🔣️.json"
open(out, "w", encoding="utf-8").write(json.dumps(document, ensure_ascii=False, indent=2) + "\n")
print("tables:", len(tables), "defs:", len(document["$defs"]))
