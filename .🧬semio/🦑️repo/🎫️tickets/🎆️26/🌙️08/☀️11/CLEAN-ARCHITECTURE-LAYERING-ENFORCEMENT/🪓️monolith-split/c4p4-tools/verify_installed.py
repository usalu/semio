import os, re, sys, importlib, subprocess, unicodedata
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import pfa
REPO = pfa.REPO
ok = True
TAGS = ['diff', 'schema', 'arena', 'digest', 'owned', 'rma', 'editor_unit', 'raster_editor']
for tag in TAGS:
    sp = importlib.import_module('specs.' + tag).SPEC
    base = os.path.join(REPO, os.path.dirname(sp['file']))
    root = os.path.join(REPO, sp['file'])
    par = open(root, encoding='utf-8').read()
    mounts = [m for m in re.findall(r'#\[path\s*=\s*"([^"]+)"\]', par) if m != '.']
    miss = [m for m in mounts if not os.path.isfile(os.path.normpath(os.path.join(base, m)))]
    declared = [c['folder'] + '/🦀️.rs' for c in sp['children']]
    unwired = [d for d in declared if d not in mounts]
    fmt = []
    files = [root] + [os.path.join(base, c['folder'], '🦀️.rs') for c in sp['children']]
    for f in files:
        r = subprocess.run(['rustfmt', '+nightly-2026-07-20', '--edition', '2021', '--config', 'skip_children=true', '--emit', 'stdout', f], capture_output=True, text=True)
        fmt.append(r.returncode)
    sib = os.listdir(base)
    first = {}
    for e in sib:
        g = re.match(r'^(.)', e).group(1)
        first.setdefault(g, []).append(e)
    mine = [c['folder'] for c in sp['children']]
    dup = {g: v for g, v in first.items() if len(v) > 1 and any(x in mine for x in v)}
    kebab = [f for f in mine if not re.match(r'^.️[a-z0-9]+(-[a-z0-9]+)*$', f)]
    docs = [c['folder'] for c in sp['children'] if not open(os.path.join(base, c['folder'], '🦀️.rs'), encoding='utf-8').read().startswith('//! ' + c['folder'][0])]
    longest = max(len((os.path.join(base, c['folder'], '🦀️.rs')).encode('utf-8')) for c in sp['children'])
    n = [open(f, encoding='utf-8').read().count('\n') for f in files]
    print(tag, 'mounts', len(mounts), 'missing', miss, 'unwired', unwired, 'rustfmt', sorted(set(fmt)), 'dup-emoji-siblings', dup, 'bad-names', kebab, 'doc-emoji-mismatch', docs, 'longest-abs', longest, 'lines parent/max-child', n[0], max(n[1:]))
    ok = ok and not miss and not unwired and not dup and not kebab and not docs and all(c == 0 for c in fmt) and longest <= 240
print('VERIFY', 'OK' if ok else 'FAIL')
