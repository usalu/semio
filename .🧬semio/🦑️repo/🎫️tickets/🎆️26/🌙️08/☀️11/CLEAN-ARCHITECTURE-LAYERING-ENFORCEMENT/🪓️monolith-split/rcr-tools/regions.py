"""Region-level dependency analysis of a crate root (top-level names plus .m / ::m member references)."""
import sys, re, collections
sys.path.insert(0, ".")
from rcrlib import *

name = sys.argv[1]
t = TARGETS[name]
src = read(f"{ROOT}/{t['lib']}")
ls = src.split("\n")
its = top_items(src)
regions = []
for i, l in enumerate(ls):
    m = re.match(r"^\s*// ?#region (.*)$", l)
    if m:
        regions.append([m.group(1), i, None])
    m = re.match(r"^\s*// ?#endregion", l)
    if m:
        regions[-1][2] = i
def unit_of(ln):
    for r in regions:
        if r[1] <= ln <= (r[2] if r[2] is not None else 10**9):
            return r[0]
    return "<outside>"
topdefs = collections.defaultdict(set)
memdefs = collections.defaultdict(set)
refs = collections.defaultdict(set)
for it in its:
    u = unit_of(it["hdr_line"])
    if it["kind"] in ("fn", "struct", "enum", "trait", "type", "const", "static", "union", "macro") and it["name"]:
        topdefs[u].add(it["name"])
    if it["kind"] in ("impl", "trait"):
        for nm, vis in impl_members(it):
            memdefs[u].add(nm)
    refs[u] |= it["ctx"]
topowner = collections.defaultdict(set)
memowner = collections.defaultdict(set)
for u, ns in topdefs.items():
    for n in ns: topowner[n].add(u)
for u, ns in memdefs.items():
    for n in ns: memowner[n].add(u)
edges = collections.defaultdict(lambda: collections.defaultdict(set))
for u in refs:
    for n, member in refs[u]:
        for o in topowner.get(n, ()):
            if o != u: edges[u][o].add(n)
        if member:
            for o in memowner.get(n, ()):
                if o != u: edges[u][o].add("." + n)
allu = [r[0] for r in regions] + ["<outside>"]
for u in allu:
    print(f"== {u}")
    for o, ns in sorted(edges[u].items(), key=lambda kv: -len(kv[1])):
        print(f"   -> {o}: {len(ns)} e.g. {sorted(ns)[:7]}")
