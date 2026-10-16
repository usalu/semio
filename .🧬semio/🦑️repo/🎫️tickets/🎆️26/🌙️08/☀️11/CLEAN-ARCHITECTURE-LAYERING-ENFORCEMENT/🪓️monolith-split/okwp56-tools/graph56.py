# 🕸️ Crate graph of the WP5/WP6 batch from the written manifests: first-party edges, rank check against the plan's rank order, cycle check.
import os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + '/../okwp0-tools')
import okcrates as C
model = C.build()[1]
crates = open(HERE + '/../🗑️generated/okwp56-b1.crates').read().split()
rank = {k: i for i, k in enumerate(C.RANK)}
edges = {}
for k in C.RANK:
    p = C.pkg_abs(k, model) + '/Cargo.toml'
    if not os.path.exists(p): continue
    t = open(p, encoding='utf8').read()
    deps = set()
    for sec in re.split(r'\n(?=\[)', t):
        if not re.match(r'\[(dependencies|target\..*dependencies)\]', sec): continue
        for m in re.finditer(r'^(semio-framework-[a-z0-9-]+)\s*=', sec, re.M):
            d = m.group(1)[len('semio-framework-'):]
            if d in rank: deps.add(d)
    edges[k] = sorted(deps, key=lambda x: rank[x])
bad = 0
for k in crates:
    print(k, '->', ', '.join(edges.get(k, [])))
    for d in edges.get(k, []):
        if rank[d] >= rank[k]: bad += 1; print('  BACK-EDGE', k, '->', d)
# cycle check over all crates with manifests
state = {}
def dfs(n, path):
    state[n] = 1
    for d in edges.get(n, []):
        if state.get(d) == 1: print('CYCLE', path + [n, d]); return True
        if d not in state and dfs(d, path + [n]): return True
    state[n] = 2
    return False
cyc = any(dfs(n, []) for n in list(edges) if n not in state)
print('back edges', bad, '| cycle', cyc)
