import sys, os, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import unitgraph
U = '/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs'
src, its = unitgraph.parse(U)
names = {}
for r in its:
    if r['kind'] in ('fn', 'struct', 'enum', 'trait', 'const', 'static', 'macro_rules', 'type') and r['name']:
        names.setdefault(r['name'], []).append(r['idx'])
# impls attach to selfty (and trait if local)
attach = collections.defaultdict(list)
for r in its:
    if r['kind'] == 'impl':
        for k in (r.get('selfty'), r.get('trait')):
            if k in names: attach[k].append(r['idx'])
# deps: item idx -> set of idx it references (including attached impls of referenced names)
def refs(r):
    out = set()
    for n in r['ids']:
        if n in names:
            for i in names[n]:
                if i != r['idx']: out.add(i)
                for j in attach.get(n, []):
                    if j != r['idx']: out.add(j)
    return out
dep = {r['idx']: refs(r) for r in its}
# add: impl item references its self type item
for r in its:
    if r['kind'] == 'impl':
        for k in (r.get('selfty'), r.get('trait')):
            if k in names:
                for i in names[k]: dep[r['idx']].add(i)
tests = [r['idx'] for r in its if r['test']]
# transitive closure of deps for each test
def closure(i):
    seen = set(); st = [i]
    while st:
        x = st.pop()
        for y in dep[x]:
            if y not in seen: seen.add(y); st.append(y)
    return seen
users = collections.defaultdict(set)
for t in tests:
    for y in closure(t): users[y].add(t)
pos = {t: k for k, t in enumerate(tests)}
rows = []
for r in its:
    if r['test'] or r['kind'] in ('use', 'mod'): continue
    u = sorted(pos[t] for t in users.get(r['idx'], []))
    rows.append((r['idx'], r['a'], r['kind'], r['name'] or r.get('selfty'), r['bytes'], len(u), (u[0], u[-1]) if u else None))
json.dump([[r['idx'], r['a'], r['b'], r['kind'], r['name'], r['bytes'], r['test']] for r in its], open(os.path.dirname(os.path.abspath(__file__)) + '/../🗑️generated/okwp8-unit-items.json', 'w'))
print('tests', len(tests))
tot = 0
for row in rows:
    print(*row)
