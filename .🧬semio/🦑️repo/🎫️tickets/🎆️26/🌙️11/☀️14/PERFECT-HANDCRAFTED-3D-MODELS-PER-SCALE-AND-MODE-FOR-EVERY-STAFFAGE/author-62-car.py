"""Author 62: city-car kernel traps (tyres, brake discs, steering wheel as loops), dished rims, steering wheel facing the driver. Usage: python3 author-62-car.py 50 [100 200 500] (idempotent)"""
import sys; sys.path.insert(0, '.')
from author_62_lib import *

A = 'vehicles/city-car'


def rim(s, side, r=0.19, h=0.075, recess=0.045, hub=0.045, t=0.007):
    a_d = h - recess
    pts = [(h, r), (-h, r), (-h, r - t), (a_d - 0.01, r - t), (a_d - 0.01, hub), (a_d, hub), (a_d, r - t), (h, r - t), (h, r)]
    return {**s, 'profile': [[rr(x if side > 0 else -x), rr(y)] for x, y in pts]}


def main(sc):
    d = load(A)
    v = d['volumes'][sc]
    B = v['body']
    for i in ('fl', 'rl', 'fr', 'rr'):
        side = 1 if i[1] == 'l' else -1
        if sc != '500':
            s = solid_by_id(B, f'{i}-tyre')
            replace(B, f'{i}-tyre', {**s, 'profile': tyre_loop(0.185, 0.3, 0, 0.185)})
        if sc == '50':
            replace(B, f'{i}-rim', rim(solid_by_id(B, f'{i}-rim'), side))
    if sc == '50':
        sw = solid_by_id(B, 'steering-wheel')
        sw['profile'] = [[-0.015, 0.16], [-0.015, 0.18], [0.015, 0.18], [0.015, 0.16], [-0.015, 0.16]]
        sw['axis'] = [0.906, 0.423, 0]
        for t in v['thermal']:
            if t['id'] == 'th-steering-rim':
                t['profile'] = [[-0.015, 0.16], [-0.015, 0.18], [0.015, 0.18], [0.015, 0.16], [-0.015, 0.16]]
                t['axis'] = [0.906, 0.423, 0]
    for t in v['thermal']:
        if t.get('kind') == 'revolve' and t['id'].startswith('th-brake-'):
            p = t['profile']
            if len(p) == 4:
                t['profile'] = p + [p[0]]
    sf = {'th-roof-paint': ['-v'], 'th-roof': ['-v'], 'th-roof-panel': ['-v'], 'th-bonnet-paint': ['-v'], 'th-bonnet': ['-v'], 'th-tailgate-paint': ['+v', '+u', '-u', '+w', '-w'], 'th-tailgate': ['+v', '+u', '-u', '+w', '-w'],
          'th-windscreen': ['-u', '-v'], 'th-hatch-glass': ['+v'], 'th-side-glass-l': ['-w'], 'th-side-glass-r': ['+w'], 'th-side-glass-front-l': ['-w'], 'th-side-glass-rear-l': ['-w'],
          'th-side-glass-front-r': ['+w'], 'th-side-glass-rear-r': ['+w']}
    for t in v['thermal']:
        if t['id'] in sf:
            t['sectionFaces'] = sf[t['id']]
    save(A, d)
    print(sc, 'bodies', len(B))


for sc in sys.argv[1:]:
    main(sc)
