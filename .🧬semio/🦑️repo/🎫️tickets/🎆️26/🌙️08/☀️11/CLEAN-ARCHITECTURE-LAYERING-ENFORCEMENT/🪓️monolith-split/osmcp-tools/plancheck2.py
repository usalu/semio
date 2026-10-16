import sys, fnmatch, collections, importlib, pickle, os
sys.path.insert(0, ".")
from graph2 import *

def assign_chunk(plan, chunk):
    for ci, (name, pats) in enumerate(plan.CRATES):
        for pat in pats:
            if fnmatch.fnmatchcase(chunk, pat): return ci
    return None

def crate_of(plan, a, chunk):
    over = getattr(plan, "ATOM_OVERRIDE", {})
    keyf = f"{chunk}::{a.name}"
    if keyf in over: tgt = over[keyf]
    elif a.name in over: tgt = over[a.name]
    else: return assign_chunk(plan, chunk)
    return [i for i, (n, _) in enumerate(plan.CRATES) if n == tgt][0]

def run(planmod, show=40, quiet=False):
    plan = importlib.import_module(planmod)
    w, edges, flags = build_graph()
    byid = {id(a): a for a in w.atoms}
    prod = [a for a in w.atoms if not flags[id(a)] and id(a) not in w.mount_atoms]
    ch = {id(a): chunk_of(a) for a in prod}
    cr = {}
    un = collections.Counter()
    for a in prod:
        c = crate_of(plan, a, ch[id(a)])
        cr[id(a)] = c
        if c is None: un[ch[id(a)]] += 1
    if un: print("UNASSIGNED:", dict(un))
    viol = collections.defaultdict(list)
    for s, ts in edges.items():
        if s not in cr or cr[s] is None: continue
        for t in ts:
            if t not in cr or cr[t] is None: continue
            if cr[t] > cr[s]: viol[(cr[s], cr[t])].append((byid[s], byid[t]))
    sz = collections.Counter(); by = collections.Counter(); n = collections.Counter()
    for a in prod:
        if cr[id(a)] is None: continue
        t = a.text
        sz[cr[id(a)]] += t.count("\n") + 1; by[cr[id(a)]] += len(t.encode()); n[cr[id(a)]] += 1
    if not quiet:
        for i, (nm, _) in enumerate(plan.CRATES):
            print(f"{i:2d} {nm:22s} atoms={n[i]:4d} lines~{sz[i]:6d} bytes~{by[i]:7d}")
    tot = sum(len(v) for v in viol.values())
    print("VIOLATIONS:", tot)
    for (s, t), lst in sorted(viol.items()):
        print(f" {plan.CRATES[s][0]} -> {plan.CRATES[t][0]}: {len(lst)}")
        seen = set()
        for a, b in lst:
            key = (a.name, b.name)
            if key in seen: continue
            seen.add(key)
            if len(seen) > show: break
            print(f"     {ch[id(a)]}::{a.name}  ->  {ch[id(b)]}::{b.name}")
    return plan, w, edges, flags, cr, ch

if __name__ == "__main__":
    run(sys.argv[1], int(sys.argv[2]) if len(sys.argv) > 2 else 40)
