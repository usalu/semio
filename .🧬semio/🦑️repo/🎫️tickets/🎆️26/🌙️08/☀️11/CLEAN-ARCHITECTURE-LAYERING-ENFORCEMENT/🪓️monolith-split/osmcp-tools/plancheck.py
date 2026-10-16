import sys, fnmatch, collections, importlib
sys.path.insert(0, ".")
from chunks import *

def load(planmod):
    p = importlib.import_module(planmod)
    return p

def assign(plan, chunk):
    for ci, (name, pats) in enumerate(plan.CRATES):
        for pat in pats:
            if fnmatch.fnmatchcase(chunk, pat): return ci
    return None

def run(planmod, show=40):
    plan = load(planmod)
    files, allitems, atoms, defs, loose = build()
    edges, amb, flags = atom_edges(files, atoms, defs)
    byid = {id(a): a for a in atoms}
    prod = [a for a in atoms if not flags[id(a)] and a.kind != 'mod']
    ch = {id(a): chunk_of(a) for a in prod}
    over = getattr(plan, "ATOM_OVERRIDE", {})
    cr = {}
    unassigned = collections.Counter()
    for a in prod:
        k = (a.name, topmod(a.file))
        c = over.get(a.name) if (a.name in over) else None
        if c is None:
            c = assign(plan, ch[id(a)])
        else:
            c = [i for i, (n, _) in enumerate(plan.CRATES) if n == c][0]
        if c is None: unassigned[ch[id(a)]] += 1
        cr[id(a)] = c
    if unassigned:
        print("UNASSIGNED chunks:", dict(unassigned)); 
    viol = collections.defaultdict(list)
    for s, ts in edges.items():
        if s not in cr or cr[s] is None: continue
        for t in ts:
            if t not in cr or cr[t] is None: continue
            if cr[t] > cr[s]:
                viol[(cr[s], cr[t])].append((byid[s], byid[t]))
    # crate sizes
    sz = collections.Counter(); by = collections.Counter()
    for a in prod:
        if cr[id(a)] is None: continue
        sz[cr[id(a)]] += a.text.count("\n") + 1
        by[cr[id(a)]] += len(a.text.encode())
    for i, (n, _) in enumerate(plan.CRATES):
        print(f"{i:2d} {n:22s} lines~{sz[i]:6d} bytes~{by[i]:7d}")
    tot = sum(len(v) for v in viol.values())
    print("VIOLATIONS (lower crate depends on higher):", tot)
    for (s, t), lst in sorted(viol.items()):
        print(f" {plan.CRATES[s][0]} -> {plan.CRATES[t][0]}: {len(lst)}")
        seen = set()
        for a, b in lst[:show]:
            key = (a.name, b.name)
            if key in seen: continue
            seen.add(key)
            print(f"     {ch[id(a)]}::{a.name}  ->  {ch[id(b)]}::{b.name}")
    return plan, cr, ch, edges, byid, prod

if __name__ == "__main__":
    run(sys.argv[1], int(sys.argv[2]) if len(sys.argv) > 2 else 40)
