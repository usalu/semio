import json, sys, collections
U = json.load(open('units.json'))
ents = {}   # id -> dict(file, name, kind, lines, bytes, units[], tokens)
name2ent = collections.defaultdict(list)
order = []
def eid(f, n): return f"{f}::{n}"
anon = 0
for f, us in U.items():
    last = None
    for u in us:
        k = u['kind']
        if u.get('test'): continue
        if k in ('use', 'comment'): continue
        if u.get('name') == '_' and last is not None:
            ents[last]['units'].append(u); continue
        if k == 'mod' and u['name'] and (u['name'].endswith('_tests') or u['name'].endswith('_test')): continue
        if k in ('fn','struct','enum','type','trait','static','mod','union','const','macro_rules!'):
            i = eid(f, u['name'])
            e = ents.setdefault(i, dict(file=f, name=u['name'], kind=k, lines=0, bytes=0, units=[], tokens=set(), vis=u['vis']))
            e['units'].append(u); last = i
            if i not in order: order.append(i)
            name2ent[u['name']].append(i)
for f, us in U.items():
    for u in us:
        k = u['kind']
        if u.get('test'): continue
        if k in ('impl', 'macro-call'):
            t = u.get('target')
            cands = [i for i in name2ent.get(t, []) if ents[i]['file'] == f] or name2ent.get(t, [])
            if cands:
                ents[cands[0]]['units'].append(u)
            else:
                i = eid(f, f"<{k}:{t}@{u['start']}>")
                ents[i] = dict(file=f, name=f"<{k}:{t}@{u['start']}>", kind=k, lines=0, bytes=0, units=[u], tokens=set(), vis='')
                order.append(i)
for i, e in ents.items():
    e['lines'] = sum(u['lines'] for u in e['units'])
    e['bytes'] = sum(u['bytes'] for u in e['units'])
    for u in e['units']: e['tokens'] |= set(u['tokens'])
dups = {n: v for n, v in name2ent.items() if len(v) > 1}
print("dup names:", dups, file=sys.stderr)
FAM = lambda f: 'kernel' if f.startswith('k') and (f=='kernel' or f.startswith('k_')) else ('manifest' if f=='manifest' or f.startswith('m_') else ('interaction' if f.startswith('interaction') else f))
edges = collections.defaultdict(set)
for i, e in ents.items():
    for t in e['tokens']:
        cands = [j for j in name2ent.get(t, []) if j != i]
        if t in ('Platform','ActionInvocation','CommandInvocation','InteractionDefinition','GranularityDefinition') or len(name2ent.get(t, [])) > 1:
            same = [j for j in name2ent.get(t, []) if ents[j]['file'] == e['file']]
            if same: cands = [j for j in same if j != i]
            else:
                fam = [j for j in name2ent.get(t, []) if FAM(ents[j]['file']) == FAM(e['file'])]
                if fam: cands = [j for j in fam if j != i]
                elif t == 'Platform' and e['file'] not in ('root','platform'): cands = [j for j in name2ent.get(t, []) if ents[j]['file']=='manifest']
        for j in cands:
            edges[i].add(j)
json.dump({'ents': {i: dict(file=e['file'], name=e['name'], kind=e['kind'], lines=e['lines'], bytes=e['bytes'], vis=e['vis'], start=min(u['start'] for u in e['units']), tokens=sorted(e['tokens']), units=[dict(start=u['start'],end=u['end'],kind=u['kind']) for u in e['units']]) for i, e in ents.items()}, 'edges': {i: sorted(v) for i, v in edges.items()}, 'order': order}, open('ents.json', 'w'), ensure_ascii=False, indent=0)
# SCC (tarjan)
sys.setrecursionlimit(100000)
idx = {}; low = {}; st = []; on = set(); sccs = []; c = [0]
def sc(v):
    idx[v] = low[v] = c[0]; c[0] += 1; st.append(v); on.add(v)
    for w in edges.get(v, ()):
        if w not in idx:
            sc(w); low[v] = min(low[v], low[w])
        elif w in on: low[v] = min(low[v], idx[w])
    if low[v] == idx[v]:
        comp = []
        while True:
            w = st.pop(); on.discard(w); comp.append(w)
            if w == v: break
        sccs.append(comp)
for v in order:
    if v not in idx: sc(v)
big = [c for c in sccs if len(c) > 1]
print("entities", len(ents), "sccs>1:", len(big))
for comp in sorted(big, key=len, reverse=True):
    files = collections.Counter(ents[x]['file'] for x in comp)
    print(len(comp), sum(ents[x]['lines'] for x in comp), dict(files))
json.dump([c for c in big], open('scc.json', 'w'), ensure_ascii=False, indent=0)
