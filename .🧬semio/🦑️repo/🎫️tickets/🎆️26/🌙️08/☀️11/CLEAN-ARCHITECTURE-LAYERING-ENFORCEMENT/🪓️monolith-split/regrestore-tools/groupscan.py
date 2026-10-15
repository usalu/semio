import sys, os, bisect
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from touched import *
tot = 0
for sid in (sys.argv[1:] or list(SPECS)):
    files = touched(sid)
    out = []
    for p in files:
        T = Tfile(os.path.join(ROOT, p))
        lang = 'rs' if p.endswith('.rs') else 'ts'
        grp, rel, ds, de = gapinfo(T.code, lang)
        if not rel:
            continue
        for n, kind, name in T.marks:
            k = bisect.bisect_left(T.fidx, n)
            if k < len(T.code) and grp[k] is not None:
                out.append((p[-60:], n + 1, kind, name[:30], T.code[k][:50]))
    tot += len(out)
    print(sid, 'touched', len(files), 'markers inside non-block groups', len(out))
    for o in out[:6]:
        print('   ', o)
print('total', tot)
