import sys; sys.path.insert(0, '.')
from lib import *
for a in sys.argv[1:]:
    d = load(a)['asset']; W = rest(d)
    print('=====', a, d['dimensions'])
    print('REST', {k: (round(ap(W[k],0,0)[0],3), round(ap(W[k],0,0)[1],3), round(math.degrees(math.atan2(W[k][1],W[k][0])),1)) for k in W})
    for sc, r in d['representations'].items():
        if r['status'] != 'authored': continue
        print('--', sc)
        for s in r['geometry']['elevation']:
            M = W[s['bone']]
            if s['kind'] == 'poly': pts = [ap(M, *p) for p in s['points']]
            else: pts = [ap(M, 0, 0)]
            xs = [p[0] for p in pts]; ys = [p[1] for p in pts]
            print(f"{s['id']:22s} {s['bone']:14s} z{s.get('z')} {'F' if s.get('filled') else 'o'} {s['kind'][:2]} x[{min(xs):.3f},{max(xs):.3f}] y[{min(ys):.3f},{max(ys):.3f}] {s.get('region') or s.get('regions')}")
