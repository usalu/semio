"""✂️ Partition v3: labels (chunk indexes and pinned bins), orphan-aware F, kernel-only items pinned."""
import sys, os, json, collections, re
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import part, part2, unitgraph
PIN_ITEMS = {'RetainedTestMembers': 'M'}
PIN_TESTS = {'committed_wire_witnesses_are_the_canonical_wire': 4}   # twenty embedded wire witnesses: placed where the file budget has room     # items that need the kernel facade: they live in the bin of their label
def compute(its, names, dep, attach, K, external, fixed_cuts=None):
    mac = {r['name']: r['idx'] for r in its if r['kind'] == 'macro_rules'}
    for r in its:
        if r['kind'] is None:
            mn = part2.macro_name(r); r['macro'] = mn
            if mn in mac: dep[r['idx']].add(mac[mn])
    pinned_names = set(PIN_ITEMS)
    # tests that use a pinned item (transitively) get its label
    test_label = {}
    def reach(i):
        seen = set(); st = [i]
        while st:
            x = st.pop()
            nxt = set(dep[x])
            rx = its[x]
            if rx['kind'] in ('struct', 'enum', 'trait', 'fn', 'const', 'static', 'type') and rx['name']:
                nxt |= set(attach.get(rx['name'], []))
            for y in nxt:
                if y not in seen: seen.add(y); st.append(y)
        return seen
    pin_idx = {r['idx']: PIN_ITEMS[r['name']] for r in its if r['name'] in pinned_names}
    for r in its:
        if r['test']:
            rs = reach(r['idx'])
            for p, lab in pin_idx.items():
                if p in rs: test_label[r['idx']] = lab
    for r in its:
        if r['test'] and r['name'] in PIN_TESTS: test_label[r['idx']] = PIN_TESTS[r['name']]
    # chunk cuts over the unpinned tests by cumulative bytes (items in between follow their first test user via users logic)
    tot = sum(r['bytes'] for r in its if test_label.get(r['idx']) is None)
    cum = 0; cuts = []; tgt = tot / K
    for r in its:
        if r['test'] and test_label.get(r['idx']) is None and cum >= tgt * (len(cuts) + 1) and len(cuts) < K - 1:
            cuts.append(r['idx'])
        if test_label.get(r['idx']) is None: cum += r['bytes']
    if fixed_cuts is not None: cuts = list(fixed_cuts)
    def label(i):
        return sum(1 for x in cuts if i >= x)
    tests = [r['idx'] for r in its if r['test']]
    users = collections.defaultdict(set)
    for t in tests:
        lab = test_label.get(t, label(t))
        for y in reach(t): users[y].add(lab)
    F = set(); ftypes = set(part2.FIXTURE_TYPES)
    for r in its:
        if r['kind'] in ('use', 'mod') or r['test']: continue
        if r['name'] in pinned_names: continue
        if r['kind'] in ('fn', 'struct', 'enum', 'trait', 'const', 'static', 'macro_rules', 'type') and r['name'] in external: F.add(r['idx'])
        if len(users.get(r['idx'], ())) >= 2: F.add(r['idx'])
    changed = True
    while changed:
        changed = False
        for i in list(F):
            r = its[i]
            if r['kind'] in ('struct', 'enum') and r['name'] not in ftypes: ftypes.add(r['name']); changed = True
        for r in its:
            i = r['idx']
            if i in F or r['kind'] in ('use', 'mod') or r['test'] or r['name'] in pinned_names: continue
            if r['kind'] == 'impl' and r.get('selfty') in ftypes: F.add(i); changed = True
            elif r['kind'] is None and r.get('macro') and (r['ids'] & ftypes): F.add(i); changed = True
        for i in list(F):
            for j in dep[i]:
                if j not in F and not its[j]['test'] and its[j]['kind'] not in ('use', 'mod'):
                    if its[j]['name'] in pinned_names: raise SystemExit('pinned item needed by F: ' + its[j]['name'])
                    F.add(j); changed = True
            r = its[i]
            if r['kind'] in ('struct', 'enum', 'trait', 'fn', 'const', 'static') and r['name']:
                for j in attach.get(r['name'], []):
                    if j not in F and not its[j]['test']: F.add(j); changed = True
    assign = {}
    for r in its:
        i = r['idx']
        if r['kind'] in ('use', 'mod'): continue
        if i in F: assign[i] = 'F'
        elif r['test']: assign[i] = test_label.get(i, label(i))
        else:
            u = users.get(i)
            if r['name'] in PIN_ITEMS: assign[i] = PIN_ITEMS[r['name']]
            elif u:
                if len(u) > 1:
                    raise SystemExit('item with several labels not in F: %s %s' % (r['name'], u))
                assign[i] = next(iter(u))
            else: assign[i] = label(i)
    # unused helpers follow the items that mention them
    dead = [r['idx'] for r in its if r['idx'] in assign and assign[r['idx']] != 'F' and not r['test'] and r['name'] not in PIN_ITEMS and not users.get(r['idx'])]
    for _ in range(6):
        moved = False
        for x in dead:
            deps_on = [y for y in assign if y != x and x in dep[y] and assign[y] != 'F']
            labs = {assign[y] for y in deps_on}
            if len(labs) == 1 and assign[x] != next(iter(labs)):
                assign[x] = next(iter(labs)); moved = True
        if not moved: break
    return assign, F, ftypes, cuts, test_label
