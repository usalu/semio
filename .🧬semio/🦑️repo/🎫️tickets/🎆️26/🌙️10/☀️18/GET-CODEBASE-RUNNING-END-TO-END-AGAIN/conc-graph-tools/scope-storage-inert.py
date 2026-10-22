import os, sys, json
ROOT = "/Users/ueli/Documents/semio"
if "--root" in sys.argv: ROOT = os.path.abspath(sys.argv[sys.argv.index("--root") + 1])
def resolve(rel):
    cur = ROOT
    for part in rel.split("/"):
        hits = [n for n in os.listdir(cur) if n.replace("️", "") == part.replace("️", "")]
        assert len(hits) == 1, (part, hits); cur = os.path.join(cur, hits[0])
    return cur
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
ENGINE = resolve(LIB + "🏗️build/🕸️graph/🕰️generations/🟨️.mjs")
SCHEMA = resolve(LIB + "🏗️build/🕸️graph/🕰️generations/🧬️schema/🔣️.json")
OLD = ["🏗️build/📥️inputs/💾️storage/📒️ledger/"]
NEW = ["🏗️build/📥️inputs/💾️storage/", "⚡️caching/💾️storage/"]
J = lambda items: "[" + ",".join(json.dumps(i, ensure_ascii=False) for i in items) + "]"
t = open(ENGINE, encoding="utf8", newline="").read()
old = "none:Object.freeze(" + J(OLD) + ")"; new = "none:Object.freeze(" + J(NEW) + ")"
if new in t: print("applied engine")
else:
    assert t.count(old) == 1; print("pending engine")
    if "--write" in sys.argv: open(ENGINE, "w", encoding="utf8", newline="").write(t.replace(old, new))
raw = open(SCHEMA, encoding="utf8").read(); d = json.loads(raw)
pol = d["definitions"]["Policy"]["const"]["implementation"]
if pol["none"] == NEW: print("applied schema")
else:
    print("pending schema"); pol["none"] = NEW
    if "--write" in sys.argv: open(SCHEMA, "w", encoding="utf8", newline="").write(json.dumps(d, ensure_ascii=False, indent=1) + ("\n" if raw.endswith("\n") else ""))
