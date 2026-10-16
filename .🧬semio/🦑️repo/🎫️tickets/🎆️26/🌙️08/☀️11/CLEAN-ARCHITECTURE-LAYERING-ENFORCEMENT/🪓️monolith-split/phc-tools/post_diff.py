#!/usr/bin/env python3
"""🔁️ Re-runs the stitched differential from the persisted workspace after manual on-disk fixes (native plus every test configuration)."""
import os, sys
from concurrent.futures import ThreadPoolExecutor
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import state, oracle_host as O

import extern_set
extern_set.install()
ws, model, roots = state.load()
names = sys.argv[1:] or list(roots)
bn = O.baseline('native', None)
bt = O.baseline('test', True)


def one(c):
    if c == '__native__':
        return c, O.split('native', ws, roots, None)
    return c, O.split('test_' + c, ws, roots, c)


with ThreadPoolExecutor(max_workers=int(os.environ.get('WORKERS', '3'))) as ex:
    res = dict(ex.map(one, ['__native__'] + names))
O.compare('native', bn, res['__native__'], set(roots), limit=60)
for c in names:
    O.compare('test:' + c, bt, res[c], set(roots), limit=60)
print('finished', flush=True)
