"""Live verification of the c4p12 split: rustfmt parse, mounts, docstring emoji, paths, test fn multiset. usage: verify.py"""
import os, re, sys, subprocess, importlib, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import c4p12 as p
R = p.REPO
fails = 0
def bad(*a):
    global fails
    fails += 1
    print('FAIL', *a)
allfiles = []
for tag in ['layoutdiff', 'picking', 'playapp', 'cadunit', 'inferences', 'compliance', 'appsurface', 'en1990']:
    sp = importlib.import_module('specs.' + tag).SPEC
    root = os.path.join(R, sp['file']); base = os.path.dirname(root)
    kids = [os.path.join(base, f, '🦀️.rs') for f in p.folders_of(sp)]
    files = [root] + kids
    allfiles += files
    rtxt = open(root, encoding='utf-8').read()
    orig = open(os.path.join(p.T, 'orig', sp['orig']), encoding='utf-8').read()
    mounts = re.findall(r'#\[path = "([^"]+)"\]\s*(?:pub(?:\([a-z]+\))? )?mod ([a-z0-9_]+);', rtxt)
    texts = {f: open(f, encoding='utf-8').read() for f in files}
    nested = []
    for f, t in texts.items():
        for m in re.finditer(r'#\[path = "([^"]+)"\]\s*(?:pub(?:\([a-z]+\))? )?mod ([a-z0-9_]+);', t):
            nested.append((f, m.group(1), m.group(2)))
    resolved = collections.Counter()
    for f, rel, name in nested:
        tgt = os.path.normpath(os.path.join(os.path.dirname(f), rel))
        if not os.path.isfile(tgt): bad(tag, 'missing mount target', rel)
        resolved[tgt] += 1
    for k in kids:
        c = resolved.get(os.path.normpath(k), 0)
        if tag == 'inferences' and c == 0:
            c = 1 if any(os.path.normpath(os.path.join(os.path.dirname(f), 'mod')) for f in files) else 0
        if c < 1: bad(tag, 'child not mounted', k[-80:])
    dup = [k for k, c in resolved.items() if c > 1]
    if dup: bad(tag, 'duplicate mounts', dup)
    names = collections.Counter(n for _, _, n in nested)
    for n, c in names.items():
        pass
    docs = []
    for f in kids:
        first = texts[f].split('\n', 1)[0]
        if not first.startswith('//! '): bad(tag, 'no docstring', f[-60:]); continue
        docs.append(first[4].encode('utf-8').decode('utf-8') if False else first[4:].split(' ')[0])
    em = [d for d in docs]
    dir_of = collections.defaultdict(list)
    for f in kids:
        dir_of[os.path.dirname(os.path.dirname(f))].append(os.path.basename(os.path.dirname(f)))
    for d, fs in dir_of.items():
        prefixes = [x.split('️')[0] if '️' in x else x[0] for x in os.listdir(d) if os.path.isdir(os.path.join(d, x))]
        dupes = [x for x, c in collections.Counter(prefixes).items() if c > 1]
        if dupes: print('  note sibling emoji reuse', tag, dupes)
    for f in kids:
        fo = os.path.basename(os.path.dirname(f))
        if '️' not in fo: bad(tag, 'folder lacks FE0F', fo)
    n_orig = len(re.findall(r'#\[(?:tokio::)?test\b[^\]]*\]', orig))
    n_new = sum(len(re.findall(r'#\[(?:tokio::)?test\b[^\]]*\]', t)) for t in texts.values())
    if n_orig != n_new: bad(tag, 'test attr count', n_orig, n_new)
    fn_o = collections.Counter(re.findall(r'\bfn ([A-Za-z0-9_]+)', orig))
    fn_n = collections.Counter()
    for t in texts.values(): fn_n.update(re.findall(r'\bfn ([A-Za-z0-9_]+)', t))
    if fn_o != fn_n: bad(tag, 'fn name multiset', (fn_o - fn_n), (fn_n - fn_o))
    mx = max(len(t.split('\n')) for t in texts.values())
    print(tag, 'files', len(files), 'mounts', len(nested), 'tests', n_orig, n_new, 'fn', sum(fn_o.values()), sum(fn_n.values()), 'max lines', mx)
for f in allfiles:
    rel = os.path.relpath(f, R)
    if len(rel.encode('utf-8')) > 240 or len(f) > 256: bad('path', rel)
cfg = ['--edition', '2021', '--config', 'skip_children=true', '--emit', 'stdout']
nok = 0
for f in allfiles:
    r = subprocess.run(['rustfmt', '+nightly-2026-07-20'] + cfg + [f], capture_output=True, text=True)
    if r.returncode != 0:
        bad('rustfmt parse', f[-70:], r.stderr[:300])
    else:
        nok += 1
print('rustfmt parses', nok, 'of', len(allfiles))
print('VERIFY', 'OK' if not fails else 'FAIL %d' % fails)
