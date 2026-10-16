#!/usr/bin/env python3
"""📐️ Layer-aware crate plan evaluation: sizes, upward edges (violations) with item names, per crate."""
import sys, json, collections, os
SZ = json.load(open('/tmp/hubw/sizes.json'))
NOISE = set()

def crate_of(mod, assign):
    best = None
    for crate, prefixes in assign.items():
        for p in prefixes:
            if mod == p or mod.startswith(p + '::'):
                if best is None or len(p) > best[1]:
                    best = (crate, len(p))
    return best[0] if best else None

def is_test(cfg):
    return any('test' in c for c in cfg)

def main(plan_path, edges_path, files_path, verbose=False, only=None):
    plan = json.load(open(plan_path))
    edges = json.load(open(edges_path)); files = json.load(open(files_path))
    order = plan['order']; assign = plan['crates']
    rank = {c: i for i, c in enumerate(order)}
    owner = {f: crate_of(m['mod'], assign) for f, m in files.items()}
    un = [f for f, c in owner.items() if c is None]
    if un: print('UNASSIGNED:', un)
    stats = collections.defaultdict(lambda: dict(lib=[0, 0, 0], test=[0, 0, 0]))
    for f, m in files.items():
        c = owner[f]
        if c is None: continue
        s = SZ.get('🌎️hub/' + f)
        if not s: continue
        if 'lib' in s and not is_test(m['cfg']):
            for k in range(3): stats[c]['lib'][k] += (1 if k == 0 else s['lib'][k])
        if 'test' in s:
            for k in range(3): stats[c]['test'][k] += (1 if k == 0 else s['test'][k])
    srcdirs = [(os.path.dirname(f), owner[f]) for f in files if owner[f]]
    for path, s in SZ.items():
        if not path.startswith('🌎️hub/'): continue
        kinds = {v[0] for v in s.values()}
        if 'embedded' not in kinds: continue
        r = path[len('🌎️hub/'):]
        best = None
        for d, c in srcdirs:
            if r.startswith(d + '/') and (best is None or len(d) > best[0]): best = (len(d), c)
        if not best: continue
        c = best[1]
        if 'lib' in s: stats[c]['lib'][0] += 1; stats[c]['lib'][1] += s['lib'][1]
        if 'test' in s: stats[c]['test'][0] += 1; stats[c]['test'][1] += s['test'][1]
    print(f"{'crate':30s} {'lib files/bytes/lines':>26s} {'test files/bytes/lines':>26s}")
    for c in order:
        s = stats[c]
        fl = lambda v: ' OVER' if (v[0] > 32 or v[1] > 262144 or v[2] > 4000) else ('  warn' if (v[0] > 28 or v[1] > 230000 or v[2] > 3500) else '')
        print(f"{c:30s} {s['lib'][0]:3d}/{s['lib'][1]:7d}/{s['lib'][2]:5d}{fl(s['lib']):6s} {s['test'][0]:3d}/{s['test'][1]:7d}/{s['test'][2]:5d}{fl(s['test'])}")
    viol = collections.defaultdict(list)
    cedges = collections.defaultdict(int)
    for a, gs in edges.items():
        ca = owner.get(a)
        if ca is None: continue
        for g, names in gs.items():
            cg = owner.get(g)
            if cg is None or cg == ca: continue
            tst = is_test(files[a]['cfg']) or is_test(files[g]['cfg'])
            if not tst and [n for n in names if not n.startswith('(mod')]:
                cedges[(ca, cg)] += 1
            real = [n for n in names if not n.startswith('(mod')]
            if rank[cg] > rank[ca] and real and not (tst and plan.get('ignoreTestViolations')):
                viol[(ca, cg, tst)].append((a, g, [n for n in real if n != '*'] or ['*']))
    print('\nCRATE DEPS (non-test):')
    deps = collections.defaultdict(set)
    for (ca, cg), n in sorted(cedges.items()): deps[ca].add(cg)
    for c in order:
        if deps[c]: print(f"  {c} -> {', '.join(sorted(deps[c], key=lambda x: rank[x]))}")
    print('\nUPWARD EDGES (violations):')
    for (ca, cg, tst), lst in sorted(viol.items(), key=lambda kv: (rank[kv[0][0]], rank[kv[0][1]])):
        if only and ca not in only: continue
        print(f"  {ca} -> {cg}{' [test]' if tst else ''}")
        for a, g, names in lst:
            ma = files[a]['mod'].replace('crate::', ''); mg = files[g]['mod'].replace('crate::', '')
            print(f"      {ma} -> {mg}: {', '.join(names)[:200]}")

if __name__ == '__main__':
    args = [a for a in sys.argv[1:] if not a.startswith('-')]
    main(args[0], args[1] if len(args) > 1 else '/tmp/hubw/lib3.json', args[2] if len(args) > 2 else '/tmp/hubw/lib3-files.json')
