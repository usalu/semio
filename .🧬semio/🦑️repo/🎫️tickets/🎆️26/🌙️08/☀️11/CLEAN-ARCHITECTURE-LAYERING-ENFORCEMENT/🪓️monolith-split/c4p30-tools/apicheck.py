import os, re, sys, importlib
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rsitems import items, name_kind
R = '/Users/ueli/Documents/semio'
T = os.path.dirname(os.path.abspath(__file__))


def pubset(text):
    out = []
    for it in items(text):
        v, k, n = name_kind(it['toks'])
        if v == 'pub' and n and k not in ('use', 'mod', 'impl'):
            out.append((k, n))
    return sorted(out)


bad = 0
for tag in ['rewriting', 'jackeditor', 'langservice', 'execution', 'host', 'jackroot', 'pointerdown', 'draweditor']:
    sp = importlib.import_module('specs.' + tag).SPEC
    orig = open(os.path.join(T, 'orig', sp['orig']), encoding='utf-8').read()
    root = open(os.path.join(R, sp['file']), encoding='utf-8').read()
    base = os.path.dirname(os.path.join(R, sp['file']))
    new = list(pubset(root))
    for c in sp['children']:
        ct = open(os.path.join(base, c['folder'], '🦀️.rs'), encoding='utf-8').read()
        ps = pubset(ct)
        if c.get('unwrap'):
            print('  unwrapped inline module', c['mod'], 'keeps its declaration', 'pub' if c.get('modvis') else 'private', '; internal pub items', len(ps))
            continue
        new += ps
        mod = c['mod']
        if ps and c.get('reexport', True):
            if not re.search(r'^pub use %s::\*;' % mod, root, re.M):
                print('NO pub use for', tag, c['folder'])
                bad += 1
    o = pubset(orig)
    ok = sorted(o) == sorted(new)
    print(tag, 'pub items', len(o), len(new), 'EQUAL' if ok else 'DIFF')
    if not ok:
        bad += 1
        print('  only-orig', sorted(set(o) - set(new))[:10], 'only-new', sorted(set(new) - set(o))[:10])
print('APICHECK', 'OK' if not bad else 'FAIL')
