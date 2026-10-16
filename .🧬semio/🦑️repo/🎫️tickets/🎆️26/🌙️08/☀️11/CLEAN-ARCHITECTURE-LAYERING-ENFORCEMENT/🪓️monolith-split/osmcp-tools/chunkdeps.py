import sys, collections
sys.path.insert(0, ".")
from graph2 import *
w, edges, flags = build_graph()
byid = {id(a): a for a in w.atoms}
prod = [a for a in w.atoms if not flags[id(a)] and id(a) not in w.mount_atoms]
ch = {id(a): chunk_of(a) for a in prod}
cw = collections.defaultdict(lambda: collections.Counter())
for s, ts in edges.items():
    for t in ts:
        if ch[s] != ch[t]: cw[ch[s]][ch[t]] += 1
pat = sys.argv[1]
import fnmatch
for c in sorted(cw):
    if fnmatch.fnmatchcase(c, pat):
        print(c)
        for d, n in sorted(cw[c].items(), key=lambda kv: kv[0]):
            print(f"     -> {d}  x{n}")
