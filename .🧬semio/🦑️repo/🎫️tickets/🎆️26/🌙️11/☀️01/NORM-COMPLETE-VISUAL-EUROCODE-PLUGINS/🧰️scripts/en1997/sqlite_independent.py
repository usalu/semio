import sys, json, struct
sys.path.insert(0, ".")
from spec import *
exec(open("sqlite_gen.py", encoding="utf8").read().split("def sql_table")[0])
Q = S + "/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures"
snap = json.load(open(Q + "/🔣️.json", encoding="utf8"))["snapshot"]
p = Q + "/🔬️independent/🔣️.json"
d = json.load(open(p, encoding="utf8"))
if any(t["name"] == "en1997_anchor" for t in d["variants"][0]["tables"]):
    sys.exit(0)

def bits(x):
    return str(struct.unpack("<q", struct.pack("<d", float(x)))[0])

def cols(t):
    names = ["id", ("document_id" if t.parent == "document" else t.parent.replace("en1997_", "") + "_id"), "ordinal", "logical_id"]
    names += [n for n, k in t.cols[1:]]
    for n, k in t.cols:
        if k == "n":
            names += [n + "_ieee754_bits", n + "_ieee754_class"]
    return names

def row(t, surrogate, parent, ordinal, rec):
    vals = [surrogate, parent, ordinal, rec["id"]]
    for n, k in t.cols[1:]:
        v = rec[camel(n)]
        vals.append(int(v) if k == "u" else (v if k == "s" else float(v)))
    for n, k in t.cols:
        if k == "n":
            vals += [bits(rec[camel(n)]), "finite"]
    return vals

new_tables = []
for t in tabs:
    if t.parent == "document":
        rows = []
        children = []
        for i, rec in enumerate(snap[t.e.camel_coll]):
            sur = f"{t.name.replace('_', '')}{i}"
            rows.append(row(t, sur, "document", i, rec))
            if t.children:
                c = t.children[0]
                for k, sub in enumerate(rec[camel(c.var)]):
                    children.append(row(c, f"{sur}/{c.name.split('_', 1)[-1].replace('_', '')}{k}", sur, k, sub))
        new_tables.append({"name": t.table, "columns": cols(t), "rows": rows, "count": len(rows)})
        if t.children:
            c = t.children[0]
            new_tables.append({"name": c.table, "columns": cols(c), "rows": children, "count": len(children)})
for v in d["variants"]:
    v["tables"] += new_tables
rows_total = 9 + sum(t["count"] for t in new_tables)
d["totalRows"] = rows_total
d["totalCells"] = 210 + sum(len(t["columns"]) * t["count"] for t in new_tables)
open(p, "w", encoding="utf8").write(json.dumps(d, ensure_ascii=False, indent=2) + "\n")
print(rows_total, d["totalCells"])
