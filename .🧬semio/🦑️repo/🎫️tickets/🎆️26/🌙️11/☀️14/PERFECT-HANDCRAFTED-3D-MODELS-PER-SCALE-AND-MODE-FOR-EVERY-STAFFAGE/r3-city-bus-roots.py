"""🧬️ Round 3 city-bus: the root fixes (dimensions, door bones and travel, anchors) and the 2D drawings of every scale and view redrawn as the true orthographic views of the body.

`python3 r3-city-bus-roots.py` writes the candidate document `🗑️generated/r3-city-bus/🔣️.json` (catalog asset, root fixes, the current `volumes` source); `--write` re-reads the catalog file and replaces only the changed members in it."""
import copy, glob, json, math, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from r3_city_bus_kit import *
import r3_city_bus_body50 as B50

T = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(T, *(['..'] * 7)))
CAT = glob.glob(os.path.join(ROOT, '🎓️teaching', '🏛️architecture', '🧍️staffage', '🗂️catalog', '*', 'city-bus', '🔣️.json'))[0]
OUT = os.path.join(T, '🗑️generated', 'r3-city-bus', '🔣️.json')
SRC = os.path.join(T, 'volumes', 'vehicles', 'city-bus.json')
GEO_KEYS = ('kind', 'x0', 'y0', 'x1', 'y1', 'r0', 'r1', 'cx', 'cy', 'rx', 'ry', 'rotationDeg', 'x', 'y', 'w', 'h', 'r', 'points', 'closed', 'd', 'followTip')
TRAVEL = {'door-m-aft': -0.6, 'door-m-fwd': 0.6, 'door-f-aft': -0.55, 'door-f-fwd': 0.55}
DOOR_X = {'door-m-aft': DOOR_M[0], 'door-m-fwd': DOOR_M[0] + LEAF, 'door-f-aft': DOOR_F[0], 'door-f-fwd': DOOR_F[0] + LEAF}


def n(v):
    s = f'{v:.3f}'.rstrip('0').rstrip('.')
    return '0' if s in ('-0', '') else s


def P(pts, close=True):
    return 'M ' + ' L '.join(f'{n(x)},{n(y)}' for x, y in pts) + (' Z' if close else '')


def R(x0, y0, x1, y1):
    return P([(x0, y0), (x1, y0), (x1, y1), (x0, y1)])


def arc_c(cx, cy, r, a0, a1):
    """cubic segments of the arc from a0 to a1 (radians), the start point is assumed current"""
    k = max(1, math.ceil(abs(a1 - a0) / (math.pi / 3)))
    out = []
    for i in range(k):
        b0 = a0 + (a1 - a0) * i / k
        b1 = a0 + (a1 - a0) * (i + 1) / k
        t = 4 / 3 * math.tan((b1 - b0) / 4)
        p1 = (cx + r * (math.cos(b0) - t * math.sin(b0)), cy + r * (math.sin(b0) + t * math.cos(b0)))
        p2 = (cx + r * (math.cos(b1) + t * math.sin(b1)), cy + r * (math.sin(b1) - t * math.cos(b1)))
        p3 = (cx + r * math.cos(b1), cy + r * math.sin(b1))
        out.append('C ' + ' '.join(f'{n(x)},{n(y)}' for x, y in (p1, p2, p3)))
    return ' '.join(out)


def path(d):
    return {'kind': 'path', 'd': d}


def rect_(x, y, w, h, r=None):
    g = {'kind': 'rect', 'x': rr(x), 'y': rr(y), 'w': rr(w), 'h': rr(h)}
    if r:
        g['r'] = r
    return g


def poly(pts, closed=True):
    return {'kind': 'poly', 'points': [[round(x, 3), round(y, 3)] for x, y in pts], 'closed': closed}


def ellipse(r):
    return {'kind': 'ellipse', 'cx': 0, 'cy': 0, 'rx': r, 'ry': r}


TAIL_SIDE = [(0.05, 0.30), (0.025, 0.34), (0.0, 0.34), (0.0, 0.62), (0.015, 0.62), (0.015, 2.55), (0.035, 2.75), (0.075, 2.86), (0.14, 2.93), (0.23, 2.965), (0.35, 2.98)]
HEAD_SIDE = [(11.45, 2.98), (11.53, 2.975), (11.605, 2.96), (11.685, 2.93), (11.745, 2.88), (11.79, 2.80), (11.815, 2.72), (11.815, 2.66), (11.789, 2.66)]
NOSE_SIDE = [(11.985, 1.19), (11.985, 0.62), (12.0, 0.62), (12.0, 0.34), (11.985, 0.34), (11.95, 0.30)]
TAIL_PLAIN = [(0.05, 0.30), (0.0, 0.36), (0.0, 2.55), (0.035, 2.75), (0.075, 2.86), (0.14, 2.93), (0.23, 2.965), (0.35, 2.98)]
NOSE_PLAIN = [(11.985, 1.19), (12.0, 0.62), (12.0, 0.36), (11.95, 0.30)]


def side_outline(plain=False):
    top = (TAIL_PLAIN if plain else TAIL_SIDE) + HEAD_SIDE + (NOSE_PLAIN if plain else NOSE_SIDE)
    d = P(top, False)
    for xa in (XF, XR):
        d += f' L {n(xa + ARCH_HALF)},{n(SKIRT)} ' + arc_c(xa, AXLE, R_ARCH, -ARCH_A, math.pi + ARCH_A)
    return d + ' Z'


def well(xa):
    return f'M {n(xa + ARCH_HALF)},{n(SKIRT)} ' + arc_c(xa, AXLE, R_ARCH, -ARCH_A, math.pi + ARCH_A) + ' Z'


def roof_band():
    pts = [(0.015, CANT)] + [p for p in TAIL_SIDE if p[1] > CANT] + HEAD_SIDE + [(11.794, CANT)]
    return P(pts)


def ngon(cx, cy, r, k, a0=0.0):
    return P([(cx + r * math.cos(a0 + 2 * math.pi * i / k), cy + r * math.sin(a0 + 2 * math.pi * i / k)) for i in range(k)])


def ringed(rc, r, k, count=10, a0=0.0):
    return ' '.join(ngon(rc * math.cos(a0 + 2 * math.pi * i / count), rc * math.sin(a0 + 2 * math.pi * i / count), r, k, a0 + 2 * math.pi * i / count) for i in range(count))


TRANSOM = BELT + 1.02
SCREEN_SIDE = [(11.59, 1.19), (11.975, 1.19), (11.789, 2.66), (11.425, 2.66)]


def seat_backs():
    xs = sorted({x + k * B50.SEAT_PITCH for runs in B50.SEAT_RUNS.values() for x, c in runs for k in range(c)})
    return ' '.join(R(x - 0.32, SILL_GLASS, x - 0.25, 1.49) for x in xs)


def plan_outline():
    rear = [(X0 - dx, z) for z, dx in dcurve(0.35, HW, TAIL_N, m=14)]
    front = [(X1 + dx, z) for z, dx in dcurve(0.55, HW, NOSE_N, m=18)]
    return list(reversed(front)) + rear


def plan_screen():
    outer = [(X1 + nose_dx(z, GLASS_D, GLASS_W), z) for z in (-1.2, -1.1, -0.9, -0.6, -0.3, 0, 0.3, 0.6, 0.9, 1.1, 1.2)]
    inner = [(X1 + nose_dx(z, 0.365, hw(2.66)) , z) for z in (1.2, 1.1, 0.9, 0.6, 0.3, 0, -0.3, -0.6, -0.9, -1.1, -1.2)]
    return outer + inner


def front_section():
    half = [(HW, SKIRT), (HW, BELT)] + [(w, y) for y, w in CANT_TABLE]
    return [(-w, y) for w, y in half] + [(w, y) for w, y in reversed(half[:-1])]


def master():
    e, f, p = {}, {}, {}
    e['body-shell'] = path(side_outline())
    e['roof-panel'] = e['roof-band'] = path(roof_band())
    for k, (a, c) in (('m', DOOR_M), ('f', DOOR_F)):
        e[f'door-opening-{k}'] = rect_(a, FLOOR, c - a, DOOR_TOP - FLOOR)
        p[f'door-opening-{k}'] = rect_(a, -HW, c - a, TW)
    big = [b for b in BAYS_R if b[1] - b[0] > 0.9]
    small = [b for b in BAYS_R if b[1] - b[0] <= 0.9]
    e['side-windows'] = path(' '.join([R(a, SILL_GLASS, c, TRANSOM - 0.015) for a, c in big] + [R(a, SILL_GLASS, c, HEAD_GLASS) for a, c in small]))
    e['upper-windows'] = e['upper-band'] = path(' '.join(R(a, TRANSOM + 0.015, c, HEAD_GLASS) for a, c in big))
    for k, (a, c) in enumerate(BAYS_R):
        e[f'pane-{k + 1}'] = rect_(a, SILL_GLASS, c - a, (TRANSOM - 0.015 if c - a > 0.9 else HEAD_GLASS) - SILL_GLASS)
    e['windscreen-edge'] = e['pane-cab'] = e['window-band-c'] = path(P(SCREEN_SIDE))
    e['window-band-a'] = path(R(BAYS_R[0][0], SILL_GLASS, BAYS_R[4][1], HEAD_GLASS))
    e['window-band-b'] = path(R(BAYS_R[5][0], SILL_GLASS, BAYS_R[6][1], HEAD_GLASS))
    e['skirt-line'] = path(' '.join(P([(a, 0.62), (c, 0.62)], False) for a, c in ((X0, 2.80), (4.10, DOOR_M[0]), (DOOR_M[1], 8.70), (10.0, DOOR_F[0]))))
    e['belt-line'] = path(' '.join(P([(a, BELT), (c, BELT)], False) for a, c in ((X0, DOOR_M[0]), (DOOR_M[1], DOOR_F[0]))))
    e['pillar-lines'] = path(P([(X0, SKIRT), (X0, CANT)], False) + ' ' + P([(11.59, 1.19), (11.415, 2.72)], False) + ' ' + P([(8.03, 0.71), (8.37, 0.71), (8.37, 1.01), (8.03, 1.01), (8.03, 0.71)], False))
    e['seat-backs'] = path(seat_backs())
    e['grab-rail'] = poly([(0.7, 2.12), (10.0, 2.12)], False)
    e['driver-cab'] = path(P([(11.44, 0.70), (11.80, 0.70), (11.80, 1.20), (11.44, 1.20)]))
    e['headlamp'] = rect_(X1 + nose_dx(1.10), 0.70, nose_dx(0.70) - nose_dx(1.10) + 0.008, 0.18, 0.02)
    e['tail-lamp'] = rect_(0.02, 0.92, 0.035, 0.52, 0.012)
    e['mirror'] = rect_(11.765, 1.84, 0.07, 0.42, 0.02)
    e['bumper-line-f'] = path(P([(X1, 0.62), (11.985, 0.62)], False))
    e['bumper-line-r'] = path(P([(0.015, 0.62), (X0, 0.62)], False))
    e['well-front'], e['well-rear'] = path(well(XF)), path(well(XR))
    e['battery-pack'] = path(P([(0, 0), (0.105, 0.167), (0.3, 0.32), (6.0, 0.32), (6.195, 0.167), (6.3, 0)]))
    e['battery-seams'] = path(' '.join(P([(1.23 + 0.96 * k, 0.24), (1.23 + 0.96 * k, 0.32)], False) for k in range(5)))
    e['hvac-unit'] = path(P([(0, 0), (0.077, 0.125), (0.22, 0.242), (0.9, 0.242), (0.9, 0.25), (1.6, 0.25), (1.6, 0.242), (2.28, 0.242), (2.423, 0.125), (2.5, 0)]))
    e['hvac-louvres'] = path(' '.join(P([(x0 + 0.11 * k, 0.19), (x0 + 0.11 * k, 0.242)], False) for x0 in (0.34, 1.72) for k in range(5)))
    for leaf in ('m-aft', 'm-fwd', 'f-aft', 'f-fwd'):
        e[f'leaf-{leaf}'] = rect_(0.004, -0.01, LEAF - 0.008, 2.06)
        e[f'leaf-{leaf}-glass'] = path(R(0.07, 0.16, LEAF - 0.07, 0.70) + ' ' + R(0.07, 0.82, LEAF - 0.07, 1.94))
        e[f'leaf-handrail-{leaf}'] = path(P([(0.07, 0.76), (LEAF - 0.07, 0.76)], False))
        p[f'leaf-{leaf}'] = rect_(DOOR_X[f'door-{leaf}'] + 0.004, -(HW + 0.031), LEAF - 0.008, 0.028)
    e['e-axle-casting'] = ellipse(0.22)
    e['air-spring-f'] = rect_(-0.125, 0.08, 0.25, 0.32, 0.04)
    e['air-spring-r'] = path(R(-0.465, -0.18, -0.215, 0.14) + ' ' + R(0.215, -0.18, 0.465, 0.14))
    for w, hub in (('f', 0.08), ('r', 0.088)):
        e[f'tyre-{w}'] = ellipse(R_TYRE)
        e[f'sidewall-{w}'] = ellipse(0.39)
        e[f'rim-{w}'] = ellipse(0.302)
        e[f'hand-holes-{w}'] = path(ringed(0.237, 0.024, 6, 10, math.radians(18)))
        e[f'hub-{w}'] = ellipse(hub)
        e[f'studs-{w}'] = path(ringed(0.1675, 0.013, 4))
    for tag, side in (('l', 1), ('r', -1)):
        zf = side * Z_FRONT
        f[f'tyre-f{tag}'] = rect_(zf - 0.1375, 0, 0.275, 0.96, 0.035)
        f[f'tyre-r{tag}'] = path(' '.join(R(side * z - 0.1375, 0, side * z + 0.1375, 0.96) for z in (Z_REAR_IN, Z_REAR_OUT)))
        z0, z1 = sorted((side * 0.70, side * 1.10))
        f[f'headlamp-{tag}'] = rect_(z0, 0.70, z1 - z0, 0.18, 0.03)
        f[f'windscreen-{tag}'] = poly([(0, 1.19), (side * 1.21, 1.19), (side * 1.175, 2.66), (0, 2.66)])
        p[f'headlamp-{tag}'] = rect_(X1 + nose_dx(0.9) - 0.01, z0, 0.02, 0.4, 0.004)
        p[f'tail-lamp-{tag}'] = rect_(0.02, side * 1.03 - 0.055, 0.035, 0.11, 0.004)
    f['headlamps'] = path(R(-1.10, 0.70, -0.70, 0.88) + ' ' + R(0.70, 0.70, 1.10, 0.88))
    f['windscreen'] = poly([(-1.21, 1.19), (1.21, 1.19), (1.175, 2.66), (-1.175, 2.66)])
    f['body-shell'] = poly(front_section())
    f['roof-cap'] = poly([(-w, y) for y, w in CANT_TABLE] + [(w, y) for y, w in reversed(CANT_TABLE[:-1])])
    f['battery-pack'] = f['roof-boxes'] = rect_(-1.04, 2.93, 2.08, 0.37, 0.06)
    f['hvac-unit'] = rect_(-0.9, 2.93, 1.8, 0.3, 0.05)
    f['bumper'] = rect_(-HW, FLOOR, 2 * HW, 0.28, 0.02)
    f['destination-sign'] = rect_(-0.78, 2.365, 1.56, 0.278, 0.01)
    f['mirror-r'] = rect_(MIRROR_R, 1.84, 0.2, 0.42, 0.03)
    f['mirror-l'] = rect_(MIRROR_L - 0.18, 1.82, 0.18, 0.36, 0.03)
    f['mirror-arm-r'] = poly([(-1.245, 2.58), (MIRROR_R + 0.1, 2.27)], False)
    f['mirror-arm-l'] = poly([(1.255, 2.02), (MIRROR_L - 0.17, 2.02)], False)
    f['axle-beam-f'] = poly([(-0.88, 0.535), (-0.77, 0.535), (-0.60, 0.225), (0.60, 0.225), (0.77, 0.535), (0.88, 0.535), (0.66, 0.115), (-0.66, 0.115)])
    f['e-axle-casting'] = rect_(-0.53, 0.105, 1.06, 0.195, 0.03)
    top = HW + 0.031 - (2.39 - BELT) * math.tan(PHI)
    f['leaf-f-fwd'] = poly([(-(HW + 0.003), 0.33), (-(HW + 0.031), 0.33), (-(HW + 0.031), BELT), (-top, 2.39), (-(top - 0.028), 2.39), (-(HW + 0.003), BELT)])
    p['body-shell'] = poly(plan_outline())
    p['roof-panel'] = rect_(X0, -1.24, X1 - X0, 2.48, 0.1)
    p['battery-pack'] = p['roof-boxes-r'] = rect_(POD_BATTERY[0], -1.04, 6.3, 2.08, 0.09)
    p['hvac-unit'] = rect_(POD_HVAC[0], -0.9, 2.5, 1.8, 0.09)
    p['windscreen'] = poly(plan_screen())
    p['mirror-r'] = rect_(11.765, MIRROR_R, 0.07, 0.2, 0.02)
    p['mirror-l'] = rect_(11.685, MIRROR_L - 0.18, 0.07, 0.18, 0.02)
    p['mirror-arm-r'] = poly([(11.47, -1.245), (11.80, MIRROR_R + 0.1)], False)
    p['mirror-arm-l'] = poly([(11.5, 1.255), (11.72, MIRROR_L - 0.17)], False)
    return {'elevation': e, 'front': f, 'plan': p}


def mirrored(half):
    """a closed outline from its right half, listed from the bottom centre line up"""
    return poly(half + [(-x, y) for x, y in reversed(half)])


def pod_front(half, y0, y1, e, m=6):
    """front view of a roof unit: the superellipse of its loft section"""
    c, r = (y0 + y1) / 2, (y1 - y0) / 2
    q = [(half * math.cos(a) ** (2 / e), r * math.sin(a) ** (2 / e)) for a in (math.pi / 2 * k / m for k in range(m + 1))]
    return poly([(x, c - y) for x, y in reversed(q)] + [(x, c + y) for x, y in q[1:]] + [(-x, c + y) for x, y in reversed(q[:-1])] + [(-x, c - y) for x, y in q[1:-1]])


def strip(p0, p1, w):
    """a tube seen from the side: the quad of width w from p0 to p1"""
    dx, dy = p1[0] - p0[0], p1[1] - p0[1]
    n = math.hypot(dx, dy)
    ox, oy = -dy / n * w / 2, dx / n * w / 2
    return poly([(p0[0] + ox, p0[1] + oy), (p1[0] + ox, p1[1] + oy), (p1[0] - ox, p1[1] - oy), (p0[0] - ox, p0[1] - oy)])


AXLE_F_FINE = [(0.632, 0.116), (0.852, 0.43), (1.02, 0.43), (1.05, 0.48), (1.02, 0.53), (0.80, 0.53), (0.75, 0.50), (0.75, 0.56), (0.80, 0.58), (0.825, 0.64), (0.825, 0.80), (0.805, 0.86), (0.70, 0.88), (0.595, 0.86), (0.576, 0.80), (0.576, 0.64), (0.60, 0.58), (0.65, 0.56), (0.65, 0.294), (0.608, 0.224)]
AXLE_F_COARSE = [(0.632, 0.116), (0.86, 0.45), (0.775, 0.505), (0.608, 0.224)]
AXLE_R_FINE = [(0.42, 0.105), (0.42, 0.17), (0.49, 0.17), (0.49, 0.23), (0.53, 0.235), (0.53, 0.26), (0.60, 0.26), (0.66, 0.38), (0.66, 0.58), (0.60, 0.70), (0.39, 0.70), (0.36, 0.66), (0.35, 0.47), (0.35, 0.275)]
AXLE_R_200 = [(0.42, 0.105), (0.42, 0.26), (0.61, 0.26), (0.66, 0.38), (0.66, 0.58), (0.61, 0.70), (0.36, 0.70), (0.36, 0.275)]
LEAF_OUT = {50: HW + 0.031, 100: HW + 0.041, 200: 1.334, 500: 1.366}
ARM = {50: 0.034, 100: 0.034, 200: 0.04}


def variant(d, view, i):
    """the geometry of a shape whose solid differs between the scales, or None where the 1:50 drawing holds"""
    if view == 'front':
        if i == 'axle-beam-f':
            return mirrored(AXLE_F_FINE if d <= 100 else AXLE_F_COARSE)
        if i == 'e-axle-casting':
            return mirrored(AXLE_R_FINE) if d <= 100 else mirrored(AXLE_R_200) if d == 200 else rect_(-0.6, 0.105, 1.2, 0.195)
        if i in ('battery-pack', 'roof-boxes'):
            return pod_front(1.04, 2.93, 3.30, 8 if d < 500 else 6)
        if i == 'hvac-unit':
            return pod_front(0.9, 2.93, 3.222 if d == 100 or d == 50 else 3.23, 8 if d < 500 else 6)
        if i == 'leaf-f-fwd' and d == 200:
            return poly([(-1.331, 0.33), (-1.293, 0.33), (-1.263, 2.39), (-1.301, 2.39)])
        if i == 'leaf-f-fwd' and d == 500:
            return poly([(-1.366, 0.33), (-1.291, 0.33), (-1.261, 2.39), (-1.336, 2.39)])
        if i == 'mirror-arm-r' and d in ARM:
            return strip((-1.245, 2.58), (MIRROR_R + 0.1, 2.27), ARM[d])
        if i == 'mirror-arm-l' and d in ARM:
            return strip((1.255, 2.02), (MIRROR_L - 0.09, 2.02), ARM[d])
    if view == 'plan':
        if i.startswith('leaf-'):
            x = DOOR_X['door-' + i[5:]]
            return rect_(x + 0.004, -LEAF_OUT[d], LEAF - 0.008, LEAF_OUT[d] - HW)
        if i == 'mirror-arm-r' and d in ARM:
            return strip((11.47, -1.245), (11.80, MIRROR_R + 0.1), ARM[d])
        if i == 'mirror-arm-l' and d in ARM:
            return strip((11.5, 1.255), (11.72, MIRROR_L - 0.09), ARM[d])
        if d == 500 and i in ('battery-pack', 'roof-boxes-r'):
            return rect_(POD_BATTERY[0] + 0.05, -1.04, 6.2, 2.08)
        if d == 500 and i == 'hvac-unit':
            return rect_(POD_HVAC[0] + 0.05, -0.9, 2.4, 1.8)
    if view == 'elevation':
        if d == 200 and i == 'battery-pack':
            return path(P([(0, 0), (0, 0.03), (0.26, 0.32), (6.04, 0.32), (6.3, 0.03), (6.3, 0)]))
        if d == 200 and i == 'hvac-unit':
            return path(P([(0, 0), (0, 0.03), (0.26, 0.25), (2.24, 0.25), (2.5, 0.03), (2.5, 0)]))
        if d == 500 and i == 'battery-pack':
            return path(R(0.05, 0, 6.25, 0.32))
        if d == 500 and i == 'hvac-unit':
            return path(R(0.05, 0, 2.45, 0.25))
    return None


DROP = {'pane-8'}
KEEP = {'tread-f', 'tread-r'}
NEW = {
    'front': [('leaf-f-fwd', 'door-f-fwd', 'body', 3, True, 0.5, 'door-leaf', 'doors'), ('axle-beam-f', 'axle-f', 'detail', 1.4, True, 0.3, 'steel-wheel', None), ('e-axle-casting', 'axle-r', 'detail', 1.3, True, 0.3, 'e-axle', 'e-axle'), ('mirror-arm-r', 'body', 'detail', 7, False, 0.3, 'paint', None), ('mirror-arm-l', 'body', 'detail', 7, False, 0.3, 'paint', None), ('mirror-r', 'body', 'detail', 7, True, 0.3, 'paint', None), ('mirror-l', 'body', 'detail', 7, True, 0.3, 'paint', None)],
    'plan': [('mirror-arm-r', 'body', 'detail', 7, False, 0.3, 'paint', None), ('mirror-arm-l', 'body', 'detail', 7, False, 0.3, 'paint', None), ('mirror-r', 'body', 'detail', 7, True, 0.3, 'paint', None), ('mirror-l', 'body', 'detail', 7, True, 0.3, 'paint', None)],
    'elevation': [('well-front', 'body', 'body', 1, True, 0.04, 'paint', None), ('well-rear', 'body', 'body', 1, True, 0.04, 'paint', None)],
}

PLAIN_FROM = 500
NO_ARMS_FROM = 500


def redraw(asset):
    m = master()
    for scale, rep in asset['representations'].items():
        d = int(scale)
        for view, shapes in rep['geometry'].items():
            out, seen = [], set()
            for s in shapes:
                i = s['id']
                if i in DROP:
                    continue
                seen.add(i)
                if i in KEEP:
                    out.append(s)
                    continue
                if i not in m[view]:
                    raise SystemExit(f'no drawing for {scale} {view} {i}')
                g = copy.deepcopy(m[view][i])
                if view == 'elevation' and i == 'body-shell' and d >= PLAIN_FROM:
                    g = path(side_outline(True))
                g = variant(d, view, i) or g
                t = {k: v for k, v in s.items() if k not in GEO_KEYS}
                if 'arm' in i and g.get('closed'):
                    t['filled'] = True
                if view == 'front' and i.startswith('tyre-'):
                    t['z'] = 1.6 if i[5] == 'f' else 1.5
                out.append({**g, **t})
            for i, bone, role, z, filled, tone, mat, region in NEW[view]:
                if i in seen or (d >= NO_ARMS_FROM and 'arm' in i):
                    continue
                g = variant(d, view, i) or copy.deepcopy(m[view][i])
                t = {'id': i, 'bone': bone, 'role': 'body' if d >= PLAIN_FROM else role, 'z': z, 'filled': filled or bool(g.get('closed')), 'tone': tone, 'material': mat}
                if region:
                    t['region'] = region
                out.append({**g, **t})
            rep['geometry'][view] = out


def roots(asset):
    log = []

    def put(obj, key, new, what):
        if obj.get(key) != new:
            log.append(f'{what}: {obj.get(key)} -> {new}')
            obj[key] = new
    put(asset['dimensions'], 'heightM', rr(HEIGHT), 'dimensions.heightM')
    put(asset['dimensions'], 'depthM', rr(MIRROR_L - MIRROR_R), 'dimensions.depthM')
    for b in asset['rig']['bones']:
        if b['id'] in DOOR_X:
            put(b, 'x', DOOR_X[b['id']], f"bone {b['id']}.x")
            put(b, 'lengthM', LEAF, f"bone {b['id']}.lengthM")
            put(b['mass'], 'comX', rr(LEAF / 2), f"bone {b['id']}.mass.comX")
            put(b['joint'], 'maxM', abs(TRAVEL[b['id']]), f"bone {b['id']}.joint.maxM")
        if b['id'] == 'battery':
            put(b['mass'], 'comY', 0.15, 'bone battery.mass.comY')
        if b['id'] == 'hvac':
            put(b['mass'], 'comY', 0.12, 'bone hvac.mass.comY')
    for a in asset['anchors']:
        if a['id'] in ('contact-r-l', 'contact-r-r'):
            put(a, 'z', rr(math.copysign((Z_REAR_OUT + Z_REAR_IN) / 2, a['z'])), f"anchor {a['id']}.z")
        if a['id'] == 'hvac-vent':
            put(a, 'y', 0.25, 'anchor hvac-vent.y')

    for region in asset['thermal']['regions']:
        ids = [l['id'] for l in region['layers']]
        if region['id'] == 'glazing' and ids == ['pane-out', 'gap', 'pane-in']:
            region['layers'][0]['id'], region['layers'][2]['id'] = 'pane-in', 'pane-out'
            log.append('thermal.regions glazing.layers: pane-out, gap, pane-in -> pane-in, gap, pane-out (innermost first)')

    def travel(bones, where):
        for k, v in bones.items():
            if k in TRAVEL and 'dx' in v and v['dx'] != 0:
                put(v, 'dx', TRAVEL[k], f'{where} {k}.dx')
    for pose in asset['poses']:
        travel(pose['bones'], f"pose {pose['id']}")
    for clip in asset['clips']:
        for key in clip['keys']:
            travel(key.get('bones', {}), f"clip {clip['id']} key t={key['t']}")
    for t in asset['structure']['assumptions']:
        for lang, old, new in (('en', 'by up to 0.45 m', 'by up to 0.6 m'), ('de', 'um bis zu 0,45 m', 'um bis zu 0,6 m')):
            if old in t[lang]:
                t[lang] = t[lang].replace(old, new)
                log.append(f'structure.assumptions {lang}: {old} -> {new}')
    return log


def member_span(text, key):
    a = text.index(f'\n  "{key}": ')
    b = text.index('\n  "', a + 1)
    return a, b


def dump_member(key, value):
    body = json.dumps(value, ensure_ascii=False, indent=1).replace('\n', '\n  ')
    return f'\n  "{key}": {body},'


if __name__ == '__main__':
    text = open(CAT, encoding='utf8').read()
    doc = json.loads(text)
    asset = doc['asset']
    before = copy.deepcopy(asset)
    log = roots(asset)
    redraw(asset)
    changed = [k for k in asset if asset[k] != before[k]]
    if '--write' in sys.argv:
        for k in changed:
            a, b = member_span(text, k)
            if k == 'thermal':
                part = text[a:b]
                assert part.count('"id": "pane-out"') == 1 and part.count('"id": "pane-in"') == 1
                part = part.replace('"id": "pane-out"', '"id": "pane-@"').replace('"id": "pane-in"', '"id": "pane-out"').replace('"id": "pane-@"', '"id": "pane-in"')
                text = text[:a] + part + text[b:]
                continue
            text = text[:a] + dump_member(k, asset[k]) + text[b:]
        check = json.loads(text)['asset']
        assert all(check[k] == asset[k] for k in asset if k != 'volumes') and check['volumes'] == before['volumes'] and list(check) == list(before)
        tmp = CAT + '.r3.tmp'
        open(tmp, 'w', encoding='utf8').write(text)
        os.replace(tmp, CAT)
        print('written', changed)
    else:
        asset['volumes'] = json.load(open(SRC))['volumes']
        os.makedirs(os.path.dirname(OUT), exist_ok=True)
        json.dump(doc, open(OUT, 'w', encoding='utf8'), ensure_ascii=False, indent=1)
        print('candidate', OUT, changed)
    print('\n'.join(log))
