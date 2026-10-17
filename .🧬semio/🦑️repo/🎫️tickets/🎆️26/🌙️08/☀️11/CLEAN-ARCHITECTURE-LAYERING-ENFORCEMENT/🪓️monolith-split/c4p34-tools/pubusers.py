import importlib, json, os, re, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import c4p34 as P
from rsitems import items, name_kind
REPO = P.REPO
roots = ['✏️s/🔌️plugins', '🧰️framework']
ls = []
for root in roots:
    ls += subprocess.run(['git', '-c', 'core.fsmonitor=false', '-c', 'core.quotepath=false', 'ls-files', '-z', '--', root], cwd=REPO, capture_output=True).stdout.decode('utf-8').split('\0')
rs = [f for f in ls if f.endswith('.rs')]
idsets = {}
TOK = re.compile(r'[A-Za-z_][A-Za-z0-9_]*')
for f in rs:
    try:
        idsets[f] = set(TOK.findall(open(os.path.join(REPO, f), encoding='utf-8').read()))
    except Exception:
        pass
print('scanned', len(idsets), 'rust files (plugins and framework)', flush=True)
tags = sys.argv[1:] or 'png jpg avi pptx dxfbin dxfschema zlib'.split()
cache = {}
for tag in tags:
    sp = importlib.import_module('specs.' + tag).SPEC
    orig = open(os.path.join(P.T, 'orig', sp['orig']), encoding='utf-8').read()
    names = {}
    for it in items(orig):
        v, k, n = name_kind(it['toks'])
        if v in ('pub', 'pub(crate)') and k in ('fn', 'struct', 'enum', 'const', 'static', 'type', 'trait') and n:
            names[n] = v
    me = sp['file']
    print('==', tag, 'pub items', len(names))
    cnt = {}
    for n, v in sorted(names.items()):
        users = [f for f, s in idsets.items() if f != me and n in s]
        cnt[n] = len(users)
        print('  %-10s %-44s %3d files' % (v, n, len(users)), ('| ' + ' ; '.join(u[-90:] for u in users[:2])) if users else '')
    cache[tag] = cnt
json.dump(cache, open(os.path.join(P.T, 'pubusers.json'), 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
