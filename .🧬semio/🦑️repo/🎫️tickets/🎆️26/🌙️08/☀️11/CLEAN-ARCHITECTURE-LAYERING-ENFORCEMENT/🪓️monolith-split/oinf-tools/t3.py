import sys, os, collections, re
sys.path.insert(0, '.')
from oinf_rs import *
INF = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite"
TY = sys.argv[1]
files = {}
for root in ("🌍️world", "🎲️board"):
    for dp, dn, fn in os.walk(os.path.join(INF, root)):
        if '🧪️tests' in dp or '🧫️fixtures' in dp: continue
        for f in fn:
            if f.endswith('.rs'):
                p = os.path.join(dp, f); files[p[len(INF)+1:]] = p
methods = {}  # name -> (file, lines, text, idents)
dups = collections.Counter()
for rel, p in sorted(files.items()):
    text = open(p, encoding='utf-8').read()
    items, _ = chunk_file(text)
    for it in items:
        if it.kind == 'impl' and it.impl_ty == TY and it.impl_trait is None:
            body_start = text.index('{', it.start)
            # find matching close brace = it.end-1
            inner = text[body_start+1:it.end-1]
            sub, _ = chunk_file(inner)
            for m in sub:
                if m.kind == 'fn':
                    key = m.name
                    dups[key] += 1
                    k2 = key if dups[key] == 1 else f'{key}#{dups[key]}'
                    methods[k2] = (rel.split('/')[-2], inner.count('\n', m.full_start, m.full_end), m)
print(len(methods), 'methods', sum(v[1] for v in methods.values()), 'lines')
names = set(k.split('#')[0] for k in methods)
graph = {}
for k, (mod, ln, m) in methods.items():
    refs = set()
    src = m.text
    for mm in re.finditer(r'(?:\.|Self::|%s::)([A-Za-z_][A-Za-z0-9_]*)' % TY, src):
        n = mm.group(1)
        if n in names and n != k.split('#')[0]:
            refs.add(n)
    graph[k] = refs
# SCC
sys.setrecursionlimit(100000)
idx = {}; low = {}; st = []; on = set(); sccs = []; c = [0]
def sc(v):
    idx[v] = low[v] = c[0]; c[0] += 1; st.append(v); on.add(v)
    for w in graph.get(v, ()):
        if w not in graph: continue
        if w not in idx:
            sc(w); low[v] = min(low[v], low[w])
        elif w in on:
            low[v] = min(low[v], idx[w])
    if low[v] == idx[v]:
        comp = []
        while True:
            w = st.pop(); on.discard(w); comp.append(w)
            if w == v: break
        sccs.append(comp)
for v in graph:
    if v not in idx: sc(v)
big = sorted(sccs, key=len, reverse=True)
print('sccs', len(sccs), 'largest', [len(b) for b in big[:8]])
for b in big[:3]:
    print(sum(methods[x][1] for x in b), 'lines;', collections.Counter(methods[x][0] for x in b))

vis = collections.Counter()
for k,(mod,ln,m) in methods.items():
    vis[m.vis or 'private'] += 1
print(vis)
import json
json.dump({k:[v[0],v[1],v[2].vis] for k,v in methods.items()}, open(f'/tmp/x/methods-{TY}.json','w'))
