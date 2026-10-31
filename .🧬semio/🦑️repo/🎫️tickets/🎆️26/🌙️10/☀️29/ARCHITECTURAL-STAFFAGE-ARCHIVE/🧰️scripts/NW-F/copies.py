import json, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import outline, S
def sig(s):
    rings = outline(s)
    r = '|'.join(';'.join(f'{round(x/1e-4)},{round(y/1e-4)}' for x, y in pts) for pts, c in rings)
    return f"{s['bone']}/{s['role']}/{'c' if rings[0][1] else 'o'}/{r}"
def check(a):
    d = json.load(open(f'{S}/🗂️catalog/🛋️furniture/{a}/🔣️.json', encoding='utf-8'))['asset']
    out = []
    for hi, lo in (('500', '200'), ('200', '100'), ('100', '50'), ('500', '100'), ('500', '50'), ('200', '50')):
        A = [sig(s) for s in d['representations'][hi]['geometry']['elevation']]
        B = [sig(s) for s in d['representations'][lo]['geometry']['elevation']]
        pool = {}
        for x in B: pool[x] = pool.get(x, 0) + 1
        shared = 0
        for x in A:
            if pool.get(x, 0) > 0: shared += 1; pool[x] -= 1
        out.append((hi, lo, shared, len(A), len(B)))
    return out
for a in sys.argv[1:]:
    print(a, [f'{h}->{l}: {s}/{na} of {nb}' + (' COPY' if s == na else '') for h, l, s, na, nb in check(a)])
