import sys, os, re, collections
sys.path.insert(0, ".")
from emit import *
lay = Layout("plan3"); em = Emitter(lay); em.run()
# access tokens per out
acc = {}
for o in lay.outs:
    names = set()
    for u in o.units:
        t = u.it.text
        names |= set(re.findall(r'\.\s*([A-Za-z_]\w*)\s*(?:::<[^>]*>)?\s*\(', t))
        names |= set(re.findall(r'\.\s*([A-Za-z_]\w*)\b(?!\s*\()', t))
        names |= set(re.findall(r'\.\s*(\d+)\b', t))
        names |= set(re.findall(r'\b([A-Za-z_]\w*)\s*:', t))      # struct literal fields
        names |= set(re.findall(r'\b([A-Za-z_]\w*)\s*[,}]', t))   # shorthand
    acc[id(o)] = names
# crates reachable
deps = em.crate_deps
reach = {}
def closure(c, seen=None):
    seen = seen or set()
    for d in deps.get(c, ()):
        if d not in seen: seen.add(d); closure(d, seen)
    return seen
tot = 0; per = collections.Counter(); sample = []
for a in lay.w.atoms:
    if id(a) not in lay.crate_of or a.kind not in ("struct", "enum"): continue
    to = em.out_of_atom(a)
    if to is None: continue
    for name, lst in em.members_of(a).items():
        for (it, d) in lst:
            if vis_rank(d["vis"]) >= 3: continue
            need = 0
            for o in lay.outs:
                if o is to or name not in acc[id(o)]: continue
                if o.crate == to.crate: need = max(need, 2)
                elif to.crate in closure(o.crate): need = max(need, 3)
            if need > vis_rank(d["vis"]):
                tot += 1; per[a.name] += 1
                if len(sample) < 25 and name not in ("new", "len"): sample.append((a.name, name, d["vis"], need))
print("members that an over-approximate scan would widen:", tot, "types:", len(per))
print(per.most_common(15)); print(sample[:25])
