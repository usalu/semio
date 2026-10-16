import os, re, sys, importlib, subprocess, unicodedata
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import pfa
REPO = pfa.REPO
ok = True
def first_grapheme(name):
    m = re.match(r'^(.)(️?)', name)
    return m.group(1)
for tag in ['bim', 'process', 'raster', 'vdi', 'en1999', 'en1993']:
    sp = importlib.import_module('specs.' + tag).SPEC
    base = os.path.join(REPO, os.path.dirname(sp['file']))
    par = open(os.path.join(base, '🦀️.rs'), encoding='utf-8').read()
    mounts = re.findall(r'#\[path = "([^"]+)"\]', par)
    miss = [m for m in mounts if not os.path.isfile(os.path.join(base, m))]
    declared = [c['folder'] + '/🦀️.rs' for c in sp['children']]
    unwired = [d for d in declared if d not in mounts]
    fmt = []
    files = [os.path.join(base, '🦀️.rs')] + [os.path.join(base, c['folder'], '🦀️.rs') for c in sp['children']]
    for f in files:
        r = subprocess.run(['rustfmt', '--edition', '2021', '--emit', 'stdout', '--config-path', os.path.join(REPO, 'rustfmt.toml')], input=open(f, encoding='utf-8').read(), capture_output=True, text=True)
        fmt.append(r.returncode)
    sib = [e for e in os.listdir(base)]
    gs = {}
    for e in sib:
        g = e[0]
        gs.setdefault(g, []).append(e)
    dup = {g: v for g, v in gs.items() if len(v) > 1 and any(x in [c['folder'] for c in sp['children']] for x in v)}
    kebab = [c['folder'] for c in sp['children'] if not re.match(r'^.️[a-z0-9]+(-[a-z0-9]+)*$', c['folder'])]
    print(tag, 'mounts', len(mounts), 'missing', miss, 'unwired', unwired, 'rustfmt rc', sorted(set(fmt)), 'dup-emoji-siblings', dup, 'bad-names', kebab)
    ok = ok and not miss and not unwired and not dup and not kebab and all(c in (0,) for c in fmt)
print('VERIFY', 'OK' if ok else 'FAIL')
