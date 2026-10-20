#!/usr/bin/env python3
"""Stage the fem-finish overlay delta as an anchored patch set under staged/. usage: stage.py (reads /private/tmp/fem-finish/ov, anchors on the live sha256)."""
import os, json, hashlib, shutil
OV = '/private/tmp/fem-finish/ov'
LIVE = os.environ.get('FF_LIVE', '/Users/ueli/Documents/semio')
OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'staged')
DELETE = [
    '✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧪️tests/🔬️unit/🍳️assembly-page-and-mounted-admission-laws/🦀️.rs',
    '✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧪️tests/🔬️unit/🧈️assembly-csr-element-build-and-job-laws/🦀️.rs',
]
sha = lambda b: hashlib.sha256(b).hexdigest()
shutil.rmtree(OUT, ignore_errors=True); os.makedirs(OUT)
ops = []
for d, _, fs in os.walk(OV):
    for f in sorted(fs):
        p = os.path.join(d, f); rel = os.path.relpath(p, OV)
        new = open(p, 'rb').read()
        lp = os.path.join(LIVE, rel)
        old = open(lp, 'rb').read() if os.path.exists(lp) else None
        if old == new: continue
        dst = os.path.join(OUT, 'files', rel); os.makedirs(os.path.dirname(dst), exist_ok=True); open(dst, 'wb').write(new)
        ops.append({'path': rel, 'op': 'add' if old is None else 'modify', 'old': sha(old) if old is not None else None, 'new': sha(new)})
for rel in DELETE:
    lp = os.path.join(LIVE, rel)
    if os.path.exists(lp): ops.append({'path': rel, 'op': 'delete', 'old': sha(open(lp, 'rb').read())})
ops.sort(key=lambda o: o['path'])
json.dump(ops, open(os.path.join(OUT, 'manifest.json'), 'w'), ensure_ascii=False, indent=1)
print(len(ops), 'operations:', {k: sum(1 for o in ops if o['op'] == k) for k in ('modify', 'add', 'delete')})
