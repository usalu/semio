"""🕸️ Prints the old replication mount graph (analysis only)."""
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
SP = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/rep4"
BASE = SP + "/base/"
R = "🧰️framework/🔨️modules/📡️replication/"
PR = R + "📦️packages/🦀️rust/🦀️.rs"
PATH_RX = re.compile(r'#\[\s*path\s*=\s*"([^"]*)"\s*\]')

def mounts(rel):
    t = open(BASE + rel, encoding="utf8").read()
    return [(os.path.normpath(os.path.join(os.path.dirname(rel), m.group(1))), m.group(1)) for m in PATH_RX.finditer(t)]

seen = {}
def go(rel, parent):
    if rel in seen:
        print("DUP", rel, parent, seen[rel]); return
    seen[rel] = parent
    if not rel.endswith(".rs") or not os.path.exists(BASE + rel):
        print("MISSING", rel); return
    for p, raw in mounts(rel):
        if p.endswith(".rs"): go(p, rel)

go(PR, None)
for k, v in seen.items(): print(k.replace(R, ""), "<-", (v or "").replace(R, ""))
print(len(seen))
allrs = []
for dp, dn, fn in os.walk(BASE + R):
    for f in fn:
        if f.endswith(".rs"): allrs.append(os.path.relpath(os.path.join(dp, f), BASE))
print("unmounted:", [a.replace(R, "") for a in allrs if a not in seen])
