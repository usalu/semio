"""✂️ Partition v2: orphan-aware F set (impls and macro invocations that mention fixture types travel with them)."""
import sys, os, json, collections, re
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import part, unitgraph
FIXTURE_TYPES = set('DemoMutation AddN DeleteN SetN AssignN LossyMutation SeverityMutation SetWarningN SetErrorN SetFatalN TimestampedMutation ValidatedMutation'.split())
def macro_name(r):
    m = re.match(r'\s*(?:#\[[^\]]*\]\s*)*([A-Za-z_][A-Za-z_0-9]*)\s*!', r['text'])
    return m.group(1) if m else None
def partition(its, names, dep, attach, cuts, external):
    tests = [r['idx'] for r in its if r['test']]
    def chunk_of_idx(i):
        return sum(1 for x in cuts if i >= x)
    # macro invocations depend on their macro
    mac = {r['name']: r['idx'] for r in its if r['kind'] == 'macro_rules'}
    for r in its:
        if r['kind'] is None:
            mn = macro_name(r)
            if mn in mac: dep[r['idx']].add(mac[mn])
            r['macro'] = mn
    users = collections.defaultdict(set)
    for t in tests:
        seen = set(); st = [t]
        while st:
            x = st.pop()
            for y in dep[x]:
                if y not in seen: seen.add(y); st.append(y)
        for y in seen: users[y].add(chunk_of_idx(t))
    F = set()
    def nm(r): return r['name'] or r.get('selfty')
    for r in its:
        if r['kind'] in ('use', 'mod') or r['test']: continue
        if r['kind'] in ('fn', 'struct', 'enum', 'trait', 'const', 'static', 'macro_rules', 'type') and r['name'] in external: F.add(r['idx'])
        if len(users.get(r['idx'], ())) >= 2: F.add(r['idx'])
    ftypes = set(FIXTURE_TYPES)
    changed = True
    while changed:
        changed = False
        for i in list(F):
            r = its[i]
            if r['kind'] in ('struct', 'enum') and r['name'] not in ftypes: ftypes.add(r['name']); changed = True
        for r in its:
            i = r['idx']
            if i in F or r['kind'] in ('use', 'mod') or r['test']: continue
            if r['kind'] == 'impl' and r.get('selfty') in ftypes: F.add(i); changed = True
            elif r['kind'] is None and r.get('macro') and (r['ids'] & ftypes): F.add(i); changed = True
        for i in list(F):
            for j in dep[i]:
                if j not in F and not its[j]['test'] and its[j]['kind'] not in ('use', 'mod'):
                    F.add(j); changed = True
            # impls attached to F-defined names travel
            r = its[i]
            if r['kind'] in ('struct', 'enum', 'trait', 'fn', 'const', 'static') and r['name']:
                for j in attach.get(r['name'], []):
                    if j not in F and not its[j]['test']: F.add(j); changed = True
    assign = {}
    for r in its:
        i = r['idx']
        if r['kind'] in ('use', 'mod'): continue
        if i in F: assign[i] = 'F'
        elif r['test']: assign[i] = chunk_of_idx(i)
        else:
            u = users.get(i)
            assign[i] = min(u) if u else chunk_of_idx(i)
    return assign, F, ftypes
