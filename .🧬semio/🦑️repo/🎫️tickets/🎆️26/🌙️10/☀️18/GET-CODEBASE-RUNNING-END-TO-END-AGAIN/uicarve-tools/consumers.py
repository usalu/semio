"""🔁 uicarve consumers: re-point dependents of semio-framework-ui to the carved crates (sources + manifests).
usage: consumers.py <alias,...> <spec.json,...> [--write]"""
import json, os, re, sys, collections
sys.path.insert(0, 'lib')
import carve, rs, reg
from carve import R, UI, FW, PKG, rd, wr
aliases = sys.argv[1].split(',')
specs = [carve.Spec(p) for p in sys.argv[2].split(',')]
write = '--write' in sys.argv
# merged view: rules + crates
merged = carve.Spec(sys.argv[2].split(',')[0])
for sp in specs[1:]:
    merged.crates.update(sp.crates)
    merged.rules = carve.Rules([r for _, r in merged.rules.rules] + [r for _, r in sp.rules.rules])
ident_to_crate = {c['ident']: c for c in merged.crates.values()}
deps = json.load(open('/Users/ueli/.cache/uicarve/deps.json'))
deps = [x for x in deps if not x['manifest'].startswith('.🧬semio') and x['kind'] != 'ws-dependencies' and x['alias'] in aliases]
consumer_files = json.load(open('/Users/ueli/.cache/uicarve/consumer-files.json'))
FEATMAP = json.load(open('specs/featmap.json', encoding='utf8')) if os.path.exists('specs/featmap.json') else {}

def manifest_roots(mp):
    """absolute entry files of a manifest: lib, bins, tests, examples, benches (explicit paths or cargo defaults)."""
    t = rd(mp); d = os.path.dirname(mp)
    roots = []
    secs = re.split(r'(?m)^(?=\[)', t)
    has_lib = False
    for sec in secs:
        h = sec.split('\n', 1)[0].strip()
        if h in ('[lib]', '[[bin]]', '[[test]]', '[[example]]', '[[bench]]'):
            m = re.search(r'(?m)^path\s*=\s*"([^"]+)"', sec)
            if m: roots.append(os.path.normpath(os.path.join(d, m.group(1))))
            if h == '[lib]': has_lib = True
    if not has_lib and os.path.exists(d + '/src/lib.rs'): roots.append(d + '/src/lib.rs')
    for sub in ('src/main.rs',):
        if os.path.exists(d + '/' + sub): roots.append(d + '/' + sub)
    for sub in ('src/bin', 'tests', 'examples', 'benches'):
        if os.path.isdir(d + '/' + sub):
            for f in sorted(os.listdir(d + '/' + sub)):
                if f.endswith('.rs'): roots.append(d + '/' + sub + '/' + f)
    return roots

_owner = {}
def build_owner():
    for m in by_manifest:
        mp = R + '/' + m
        seen = []
        for r in manifest_roots(mp): carve.reachable(r, seen)
        for f in seen:
            if f.endswith('.rs'): _owner.setdefault(os.path.relpath(f, R), []).append(m)

by_manifest = collections.defaultdict(list)
for x in deps: by_manifest[x['manifest']].append(x)
files_of = collections.defaultdict(list)
build_owner()
for f in consumer_files:
    for m in _owner.get(f, []):
        if m in by_manifest: files_of[m].append(f)
orphans = [f for f in consumer_files if f not in _owner]
report = []
done_files = {}
written = set()
def file_text(f, als_all):
    if f in done_files: return done_files[f]
    old = rd(R + '/' + f)
    new = carve.rewrite_text(old, merged, None, None, only_roots=als_all)
    done_files[f] = (old, new)
    return done_files[f]
file_aliases = collections.defaultdict(set)
for m, es in by_manifest.items():
    for f in files_of.get(m, []):
        for e in es: file_aliases[f].add(e['alias'])
for m, entries in sorted(by_manifest.items()):
    als = {e['alias'] for e in entries}
    used = collections.Counter(); touched = 0
    for f in files_of.get(m, []):
        old, new = file_text(f, file_aliases[f])
        residual = [mm.group(0) for mm in re.finditer(r'\b(%s)\b\s*::\s*\w+' % '|'.join(map(re.escape, als)), rs.mask(new))]
        if residual: report.append(('RESIDUAL', f[-70:], residual[:3]))
        if new != old:
            touched += 1
            if write and f not in written: wr(R + '/' + f, new); written.add(f)
        for ident in re.findall(r'\b(semio_framework_[a-z0-9_]+)\s*::', rs.mask(new)):
            if ident in ident_to_crate or ident in EXTERNAL_OK if False else ident in ident_to_crate: used[ident] += 1
    # manifest edit
    mp = R + '/' + m
    t = rd(mp)
    for e in entries:
        a = re.escape(e['alias'])
        mm = re.search(r'(?m)^(?P<ind>[ \t]*)"?%s"?\s*=\s*\{[^\n]*\}[ \t]*\n' % a, t)
        if not mm: report.append(('NOLINE', m[-70:], e['alias'])); continue
        line = mm.group(0)
        old_feats = e['spec'].get('features', [])
        mdir = os.path.dirname(mp)
        new_lines = []
        for ident, n in sorted(used.items()):
            c = ident_to_crate[ident]
            feats = []
            for f in old_feats:
                for ck, fl in FEATMAP.get(f, {}).items():
                    if ck == c['key']: feats += fl
            parts = ['path = "%s"' % os.path.relpath(c['abs'] + '/' + PKG, mdir)]
            if feats: parts.append('features = [%s]' % ', '.join('"%s"' % x for x in feats))
            if e['spec'].get('optional'): parts.append('optional = true')
            new_lines.append('%s%s = { %s }\n' % (mm.group('ind'), c['pkg'], ', '.join(parts)))
        new_lines = [l for l in new_lines if l.split(' ')[0].strip() not in {x.strip().split(' ')[0] for x in t.split('\n') if x.strip().startswith('semio-framework-ui-')} or True]
        t = t[:mm.start()] + ''.join(new_lines) + t[mm.end():]
    if write: wr(mp, t)
    report.append(('OK', m[-80:], dict(used), 'files', touched, 'of', len(files_of.get(m, []))))
for r in report: print(*r)
print('consumer files without a dependent manifest among the processed aliases:', len([f for f in orphans if re.search(r'\\b(%s)\\b' % '|'.join(aliases), rd(R + '/' + f))]))
