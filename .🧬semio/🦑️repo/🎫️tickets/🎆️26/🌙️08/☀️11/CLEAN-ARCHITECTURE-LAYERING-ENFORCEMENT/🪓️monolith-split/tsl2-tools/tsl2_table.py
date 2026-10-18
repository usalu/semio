import sys, json, re, collections
R = "/Users/ueli/Documents/semio"
G = R + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
label = sys.argv[1]
res = json.load(open(f"{G}/tsl2-{label}.json", encoding="utf8"))
rows = []
for x in res:
    if x["rc"] == 0: continue
    n = collections.Counter(s for s, _ in x["rows"])
    lg = open(f"{G}/tsl2-{label}/{x['key']}.log", encoding="utf8", errors="replace").read().split("\n")
    first = ""
    for i, l in enumerate(lg):
        if l.startswith("error"):
            first = l[:170]
            break
    if not first:
        for l in lg:
            if re.search(r"Cannot find|SyntaxError|ReferenceError|TypeError|ENOENT|Error:", l):
                first = "~" + l.strip()[:170]; break
    f = x["file"].replace("🧰️framework/", "").replace("🛍️products/🦑️repo/🔨️modules/", "P:").replace("🔨️modules/🏃️process/", "X:")
    rows.append((f, x["rc"], n.get("failed", 0), n.get("passed", 0), first.replace(R, "")))
for r in rows: print("\t".join(map(str, r)))
