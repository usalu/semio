"""🧪 Resolves every `protocol::…` path used by consumers outside the replication tree in the old and the new facade."""
import re, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rlib import *
old = World(FACADE_ROOT, staged=False)
new = World(FACADE_ROOT, staged=False, loader=lambda rel: open(REPO + rel, encoding="utf8").read() if os.path.exists(REPO + rel) else None)
paths = []
for l in open("/tmp/consumer_paths.txt", encoding="utf8"):
    m = re.match(r"\s*(\d+)\s+(\S+)", l)
    if m: paths.append((int(m.group(1)), m.group(2)))
groups = []
for l in open("/tmp/consumer_groups.txt", encoding="utf8"):
    m = re.match(r"use (protocol|replication)::\{(.*)\}", l.strip())
    if not m: continue
    inner = m.group(2)
    for leaf in usetree.parse(inner): groups.append((1, "protocol::" + "::".join(leaf[0])))
import rewrite, rcrates as C
model = rewrite.Model({})
by_ident = {C.ident(k): k for k in C.KEYS}
def deref(res, rest_consumed_segs):
    """Follows an external first-party path into the carved crate's world."""
    if res is None or res[0] != "ext" or res[1][0] not in by_ident: return res
    w = model.worlds[by_ident[res[1][0]]]
    r, c = w.walk(w.root, ["crate"] + list(res[1][1:]), mode=2)
    return r
bad = {}
ok = 0; unk = 0
for n, p in paths + groups:
    segs = p.split("::")[1:]
    if not segs: continue
    ro, co = old.walk(old.root, ["crate"] + segs, mode=2)
    rn, cn = new.walk(new.root, ["crate"] + segs, mode=2)
    if ro is None:
        unk += 1; continue
    rn = deref(rn, None)
    if rn is not None and rn[0] == 'mod' and ro[0] == 'ext': rn = ro
    if rn is None or (rn[0] != ro[0] and not (rn[0] == "ext" and ro[0] == "ext")):
        bad[p] = (n, ro[0], rn[0] if rn else None, co, cn)
    else: ok += 1
print("consumer paths ok", ok, "unresolvable-in-old", unk, "regressions", len(bad))
for p, v in sorted(bad.items(), key=lambda x: -x[1][0])[:40]: print(p, v)
