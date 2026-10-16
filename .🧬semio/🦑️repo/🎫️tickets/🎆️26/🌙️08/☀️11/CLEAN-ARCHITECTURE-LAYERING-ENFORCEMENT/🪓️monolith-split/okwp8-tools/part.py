"""✂️ Partition of the unit file into chunks and the shared fixture set F."""
import sys, os, json, collections, re
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import unitgraph
R = '/Users/ueli/Documents/semio/'
U = R + '🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs'
EXTERNAL = None
def load():
    src, its = unitgraph.parse(U)
    names = collections.defaultdict(list)
    for r in its:
        if r['kind'] in ('fn', 'struct', 'enum', 'trait', 'const', 'static', 'macro_rules', 'type') and r['name']:
            names[r['name']].append(r['idx'])
    return src, its, names
def build_dep(its, names):
    attach = collections.defaultdict(list)
    for r in its:
        if r['kind'] == 'impl':
            for k in (r.get('selfty'), r.get('trait')):
                if k in names: attach[k].append(r['idx'])
    dep = {}
    for r in its:
        s = set()
        for n in r['ids']:
            if n in names:
                for i in names[n]:
                    if i != r['idx']: s.add(i)
        dep[r['idx']] = s
    for r in its:
        if r['kind'] == 'impl':
            for k in (r.get('selfty'), r.get('trait')):
                if k in names:
                    for i in names[k]: dep[r['idx']].add(i)
    # reverse: a type depends on nothing from its impls, but impls travel with the type (attach)
    return dep, attach
def partition(its, names, dep, attach, cuts, external):
    """cuts: list of item idx; test items with idx >= cuts[i] go to chunk i+1."""
    tests = [r['idx'] for r in its if r['test']]
    def chunk_of_idx(i):
        c = 0
        for x in cuts:
            if i >= x: c += 1
        return c
    users = collections.defaultdict(set)
    for t in tests:
        seen = set(); st = [t]
        while st:
            x = st.pop()
            for y in dep[x]:
                if y not in seen: seen.add(y); st.append(y)
        for y in seen: users[y].add(chunk_of_idx(t))
    F = set()
    for r in its:
        if r['kind'] in ('use', 'mod') or r['test']: continue
        nm = r['name'] or r.get('selfty')
        if nm in external: F.add(r['idx'])
        if len(users.get(r['idx'], ())) >= 2: F.add(r['idx'])
    # impl attached to F types travel; closure
    changed = True
    while changed:
        changed = False
        for r in its:
            if r['idx'] in F or r['kind'] in ('use', 'mod') or r['test']: continue
            if r['kind'] == 'impl':
                st, tr = r.get('selfty'), r.get('trait')
                if (st in names and all(i in F for i in names[st])) :
                    F.add(r['idx']); changed = True
        for i in list(F):
            for j in dep[i]:
                if j not in F and not its[j]['test'] and its[j]['kind'] not in ('use', 'mod'):
                    F.add(j); changed = True
    assign = {}
    for r in its:
        i = r['idx']
        if r['kind'] in ('use', 'mod'): continue
        if i in F: assign[i] = 'F'
        elif r['test']: assign[i] = chunk_of_idx(i)
        else:
            u = users.get(i)
            assign[i] = min(u) if u else chunk_of_idx(i)
    return assign, F
if __name__ == '__main__':
    pass
