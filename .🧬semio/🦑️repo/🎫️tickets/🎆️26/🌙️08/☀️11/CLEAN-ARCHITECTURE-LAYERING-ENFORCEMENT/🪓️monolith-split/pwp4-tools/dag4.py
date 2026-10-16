# 🧭️ WP4 layering check: item graph with every identifier token (fn values, statics, consts), final assignment, rank test against Appendix H.
import json, re, collections, sys
from model4 import *
files, items, defs, owner, owner2, text, final1 = load()
IDENT = PT.IDENT
def tokens2(body):
    out = set()
    for m in IDENT.finditer(body):
        w = m.group(0); st = m.start()
        if w in PT.GENERIC: continue
        pre = body[max(0, st - 2):st].rstrip()
        if pre.endswith('.') and not pre.endswith('..'): continue
        out.add(w)
    return out
L = {f: v[0] for f, v in text.items()}
for it in items:
    if 'body' in it: continue
    it['toks2'] = tokens2('\n'.join(L[it['file']][it['s'] - 1:it['e']])) if it['file'] in L else set(it['toks'])
for it in items:
    if 'toks2' not in it: it['toks2'] = set(it['toks'])
final0 = final1
unit = lambda i: owner2.get(i, i)
OVR = json.load(open(HERE + '/overrides4.json')) if os.path.exists(HERE + '/overrides4.json') else {}
crate = {}
LIFTS = ('🧩️extension/🛬️lifted/🏃️runtime/', '🏗️builder/🔌️program/🛬️lifted/🏃️runtime/')
for it in items:
    u = unit(it['id'])
    if u in final0: crate[u] = final0[u]
    f = it['file']
    if u in final0 and (WP4F(f) or f.startswith(LIFTS)):
        c = PT.file_crate(f)
        if not c.startswith('TOP'): crate[u] = c
STATIC_CRATE = json.load(open(HERE + '/statics4.json'))
for it in items:
    if it['kind'] == 'static' and it['defname'] in STATIC_CRATE and WP4F(it['file']): crate[it['id']] = STATIC_CRATE[it['defname']]
# per-item overrides: key "file|defname|s" or "file|defname"
for it in items:
    for key in (f"{it['file']}|{it['defname']}|{it['s']}", f"{it['file']}|{it['defname']}"):
        if key in OVR: crate[unit(it['id'])] = OVR[key]
# adjacency with toks2
dm = collections.defaultdict(list)
for name, ids in defs.items(): dm[name] = [j for j in ids if items[j]['kind'] not in ('impl', 'use', 'mod')]
adj = collections.defaultdict(set)
prod = [it for it in items if unit(it['id']) in crate]
for it in prod:
    u = unit(it['id'])
    for t in it['toks2']:
        ds = dm.get(t)
        if not ds or len(ds) > 3 or t == it['defname']: continue
        for j in ds:
            v = unit(j)
            if v != u and v in crate: adj[u].add(v)
bad = []
for u, vs in adj.items():
    for v in vs:
        cu, cv = crate[u], crate[v]
        if cu == cv: continue
        ru, rv = RANK.get(cu), RANK.get(cv)
        if ru is None or rv is None: continue
        if cu.startswith('TOP') or cv.startswith('TOP'): continue
        if rv > ru:
            iu, iv = items[u], items[v]
            if WP4F(iu['file']) or WP4F(iv['file']) or cu in WP4_CRATES or cv in WP4_CRATES:
                bad.append((cu, iu['file'].split('/')[-2] + ':' + str(iu['s']), iu['defname'] or iu.get('target') or iu['head'][:30], cv, iv['file'].split('/')[-2] + ':' + str(iv['s']), iv['defname'] or iv['head'][:30]))
print('upward edges touching WP4:', len(bad))
agg = collections.Counter((b[0], b[3]) for b in bad)
for k, v in sorted(agg.items()): print('  ', k, v)
if '-v' in sys.argv:
    for b in sorted(bad): print('   ', b)
