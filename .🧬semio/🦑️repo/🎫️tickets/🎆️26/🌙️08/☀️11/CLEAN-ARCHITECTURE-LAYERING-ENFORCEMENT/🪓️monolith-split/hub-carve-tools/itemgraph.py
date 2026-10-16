#!/usr/bin/env python3
"""🕸️ Item-level dependency graph of one crate: nodes are top-level items (impls folded into their self type), edges are resolved name uses; reports SCCs that span files."""
import sys, os, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import hcmod, carve, ws as wsm
from carve import Model, Def, def_of, free_idents, impl_self_ident, inline_descendants, rel

def build(root_path, ws=None):
    ws = ws or wsm.WS()
    hcmod.FS = ws
    model = Model(ws, {'crate': root_path})
    hcmod.CRATES.clear()
    nodes = {}      # key -> dict(file, name, kind, lines, items)
    owner_of_impl = {}
    allmods = [m for m in hcmod.walk(model.roots['crate']) if not getattr(m, 'missing', False)]
    # nodes
    for S in allmods:
        for name, its in S.items.items():
            for it in its:
                if it.kind in ('use', 'impl', 'extern_crate'):
                    continue
                if it.kind == 'mod' and name in S.children and S.children[name].inline is False:
                    continue
                key = (S.file_root().file, carve.chain_of(S), name)
                nodes.setdefault(key, dict(file=S.file_root().file, chain=carve.chain_of(S), name=name, kind=it.kind, items=[], mod=S))
                nodes[key]['items'].append(it)
    # impls attach
    for S in allmods:
        for it in S.all_items:
            if it.kind != 'impl':
                continue
            ident = impl_self_ident(it)
            key = None
            if ident:
                r = hcmod.lookup(S, ident)
                if r is not None and r.kind == 'item' and r.mod is not None:
                    d = def_of(r)
                    key = (d.file, d.chain, d.name)
            if key is None or key not in nodes:
                key = (S.file_root().file, carve.chain_of(S), '<impl:%s>' % (ident or '?'))
                nodes.setdefault(key, dict(file=S.file_root().file, chain=carve.chain_of(S), name='<impl:%s>' % (ident or '?'), kind='impl', items=[], mod=S))
            nodes[key]['items'].append(it)
            owner_of_impl[id(it)] = (key, S)
    edges = collections.defaultdict(set)
    for key, nd in nodes.items():
        for it in nd['items']:
            S = nd['mod']
            # locate scope module for this item (impl may be in another module than owner)
            if id(it) in owner_of_impl:
                S = owner_of_impl[id(it)][1]
            toks = [t for t in it.toks_all]
            for i, j, segs in free_idents(toks, 0, len(toks)):
                first = segs[0]
                if first in hcmod.KEYWORDS and first not in ('crate', 'self', 'super'):
                    continue
                # try longest prefix that resolves to item/mod
                r = None
                for k in range(len(segs), 0, -1):
                    r = hcmod.resolve(S, segs[:k], None, False, exact=True)
                    if r is not None and r.kind in ('item', 'variant', 'mod'):
                        break
                    r = None
                if r is None:
                    continue
                d = def_of(r)
                if d is None:
                    continue
                tkey = (d.file, d.chain, d.name if r.kind != 'mod' else d.name)
                if r.kind == 'mod':
                    continue
                if tkey in nodes and tkey != key:
                    edges[key].add(tkey)
    return model, nodes, edges

def sccs(nodes, edges):
    index = {}
    low = {}
    onstack = set()
    stack = []
    out = []
    idx = [0]
    sys.setrecursionlimit(100000)
    def strong(v):
        index[v] = low[v] = idx[0]; idx[0] += 1
        stack.append(v); onstack.add(v)
        for w in edges.get(v, ()):
            if w not in index:
                strong(w); low[v] = min(low[v], low[w])
            elif w in onstack:
                low[v] = min(low[v], index[w])
        if low[v] == index[v]:
            comp = []
            while True:
                w = stack.pop(); onstack.discard(w); comp.append(w)
                if w == v: break
            out.append(comp)
    for v in nodes:
        if v not in index:
            strong(v)
    return out

def node_lines(src_cache, nd):
    n = 0
    for it in nd['items']:
        n += it.b and it.toks_all[-1].el - it.toks_all[0].sl + 1
    return n

if __name__ == '__main__':
    root = sys.argv[1]
    model, nodes, edges = build(root)
    comps = [c for c in sccs(nodes, edges) if len(c) > 1]
    print(len(nodes), 'nodes', sum(len(v) for v in edges.values()), 'edges', len(comps), 'cyclic SCCs')
    for c in sorted(comps, key=lambda c: -len(c)):
        files = sorted({rel(k[0]).split('/')[-2] for k in c})
        print(f'SCC size {len(c)} spanning {len(files)} files: {files}')
        if '-v' in sys.argv:
            for k in sorted(c):
                print('    ', rel(k[0]).split('/')[-2], '::'.join(k[1] + (k[2],)))
    json.dump({'nodes': {'|'.join([rel(k[0]), '::'.join(k[1]), k[2]]): dict(kind=v['kind'], lines=node_lines(None, v)) for k, v in nodes.items()},
               'edges': {'|'.join([rel(k[0]), '::'.join(k[1]), k[2]]): ['|'.join([rel(t[0]), '::'.join(t[1]), t[2]]) for t in v] for k, v in edges.items()}}, open(sys.argv[2] if len(sys.argv) > 2 and not sys.argv[2].startswith('-') else '/tmp/hubw/itemgraph.json', 'w'), ensure_ascii=False)
