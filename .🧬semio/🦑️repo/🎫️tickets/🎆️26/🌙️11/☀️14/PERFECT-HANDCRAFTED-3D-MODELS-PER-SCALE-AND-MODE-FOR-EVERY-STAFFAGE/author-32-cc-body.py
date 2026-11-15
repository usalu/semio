"""One-off body repairs of city-car (author 32): glass panes sit proud of the cabin surface so that they are exposed shells; engine fill below the bonnet slab midplane."""
import sys, math, json; sys.path.insert(0, '.')
from author_32_lib import *

A = 'vehicles/city-car'
d = load(A)

def cabin_fn(sections):
    secs = sorted(sections, key=lambda s: s['at'][1])
    def at(y):
        for a, b in zip(secs, secs[1:]):
            if y <= b['at'][1] + 1e-9:
                t = 0 if b['at'][1] == a['at'][1] else (y - a['at'][1]) / (b['at'][1] - a['at'][1])
                t = min(max(t, 0), 1)
                l = lambda k: a[k] + (b[k] - a[k]) * t
                return a['at'][0] + (b['at'][0] - a['at'][0]) * t, l('rx'), l('ry'), a.get('exponent', 2)
        a = secs[-1]
        return a['at'][0], a['rx'], a['ry'], a.get('exponent', 2)
    def z(x, y):
        cx, rx, ry, e = at(y)
        u = abs(x - cx) / rx
        return 0.0 if u >= 1 else ry * (1 - u ** e) ** (1 / e)
    def xfront(y):
        cx, rx, ry, e = at(y)
        return cx + rx
    return z, xfront

PROTR = {'50': (0.004, 0.0045), '100': (0.008, 0.008), '200': (0.015, 0.011)}
for sc, (protr, ry) in PROTR.items():
    v = d['volumes'][sc]
    cab = next(s for s in v['body'] if s['id'] == 'cabin')
    z, xf = cabin_fn(cab['sections'])
    for s in v['body']:
        if s['id'].startswith('side-glass'):
            side = 1 if s['id'].endswith('-l') else -1
            k = (0.6 - 0.712) / 0.415
            for sec in s['sections']:
                x, yc = sec['at'][0], sec['at'][1]
                h = sec['rx'] * 0.415 / math.hypot(0.415, 0.112)
                ys = [yc - h + i * 2 * h / 8 for i in range(9)]
                z0 = max(z(x, y) - (-k) * 0 + k * 0 for y in [yc])  # placeholder
                m = max(z(x, y) + (-k) * (y - yc) * 0 for y in ys)
                # plane z(y) = z0 + k*(y-yc) (k<0: narrower upward); need z0 + k*(y-yc) >= cabin_z(x,y) - protr + ry ... outer face = plane + ry >= cabin + protr
                z0 = max(z(x, y) - k * (y - yc) for y in ys) + protr - ry
                sec['at'][2] = round(side * z0, 5)
                sec['ry'] = ry
        if s['id'] == 'windscreen':
            s['sections'][0]['at'] = [0.9, 1.012, 0]; s['sections'][-1]['at'] = [0.26, 1.432, 0]
            p0, p1 = s['sections'][0], s['sections'][-1]
            y0, y1 = p0['at'][1], p1['at'][1]
            pts = [(xf(y0 + (y1 - y0) * i / 12), y0 + (y1 - y0) * i / 12) for i in range(13)]
            ax, ay = pts[-1][0] - pts[0][0], pts[-1][1] - pts[0][1]
            n = math.hypot(ax, ay)
            nx, ny = ay / n, -ax / n
            if ny < 0: nx, ny = -nx, -ny
            off = max((px - pts[0][0]) * nx + (py - pts[0][1]) * ny for px, py in pts)
            for sec in s['sections']:
                px, py = xf(sec['at'][1]), sec['at'][1]
                sec['at'][0] = round(px + nx * (off + protr - ry), 5)
                sec['at'][1] = round(py + ny * (off + protr - ry), 5)
                sec['ry'] = ry
            s['_n'] = [nx, ny]
            del s['_n']
for s in d['volumes']['100']['body']:
    if s['id'] == 'hatch-glass':
        for q in s['sections']:
            q['at'][1] = round(-0.0169 - 0.012 if q['at'][0] > 0.4 else -0.00625 - 0.012, 5)

for s in d['volumes']['100']['body']:
    if s['id'] == 'roof-skin':
        s['sections'] = [{'at': [x, y, 0], 'rx': rx, 'ry': 0.02, 'exponent': 6, 'rotation': [0, -0.70711, 0, 0.70711]} for x, y, rx in ((0.22, 1.476, 0.5), (-0.2, 1.495, 0.52), (-0.9, 1.496, 0.52), (-1.46, 1.415, 0.46))]

def yc(x):
    pts = [(0.95, 0.96), (1.529, 0.8755), (1.88, 0.80), (1.9, 0.79)]
    for (x0, y0), (x1, y1) in zip(pts, pts[1:]):
        if x <= x1: return y0 + (y1 - y0) * (x - x0) / (x1 - x0)
    return pts[-1][1]
for sc in ('50', '100', '200', '500'):
    for s in d['volumes'][sc]['body']:
        if s['id'] == 'engine-block':
            for q in s['sections']:
                bot = q['at'][1] - q['ry'] if sc == '50' else None
                x = q['at'][0]
                top = yc(x) - 0.004
                if bot is None:
                    bot = q['at'][1] - q['ry']
                q['at'][1] = round((bot + top) / 2, 5); q['ry'] = round(max(0.004, (top - bot) / 2), 5)
for s in d['volumes']['200']['body']:
    if s['id'] == 'silencer':
        s['p0'][1] = 0.18; s['p1'][1] = 0.18

def lerp(pts, x):
    for (x0, y0), (x1, y1) in zip(pts, pts[1:]):
        if x <= x1 + 1e-9: return y0 + (y1 - y0) * (x - x0) / (x1 - x0 if x1 != x0 else 1)
    return pts[-1][1]
for sc in ('100', '200', '500'):
    v = d['volumes'][sc]
    bl = next(s for s in v['body'] if s['id'] == 'body-lower')
    tops = sorted((q['at'][0], q['at'][1] + q['ry']) for q in bl['sections'])
    for s in v['body']:
        if s['id'] == 'engine-block':
            for q in s['sections']:
                x = q['at'][0]
                top = yc(x) - 0.004
                bot = lerp(tops, x) - 0.06
                q['at'][1] = round((bot + top) / 2, 5); q['ry'] = round((top - bot) / 2, 5)
save(A, d)
print('ok')
