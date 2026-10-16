#!/usr/bin/env python3
"""🚦️ End-to-end generation with privacy repair: native and every test configuration in parallel, widenings from the evidence, then the final differentials."""
import os, sys, collections, json
from concurrent.futures import ThreadPoolExecutor
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import plan_host, oracle_host as O, autovis, dump, state

ROUNDS = int(os.environ.get('ROUNDS', '3'))
WORKERS = int(os.environ.get('WORKERS', '4'))
ws, C, roots, base_ws = plan_host.prepare()
print('prepared; changed', len(ws.changed()), 'vis applied', C.vis_log[0], 'missed', len(C.vis_log[1]), flush=True)
names = [c for c in (sys.argv[1:] or list(roots))]
bn = O.baseline('native', None)
bt = O.baseline('test', True)


def run_all():
    def one(c):
        if c == '__native__':
            return c, O.split('native', ws, roots, None)
        return c, O.split('test_' + c, ws, roots, c)
    with ThreadPoolExecutor(max_workers=WORKERS) as ex:
        return dict(ex.map(one, ['__native__'] + names))


for r in range(ROUNDS + 1):
    res = run_all()
    if r == ROUNDS:
        break
    C.build_new(roots)
    patches = collections.defaultdict(dict)
    why = []
    for c, s in res.items():
        p, w = autovis.plan(C, s[1], roots)
        for f, ps in p.items():
            patches[f].update(ps)
        why += w
    n = autovis.apply(ws, patches)
    autovis.save_decisions(autovis.load_decisions() + why)
    print(f'repair round {r}: {n} widenings', collections.Counter((w['code'], w['kind']) for w in why), flush=True)
    if n == 0:
        break
    C.build_new(roots)
O.compare('native', bn, res['__native__'], set(roots), limit=int(os.environ.get('LIMIT', '60')))
for c in names:
    O.compare('test:' + c, bt, res[c], set(roots), limit=int(os.environ.get('LIMIT', '60')))
state.save(ws)
print('dumped', dump.dump(ws))
print('finished', flush=True)
