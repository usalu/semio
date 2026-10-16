"""Lane planning pipeline: seeds -> units -> cycle handling -> repair -> bands."""
from planeval import *
from units import *

def build_units2(nodes, ids, detached, split_types):
    """Like build_units but types in split_types never absorb their methods (each method is its own unit)."""
    tn = {}
    units = []; unit_of = {}
    block_of_type = {}
    for i in sorted(ids):
        n = nodes[i]
        if n.kind in ('struct','enum','union','type','trait') and n.name not in block_of_type:
            u = U(); u.id = len(units); u.members = [i]; u.lines = n.lines; u.kind = 'block'; u.name = n.name; u.mod = n.mod
            units.append(u); block_of_type[n.name] = u.id; unit_of[i] = u.id
    for i in sorted(ids):
        if i in unit_of: continue
        n = nodes[i]
        owner = None
        if n.kind in ('method','assoc-const','assoc-type','traitimpl') and i not in detached and n.parent in block_of_type and n.parent not in split_types:
            owner = block_of_type[n.parent]
        if n.kind in ('struct','enum','union','type','trait') and n.name in block_of_type and block_of_type[n.name] != unit_of.get(i):
            owner = block_of_type[n.name]
        if owner is not None:
            u = units[owner]; u.members.append(i); u.lines += n.lines; unit_of[i] = owner
        else:
            u = U(); u.id = len(units); u.members = [i]; u.lines = n.lines; u.kind = n.kind; u.name = (n.parent + '::' if n.parent else '') + str(n.name); u.mod = n.mod
            units.append(u); unit_of[i] = u.id
    return units, unit_of

def detach_loop2(nodes, g, ids, srank, split_types, max_rounds=80, verbose=False):
    detached = set()
    for rnd in range(max_rounds):
        units, unit_of = build_units2(nodes, ids, detached, split_types)
        ug = unit_graph(nodes, g, units, unit_of, ids)
        comps = [c for c in scc(ug) if len(c) > 1]
        if not comps: return detached, units, unit_of, ug
        added = 0
        for comp in comps:
            cs = set(comp)
            urank = {}
            for uid in comp:
                u = units[uid]
                tm = [m for m in u.members if nodes[m].kind in ('struct','enum','union','type','trait')]
                urank[uid] = srank[(tm or u.members)[0]]
            for uid in comp:
                u = units[uid]
                for m in u.members:
                    n = nodes[m]
                    if n.kind not in ('method','assoc-const','assoc-type'): continue
                    for d in g[m]:
                        if d in ids and unit_of[d] in cs and unit_of[d] != uid and urank[unit_of[d]] > urank[uid]:
                            if m not in detached: detached.add(m); added += 1
                            break
        if added == 0: return detached, units, unit_of, ug
    return detached, units, unit_of, ug

class UN: pass

def run_lane(nodes, g, ids, seed, order, split_types=(), budget=3200, verbose=False, bias=1.0, pins=None):
    pos = {b: i for i, b in enumerate(order)}
    srank = {n.id: pos[seed(n)] for n in nodes if n.id in ids}
    detached, units, unit_of, ug = detach_loop2(nodes, g, ids, srank, set(split_types))
    units, unit_of, _, rep = merge_cycles(nodes, units, unit_of, ug)
    ug = unit_graph(nodes, g, units, unit_of, ids)
    unodes = []
    for u in units:
        x = UN(); x.lines = u.lines; unodes.append(x)
    useed = {}
    for u in units:
        tm = [m for m in u.members if nodes[m].kind in ('struct','enum','union','type','trait')]
        useed[u.id] = srank[(tm or u.members)[0]]
    uids = set(u.id for u in units)
    pinned = set()
    if pins:
        for u in units:
            ps = [pins[m] for m in u.members if m in pins]
            if ps:
                useed[u.id] = max(pos[p] for p in ps); pinned.add(u.id)
    A, moves = repair(unodes, ug, useed, uids, bias=bias, pinned=pinned)
    mv = {x for x, a, b in moves if A[x] != useed[x]}
    sizes = collections.Counter()
    for u in units: sizes[A[u.id]] += u.lines
    tyband = {}
    for u in units:
        for m in u.members:
            if nodes[m].kind in ('struct','enum','union','type','trait'): tyband.setdefault(nodes[m].name, A[u.id])
    # ext: methods/assoc whose unit band > type band
    ext = []
    for u in units:
        for m in u.members:
            n = nodes[m]
            if n.kind in ('method','assoc-const','assoc-type','traitimpl') and n.parent in tyband and A[u.id] > tyband[n.parent]:
                ext.append(n)
    return dict(pinned=pinned, units=units, unit_of=unit_of, A=A, useed=useed, moved=mv, sizes=sizes, ext=ext, detached=detached, merged=rep, srank=srank, order=order, tyband=tyband, ug=ug, unodes=unodes)
