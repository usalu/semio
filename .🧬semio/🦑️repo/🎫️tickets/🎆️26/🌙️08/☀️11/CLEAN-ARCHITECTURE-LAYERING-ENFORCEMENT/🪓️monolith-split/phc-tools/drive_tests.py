#!/usr/bin/env python3
"""🧪️ Per-crate test-configuration differentials, run in parallel against the pristine monolith."""
import os, sys, collections
from concurrent.futures import ThreadPoolExecutor
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import plan_host, oracle_host as O

ws, C, roots, base_ws = plan_host.prepare()
print('prepared; vis log', C.vis_log[0], 'missed', C.vis_log[1][:3], flush=True)
names = sys.argv[1:] or list(roots)
b = O.baseline('test', True)
def one(c):
    s = O.split('test_' + c, ws, roots, c)
    return c, s
with ThreadPoolExecutor(max_workers=int(os.environ.get('WORKERS', '4'))) as ex:
    for c, s in ex.map(one, names):
        O.compare('test:' + c, b, s, set(roots), limit=int(os.environ.get('LIMIT', '40')))
        sys.stdout.flush()
print('finished', flush=True)
