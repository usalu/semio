"""Item dependency edges with receiver-aware method resolution (own-type first, family-filtered)."""
import sys, os, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from graph2 import *

def family(name, mod):
    if name:
        if name.startswith('Board'): return 'b'
        if name.startswith('Dag'): return 'd'
    if mod:
        if '➕️normal' in mod: return 'b'
        if '🕸️dag' in mod: return 'd'
    return None

def edges2(nodes, ambiguous='all'):
    top, meth = indexes(nodes)
    g = collections.defaultdict(set)
    amb = collections.defaultdict(set)
    for n in nodes:
        for idn in n.plain:
            if idn == '_': continue
            cands = top.get(idn, ())
            same = [d for d in cands if d.mod == n.mod]
            if same and len(cands) > 1: cands = same
            for d in cands:
                if d is n: continue
                if d.kind == 'fn' and idn not in n.calls: continue
                g[n.id].add(d.id)
        for (ty, nm) in n.assoc:
            for d in meth.get(nm, ()):
                if d.parent == ty and d is not n: g[n.id].add(d.id)
        nfam = family(n.parent or n.name, n.mod)
        for nm in n.dots:
            ms = meth.get(nm, ())
            if not ms: continue
            if n.parent is not None:
                own = [d for d in ms if d.parent == n.parent and d is not n]
                if own:
                    for d in own: g[n.id].add(d.id)
                    continue
            continue
        for sname in n.astrs:
            for d in top.get(sname, ()):
                if d.kind == 'fn' and d is not n:
                    g[n.id].add(d.id); g[d.id].add(n.id)
        if n.parent and n.kind != 'struct':
            for d in top.get(n.parent, ()): g[n.id].add(d.id)
    return g, amb
