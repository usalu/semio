"""One-off body repairs of city-bus 1:50 (author 32): wheelhouse plates, right-side windows, panes and door leaves thick enough for nested layer shells, door opening markers at the feature."""
import sys; sys.path.insert(0, '.')
from author_32_kit import *

def arch_edges(shell):
    """wheel arch walls are vertical in the drawings: a full-height section just outside each arch edge"""
    full = {'at': [0, 1.625, 0], 'rx': 1.275, 'ry': 1.325, 'exponent': 12, 'rotation': [0.70711, 0, 0.70711, 0]}
    xs = sorted(q['at'][0] for q in shell['sections'])
    secs = [q for q in shell['sections'] if not q.get('_edge')]
    for x in (2.83, 4.07, 8.73, 9.97):
        if any(abs(q['at'][0] - x) < 0.005 and q['at'][1] - q['ry'] < 0.31 for q in secs): continue
        q = copy.deepcopy(full); q['at'][0] = x
        secs.append(q)
    secs.sort(key=lambda q: q['at'][0])
    shell['sections'] = secs

A = 'vehicles/city-bus'
d = load(A)
v = d['volumes']['50']
B = {s['id']: s for s in v['body']}
OUT = 1.28325

def thick_pane(s, side):
    t = 0.06
    s['size'][2] = t
    s['transform']['translation'][2] = round(side * (OUT - t / 2), 5)

for s in v['body']:
    if s['id'].startswith(('side-window', 'upper-window')) and s['id'] != 'x':
        thick_pane(s, 1)
v['body'] = [s for s in v['body'] if not s['id'].startswith(('win-r-', 'wheelhouse-'))]
for s in v['body']:
    if s['id'].startswith(('side-window', 'upper-window')) and 'repeat' in s:
        if 'gap' in s['id']:
            del s['repeat']
        else:
            s['repeat'] = {'count': 2, 'translation': [0, 0, round(-2 * s['transform']['translation'][2], 5)]}
for k, s in enumerate(v['body']):
    if s['id'].startswith('leaf-') and s['kind'] == 'box' and s['id'].count('-') == 2:
        s['size'][2] = 0.09
        s['transform']['translation'][2] = -1.24
    if s['id'].startswith('leaf-') and s['id'].endswith('-glass'):
        s['size'][2] = 0.06
        s['transform']['translation'][2] = -1.262
    if s['id'].startswith('door-opening'):
        s['size'][2] = 0.01
for x0, x1, n in ((2.6, 4.5, 'r'), (8.0, 10.2, 'f')):
    for sg, sd in ((1, 'l'), (-1, 'r')):
        v['body'].append(box(f'wheelhouse-{n}-{sd}', 'body', 'paint', 'body', 0.35, (x1 - x0, 0.85, 0.03), ((x0 + x1) / 2, 0.725, sg * 0.8)))
ws = next(x for x in v['body'] if x['id'] == 'windscreen')
a0, a1 = [0, 0], [0, 0]
if 'orig' not in ws:
    pass
ws['sections'][0]['at'] = [11.99, 1.2, 0.625]; ws['sections'][-1]['at'] = [11.8, 2.68, 0.625]
import math
dx, dy = ws['sections'][-1]['at'][0] - ws['sections'][0]['at'][0], ws['sections'][-1]['at'][1] - ws['sections'][0]['at'][1]
n = math.hypot(dx, dy)
nx, ny = dy / n, -dx / n
if nx < 0: nx, ny = -nx, -ny
print('windscreen normal', nx, ny)
for q in ws['sections']:
    q['ry'] = 0.021
    q['at'][0] = round(q['at'][0] - nx * 0.015, 5); q['at'][1] = round(q['at'][1] - ny * 0.015, 5)
v['body'] = [x for x in v['body'] if x['id'] != 'windscreen-backing']
back = copy.deepcopy(ws)
back.update({'id': 'windscreen-backing', 'material': 'paint', 'role': 'body', 'tone': 0.4})
for q in back['sections']:
    q['ry'] = 0.06
    q['at'][0] = round(q['at'][0] - nx * (0.021 + 0.06 - 0.006 + 0.006), 5); q['at'][1] = round(q['at'][1] - ny * (0.021 + 0.06 - 0.006 + 0.006), 5)
v['body'].append(back)
for s_ in v['body']:
    if s_['id'].startswith('leaf-') and s_['id'].endswith('-glass'):
        s_['transform']['translation'][2] = -1.254
arch_edges(next(x for x in v['body'] if x['id'] == 'body-shell'))
save(A, d)
print(len(v['body']))
