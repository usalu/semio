"""EN 1997 wave 2 SQLite pins for the five pile and wall columns: recomputes `schemaBytes` and `tableWidths` by replaying the committed DDL in python `sqlite3` (third party), appends the zero-valued columns to the independent complete-domain fixture and moves the native extent corpus by the float columns added (finite 22, NaN 11, infinity 32 bytes each). Idempotent: python -I sqlite_pins_wave2.py"""
import json, re, sqlite3, struct

S = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot"
Q = S + "/🧪️tests/🧫️fixtures"
NEW = {"en1997_pile": ("tension_variable", ["bending_stiffness_ei", "horizontal_subgrade_modulus", "lateral_head_load"]), "en1997_retaining_wall": ("horizontal_permanent", ["seismic_kh", "seismic_kv"])}
ADDED = 5

c = sqlite3.connect(":memory:")
c.executescript(open(S + "/🗄️.sql", encoding="utf-8").read())
tables = c.execute("select name, sql from sqlite_master where type='table' and name like 'en1997_%'").fetchall()
schema_bytes = sum(len(n) + len(s) for n, s in tables)
widths = {n: len(c.execute(f"pragma table_info({n})").fetchall()) for n, _ in tables}

p = Q + "/🎛️semantic.json"
semantic = json.load(open(p, encoding="utf-8"))
semantic["schemaBytes"], semantic["tableWidths"] = schema_bytes, dict(sorted(widths.items()))
open(p, "w", encoding="utf-8").write(json.dumps(semantic, indent=2, ensure_ascii=False) + "\n")

p = Q + "/🔬️independent/🔣️.json"
d = json.load(open(p, encoding="utf-8"))
cells = 0
for variant in d["variants"]:
    for t in variant["tables"]:
        if t["name"] not in NEW:
            continue
        after, names = NEW[t["name"]]
        if names[0] in t["columns"]:
            continue
        at = t["columns"].index(after) + 1
        t["columns"][at:at] = names
        for name in names:
            t["columns"] += [name + "_ieee754_bits", name + "_ieee754_class"]
        for row in t["rows"]:
            row[at:at] = [0.0] * len(names)
            for _ in names:
                row += ["0", "finite"]
        cells += t["count"] * len(names) * 3
if cells:
    d["totalCells"] += cells // len(d["variants"])
open(p, "w", encoding="utf-8").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")

p = Q + "/📏️native-extent/🔣️.json"
corpus = json.load(open(p, encoding="utf-8"))
policy = corpus["scalarPolicy"]
if corpus["cases"][0]["valueBytes"] == 3077:
    for case in corpus["cases"]:
        if case["id"] == "empty":
            continue
        word = case.get("word")
        per = policy["finiteBinary64"]
        if word:
            bits = int(word, 16)
            exponent, fraction = (bits >> 52) & 0x7FF, bits & ((1 << 52) - 1)
            per = policy["nanBinary64"] if exponent == 0x7FF and fraction else policy["infiniteBinary64"] if exponent == 0x7FF else policy["finiteBinary64"]
        case["valueBytes"] += ADDED * per
    open(p, "w", encoding="utf-8").write(json.dumps(corpus, indent=2, ensure_ascii=False) + "\n")
print(schema_bytes, {k: widths[k] for k in NEW}, [(c["id"], c["valueBytes"]) for c in corpus["cases"]])
