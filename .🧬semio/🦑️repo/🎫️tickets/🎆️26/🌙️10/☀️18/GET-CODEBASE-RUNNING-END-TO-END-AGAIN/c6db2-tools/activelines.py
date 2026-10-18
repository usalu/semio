"""📏 C6DB2 active-line and byte delta of a staged split against its snapshot. usage: activelines.py <tag>... (module splits; stage dir /Users/ueli/.cache/c6db2/stage/<tag>)"""
import os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
def act(t):
    return sum(1 for l in t.split('\n') if l.strip() and not l.strip().startswith('//'))
for tag in sys.argv[1:]:
    o = open(f'{HERE}/orig/{tag}.rs', encoding='utf-8').read()
    n = b = 0
    for dp, dn, fn in os.walk(f'/Users/ueli/.cache/c6db2/stage/{tag}'):
        for f in fn:
            if f.endswith('.rs'):
                t = open(os.path.join(dp, f), encoding='utf-8').read()
                n += act(t); b += len(t.encode())
    print(tag, 'active', act(o), '->', n, 'delta', n - act(o), 'bytes', len(o.encode()), '->', b, 'delta', b - len(o.encode()))
