exec(open('w18.py').read())
from wengine import FIELD2COMP
# function requirement per unit: from the staged text, narrow fn params name comps: use node idents -> TY names
tyname2c = compty
def unit_req(u):
    r = set()
    for m in u.members:
        for idn in nodes[m].plain:
            if idn in tyname2c: r.add(tyname2c[idn])
    return r
home = {}
state_names = {'World3dState'} | HOSTFN
for u in units:
    names = {nodes[m].parent or nodes[m].name for m in u.members}
    if names & state_names or any(nodes[m].name in HOSTFN for m in u.members): home[u.id] = 'state'; continue
    tys = [nodes[m].name for m in u.members if nodes[m].kind in ('struct','enum','union','type')]
    if tys:
        t = tys[0]
        if t in compty: home[u.id] = compty[t]; continue
        if t in single: home[u.id] = single[t]; continue
    home[u.id] = None
# functions: highest comp (by order below) in Req
corder = ['session','camera','selection','gumball','interaction','scene','terrain','retirement','assets','bridge','overlay','trace','authority']
crank = {c: i for i, c in enumerate(corder)}
for u in units:
    if home[u.id] is None and u.kind in ('fn',):
        r = unit_req(u)
        if r: home[u.id] = max(r, key=lambda c: crank[c])
# iterate for the rest: lowest home among users (callers), default model
users = collections.defaultdict(set)
for a in ug:
    for b in ug[a]: users[b].add(a)
for _ in range(10):
    ch = False
    for u in units:
        if home[u.id] is None:
            hs = [home[x] for x in users[u.id] if home[x] is not None]
            if hs and len(hs) == len(users[u.id]):
                h = min(hs, key=lambda c: -1 if c == 'model' else (99 if c == 'state' else crank[c]))
                home[u.id] = h; ch = True
    if not ch: break
for u in units:
    if home[u.id] is None: home[u.id] = 'model'
sizes = collections.Counter()
for u in units: sizes[home[u.id]] += u.lines
print(dict(sizes))
E = collections.Counter()
for a in ug:
    for b in ug[a]:
        if home[a] != home[b]: E[(home[a], home[b])] += 1
# cycles among comps
cg = collections.defaultdict(set)
for (a, b) in E: cg[a].add(b)
cs = [c for c in scc({k: v for k, v in cg.items()}) if len(c) > 1]
print('comp-level SCCs:', cs)
for (a, b), n in sorted(E.items(), key=lambda kv: -kv[1])[:60]: print(a, '->', b, n)
