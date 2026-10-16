import sys, os, collections, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from cut import *
def cluster_graph(cut):
    g = collections.defaultdict(lambda: collections.defaultdict(list))
    for key, e in cut.ents.items():
        c = e['cluster']
        if c in (None, 'UNASSIGNED') or e['name'].startswith('<'): continue
        fam = FAMILY(e['file'])
        for tk in e['tokens']:
            if tk in NOISE or (c, tk) in FALSE_EDGES or tk == e['name']: continue
            cands = [x for x in cut.name2ents.get(tk, []) if x != key]
            if not cands: continue
            same = [x for x in cands if cut.ents[x]['file'] == e['file']]
            if same: continue
            fam_c = [x for x in cands if FAMILY(x[0]) == fam]
            use = fam_c or cands
            for x in use[:1] if len(use) > 1 else use:
                d = cut.ents[x]['cluster']
                if d and d != c: g[c][d].append((e['name'], tk))
    return g
def sccs(g):
    nodes = set(g) | {d for m in g.values() for d in m}
    idx = {}; low = {}; st = []; on = set(); out = []; cnt = [0]
    sys.setrecursionlimit(10000)
    def sc(v):
        idx[v] = low[v] = cnt[0]; cnt[0] += 1; st.append(v); on.add(v)
        for w in g.get(v, {}):
            if w not in idx: sc(w); low[v] = min(low[v], low[w])
            elif w in on: low[v] = min(low[v], idx[w])
        if low[v] == idx[v]:
            comp = []
            while True:
                w = st.pop(); on.discard(w); comp.append(w)
                if w == v: break
            out.append(comp)
    for v in sorted(nodes):
        if v not in idx: sc(v)
    return [c for c in out if len(c) > 1]
if __name__ == '__main__':
    cut = Cut(sys.argv[1] if len(sys.argv) > 1 else REPO)
    g = cluster_graph(cut)
    for c in sorted(g): print(c, '->', ', '.join(f"{d}({len(v)})" for d, v in sorted(g[c].items())))
    print('CYCLES', sccs(g))
