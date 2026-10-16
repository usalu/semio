"""Unit (type-block) graph with cycle detachment for the os-infinite carve."""
from settle import *

class U:
    __slots__ = ('id','members','lines','kind','name','mod')
    def __repr__(self): return f'<U {self.kind} {self.name} {self.lines}>'

def type_names(nodes):
    d = {}
    for n in nodes:
        if n.kind in ('struct','enum','union','type','trait'): d.setdefault(n.name, []).append(n.id)
    return d

def build_units(nodes, ids, detached=frozenset()):
    """Block = type def + its methods/traitimpls/assoc (unless detached). Returns units list, unit_of (node id -> unit idx)."""
    tn = type_names(nodes)
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
        if n.kind in ('method','assoc-const','assoc-type','traitimpl') and i not in detached and n.parent in block_of_type:
            owner = block_of_type[n.parent]
        if n.kind in ('struct','enum','union','type','trait') and n.name in block_of_type:
            owner = block_of_type[n.name]
        if owner is not None:
            u = units[owner]; u.members.append(i); u.lines += n.lines; unit_of[i] = owner
        else:
            u = U(); u.id = len(units); u.members = [i]; u.lines = n.lines; u.kind = n.kind; u.name = (n.parent + '::' if n.parent else '') + str(n.name); u.mod = n.mod
            units.append(u); unit_of[i] = u.id
    return units, unit_of

def unit_graph(nodes, g, units, unit_of, ids):
    ug = {u.id: set() for u in units}
    for u in units:
        for m in u.members:
            for d in g[m]:
                if d in ids and unit_of[d] != u.id: ug[u.id].add(unit_of[d])
    return ug

def detach_loop(nodes, g, ids, seed_rank, max_rounds=60, verbose=True):
    """seed_rank: node id -> int rank guiding which edges are 'upward'. Detach methods whose edges go up inside an SCC."""
    detached = set()
    for rnd in range(max_rounds):
        units, unit_of = build_units(nodes, ids, detached)
        ug = unit_graph(nodes, g, units, unit_of, ids)
        comps = [c for c in scc(ug) if len(c) > 1]
        if not comps:
            if verbose: print('detach: acyclic after', rnd, 'rounds; detached', len(detached))
            return detached, units, unit_of, ug
        added = 0
        for comp in comps:
            cs = set(comp)
            urank = {}
            for uid in comp:
                u = units[uid]
                # unit rank = rank of its defining (type) member
                tm = [m for m in u.members if nodes[m].kind in ('struct','enum','union','type','trait')]
                urank[uid] = seed_rank[(tm or u.members)[0]]
            for uid in comp:
                u = units[uid]
                for m in u.members:
                    n = nodes[m]
                    if n.kind not in ('method','assoc-const','assoc-type'): continue
                    up = False
                    for d in g[m]:
                        if d in ids and unit_of[d] in cs and unit_of[d] != uid and urank[unit_of[d]] > urank[uid]:
                            up = True; break
                    if up and m not in detached:
                        detached.add(m); added += 1
        if verbose: print(f'round {rnd}: {sum(len(c) for c in comps)} units in {len(comps)} cyclic SCCs; detached +{added}')
        if added == 0:
            print('detach: stuck; remaining SCCs', [[units[x] for x in c][:6] for c in comps][:3])
            return detached, units, unit_of, ug
    return detached, units, unit_of, ug


def merge_cycles(nodes, units, unit_of, ug, limit=2500):
    """Merge remaining cyclic SCC units into single units."""
    comps = [c for c in scc(ug) if len(c) > 1]
    if not comps: return units, unit_of, ug, []
    merged_report = []
    remap = {}
    new_units = []
    cset = {}
    for c in comps:
        for x in c: cset[x] = tuple(sorted(c))
    done = {}
    for u in units:
        if u.id in cset:
            key = cset[u.id]
            if key not in done:
                m = U(); m.id = len(new_units); m.members = []; m.lines = 0; m.kind = 'merged'
                names = []
                for x in key:
                    m.members += units[x].members; m.lines += units[x].lines; names.append(units[x].name)
                m.name = '+'.join(names)[:80]; m.mod = units[key[0]].mod
                new_units.append(m); done[key] = m.id
                merged_report.append((m.lines, names))
            remap[u.id] = done[key]
        else:
            v = U(); v.id = len(new_units); v.members = list(u.members); v.lines = u.lines; v.kind = u.kind; v.name = u.name; v.mod = u.mod
            new_units.append(v); remap[u.id] = v.id
    new_unit_of = {i: remap[uid] for i, uid in unit_of.items()}
    return new_units, new_unit_of, None, merged_report
