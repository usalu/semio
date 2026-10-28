# [aud-staged-g] read-only drift and anchor check of staged batches (no writes)
import json, hashlib, os, sys, collections
R = '/Users/ueli/Documents/semio/'
T = R + '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/'
def sha(p):
    return hashlib.sha256(open(p, 'rb').read()).hexdigest() if os.path.isfile(p) else 'absent'
def rd(p):
    return open(p, encoding='utf-8').read() if os.path.isfile(p) else None
def edits_status(path, edits):
    c = collections.Counter(); det = []
    for e in edits:
        f = R + e['file']; t = rd(f)
        if t is None: c['absent'] += 1; det.append('absent ' + e['file']); continue
        o, n = t.count(e['old']), t.count(e['new']) if e.get('new') else 0
        if o == 1: c['anchor-ok'] += 1
        elif o == 0 and n >= 1: c['applied'] += 1
        else: c['ANCHOR-BAD(count=%d)' % o] += 1; det.append('%s %s' % (o, e.get('name', '') + ' ' + e['file'][-70:]))
    return dict(c), det[:6]
def hash_status(pairs):
    c = collections.Counter(); det = []
    for rel, (b, n) in pairs:
        s = sha(R + rel)
        st = 'base' if s == b else ('applied' if n and s == n else 'DRIFTED')
        c[st] += 1
        if st == 'DRIFTED': det.append(rel[-80:])
    return dict(c), det[:8]
out = {}
for name, p in [('gram2-index-chunks', 'gram2-g-tools/staged/index-chunks/edits.json'),
                ('walk-walks', 'walk-g-tools/staged/walks/edits.json'),
                ('walk-footprint', 'walk-g-tools/staged/footprint/edits.json'),
                ('rev3fix', 'rev3fix-g-tools/staged/rev3fix/edits.json')]:
    d = json.load(open(T + p, encoding='utf-8'))
    out[name] = edits_status(p, d['edits'])
rb = json.load(open(T + 'rev3fix-g-tools/staged/rev3fix/bases.json', encoding='utf-8'))
out['rev3fix-bases'] = hash_status([(k, (v, None)) for k, v in rb.items()])
si = json.load(open(T + 'setup-g-tools/staged/index-edited.json', encoding='utf-8'))
out['setup-index-edited'] = hash_status([(k, (v['base'], v['new'])) for k, v in si.items()])
for name in ['core', 'nx-patch']:
    mf = json.load(open(T + 'nxfix-g-tools/staged/%s/manifest.json' % name, encoding='utf-8'))
    out['nxfix-' + name] = hash_status([(k, (v['base'], v['new'])) for k, v in mf.items()])
cb = json.load(open(T + 'conv-g-tools/base-hashes.json', encoding='utf-8'))
out['conv-g-base'] = hash_status([(k, (v, None)) for k, v in cb.items()])
xs = []
for line in open(T + 'xplat3-g-tools/bases.tsv', encoding='utf-8'):
    line = line.rstrip('\n')
    if not line: continue
    h, rel = line.split('\t', 1)
    xs.append((rel, (h, None)))
out['xplat3-bases'] = hash_status(xs)
for k, v in out.items():
    print(k, json.dumps(v, ensure_ascii=False))
