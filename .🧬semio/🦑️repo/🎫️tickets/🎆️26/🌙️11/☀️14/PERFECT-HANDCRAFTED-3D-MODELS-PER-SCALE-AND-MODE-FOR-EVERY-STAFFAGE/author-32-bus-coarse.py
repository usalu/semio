"""One-off: city-bus bodies for 1:100, 1:200, 1:500: shell, roof panel, battery, hvac, windscreen are the kit refits to each scale's drawing; everything else is authored here."""
import sys; sys.path.insert(0, '.')
from author_32_kit import *
A = 'vehicles/city-bus'
d = load(A)
sys.path.insert(0, '.')
b50 = {s['id']: s for s in d['volumes']['50']['body']}
RQ = [0.70711, 0, 0.70711, 0]

def keep(sc, ids):
    return [copy.deepcopy(s) for s in d['volumes'][sc]['body'] if s['id'] in ids]

def pane(id, x0, x1, y0, y1, side, t, out=1.28325, region=None):
    return box(id, 'body', 'glass', 'glazing', 0.8, (x1 - x0, y1 - y0, t), ((x0 + x1) / 2, (y0 + y1) / 2, side * (out - t / 2)))

TYRE = [(-0.15, 0.3792), (-0.132, 0.45648), (-0.09, 0.48), (0.09, 0.48), (0.132, 0.45648), (0.15, 0.3792)]
def wheels(lvl):
    out = []
    for sg, n in ((1, 'l'), (-1, 'r')):
        for bone, p in (('wheel-f', 'f'), ('wheel-r', 'r')):
            o = (0, 0, sg * 1.05)
            if lvl == 500:
                out.append(rev(f'{p}{n}-wheel', bone, 'rubber', 'body', 0.1, [(-0.15, 0.0), (-0.15, 0.48), (0.15, 0.48), (0.15, 0.0)], o)); continue
            out.append(rev(f'{p}{n}-tyre', bone, 'rubber', 'body', 0.1, TYRE if lvl == 100 else [(-0.15, 0.38), (-0.11, 0.48), (0.11, 0.48), (0.15, 0.38)], o))
            out.append(rev(f'{p}{n}-rim', bone, 'steel-wheel', 'detail', 0.7, [(-0.14, 0.286), (0.14, 0.286)], o))
            if lvl == 100:
                out.append(rev(f'{p}{n}-hub', bone, 'steel-wheel', 'detail', 0.55, [(0, 0.11), (0.17, 0.11)], o))
    return out

for sc in ('100', '200', '500'):
    lvl = int(sc)
    L = keep(sc, ('body-shell', 'roof-panel', 'battery-pack', 'hvac-unit') + (('windscreen',) if lvl < 500 else ()))
    t = {100: 0.04, 200: 0.05, 500: 0.075}[lvl]
    if lvl == 100:
        L += [pane('glass-lower-l-a', 0.3, 6.25, 1.2, 2.4, 1, t), pane('glass-lower-l-b', 6.35, 11.8, 1.2, 2.4, 1, t), pane('glass-lower-r-a', 0.3, 6.25, 1.2, 2.4, -1, t), pane('glass-lower-r-b', 7.55, 10.25, 1.2, 2.4, -1, t),
              pane('glass-upper-l', 0.3, 11.0, 2.48, 2.78, 1, t), pane('glass-upper-r', 0.3, 11.0, 2.48, 2.78, -1, t)]
    elif lvl == 200:
        L += [pane('glass-lower-l', 0.3, 11.8, 1.2, 2.4, 1, t), pane('glass-lower-r', 0.3, 10.25, 1.2, 2.4, -1, t)]
    else:
        L += [pane('glass-band-l', 0.3, 11.8, 1.2, 2.78, 1, t), pane('glass-band-r', 0.3, 10.25, 1.2, 2.78, -1, t)]
    lt = {100: 0.09, 200: 0.09, 500: 0.09}[lvl]
    for n in ('m-aft', 'm-fwd', 'f-aft', 'f-fwd'):
        s = copy.deepcopy(b50['leaf-' + n]); s['size'][2] = 0.14; s['transform']['translation'][2] = -1.23; L.append(s)
    L.append(copy.deepcopy(b50['e-axle-casting']))
    L.append(copy.deepcopy(b50['axle-beam-r'])); L.append(copy.deepcopy(b50['axle-beam-f']))
    if lvl == 200:
        L.append(copy.deepcopy(b50['bumper-front'])); L.append(copy.deepcopy(b50['bumper-rear']))
    if lvl == 100:
        L += [copy.deepcopy(b50['mirror-l']), copy.deepcopy(b50['mirror-r'])]
    L += wheels(lvl)
    for s_ in L:
        if s_['id'].startswith('bumper'): s_.pop('cornerRadius', None)
    if lvl == 500:
        for s in L:
            if s['id'] == 'axle-beam-r' or s['id'] == 'axle-beam-f': s['r0'] = s['r1'] = 0.04
    if lvl == 500:
        for s_ in L:
            if s_['id'] == 'roof-panel':
                s_['sections'] = [{'at': [x, y, 0], 'rx': 1.2, 'ry': 0.045, 'exponent': 10, 'rotation': [0.70711, 0, 0.70711, 0]} for x, y in ((0.3, 2.935), (11.0, 2.935), (11.6, 2.895))]
        for s_ in L:
            if s_['id'] in ('battery-pack', 'hvac-unit'):
                prof = ((1.0, 3.0), (1.10, 3.14), (7.20, 3.14), (7.30, 3.0)) if s_['id'] == 'battery-pack' else ((8.0, 3.0), (8.12, 3.17), (10.28, 3.17), (10.5, 3.0))
                rx = s_['sections'][0]['rx']
                x0 = s_['sections'][0]['at'][0]
                bot = 2.985
                s_['sections'] = [{'at': [round(x - (1.0 if s_['id'] == 'battery-pack' else 8.0), 5), round((bot + top) / 2 - 2.98, 5), 0], 'rx': rx, 'ry': round((top - bot) / 2, 5), 'exponent': 6, 'rotation': [0.70711, 0, 0.70711, 0]} for x, top in prof]
    if lvl == 100:
        for s_ in L:
            if s_['id'] == 'windscreen':
                s_['sections'][0]['at'] = [11.99, 1.2, 0.625]; s_['sections'][1]['at'] = [11.8, 2.68, 0.625]
                for q_ in s_['sections']:
                    q_['ry'] = 0.021; q_['at'][0] = round(q_['at'][0] - 0.99186 * 0.021, 5); q_['at'][1] = round(q_['at'][1] - 0.12733 * 0.021, 5)
    shell = next(x for x in L if x['id'] == 'body-shell')
    full = {'at': [0, 1.625, 0], 'rx': 1.275, 'ry': 1.325, 'exponent': 12, 'rotation': [0.70711, 0, 0.70711, 0]}
    for x in (2.83, 4.07, 8.73, 9.97):
        if any(abs(q['at'][0] - x) < 0.005 for q in shell['sections']): continue
        q = copy.deepcopy(full); q['at'][0] = x
        shell['sections'].append(q)
    for q in shell['sections']:
        q['exponent'] = 12
        top = q['at'][1] + q['ry']
        if 0.4 < q['at'][0] < 11.6 and top > 2.96:
            bot = q['at'][1] - q['ry']
            q['at'][1] = round((bot + 2.95) / 2, 5); q['ry'] = round((2.95 - bot) / 2, 5)
    if lvl == 500:
        shell['sections'] = [q for q in shell['sections'] if not (2.835 < q['at'][0] < 4.065 or 8.735 < q['at'][0] < 9.965)]
    shell['sections'].sort(key=lambda q: q['at'][0])
    d['volumes'][sc]['body'] = L
save(A, d)
for sc in (100, 200, 500): print(sc, len(d['volumes'][str(sc)]['body']))
