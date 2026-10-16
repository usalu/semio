import sys, collections
sys.path.insert(0, ".")
from plancheck2 import *
planmod = sys.argv[1]
plan, w, edges, flags, cr, ch = run(planmod, quiet=True, show=0)
names = [n for n, _ in plan.CRATES]
pieces = collections.defaultdict(lambda: collections.defaultdict(lambda: [0, 0, []]))
for a in w.atoms:
    if id(a) not in cr or cr[id(a)] is None: continue
    parts = [a.item] + a.impls
    for it in parts:
        rec = pieces[it.file.path][cr[id(a)]]
        rec[0] += it.text.count("\n")+1; rec[1] += 1
        rec[2].append(a.name)
for path, per in sorted(pieces.items()):
    rel = os.path.relpath(path, MCP)
    tot = sum(v[0] for v in per.values())
    items = sorted(per.items(), key=lambda kv: -kv[1][0])
    if len(items) > 1:
        print(rel, tot)
        for ci, (l, n, nm) in items: print(f"      {names[ci]:16s} {l:5d}L {n:3d} atoms  {nm[:6]}")
