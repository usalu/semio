import sys, re, collections
sys.path.insert(0, ".")
from prod import *
files = walk(LIBROOT, ["crate"])
def topmod(f):
    mp = f.modpath
    return mp[1] if len(mp) > 1 else "(root-glue)"
MODS = set(topmod(f) for f in files.values())
mode = sys.argv[1] if len(sys.argv) > 1 else "prod"
g = collections.defaultdict(set)
for p, f in files.items():
    t = topmod(f)
    code = prod_code(f, files) if mode == "prod" else f.code
    for m in re.finditer(r'\bcrate::(\w+)', code):
        if m.group(1) in MODS and m.group(1) != t: g[t].add(m.group(1))
    for m in re.finditer(r'\bcrate::\{', code):
        paths,_ = parse_use_tree(code, m.start())
        for pp in paths:
            if len(pp) > 1 and pp[1] in MODS and pp[1] != t: g[t].add(pp[1])
def tarjan(g, nodes):
    idx = {}; low = {}; st = []; on = set(); res = []; c = [0]
    def sc(v):
        idx[v] = low[v] = c[0]; c[0] += 1; st.append(v); on.add(v)
        for w in g.get(v, ()):
            if w not in idx: sc(w); low[v] = min(low[v], low[w])
            elif w in on: low[v] = min(low[v], idx[w])
        if low[v] == idx[v]:
            comp = []
            while True:
                w = st.pop(); on.discard(w); comp.append(w)
                if w == v: break
            res.append(sorted(comp))
    for v in sorted(nodes):
        if v not in idx: sc(v)
    return res
for comp in tarjan(g, MODS - {"test_allocation", "source_builders"}):
    print(len(comp), comp)
