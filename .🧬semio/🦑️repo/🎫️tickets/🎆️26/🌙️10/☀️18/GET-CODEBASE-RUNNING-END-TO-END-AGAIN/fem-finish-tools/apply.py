#!/usr/bin/env python3
"""Anchored apply of the staged fem-finish set. Dry run by default; --write applies all-or-nothing. Refuses on any content drift."""
import os, sys, json, hashlib
LIVE = os.environ.get('FF_LIVE', '/Users/ueli/Documents/semio')
S = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'staged')
sha = lambda b: hashlib.sha256(b).hexdigest()
ops = json.load(open(os.path.join(S, 'manifest.json')))
bad = []
for o in ops:
    p = os.path.join(LIVE, o['path'])
    cur = open(p, 'rb').read() if os.path.exists(p) else None
    if o['op'] == 'add':
        if cur is not None and sha(cur) != o['new']: bad.append((o['path'], 'exists with other content'))
    elif o['op'] == 'delete':
        if cur is not None and sha(cur) != o['old']: bad.append((o['path'], 'drifted'))
    else:
        if cur is None: bad.append((o['path'], 'missing'))
        elif sha(cur) not in (o['old'], o['new']): bad.append((o['path'], 'drifted'))
already = sum(1 for o in ops if (o['op'] != 'delete' and os.path.exists(os.path.join(LIVE, o['path'])) and sha(open(os.path.join(LIVE, o['path']), 'rb').read()) == o.get('new')) or (o['op'] == 'delete' and not os.path.exists(os.path.join(LIVE, o['path']))))
print(f'{len(ops)} operations, {already} already applied, {len(bad)} drifted')
for b in bad[:60]: print('  DRIFT', b[0][-120:], b[1])
if bad or '--write' not in sys.argv: sys.exit(1 if bad else 0)
tmp = []
for o in ops:
    p = os.path.join(LIVE, o['path'])
    if o['op'] == 'delete': tmp.append((p, None)); continue
    tmp.append((p, open(os.path.join(S, 'files', o['path']), 'rb').read()))
for p, data in tmp:
    if data is None:
        if os.path.exists(p): os.remove(p)
        continue
    os.makedirs(os.path.dirname(p), exist_ok=True)
    t = p + '.ffx'; open(t, 'wb').write(data); os.replace(t, p)
print('applied', len(tmp))
