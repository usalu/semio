import sys, json, re, collections
R = "/Users/ueli/Documents/semio"
G = R + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
st = json.load(open(G + "/tsl2-state.json", encoding="utf8"))
inscope = set(l for l in open(G + "/tsl2-inscope.txt", encoding="utf8").read().split("\n") if l)
ENV = re.compile(r"Original (native caller|bounded|process|resource|immutable|SDK|historical|installed|loaded|collection|short|default|Source|flag|Core|Cargo|multi|published|compiler|ticket|build|descriptor|dashboard|parent)|Live Original|Exact (canonical persistent|original parent|same-wire|graph purpose|finite|owned)|Nested persistent image|Actual (finite|ticket|incoming|consumed|receiver|batch)|Authored phase|Required original|Build test (law body deadline|receiver)|Finite owned|Dashboard is not installed|birth already received|admission closed|Select the active ticket|Portable workspace policy|Materialized absolute path|daemon terminated")
out = collections.defaultdict(list)
for f, v in st.items():
    if f not in inscope or v["rc"] == 0: continue
    lg = open(f"{G}/tsl2-{v['label']}/{v['key']}.log", encoding="utf8", errors="replace").read().split("\n")
    errs = [l for l in lg if l.startswith("error:") or re.match(r"^(\w*Error|ReferenceError|SyntaxError|TypeError)\b", l)]
    first = errs[0] if errs else ("rc=%s" % v["rc"])
    if v["rc"] < 0: cls = "killed"
    elif not errs: cls = "noerror"
    elif ENV.search(first): cls = "env"
    else: cls = "other"
    out[cls].append((f.replace("🧰️framework/","").replace("🛍️products/🦑️repo/🔨️modules/","P:").replace("🔨️modules/🏃️process/","X:"), v["failed"], first[:150]))
for cls, rows in out.items(): print(cls, len(rows), sum(r[1] for r in rows))
which = sys.argv[1] if len(sys.argv) > 1 else None
if which:
    for r in sorted(out[which]): print("\t".join(map(str, r)))
