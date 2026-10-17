"""🧭 C4P47 include resolution proof for the four module-style splits (gr, shs, cab, ph) on the LIVE tree: the multiset of absolute include_str!/include_bytes!/include! targets of the snapshot equals the one of root plus children, and every target exists. usage: incl-mod.py"""
import collections, importlib, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
R = '/Users/ueli/Documents/semio/'
HERE = os.path.dirname(os.path.abspath(__file__))
RX = re.compile(r'include(?:_str|_bytes)?!\(\s*"([^"]+)"')
ok = True
for t in ['gr', 'shs', 'cab', 'ph']:
    sp = importlib.import_module('specs.' + t).SPEC
    base = R + os.path.dirname(sp['file'])
    orig = collections.Counter(os.path.normpath(os.path.join(base, m)) for m in RX.findall(open(HERE + '/orig/' + sp['orig'], encoding='utf-8').read()))
    new = collections.Counter()
    for f, d in [(R + sp['file'], base)] + [(base + '/' + c['folder'] + '/🦀️.rs', base + '/' + c['folder']) for c in sp['children']]:
        for m in RX.findall(open(f, encoding='utf-8').read()):
            new[os.path.normpath(os.path.join(d, m))] += 1
    miss = [p for p in new if not os.path.exists(p)]
    print(t, 'literals orig', sum(orig.values()), 'new', sum(new.values()), 'same-targets', orig == new, 'missing', len(miss))
    ok = ok and orig == new and not miss
print('INCLUDES', 'OK' if ok else 'FAILED')
