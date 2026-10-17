import importlib, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import c4p48 as P
from rsitems import items, name_kind

def pubset(text, only=None):
    out = set()
    for it in items(text):
        v, k, n = name_kind(it['toks'])
        if k in ('use', 'impl', 'mod') and not (k == 'use' and v == 'pub'):
            if k == 'mod' and v == 'pub' and n:
                out.add(('mod', n))
            continue
        if v == 'pub' and n and k not in ('use',):
            out.add((k, n))
    return out

def pub_methods(text):
    out = set()
    for m in re.finditer(r'^\s+pub\s+(?:async\s+|const\s+|unsafe\s+)*fn\s+(\w+)', text, re.M):
        out.add(m.group(1))
    return out

bad = 0
for t in sys.argv[1:]:
    sp = importlib.import_module('specs.' + t).SPEC
    orig = open(os.path.join(P.T, 'orig', sp['orig']), encoding='utf-8').read()
    base = os.path.join(P.REPO, os.path.dirname(sp['file']))
    parent = open(os.path.join(P.REPO, sp['file']), encoding='utf-8').read()
    names = pubset(parent)
    methods = pub_methods(parent)
    pubmods = set(re.findall(r'^pub use (\w+)::\*;', parent, re.M))
    for c in sp['children']:
        text = open(os.path.join(base, c['folder'], '🦀️.rs'), encoding='utf-8').read()
        methods |= pub_methods(text)
        if c['mod'] in pubmods:
            names |= pubset(text)
        else:
            lost = pubset(text)
            if lost:
                print(t, 'child', c['folder'], 'has pub items but is not pub-reexported:', sorted(lost)[:5]); bad += 1
    o = pubset(orig)
    om = pub_methods(orig)
    print(t, 'pub items orig', len(o), 'new', len(names), 'missing', sorted(o - names)[:8], 'extra', sorted(names - o)[:8], '| pub methods orig', len(om), 'new', len(methods), 'diff', sorted(om ^ methods)[:6])
    if o != names or om != methods: bad += 1
print('PUBAPI', 'OK' if not bad else 'DIFF')
