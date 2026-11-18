"""Silhouette repairs round 1: restores the byte-stable volumes of the four assets my first pass reformatted and applies the same fixes as in-place span edits (elderly-with-cane, wheelchair-user, pig, bar-stool)."""
import importlib.util, json, re
spec = importlib.util.spec_from_file_location('s', 'sil-span.py'); s = importlib.util.module_from_spec(spec); spec.loader.exec_module(s)
def restore(slug):
    cur, new = s.restore_volumes(slug)
    open(s.path_of(slug), "w", encoding="utf8").write(new)
def pv(d, node, key): return s.child(node, key)
for slug in ('🛋️furniture/bar-stool', '👤️humans/elderly-with-cane', '👤️humans/wheelchair-user', '🐕️animals/pig'): restore(slug)

d = s.Doc('🛋️furniture/bar-stool')
leg = d.find('500', 'leg'); d.rep(s.child(leg, 'p1') if False else dict(zip(('s', 'e'), (0, 0))), '') if False else None
p1 = s.child(leg, 'p1'); d.rep(p1, '[0.20875, 0.0125, 0.0]')
ring = d.find('500', 'ring'); prof = s.child(ring, 'profile')
for pt in prof['items']:
    r = s.child(pt, '1'); v = float(d.t[r['s']:r['e']]); d.rep(r, repr(round(v - 0.0225, 5)))
d.commit()

d = s.Doc('👤️humans/elderly-with-cane')
for sc, sid, off, r in (('200', 'cane-shaft', -0.0075, 0.018), ('200', 'cane-ferrule', -0.0075, None), ('500', 'cane', -0.022, 0.0325)):
    n = d.find(sc, sid)
    for k in ('p0', 'p1'):
        a = s.child(n, k); v = json.loads(d.t[a['s']:a['e']]); d.rep(a, s.flat([v[0], off, v[2]]))
    if r is not None:
        for k in ('r0', 'r1'): d.rep(s.child(n, k), repr(r))
d.commit()

d = s.Doc('👤️humans/wheelchair-user')
for sid in ('rear-tyre-l', 'rear-tyre-r'):
    for pt in s.child(d.find('500', sid), 'profile')['items']:
        r = s.child(pt, '1'); v = float(d.t[r['s']:r['e']]); d.rep(r, '0.24' if abs(v - 0.2655) < 1e-9 else '0.305')
d.commit()

d = s.Doc('🐕️animals/pig')
n = d.find('500', 'hindbody'); secs = s.child(n, 'sections')
def sub_edit(text, kv):
    root = s.parse(text); edits = []
    for k, v in kv.items():
        node = s.child(root, k); edits.append((node['s'], node['e'], v))
    for a, b, x in sorted(edits, reverse=True): text = text[:a] + x + text[b:]
    return text
s0, s1 = secs['items'][0], secs['items'][1]
t0, t1 = d.t[s0['s']:s0['e']], d.t[s1['s']:s1['e']]
d.rep(s0, sub_edit(t0, {'at': '[-0.2534, -0.105, 0]'}))
d.rep(s1, sub_edit(t1, {'at': '[%s, -0.1115, 0]' % json.loads(d.t[s.child(s1, 'at')['s']:s.child(s1, 'at')['e']])[0], 'rx': '0.1198', 'ry': '0.125'}))
indent = re.search(r'\n([ ]*)$', d.t[:s1['s']]).group(1) if re.search(r'\n([ ]*)$', d.t[:s1['s']]) else ''
new = sub_edit(t1, {'at': '[-0.2097, -0.118, 0]', 'rx': '0.0975', 'ry': '0.105'})
d.rep({'s': s1['s'], 'e': s1['s']}, new + ',\n' + indent)
d.edits = [e for e in d.edits if not (e[0] == s1['s'] and e[1] == s1['e'])] + [(s1['s'], s1['e'], sub_edit(t1, {'at': '[%s, -0.1115, 0]' % json.loads(d.t[s.child(s1, 'at')['s']:s.child(s1, 'at')['e']])[0], 'rx': '0.1198', 'ry': '0.125'}))]
d.commit()
