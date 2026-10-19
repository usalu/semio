"""🔁 uicarve consumers2: re-point dependents of semio-framework-ui to the carved crates (sources + manifests).
usage: consumers2.py <alias,...> <spec.json,...> [--write]"""
import json, os, re, sys, collections
sys.path.insert(0, 'lib')
import carve, rs, reg
from carve import R, UI, FW, PKG, rd, wr
aliases = sys.argv[1].split(','); write = '--write' in sys.argv
paths = sys.argv[2].split(',')
merged = carve.Spec(paths[0])
merged.crates = {}; _rules = []
for p in paths:
    sp = carve.Spec(p); ns = sp.d['tree']
    for k, c in sp.crates.items():
        c = dict(c); c['key'] = ns + ':' + k; merged.crates[ns + ':' + k] = c
    for _, r_ in sp.rules.rules:
        r_ = dict(r_)
        if r_.get('crate'): r_['crate'] = ns + ':' + r_['crate']
        _rules.append(r_)
merged.rules = carve.Rules(_rules)
ident_to_crate = {c['ident']: c for c in merged.crates.values()}
FEATMAP = json.load(open('specs/featmap.json', encoding='utf8'))
deps = json.load(open('/Users/ueli/.cache/uicarve/deps.json'))
deps = [x for x in deps if not x['manifest'].startswith('.🧬semio') and x['kind'] != 'ws-dependencies' and x['alias'] in aliases]
consumer_files = json.load(open('/Users/ueli/.cache/uicarve/consumer-files.json'))
by_manifest = collections.defaultdict(list)
for x in deps: by_manifest[x['manifest']].append(x)

def manifest_roots(mp):
    t = rd(mp); d = os.path.dirname(mp); roots = []; has_lib = False
    for sec in re.split(r'(?m)^(?=\[)', t):
        h = sec.split('\n', 1)[0].strip()
        if h in ('[lib]', '[[bin]]', '[[test]]', '[[example]]', '[[bench]]'):
            m = re.search(r'(?m)^path\s*=\s*"([^"]+)"', sec)
            if m: roots.append(os.path.normpath(os.path.join(d, m.group(1))))
            if h == '[lib]': has_lib = True
    if not has_lib and os.path.exists(d + '/src/lib.rs'): roots.append(d + '/src/lib.rs')
    if os.path.exists(d + '/src/main.rs'): roots.append(d + '/src/main.rs')
    for sub in ('src/bin', 'tests', 'examples', 'benches'):
        if os.path.isdir(d + '/' + sub):
            for f in sorted(os.listdir(d + '/' + sub)):
                if f.endswith('.rs'): roots.append(d + '/' + sub + '/' + f)
    return roots

owner = {}
for m in by_manifest:
    seen = []
    for r_ in manifest_roots(R + '/' + m): carve.reachable(r_, seen)
    for f in seen:
        if f.endswith('.rs'): owner.setdefault(os.path.relpath(f, R), []).append(m)
files_of = collections.defaultdict(list)
file_aliases = collections.defaultdict(set)
def fallback_owner(f):
    d = os.path.dirname(R + '/' + f)
    while d.startswith(R) and d != R:
        c = os.path.relpath(d + '/📦️packages/🦀️rust/Cargo.toml', R)
        if c in by_manifest: return [c]
        d = os.path.dirname(d)
    return []
for f in consumer_files:
    if not os.path.exists(R + '/' + f): continue
    for m in (owner.get(f) or fallback_owner(f)):
        if m in by_manifest:
            files_of[m].append(f)
            for e in by_manifest[m]: file_aliases[f].add(e['alias'].replace('-', '_'))
cache = {}
def texts(f):
    if f not in cache:
        old = rd(R + '/' + f); new = carve.rewrite_text(old, merged, None, None, only_roots=file_aliases[f])
        cache[f] = (old, new)
    return cache[f]
def first_idents(text):
    m = rs.mask(text)
    return set(re.findall(r'(?<![\w:.])([A-Za-z_][A-Za-z0-9_]*)\s*::', m)) | set(re.findall(r'\buse\s+([A-Za-z_][A-Za-z0-9_]*)\s*(?:;|as\b)', m))
def known(ident):
    if ident in ident_to_crate: return ('internal', ident_to_crate[ident])
    k = reg.EXT_BY_IDENT.get(ident)
    if k: return ('ext', k)
    if ident.startswith('semio_framework_'):
        pk = ident.replace('_', '-')
        if reg.index().get(pk): return ('index', pk)
    return None
report = []; written = set()
for m, entries in sorted(by_manifest.items()):
    mp = R + '/' + m; mdir = os.path.dirname(mp)
    introduced = set(); touched = 0
    als = {e['alias'].replace('-', '_') for e in entries}
    for f in files_of.get(m, []):
        old, new = texts(f)
        residual = [mm.group(0) for mm in re.finditer(r'\b(%s)\b\s*::\s*\w+' % '|'.join(map(re.escape, als)), rs.mask(new))]
        if residual: report.append(('RESIDUAL', f[-70:], residual[:3]))
        if new != old:
            touched += 1
            introduced |= first_idents(new) - first_idents(old)
            if write and f not in written: wr(R + '/' + f, new); written.add(f)
    intro = {i: known(i) for i in introduced}
    intro = {i: k for i, k in intro.items() if k}
    t = rd(mp)
    for e in entries:
        a = re.escape(e['alias'])
        mm = re.search(r'(?m)^(?P<ind>[ \t]*)"?%s"?\s*=\s*\{[^\n]*\}[ \t]*\n' % a, t)
        if not mm:
            hdr = {'dependencies': 'dependencies', 'dev-dependencies': 'dev-dependencies', 'build-dependencies': 'build-dependencies'}[e['kind']]
            header = '[%s]' % hdr if not e['tgt'] else "[target.'%s'.%s]" % (e['tgt'], hdr)
            hm = re.search(r'(?m)^' + re.escape(header) + r'[ \t]*\n', t)
            if not hm:
                t = t.rstrip('\n') + '\n\n' + header + '\n'
                hm = re.search(r'(?m)^' + re.escape(header) + r'[ \t]*\n', t)
            nxt = re.search(r'(?m)^\[', t[hm.end():])
            end = hm.end() + nxt.start() if nxt else len(t)
            body_end = end
            while body_end > hm.end() and t[body_end - 1] == '\n' and t[body_end - 2:body_end] == '\n\n': body_end -= 1
            class _M:
                def __init__(self, st, en): self._s, self._e = st, en
                def start(self): return self._s
                def end(self): return self._e
                def group(self, n): return ''
            mm = _M(body_end, body_end)
            report.append(('READD', m[-70:], e['alias']))
        sec_start = t.rfind('\n[', 0, mm.start()) + 1
        sec_end = t.find('\n[', mm.end())
        section = t[sec_start: sec_end if sec_end >= 0 else len(t)]
        existing = set(re.findall(r'(?m)^"?([A-Za-z0-9_\-]+)"?\s*=', section))
        old_feats = e['spec'].get('features', [])
        lines = []
        for ident, (kind, obj) in sorted(intro.items()):
            if kind == 'internal':
                key = obj['pkg']; spath = obj['abs'] + '/' + PKG; tail = {}
                ck = obj['key'].split(':', 1)[1]
            elif kind == 'ext':
                key = obj; ext = reg.EXT[key]; sp = ext.get('dependencies') or ext.get('dev') or {}
                if 'abspath' not in sp: continue
                spath = sp['abspath']; tail = sp; ck = 'ext:' + ident
            else:
                key = obj; spath = os.path.dirname(R + '/' + reg.index()[key]); tail = {}; ck = 'ext:' + ident
            if key in existing: continue
            feats = []
            for f in old_feats:
                feats += FEATMAP.get(f, {}).get(ck, [])
            parts = ['path = "%s"' % os.path.relpath(spath, mdir)]
            if tail.get('package'): parts.append('package = "%s"' % tail['package'])
            if feats: parts.append('features = [%s]' % ', '.join('"%s"' % x for x in sorted(set(feats))))
            if e['spec'].get('optional'): parts.append('optional = true')
            lines.append('%s%s = { %s }\n' % (mm.group('ind'), key, ', '.join(parts)))
        t = t[:mm.start()] + ''.join(lines) + t[mm.end():]
    if write: wr(mp, t)
    report.append(('OK', m[-80:], sorted(intro), 'files', touched, 'of', len(files_of.get(m, []))))
for r_ in report: print(*r_)
orph = [f for f in consumer_files if f not in owner and os.path.exists(R + '/' + f) and re.search(r'\b(%s)\b' % '|'.join(aliases), rd(R + '/' + f))]
print('files naming the aliases but owned by no processed manifest:', len(orph)); open('lists/consumer-orphans.txt','w').write('\n'.join(orph))
