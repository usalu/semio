"""🦆 Mallard wing and shoulder girdle rebuilt by hand: coracoid on its lateral rest placement, humerus, forearm and hand at their skeletal lengths, ten primaries and ten secondaries on five feather tract bones each, folded along the flank at rest and fanned in `wing-stretch`.
Usage: python3 rework-mallard-duck-wing.py [--write]   (default: candidate under 🗑️generated/rework-mallard-duck/cand/🔣️.json)"""
import json, math, os, sys

T = os.path.dirname(os.path.abspath(__file__))
CATALOG = '/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/mallard-duck/🔣️.json'
SOURCE = os.path.join(T, 'volumes/animals/mallard-duck.json')
CANDIDATE = os.path.join(T, '🗑️generated/rework-mallard-duck/cand/🔣️.json')

STERNAL = (0.114, -0.050, -0.011)
SHOULDER = (0.105, -0.015, -0.030)
ELBOW = (0.015, -0.003, -0.052)
WRIST = (0.088, -0.033, -0.057)
FINGER = (0.004, -0.020, -0.056)
HULL = [(-0.152, 0.0185, 0.0054, 0.0064), (-0.137, 0.0088, 0.0134, 0.0112), (-0.12, 0.001, 0.0204, 0.0164), (-0.105, -0.0055, 0.0264, 0.0219), (-0.088, -0.0105, 0.0334, 0.0289), (-0.07, -0.0146, 0.0401, 0.0363), (-0.06, -0.0165, 0.044, 0.0405), (-0.03, -0.021, 0.052, 0.049), (0, -0.024, 0.058, 0.055), (0.03, -0.026, 0.062, 0.06), (0.06, -0.0262, 0.063, 0.0628), (0.09, -0.0248, 0.061, 0.0633), (0.115, -0.0215, 0.057, 0.0615), (0.135, -0.018, 0.0504, 0.0574)]
PRIMARY = {'stem': 'p', 'start': 0.06, 'step': 0.094, 'angle': [176 - 0.55 * j for j in range(10)], 'length': [0.098, 0.104, 0.110, 0.116, 0.122, 0.128, 0.134, 0.139, 0.142, 0.138], 'half': 0.0125, 'lift': [0.0005 + 0.0002 * j for j in range(10)], 'tone': 0.5, 'spread': [24, 38, 52, 66, 80], 'count': 5, 'base': 'WRIST', 'tip': 'FINGER', 'parent': 'hand', 'name': 'primary'}
SECONDARY = {'stem': 's', 'start': 0.08, 'step': 0.095, 'angle': [174 + 0.4 * j for j in range(10)], 'length': [0.100 - 0.002 * j for j in range(10)], 'half': 0.015, 'lift': [0.0026 + 0.0002 * j for j in range(10)], 'tone': 0.3, 'spread': [14, 7.5, 1, -5.5, -12], 'count': 5, 'base': 'WRIST', 'tip': 'ELBOW', 'parent': 'forearm', 'name': 'secondary'}
TERTIAL = {'stem': 't', 'start': 0.5, 'step': 0.3, 'angle': [172, 173.5], 'length': [0.098, 0.09], 'half': 0.016, 'lift': [0.0048, 0.005], 'tone': 0.68, 'spread': [-30], 'count': 1, 'base': 'SHOULDER', 'tip': 'ELBOW', 'parent': 'humerus', 'name': 'tertial'}
TRACTS = (PRIMARY, SECONDARY, TERTIAL)
JOINTS = {'SHOULDER': SHOULDER, 'ELBOW': ELBOW, 'WRIST': WRIST, 'FINGER': FINGER}
STATIONS = [(0, 0.3), (0.12, 0.6), (0.35, 1), (0.6, 1), (0.82, 0.85), (0.95, 0.55), (1, 0.2)]
STRETCH = {'humerus': (-0.45, 0.55, -0.70), 'forearm': (0.35, 0.60, -0.72), 'hand': (-0.25, 0.50, -0.83)}
OLD_WING = {'o': (0.105, -0.015), 'deg': 169.5}
OLD_HAND = {'o': (0.0880, -0.0330), 'deg': math.degrees(math.atan2(0.1530, -0.9882))}
BLADES = {'50': None, '100': (0.017, 0.0008, [0, 1, 2, 3, 4]), '200': (0.02, 0.00075, [0, 2, 4]), '500': (0.03, 0.002, [0, 2, 4])}
WIDTHS = [0.25] + [w for _, w in STATIONS]


def sub(a, b): return [a[i] - b[i] for i in range(3)]
def add(a, b): return [a[i] + b[i] for i in range(3)]
def mul(a, k): return [x * k for x in a]
def dot(a, b): return sum(a[i] * b[i] for i in range(3))
def cross(a, b): return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
def norm(a): return math.sqrt(dot(a, a))
def unit(a): return mul(a, 1 / norm(a))
def away(a, u): return unit(sub(a, mul(u, dot(a, u))))
def local(frame, origin, p): return [dot(sub(p, origin), axis) for axis in frame]
def turn(frame, d): return [dot(d, axis) for axis in frame]
def rnd(v, n=5): return [round(x, n) for x in v]


def quat(a, b, c):
    """🧭 Unit quaternion [x, y, z, w] of the rotation whose columns are a, b, c."""
    m = [[a[0], b[0], c[0]], [a[1], b[1], c[1]], [a[2], b[2], c[2]]]
    tr = m[0][0] + m[1][1] + m[2][2]
    if tr > 0:
        s = math.sqrt(tr + 1) * 2
        q = [(m[2][1] - m[1][2]) / s, (m[0][2] - m[2][0]) / s, (m[1][0] - m[0][1]) / s, s / 4]
    elif m[0][0] > m[1][1] and m[0][0] > m[2][2]:
        s = math.sqrt(1 + m[0][0] - m[1][1] - m[2][2]) * 2
        q = [s / 4, (m[0][1] + m[1][0]) / s, (m[0][2] + m[2][0]) / s, (m[2][1] - m[1][2]) / s]
    elif m[1][1] > m[2][2]:
        s = math.sqrt(1 + m[1][1] - m[0][0] - m[2][2]) * 2
        q = [(m[0][1] + m[1][0]) / s, s / 4, (m[1][2] + m[2][1]) / s, (m[0][2] - m[2][0]) / s]
    else:
        s = math.sqrt(1 + m[2][2] - m[0][0] - m[1][1]) * 2
        q = [(m[0][2] + m[2][0]) / s, (m[1][2] + m[2][1]) / s, s / 4, (m[1][0] - m[0][1]) / s]
    return rnd(q)


def qmul(p, q):
    """✖️ Hamilton product of two [x, y, z, w] quaternions."""
    x1, y1, z1, w1 = p
    x2, y2, z2, w2 = q
    return [w1 * x2 + x1 * w2 + y1 * z2 - z1 * y2, w1 * y2 - x1 * z2 + y1 * w2 + z1 * x2, w1 * z2 + x1 * y2 - y1 * x2 + z1 * w2, w1 * w2 - x1 * x2 - y1 * y2 - z1 * z2]


def placement(parent, origin, frame):
    """🦴 Rest placement (x, y, z, angleDeg, abductionDeg, twistDeg) of a frame (u, v, w) at origin under the parent (origin, frame)."""
    po, pf = parent
    x, y, z = local(pf, po, origin)
    u, v, w = (turn(pf, axis) for axis in frame)
    b = math.asin(max(-1, min(1, u[2])))
    a = math.atan2(u[1], u[0])
    ca, sa, cb, sb = math.cos(a), math.sin(a), math.cos(b), math.sin(b)
    v0 = [-sa, ca, 0]
    w0 = [-ca * sb, -sa * sb, cb]
    t = math.atan2(-dot(v, w0), dot(v, v0))
    return {'x': round(x, 5), 'y': round(y, 5), 'z': round(z, 5), 'angleDeg': round(math.degrees(a), 4), 'abductionDeg': round(math.degrees(b), 4), 'twistDeg': round(math.degrees(t), 4)}


def offset(rest, target):
    """🎯 Pose offset that turns the rest frame into the target frame, about the bone's own rest axes."""
    p = placement(((0, 0, 0), rest), (0, 0, 0), target)
    return {k: p[k] for k in ('angleDeg', 'abductionDeg', 'twistDeg')}


def hull(x, y):
    """🥚 Half breadth of the plumage hull at station x and height y (trunk frame)."""
    rows = HULL
    x = max(rows[0][0], min(rows[-1][0], x))
    for lo, hi in zip(rows, rows[1:]):
        if lo[0] <= x <= hi[0]:
            k = (x - lo[0]) / (hi[0] - lo[0])
            cy, rz, ry = (lo[i] + k * (hi[i] - lo[i]) for i in (1, 2, 3))
            return max(0.011, rz * (1 - min(1, abs(y - cy) / ry) ** 2.2) ** (1 / 2.2))
    return 0.011


def skin(x, y, lift):
    """📍 Point on the near flank lifted off the hull."""
    return [x, y, -(hull(x, y) + lift)]


def strip(points, halves, thick, frame, origin, exponent=2):
    """🪶 Loft sections of a thin strip through world points that lies in the flank surface, stated in a bone frame."""
    sections = []
    for i, p in enumerate(points):
        d = unit(sub(points[min(i + 1, len(points) - 1)], points[max(i - 1, 0)]))
        slope = (hull(p[0], p[1] + 0.002) - hull(p[0], p[1] - 0.002)) / 0.004
        b = unit([0, 1, -slope])
        t = unit(cross(b, d))
        b = cross(d, t)
        sections.append({'at': rnd(local(frame, origin, p)), 'rx': round(thick if isinstance(thick, float) else thick[i], 5), 'ry': round(halves[i], 5), 'exponent': exponent, 'rotation': quat(turn(frame, t), turn(frame, b), turn(frame, d))})
    return sections


def along(bone, rows):
    """🦴 Loft along a bone axis: rows of (x, thickness along w, chord along v)."""
    return [{'at': [round(x, 5), 0, 0], 'rx': rx, 'ry': ry, 'exponent': 2, 'rotation': quat([0, 0, 1], [0, -1, 0], [1, 0, 0])} for x, rx, ry in rows]


def round_loft(rows):
    """🦴 Round bone shaft: rows of (x, y, z, radius) in the bone frame, axis along u."""
    return [{'at': [round(x, 5), round(y, 5), round(z, 5)], 'rx': r, 'ry': r, 'exponent': 2, 'rotation': [0.5, 0.5, 0.5, 0.5]} for x, y, z, r in rows]


def mirrored(solid, side):
    """🪞 The far twin of a near solid: local z and the x, y parts of every rotation negated, ids and bones renamed."""
    s = json.loads(json.dumps(solid).replace('near-', 'far-').replace('-n"', '-f"')) if side == 'f' else solid
    if side == 'n':
        return s
    for q in s.get('sections', []):
        q['at'][2] = -q['at'][2]
        q['rotation'] = [-q['rotation'][0], -q['rotation'][1], q['rotation'][2], q['rotation'][3]]
    if 'transform' in s:
        s['transform']['translation'][2] = -s['transform']['translation'][2]
    return s


def frames():
    """🧭 Rest frames (origin, (u, v, w)) of the near girdle, wing and feather tract bones in the trunk frame."""
    z = [0, 0, 1]
    out = {'trunk': ((0, 0, 0), ([1, 0, 0], [0, 1, 0], [0, 0, 1]))}
    u = unit(sub(SHOULDER, STERNAL))
    w = away([1, 0, 0], u)
    out['coracoid'] = (STERNAL, (u, cross(w, u), w))
    for name, a, b in (('humerus', SHOULDER, ELBOW), ('forearm', ELBOW, WRIST), ('hand', WRIST, FINGER)):
        u = unit(sub(b, a))
        w = away(z, u)
        out[name] = (a, (u, cross(w, u), w))
    for tract in TRACTS:
        parent = tract['parent']
        for k in range(tract['count']):
            quills = [quill(tract, j) for j in (2 * k, 2 * k + 1)]
            origin = mul(add(quills[0][0], quills[1][0]), 0.5)
            end = mul(add(quills[0][-1], quills[1][-1]), 0.5)
            u = unit(sub(end, origin))
            w = away(out[parent][1][2], u)
            out[f'{tract["stem"]}{k + 1}'] = (origin, (u, cross(w, u), w))
    return out


def quill(tract, j, lift=None):
    """🪶 Centre line of feather j of a tract: its root on the bone axis, then stations on the flank."""
    s = tract['start'] + tract['step'] * j
    base, tip = JOINTS[tract['base']], JOINTS[tract['tip']]
    root = add(base, mul(sub(tip, base), s))
    a = math.radians(tract['angle'][j])
    return [root] + [skin(root[0] + math.cos(a) * tract['length'][j] * t, root[1] + math.sin(a) * tract['length'][j] * t, tract['lift'][j] if lift is None else lift) for t, _ in STATIONS]


def wing_solids(scale, f):
    """🧊 Near body solids of the wing at one scale and the structure solids of girdle, wing skeleton and feather shafts."""
    body, structure = [], []
    base = {'material': 'wing-feathers', 'role': 'body', 'capStart': True, 'capEnd': True}
    flat = scale == '200'
    thick = {'50': 1, '100': 1, '200': 1, '500': 1}[scale]
    if flat:
        body.append({'id': 'near-upper-wing', 'kind': 'loft', 'bone': 'humerus-n', **base, 'tone': 0.62, 'sheet': True, 'sections': along('humerus', [(-0.004, 0.00075, 0.008), (0.02, 0.00075, 0.016), (0.07, 0.00075, 0.016), (0.097, 0.00075, 0.009)])})
        body.append({'id': 'near-forearm', 'kind': 'loft', 'bone': 'forearm-n', **base, 'tone': 0.62, 'sheet': True, 'sections': along('forearm', [(-0.004, 0.00075, 0.009), (0.015, 0.00075, 0.016), (0.06, 0.00075, 0.016), (0.083, 0.00075, 0.009)])})
        body.append({'id': 'near-hand', 'kind': 'loft', 'bone': 'hand-n', **base, 'tone': 0.62, 'sheet': True, 'sections': along('hand', [(-0.004, 0.00075, 0.009), (0.015, 0.00075, 0.016), (0.06, 0.00075, 0.016), (0.088, 0.00075, 0.008)])})
    else:
        fore = 0.0076 if scale == '100' else 0.0065
        hand = 0.0076 if scale == '100' else 0.005
        body.append({'id': 'near-upper-wing', 'kind': 'loft', 'bone': 'humerus-n', **base, 'tone': 0.62, 'sections': along('humerus', [(-0.004, 0.006, 0.007), (0.02, 0.009, 0.012), (0.05, 0.008, 0.011), (0.08, 0.0065, 0.0095), (0.097, 0.005, 0.007)])})
        body.append({'id': 'near-forearm', 'kind': 'loft', 'bone': 'forearm-n', **base, 'tone': 0.62, 'sections': along('forearm', [(-0.004, 0.005, 0.007), (0.015, fore, 0.011), (0.045, fore, 0.012), (0.07, 0.0055, 0.009), (0.083, 0.0045, 0.006)])})
        body.append({'id': 'near-hand', 'kind': 'loft', 'bone': 'hand-n', **base, 'tone': 0.62, 'sections': along('hand', [(-0.004, 0.0045, 0.006), (0.02, hand, 0.009), (0.055, hand if scale == '100' else 0.0045, 0.0078 if scale == '100' else 0.0065), (0.08, 0.0038, 0.0045), (0.088, 0.0038, 0.0038)])})
    if scale != '500':
        wide = 1.25 if scale == '200' else 1
        o, fr = f['forearm']
        line = [add(ELBOW, mul(sub(WRIST, ELBOW), t)) for t in (-0.12, 0.1, 0.35, 0.65, 0.9, 1.1)]
        body.append({'id': 'near-wing-coverts', 'kind': 'loft', 'bone': 'forearm-n', **base, 'tone': 0.66, 'sheet': True, 'sections': strip([skin(p[0] - 0.006, p[1] + 0.004, 0.004) for p in line], [0.008, 0.017, 0.02, 0.019, 0.015, 0.007], 0.00075 if flat else 0.001, fr, o, 2.3)})
        o, fr = f['hand']
        line = [add(WRIST, mul(sub(FINGER, WRIST), t)) for t in (-0.1, 0.12, 0.45, 0.8, 1.05, 1.22)]
        body.append({'id': 'near-hand-coverts', 'kind': 'loft', 'bone': 'hand-n', **base, 'tone': 0.58, 'sheet': True, 'sections': strip([skin(p[0], p[1], 0.003) for p in line], [h * wide for h in (0.006, 0.012, 0.013, 0.0125, 0.008, 0.004)], 0.00065 if not flat else 0.00075, fr, o, 2.3)})
    blades = BLADES[scale]
    for tract in TRACTS:
        name = tract['name']
        for k in range(tract['count']):
            bone = f'{tract["stem"]}{k + 1}'
            o, fr = f[bone]
            first = quill(tract, 2 * k)
            if blades is None:
                for j in (2 * k, 2 * k + 1):
                    path = quill(tract, j)
                    body.append({'id': f'near-{name}-{j + 1}', 'kind': 'loft', 'bone': f'{bone}-n', **base, 'tone': round(tract['tone'] + 0.03 * (j % 2), 2), 'sheet': True, 'sections': strip(path, [tract['half'] * w for w in WIDTHS], 0.0005, fr, o)})
            elif k in blades[2]:
                p0, p1 = quill(tract, 2 * k, tract['lift'][2 * k]), quill(tract, 2 * k + 1, tract['lift'][2 * k])
                path = [mul(add(p, q), 0.5) for p, q in zip(p0, p1)]
                path = [path[0]] + [skin(p[0], p[1], tract['lift'][2 * k]) for p in path[1:]]
                body.append({'id': f'near-{name}-blade-{k + 1}', 'kind': 'loft', 'bone': f'{bone}-n', **base, 'tone': tract['tone'], 'sheet': True, 'sections': strip(path, [blades[0] * w for w in WIDTHS], blades[1], fr, o)})
            shaft = [local(fr, o, p) for p in [first[0], first[1], first[3], first[5], first[7]]]
            structure.append({'id': f'near-rachis-{bone}', 'kind': 'loft', 'bone': f'{bone}-n', 'material': 'bone', 'role': 'body', 'tone': 0.88, 'slender': True, 'member': f'{bone}-n', 'part': 'member', 'capStart': True, 'capEnd': True, 'sections': round_loft([(p[0], p[1], p[2], r) for p, r in zip(shaft, (0.0011, 0.0011, 0.0009, 0.0007, 0.0005))])})
    bone = {'material': 'bone', 'role': 'body', 'tone': 0.88, 'part': 'member', 'capStart': True, 'capEnd': True}
    lc = norm(sub(SHOULDER, STERNAL)); lh = norm(sub(ELBOW, SHOULDER)); lf = norm(sub(WRIST, ELBOW)); lm = norm(sub(FINGER, WRIST))
    long = {} if scale == '500' else {'slender': True}
    structure.append({'id': 'near-coracoid', 'kind': 'loft', 'bone': 'coracoid-n', **bone, **({'slender': True} if scale != '500' else {}), 'member': 'coracoid-n', 'sections': round_loft([(-0.002, 0, 0, 0.001), (lc + 0.001, 0, 0, 0.001)] if scale == '200' else [(-0.002, 0, 0, 0.0042), (0.006, 0, 0, 0.003), (lc * 0.5, 0, 0, 0.0022), (lc - 0.006, 0, 0, 0.0028), (lc + 0.001, 0, 0, 0.0036)])})
    structure.append({'id': 'near-humerus', 'kind': 'loft', 'bone': 'humerus-n', **bone, **long, 'member': 'humerus-n', 'sections': round_loft([(-0.002, 0, 0, 0.0052), (0.008, 0, 0, 0.0046), (0.02, 0, 0, 0.0034), (lh * 0.5, 0, 0, 0.0031), (lh - 0.012, 0, 0, 0.0034), (lh - 0.003, 0, 0, 0.0048), (lh + 0.002, 0, 0, 0.0042)])})
    if scale == '50':
        structure.append({'id': 'near-ulna', 'kind': 'loft', 'bone': 'forearm-n', **bone, 'slender': True, 'member': 'forearm-n', 'sections': round_loft([(-0.003, 0, 0, 0.0036), (0.008, -0.0015, 0, 0.0029), (lf * 0.5, -0.0035, 0, 0.0025), (lf - 0.008, -0.0015, 0, 0.0027), (lf + 0.002, 0, 0, 0.0033)])})
        structure.append({'id': 'near-radius', 'kind': 'loft', 'bone': 'forearm-n', **bone, 'slender': True, 'member': 'forearm-n', 'sections': round_loft([(0.001, 0.0028, 0, 0.0022), (0.01, 0.0042, 0, 0.0016), (lf * 0.5, 0.0052, 0, 0.0015), (lf - 0.008, 0.0038, 0, 0.0017), (lf, 0.0024, 0, 0.0023)])})
        structure.append({'id': 'near-carpometacarpus', 'kind': 'loft', 'bone': 'hand-n', **bone, 'slender': True, 'member': 'hand-n', 'sections': round_loft([(-0.002, 0, 0, 0.0034), (0.006, 0.0006, 0, 0.0026), (0.028, 0.0008, 0, 0.0021), (0.05, 0.0004, 0, 0.0023), (0.058, 0, 0, 0.0027), (0.066, -0.0005, 0, 0.0022), (0.076, -0.0003, 0, 0.0016), (lm, 0, 0, 0.0009)])})
        structure.append({'id': 'near-minor-metacarpal', 'kind': 'loft', 'bone': 'hand-n', **bone, 'slender': True, 'member': 'hand-n', 'sections': round_loft([(0.005, -0.0012, 0, 0.0014), (0.018, -0.0042, 0, 0.0011), (0.036, -0.0046, 0, 0.0011), (0.052, -0.0016, 0, 0.0013)])})
        structure.append({'id': 'near-alula-digit', 'kind': 'loft', 'bone': 'hand-n', **bone, 'slender': True, 'member': 'hand-n', 'sections': round_loft([(0.003, 0.0022, 0, 0.0015), (0.01, 0.0044, 0, 0.0012), (0.018, 0.0052, 0, 0.0007)])})
        for name, r in (('humerus', 0.0052), ('forearm', 0.0046), ('hand', 0.0039)):
            structure.append({'id': f'joint-{name}-n', 'kind': 'ellipsoid', 'bone': f'{name}-n', 'material': 'bone', 'role': 'body', 'tone': 0.7, 'radii': [r, r, r], 'member': f'{name}-n', 'part': 'joint'})
    else:
        structure.append({'id': 'near-forearm-bones', 'kind': 'loft', 'bone': 'forearm-n', **bone, **long, 'member': 'forearm-n', 'sections': round_loft([(-0.003, 0, 0, 0.0038), (0.01, 0, 0, 0.0032), (lf * 0.5, 0, 0, 0.003), (lf - 0.008, 0, 0, 0.0032), (lf + 0.002, 0, 0, 0.0036)])})
        structure.append({'id': 'near-manus', 'kind': 'loft', 'bone': 'hand-n', **bone, **long, 'member': 'hand-n', 'sections': round_loft([(-0.002, 0, 0, 0.0034), (0.01, 0, 0, 0.0026), (0.05, 0, 0, 0.0024), (0.07, 0, 0, 0.0017), (lm, 0, 0, 0.001)])})
    return body, structure


def rebased(solid, old, side):
    """🔁 A loft written in an old sagittal wing or hand frame, restated on the trunk."""
    a = math.radians(old['deg'])
    c, s = math.cos(a), math.sin(a)
    q0 = [0, 0, math.sin(a / 2), math.cos(a / 2)]
    out = {**solid, 'bone': 'trunk'}
    out.pop('weights', None)
    out['sections'] = [{**q, 'at': rnd([old['o'][0] + c * q['at'][0] - s * q['at'][1], old['o'][1] + s * q['at'][0] + c * q['at'][1], q['at'][2] + (-0.03 if side == 'n' else 0.03)]), 'rotation': rnd(qmul(q0, q['rotation']))} for q in solid['sections']]
    return out


def shape_on_trunk(shape, old):
    """✏️ An elevation shape drawn in an old wing or hand frame, restated on the trunk."""
    a = math.radians(old['deg'])
    c, s = math.cos(a), math.sin(a)
    shape['points'] = [rnd([old['o'][0] + c * x - s * y, old['o'][1] + s * x + c * y]) for x, y in shape['points']]


def stretch(f):
    """🪽 Offsets of the near wing bones in `wing-stretch`: the wing raised and three quarters open, feathers fanned in the wing plane."""
    h, fo, g = (unit(STRETCH[k]) for k in ('humerus', 'forearm', 'hand'))
    n = unit(cross(h, fo))
    dorsal = mul(n, -1)
    g = away(g, n)
    c = unit(cross(dorsal, fo))
    o = away(away(g, c), n)
    world = {'trunk': f['trunk'][1], 'coracoid': f['coracoid'][1]}
    offsets = {}
    for name, parent, u in (('humerus', 'coracoid', h), ('forearm', 'humerus', fo), ('hand', 'forearm', g)):
        target = (u, cross(n, u), n)
        rest_local = [turn(f[parent][1], axis) for axis in f[name][1]]
        rest_posed = [[sum(world[parent][k][i] * axis[k] for k in range(3)) for i in range(3)] for axis in rest_local]
        offsets[name] = offset(rest_posed, target)
        world[name] = target
    for tract in TRACTS:
        parent = tract['parent']
        for k in range(tract['count']):
            name = f'{tract["stem"]}{k + 1}'
            a = math.radians(tract['spread'][k])
            u = unit(add(mul(c, math.cos(a)), mul(o, math.sin(a))))
            target = (u, cross(n, u), n)
            rest_local = [turn(f[parent][1], axis) for axis in f[name][1]]
            rest_posed = [[sum(world[parent][k2][i] * axis[k2] for k2 in range(3)) for i in range(3)] for axis in rest_local]
            offsets[name] = offset(rest_posed, target)
    return offsets


def build():
    """🔨 The whole asset document with the new rig, pose, solids and re-bound drawings."""
    text = open(CATALOG).read().replace('"wing-n"', '"humerus-n"').replace('"wing-f"', '"humerus-f"')
    doc = json.loads(text)
    asset = doc['asset']
    f = frames()
    bones = asset['rig']['bones']
    old = {b['id']: b for b in bones}
    chain = [('coracoid', 'trunk', 'Coracoid, the strut from the sternum to the shoulder joint', 'Rabenbein, die Strebe vom Brustbein zum Schultergelenk', (-4, 4)), ('humerus', 'coracoid', 'upper wing (humerus)', 'Oberflügel (Humerus)', None), ('forearm', 'humerus', 'forearm (ulna and radius) with the secondaries', 'Unterarm (Elle und Speiche) mit den Armschwingen', None), ('hand', 'forearm', 'hand (carpometacarpus and digits) with the primaries', 'Hand (Carpometacarpus und Finger) mit den Handschwingen', None)]
    chain += [(f'p{k + 1}', 'hand', f'primaries P{2 * k + 1} and P{2 * k + 2}', f'Handschwingen P{2 * k + 1} und P{2 * k + 2}', (-140, 140)) for k in range(5)]
    chain += [('t1', 'humerus', 'tertials', 'Schirmfedern', (-140, 140))]
    chain += [(f's{k + 1}', 'forearm', f'secondaries S{2 * k + 1} and S{2 * k + 2}', f'Armschwingen S{2 * k + 1} und S{2 * k + 2}', (-140, 140)) for k in range(5)]
    lengths = {'coracoid': norm(sub(SHOULDER, STERNAL)), 'humerus': norm(sub(ELBOW, SHOULDER)), 'forearm': norm(sub(WRIST, ELBOW)), 'hand': norm(sub(FINGER, WRIST))}
    pose = stretch(f)
    made = []
    for side, word, wort, sign in (('n', 'Near', 'Nah', 1), ('f', 'Far', 'Fern', -1)):
        for name, parent, en, de, limits in chain:
            p = placement(f[parent], f[name][0], f[name][1])
            p = {**p, 'z': sign * p['z'], 'abductionDeg': sign * p['abductionDeg'], 'twistDeg': sign * p['twistDeg']}
            tract = next((t for t in TRACTS if t['stem'] == name[0] and name[1:].isdigit()), None)
            length = lengths.get(name) or norm(sub(mul(add(quill(tract, 2 * (int(name[1]) - 1))[-1], quill(tract, 2 * (int(name[1]) - 1) + 1)[-1]), 0.5), f[name][0]))
            o = pose.get(name, {'angleDeg': 0, 'twistDeg': 0})
            joint = {'type': 'revolute', 'minDeg': min(-20, math.floor(o['angleDeg'] - 15)), 'maxDeg': max(20, math.ceil(o['angleDeg'] + 15)), 'twistMinDeg': -math.ceil(abs(o['twistDeg']) + 20), 'twistMaxDeg': math.ceil(abs(o['twistDeg']) + 20)}
            if limits and name == 'coracoid':
                joint = {'type': 'revolute', 'minDeg': limits[0], 'maxDeg': limits[1]}
            bone = {'id': f'{name}-{side}', 'parent': 'trunk' if parent == 'trunk' else f'{parent}-{side}', 'title': {'en': f'{word} {en}' if name != 'coracoid' else f'{word} coracoid, the strut from the sternum to the shoulder joint', 'de': f'{wort}e {de}' if name[0] in 'pst' and name[1:].isdigit() else (f'{wort}es Rabenbein, die Strebe vom Brustbein zum Schultergelenk' if name == 'coracoid' else f'{wort}e{"r" if name in ("humerus", "forearm") else ""} {de}')}, **p, 'lengthM': round(length, 5), 'joint': joint}
            before = old.get(f'{name}-{side}')
            if before and 'mass' in before:
                bone['mass'] = {**before['mass'], 'comX': round(min(before['mass']['comX'], length * 0.5), 4)}
            made.append(bone)
    mine = lambda i: i.split('-')[0] in ('coracoid', 'humerus', 'forearm', 'hand') or i[0] in 'pst' and i[1:-2].isdigit()
    first = next(i for i, b in enumerate(bones) if mine(b['id']))
    bones[:] = [b for b in bones if not mine(b['id'])]
    bones[first:first] = made
    for p in asset['poses']:
        stale = [k for k in p['bones'] if mine(k)]
        if stale and p['id'] != 'wing-stretch':
            print('[DEBUG] pose', p['id'], 'turned wing bones', stale)
        for k in stale:
            del p['bones'][k]
        if p['id'] == 'wing-stretch':
            for name, o in pose.items():
                p['bones'][f'{name}-n'] = {k: v for k, v in o.items()}
                p['bones'][f'{name}-f'] = {'angleDeg': o['angleDeg'], 'abductionDeg': -o['abductionDeg'], 'twistDeg': -o['twistDeg']}
            p['description'] = {'en': 'Standing with both wings raised and three quarters open after bathing: the humerus lifts out of the flank plumage, elbow and wrist open, and the secondaries on the forearm and the primaries on the hand fan out into one wing surface.', 'de': 'Stehend mit beiden angehobenen, zu drei Vierteln geöffneten Flügeln nach dem Baden: der Oberarm hebt sich aus dem Flankengefieder, Ellbogen und Handgelenk öffnen sich, und die Armschwingen am Unterarm und die Handschwingen an der Hand fächern sich zu einer Flügelfläche auf.'}
    for clip in asset['clips']:
        if any(k in json.dumps(clip) for k in ('"humerus-', '"forearm-', '"hand-')):
            print('[DEBUG] clip', clip['id'], 'names wing bones')
    for scale, rep in asset['representations'].items():
        for view, shapes in (rep.get('geometry') or {}).items():
            for shape in shapes:
                stem = shape.get('bone', '').split('-')[0]
                if stem not in ('humerus', 'forearm', 'hand'):
                    continue
                if view == 'elevation':
                    shape_on_trunk(shape, OLD_WING if stem == 'humerus' else OLD_HAND)
                shape['bone'] = 'trunk'
                if 'solid' in shape:
                    shape['solid'].pop('weights', None)
    folded = ('near-wing', 'far-wing', 'speculum', 'near-primaries', 'far-primaries', 'primary-lines', 'far-outer-primary', 'near-outer-primary', 'near-wing-2', 'near-wing-3', 'far-wing-2', 'far-wing-3')
    for scale, rep in asset['representations'].items():
        geometry = rep.get('geometry')
        if not geometry:
            continue
        for view in geometry:
            geometry[view] = [q for q in geometry[view] if q['id'] not in folded and '-vane-' not in q['id']]
        blades = BLADES[scale]
        order = 0
        for tract in TRACTS:
            for k in range(tract['count']):
                if blades and k not in blades[2]:
                    continue
                bone = f'{tract["stem"]}{k + 1}'
                o, fr = f[bone]
                quills = [quill(tract, j) for j in (2 * k, 2 * k + 1)]
                reach = max(dot(sub(q[-1], o), fr[0]) for q in quills)
                half = blades[0] if blades else tract['half'] * 1.45
                for part, (t0, t1) in enumerate(((0, 0.55), (0.5, 1))):
                    ts = [t0 + (t1 - t0) * i / 3 for i in range(4)]
                    width = lambda t: half * max(0.22, min(1, t / 0.2, (1.02 - t) / 0.22))
                    points = [[round(reach * t, 5), round(width(t), 5)] for t in ts] + [[round(reach * t, 5), round(-width(t), 5)] for t in reversed(ts)]
                    ws = [dot(sub(p, o), fr[2]) for q in quills for p in q[1:] if t0 * reach - 0.01 <= dot(sub(p, o), fr[0]) <= t1 * reach + 0.01]
                    lo, hi = min(ws) - 0.003, max(ws) + 0.003
                    if hi - lo < 0.012:
                        lo, hi = (lo + hi) / 2 - 0.006, (lo + hi) / 2 + 0.006
                    for side, sign in (('f', -1), ('n', 1)):
                        z0, z1 = (o[2] + lo, o[2] + hi) if side == 'n' else (-(o[2] + hi), -(o[2] + lo))
                        shape = ({'id': f'{"near" if side == "n" else "far"}-vane-{bone}-{part + 1}', 'bone': f'{bone}-{side}', 'role': 'body', 'z': round((4.55 if side == 'n' else 1.35) + 0.004 * order, 3), 'filled': True, 'tone': tract['tone'] if side == 'n' else round(tract['tone'] - 0.08, 2), 'region': 'wing-plumage', 'kind': 'poly', 'points': points, 'closed': True, 'solid': {'kind': 'extruded', 'sweep': 'w', 'z0': round(z0, 5), 'z1': round(z1, 5)}})
                        geometry['elevation'].append(shape)
                        geometry['front'].append({**shape, 'z': round(1 + 0.004 * order, 3), 'solid': dict(shape['solid'])})
                        geometry['plan'].append({**shape, 'z': round(3.4 + 0.004 * order, 3), 'solid': dict(shape['solid'])})
                    order += 1
    gone = ('-wing', '-speculum', '-primaries', '-humerus', '-forearm-bones', '-manus', '-coracoid', '-wing-down', '-primaries-down', '-upper-wing', '-forearm', '-hand', '-wing-coverts', '-hand-coverts', '-ulna', '-radius', '-carpometacarpus', '-minor-metacarpal', '-alula-digit')
    feather = lambda i: any(k in i for k in ('-primary-', '-secondary-', '-tertial-', '-rachis-')) or i.startswith(('joint-humerus', 'joint-forearm', 'joint-hand'))
    for scale, v in asset['volumes'].items():
        scap = {}
        for name in ('body', 'thermal', 'structure'):
            kept = []
            for s in v[name]:
                if s['id'].startswith('joint-wing') or feather(s['id']) or s['id'].endswith(gone) and ('near-' in s['id'] or 'far-' in s['id']):
                    continue
                if 'scapulars' in s['id'] and 'sections' in s and s['bone'] != 'trunk':
                    s = rebased(s, OLD_WING, 'n' if 'near-' in s['id'] else 'f')
                kept.append(s)
            v[name] = kept
        body, structure = wing_solids(scale, f)
        for side in ('n', 'f'):
            for s in body:
                s = mirrored(s, side)
                v['body'].append(s)
                v['thermal'].append({'id': f'th-{s["id"]}', 'of': s['id'], 'region': 'wing-plumage', 'layer': 'wing-contour'})
            v['structure'] += [mirrored(s, side) for s in structure]
        for s in v['structure']:
            if s['id'].endswith('digit-hallux') and scale in ('100', '200') and s['sections'][0]['rx'] < 0.0015:
                for q in s['sections']:
                    q['rx'], q['ry'] = round(q['rx'] * 1.65, 6), round(q['ry'] * 1.65, 6)
        for s in v['body']:
            if s['id'].endswith('-tarsus') and scale == '200' and min(q['rx'] for q in s['sections'] if q['rx'] > 0.003) < 0.0044:
                for q in s['sections']:
                    q['rx'], q['ry'] = round(q['rx'] * 1.07, 6), round(q['ry'] * 1.07, 6)
            if '-web-' in s['id'] and scale in ('100', '200'):
                spans = [max(p[i] for p in s['vertices']) - min(p[i] for p in s['vertices']) for i in range(3)]
                i = spans.index(min(spans))
                if spans[i] > 0.0055:
                    continue
                mid = (max(p[i] for p in s['vertices']) + min(p[i] for p in s['vertices'])) / 2
                for p in s['vertices']:
                    p[i] = round(mid + (p[i] - mid) * 1.25, 6)
        if scale == '200' and not any(s['id'] == 'near-rib-2' for s in v['structure']):
            v['structure'] += [json.loads(json.dumps(s)) for s in asset['volumes']['100']['structure'] if s['id'] in ('near-rib-2', 'far-rib-2')]
        wing = {'50': ('Wings at 1:50: each wing hangs from the coracoid on humerus (93 mm), forearm (79 mm) and hand (85 mm); ten primaries on the hand and ten secondaries on the forearm sit in pairs on five feather tract bones each, two tertials on a tract bone of the humerus, every feather a vane of its real 1 mm thickness; folded they lie stacked along the flank under the covert shields and the scapulars of the trunk, in the wing stretch they fan into one wing surface.', 'Flügel bei 1:50: jeder Flügel hängt am Rabenbein an Oberarm (93 mm), Unterarm (79 mm) und Hand (85 mm); zehn Handschwingen an der Hand und zehn Armschwingen am Unterarm sitzen paarweise auf je fünf Federflur-Knochen, zwei Schirmfedern auf einem Flurknochen des Oberarms, jede Feder eine Fahne ihrer wirklichen Dicke von 1 mm; angelegt liegen sie gestapelt an der Flanke unter den Deckfederschilden und den Schulterfedern des Rumpfs, beim Flügelstrecken fächern sie sich zu einer Flügelfläche auf.'), '100': ('Wings at 1:100: the same wing bones; each feather pair is merged into one blade per tract bone (34 mm wide), the covert shields stay as thin sheets and forearm and hand are thickened to the 15 mm feature.', 'Flügel bei 1:100: dieselben Flügelknochen; jedes Federpaar ist zu einem Blatt je Flurknochen (34 mm breit) zusammengefasst, die Deckfederschilde bleiben dünne Blätter, Unterarm und Hand sind auf das Merkmal von 15 mm verdickt.'), '200': ('Wings at 1:200: three blades for the primaries, three for the secondaries and one for the tertials (40 mm wide), upper wing, forearm and hand as thin panels in the wing plane, so the wing still opens as a surface.', 'Flügel bei 1:200: drei Blätter für die Handschwingen, drei für die Armschwingen und eines für die Schirmfedern (40 mm breit), Oberflügel, Unterarm und Hand als dünne Flächen in der Flügelebene, damit sich der Flügel weiterhin als Fläche öffnet.'), '500': ('Wings at 1:500: three primary blades, three secondary blades and one tertial blade on the wing bones, no covert shields.', 'Flügel bei 1:500: drei Handschwingenblätter, drei Armschwingenblätter und ein Schirmfederblatt an den Flügelknochen, keine Deckfederschilde.')}[scale]
        girdle = ('Shoulder girdle and wing skeleton: the coracoid is its own bone from the sternum up and out to the shoulder joint (41 mm, on its lateral rest placement), humerus, ulna and radius (one member from 1:100 on), carpometacarpus with the wing digit, and one feather shaft per tract bone.', 'Schultergürtel und Flügelskelett: das Rabenbein ist ein eigener Knochen vom Brustbein nach oben außen zum Schultergelenk (41 mm, seitlich in Ruhelage gesetzt), Humerus, Elle und Speiche (ab 1:100 ein Stab), Carpometacarpus mit dem Flügelfinger und ein Federschaft je Flurknochen.')
        d = v['decisions']
        for entry, (en, de) in ((d[0], wing), (next((x for x in d if 'tructur' in x['en'] and 'Thermal' not in x['en'][:8]), d[-1]), girdle)):
            if en not in entry['en']:
                entry['en'] += ' ' + en
                entry['de'] += ' ' + de
    return doc


doc = build()
if '--write' in sys.argv:
    json.dump({'volumes': doc['asset']['volumes']}, open(SOURCE, 'w'), ensure_ascii=False)
    json.dump(doc, open(CATALOG, 'w'), ensure_ascii=False, indent=2)
    print('written', CATALOG)
else:
    os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
    json.dump(doc, open(CANDIDATE, 'w'), ensure_ascii=False, indent=2)
    print('candidate', CANDIDATE)
