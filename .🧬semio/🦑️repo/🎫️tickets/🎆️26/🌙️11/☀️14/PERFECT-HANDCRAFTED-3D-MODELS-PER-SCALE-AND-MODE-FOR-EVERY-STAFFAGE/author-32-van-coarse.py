"""One-off: delivery-van bodies for 1:100, 1:200 and 1:500 from the kit-coarsened shells (body-shell, roof-cap, hood-panel, windscreen, door windows are the kit's refits to each scale's own drawings; every other solid is authored here)."""
import sys; sys.path.insert(0, '.')
from author_32_kit import *
A = 'vehicles/delivery-van'
d = load(A)
b50 = {s['id']: s for s in d['volumes']['50']['body']}

def keep(sc, ids):
    v = d['volumes'][sc]
    return [copy.deepcopy(s) for s in v['body'] if s['id'] in ids]

TYRE = [(-0.1, 0.29192), (-0.088, 0.34105), (-0.06, 0.356), (0.06, 0.356), (0.088, 0.34105), (0.1, 0.29192)]
def wheels(level):
    out = []
    for sg, n in ((1, 'l'), (-1, 'r')):
        for bone, p in (('wheel-f', 'f'), ('wheel-r', 'r')):
            o = (0, 0, sg * 0.84)
            if level == 500:
                out.append(rev(f'{p}{n}-wheel', bone, 'rubber', 'body', 0.1, [(-0.1, 0.0), (-0.1, 0.356), (0.1, 0.356), (0.1, 0.0)], o))
                continue
            out.append(rev(f'{p}{n}-tyre', bone, 'rubber', 'body', 0.1, TYRE if level == 100 else [(-0.1, 0.29), (-0.07, 0.356), (0.07, 0.356), (0.1, 0.29)], o))
            out.append(rev(f'{p}{n}-rim', bone, 'steel-rim', 'detail', 0.65, [(-0.09, 0.2), (0.09, 0.2)], o))
            if level == 100:
                out.append(rev(f'{p}{n}-hub', bone, 'steel-rim', 'detail', 0.7, [(-0.1, 0.055), (0.1, 0.055)], o))
    return out

def common(level):
    L = []
    for ax, x in (('axle-f', 1.7), ('axle-r', -1.6)):
        r = {100: 0.05, 200: 0.055, 500: 0.04}[level]
        L.append(cap(ax + '-beam', ax, 'cast-iron', 'detail', 0.35, (0, 0, -0.84), (0, 0, 0.84), r))
    L.append(copy.deepcopy(b50['slider-panel']))
    L.append(copy.deepcopy(b50['rear-leaf-r'])); L.append(copy.deepcopy(b50['rear-leaf-l']))
    return L

for sc in ('100', '200', '500'):
    lvl = int(sc)
    L = keep(sc, ('body-shell', 'roof-cap', 'hood-panel', 'windscreen'))
    if lvl == 100:
        L = [x for x in L if x['id'] != 'hood-panel']
        L.append(loft('hood-panel', 'chassis', 'paint', 'body', 0.55, [sec((x, y + 0.01, 0), rx, 0.02, 8, [0.70711, 0, 0.70711, 0]) for x, y, rx in ((1.88, 1.0625, 0.82), (2.2, 1.02, 0.82), (2.62, 0.8775, 0.76))]))
    if lvl <= 200: L += keep(sc, ('door-window-l', 'door-window-r'))
    L += common(lvl)
    L += wheels(lvl)
    if lvl == 100:
        L += [copy.deepcopy(b50[i]) for i in ('mirror-l', 'mirror-r', 'grille', 'tail-pipe', 'slider-track-low', 'slider-track-high')]
        for s in L:
            if s['id'] in ('slider-track-low', 'slider-track-high'): s['tone'] = 0.4
    if lvl == 200:
        L += [copy.deepcopy(b50[i]) for i in ('slider-track-high', 'mirror-l', 'mirror-r')]
    if lvl == 500:
        for sg, n in ((1, 'l'), (-1, 'r')):
            L.append(ell(f'mirror-{n}', 'chassis', 'plastic-trim', 'detail', 0.35, (0.06, 0.05, 0.045), (1.6, 1.15, sg * 0.95)))
        for s in L:
            if s['id'] in ('slider-panel', 'rear-leaf-r', 'rear-leaf-l'):
                s['size'][2] = 0.08
                s.pop('cornerRadius', None)
    d['volumes'][sc]['body'] = L
save(A, d)
for sc in (100, 200, 500):
    print(sc, len(d['volumes'][str(sc)]['body']))
