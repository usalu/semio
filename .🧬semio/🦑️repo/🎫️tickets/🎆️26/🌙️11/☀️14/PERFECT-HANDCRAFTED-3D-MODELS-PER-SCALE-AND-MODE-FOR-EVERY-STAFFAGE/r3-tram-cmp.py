"""🧭️ Round 3 tram: points of the projected body outline of one bone (r3-project2d output) that lie outside the drawn filled shapes of that bone by more than a limit, binned (`python3 r3-tram-cmp.py 50 elevation body-end 0.012`); `--reverse` lists drawn boundary points outside the projection."""
import json, math, re, sys
from collections import Counter

scale, view, bone, lim = sys.argv[1], sys.argv[2], sys.argv[3], float(sys.argv[4])
rev = '--reverse' in sys.argv


def rings(s):
    if s['kind'] == 'poly':
        return [s['points']] if s.get('closed') else []
    if s['kind'] == 'rect':
        x, y, w, h = s['x'], s['y'], s['w'], s['h']
        return [[[x, y], [x + w, y], [x + w, y + h], [x, y + h]]]
    if s['kind'] == 'ellipse':
        return [[[s['cx'] + s['rx'] * math.cos(t / 24 * 2 * math.pi), s['cy'] + s['ry'] * math.sin(t / 24 * 2 * math.pi)] for t in range(24)]]
    if s['kind'] == 'path':
        out = []
        for sub in s['d'].split('Z'):
            v = list(map(float, re.findall(r'-?\d+\.?\d*(?:e-?\d+)?', sub)))
            if v:
                out.append(list(zip(v[0::2], v[1::2])))
        return out
    return []


def inside(p, r):
    x, y = p
    c = False
    for i in range(len(r)):
        (x1, y1), (x2, y2) = r[i], r[i - 1]
        if (y1 > y) != (y2 > y) and x < (x2 - x1) * (y - y1) / (y2 - y1) + x1:
            c = not c
    return c


def segd(p, a, b):
    dx, dy = b[0] - a[0], b[1] - a[1]
    L = dx * dx + dy * dy
    t = 0 if L == 0 else max(0, min(1, ((p[0] - a[0]) * dx + (p[1] - a[1]) * dy) / L))
    return math.hypot(p[0] - a[0] - t * dx, p[1] - a[1] - t * dy)


d = json.load(open(f'🗑️generated/r3-project2d/vehicles/tram.s{scale}.{view}.{bone}.json'))
proj = [r for b in d['bones'] for s in b['shapes'] for r in rings(s)]
mine = [r for s in json.load(open(f'r3-tram-2d/s{scale}.{view}.json')) if s['bone'] == bone and s.get('filled') and s['role'] in ('body', 'detail', 'glazing') for r in rings(s)]
src, ref = (mine, proj) if rev else (proj, mine)
bad = []
for r in src:
    for i in range(len(r)):
        a, b = r[i], r[i - 1]
        for k in range(6):
            p = (a[0] + (b[0] - a[0]) * k / 6, a[1] + (b[1] - a[1]) * k / 6)
            if any(inside(p, q) for q in ref):
                continue
            dist = min(segd(p, q[j], q[j - 1]) for q in ref for j in range(len(q)))
            if dist > lim:
                bad.append((round(p[0], 3), round(p[1], 3), round(dist, 3)))
c = Counter((round(x * 4) / 4, round(y * 4) / 4) for x, y, _ in bad)
print('[DEBUG]', len(bad), c.most_common(15))
print('[DEBUG]', sorted(bad, key=lambda t: -t[2])[:12])
