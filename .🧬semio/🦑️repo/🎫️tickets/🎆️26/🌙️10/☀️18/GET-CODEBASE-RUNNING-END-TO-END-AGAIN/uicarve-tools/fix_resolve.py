"""🔧 uicarve fix_resolve: for unresolved names in companion test crates, import them from where they are defined (widening to pub).
usage: fix_resolve.py <spec.json> <final.err> [--write]"""
import json, os, re, sys, collections
sys.path.insert(0, 'lib')
import carve, rs, peel, crategen, widen as WD
from carve import R, UI, rd, wr
spec = carve.Spec(sys.argv[1]); errf = sys.argv[2]; write = '--write' in sys.argv
by_ident = {c['ident']: k for k, c in spec.crates.items()}
# per crate: file -> module path ('' for root)
modpaths = {}
def walk(path, prefix, key, seen):
    if path in seen or not path.endswith('.rs') or not os.path.exists(path): return
    seen.add(path); modpaths.setdefault(path, (key, prefix))
    for name, p, attrs, pub in carve.mounts(path):
        if name is None: continue
        walk(p, prefix + [name], key, seen)
for k, c in spec.crates.items():
    root = spec.root_file(c)
    if c.get('mounts'):
        for m in c['mounts']:
            f = R + '/' + m['file'] if not m['file'].startswith('/') else m['file']
            walk(f, [m['name']], k, set())
    elif os.path.exists(root): walk(root, [], k, set())
defs = collections.defaultdict(list)
for f, (k, pref) in modpaths.items():
    s = rd(f)
    for it in peel.top_items(s):
        if it.name and it.kind in ('fn', 'const', 'static', 'struct', 'enum', 'type', 'trait', 'union'):
            defs[it.name].append((k, pref, f, it))
need = collections.defaultdict(set)
for l in open(errf, encoding='utf-8'):
    l = l.strip()
    if not l.startswith('{'): continue
    try: d = json.loads(l)
    except Exception: continue
    code = (d.get('code') or {}).get('code')
    msg = d['message']
    m = re.match(r"cannot find (?:value|function|type|struct|trait|macro|attribute|derive macro|tuple struct or tuple variant|unit struct|crate|module) `([^`]+)` in this scope", msg) or re.match(r"failed to resolve: use of undeclared (?:type|crate|module) `([^`]+)`", msg)
    if not m: continue
    prim = [s for s in d['spans'] if s['is_primary']]
    if not prim: continue
    mm = re.match(r'__m_([A-Za-z0-9_]+)/', prim[0]['file_name'])
    if not mm: continue
    need[mm.group(1)].add(m.group(1))
report = []
for ident, names in sorted(need.items()):
    ck = by_ident.get(ident)
    if not ck or not spec.crates[ck].get('tests_only'):
        report.append(('SKIP-nontest', ident, sorted(names)[:8])); continue
    c = spec.crates[ck]; owner = c.get('owner')
    lines = []
    for n in sorted(names):
        cands = defs.get(n, [])
        if not cands: report.append(('UNDEFINED', ident, n)); continue
        pick = next((x for x in cands if x[0] == owner), None) or next((x for x in cands if spec.crates[x[0]].get('tests_only') is None), None) or cands[0]
        tk, pref, f, it = pick
        if tk == ck: continue
        tgt = spec.crates[tk]
        path = '::'.join([tgt['ident']] + pref + [n])
        lines.append('use %s;' % path)
        if write: WD.widen([f], {(n, WD.KIND.get({'fn': 'function', 'const': 'constant', 'static': 'static', 'struct': 'struct', 'enum': 'enum', 'type': 'type alias', 'trait': 'trait', 'union': 'union'}[it.kind]))})
    if lines:
        root = spec.root_file(c)
        s = rd(root)
        add = [l for l in lines if l not in s]
        if add and write:
            us = peel.header_uses(s)
            at = max(u.end for u in us) if us else s.index('#![cfg(test)]') + len('#![cfg(test)]')
            wr(root, s[:at] + '\n' + '\n'.join(add) + s[at:])
        report.append(('ADD', ident, len(add)))
for r_ in report: print(*r_)
