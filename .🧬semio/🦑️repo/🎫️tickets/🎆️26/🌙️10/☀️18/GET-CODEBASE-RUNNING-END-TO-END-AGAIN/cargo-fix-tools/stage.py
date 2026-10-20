#!/usr/bin/env python3
"""Stage the overlay delta (r0 -> r) as an anchored patch set under staged/. usage: stage.py [--live ROOT]"""
import os, sys, json, hashlib, shutil
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..')))
from winpath import place
R0, R = '/private/tmp/cargo-fix/r0', '/private/tmp/cargo-fix/r'
LIVE = '/Users/ueli/Documents/semio'
OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'staged')
sha = lambda b: hashlib.sha256(b).hexdigest()
def walk(root):
    for d, dn, fn in os.walk(root):
        for f in fn:
            if f.endswith('.rs') or f == 'Cargo.toml':
                p = os.path.join(d, f)
                yield os.path.relpath(p, root)
files0 = set(walk(R0)); files1 = set(walk(R))
ops = []
EXCLUDE = ('/🌱️value/🛬️decode/♻️retirement/', '/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/📂️sources/')
shutil.rmtree(OUT, ignore_errors=True); os.makedirs(OUT)
for rel in sorted(files0 | files1):
    a = open(os.path.join(R0, rel), 'rb').read() if rel in files0 else None
    b = open(os.path.join(R, rel), 'rb').read() if rel in files1 else None
    if a == b or any(x in rel for x in EXCLUDE): continue
    if b is None: ops.append({'path': rel, 'op': 'delete', 'old': sha(a)}); continue
    place(os.path.join(OUT, 'files'), rel, b)
    ops.append({'path': rel, 'op': 'add' if a is None else 'modify', 'old': sha(a) if a is not None else None, 'new': sha(b)})
json.dump(ops, open(os.path.join(OUT, 'manifest.json'), 'w'), ensure_ascii=False, indent=1)
print(len(ops), 'operations:', {k: sum(1 for o in ops if o['op'] == k) for k in ('modify', 'add', 'delete')})
