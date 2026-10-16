#!/usr/bin/env python3
# 🔬️ Residue items (still in the old chunk files): who uses them, which items they use.
import json, collections
from analyze2 import *
ms, its = load()
defs, edges = graph(its)
rev = collections.defaultdict(set)
for i, ds in edges.items():
    for j in ds: rev[j].add(i)
for i, it in enumerate(its):
    if it["mk"] != "chunk": continue
    users = collections.Counter(its[u]["crate"] or "RES:" + its[u]["slug"] for u in rev[i])
    deps = collections.Counter(its[d]["crate"] or "RES:" + its[d]["slug"] for d in edges[i])
    print(f"{it['slug'][:26]:26} {it['kind']:10} {it['name'][:44]:44} {it['end']-it['start']+1:4}L users={dict(users)} deps={dict(deps)}"[:300])
