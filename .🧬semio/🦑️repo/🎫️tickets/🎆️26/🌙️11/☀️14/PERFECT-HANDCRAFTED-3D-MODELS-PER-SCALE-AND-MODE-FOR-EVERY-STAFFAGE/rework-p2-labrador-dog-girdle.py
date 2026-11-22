"""🐕 Labrador: shoulder and pelvic girdle on lateral rest placement, hand-chosen numbers.

Usage: python3 rework-p2-labrador-dog-girdle.py [--apply]
Without --apply the candidate goes to 🗑️generated/rework-labrador-dog/labrador-dog/🔣️.json.
See rig-lateral-rest.md (sections 1.4, 1.5, 1.6); frame helpers as in rework-red-fox-girdle.py.
World points below are rest coordinates with the hip joint axis at the origin: [ahead, up, left].
"""
import glob
import json
import math
import os
import subprocess
import sys

import rework_red_fox_linear as np

T = os.path.dirname(os.path.abspath(__file__))
CATALOG = glob.glob('/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/labrador-dog/🔣️.json')[0]
SOURCE = os.path.join(T, 'volumes/animals/labrador-dog.json')
CANDIDATE = os.path.join(T, '🗑️generated/rework-labrador-dog/labrador-dog/🔣️.json')

SI_JOINT = (0.038, 0.028, 0.026)
HIP_JOINT = (0.0, 0.0, 0.055)
LIMB = [0.5, 0.5, 0.5, 0.5]


def rot(a=0.0, b=0.0, t=0.0):
    a, b, t = (math.radians(v) for v in (a, b, t))
    rz = np.array([[math.cos(a), -math.sin(a), 0], [math.sin(a), math.cos(a), 0], [0, 0, 1]])
    ry = np.array([[math.cos(b), 0, -math.sin(b)], [0, 1, 0], [math.sin(b), 0, math.cos(b)]])
    rx = np.array([[1, 0, 0], [0, math.cos(t), math.sin(t)], [0, -math.sin(t), math.cos(t)]])
    return rz @ ry @ rx


def angles(m):
    b = math.asin(max(-1.0, min(1.0, m[2, 0])))
    a = math.atan2(m[1, 0], m[0, 0])
    rest = rot(math.degrees(a), math.degrees(b)).T @ m
    t = math.atan2(rest[1, 2], rest[1, 1])
    return tuple(round(math.degrees(v), 4) + 0.0 for v in (a, b, t))


def offset_of(o):
    return rot(o.get('angleDeg', 0), o.get('abductionDeg', 0), o.get('twistDeg', 0))


def frames(bones):
    out = {}
    for bone in bones:
        local = rot(bone['angleDeg'], bone.get('abductionDeg', 0), bone.get('twistDeg', 0))
        at = np.array([bone.get('x', 0), bone.get('y', 0), bone.get('z', 0)])
        if bone.get('parent') is None:
            out[bone['id']] = (local, at)
        else:
            pb, po = out[bone['parent']]
            out[bone['id']] = (pb @ local, po + pb @ at)
    return out


def quat(m):
    tr = m[0, 0] + m[1, 1] + m[2, 2]
    if tr > 0:
        s = math.sqrt(tr + 1) * 2
        q = [(m[2, 1] - m[1, 2]) / s, (m[0, 2] - m[2, 0]) / s, (m[1, 0] - m[0, 1]) / s, s / 4]
    elif m[0, 0] > m[1, 1] and m[0, 0] > m[2, 2]:
        s = math.sqrt(1 + m[0, 0] - m[1, 1] - m[2, 2]) * 2
        q = [s / 4, (m[0, 1] + m[1, 0]) / s, (m[0, 2] + m[2, 0]) / s, (m[2, 1] - m[1, 2]) / s]
    elif m[1, 1] > m[2, 2]:
        s = math.sqrt(1 + m[1, 1] - m[0, 0] - m[2, 2]) * 2
        q = [(m[0, 1] + m[1, 0]) / s, s / 4, (m[1, 2] + m[2, 1]) / s, (m[0, 2] - m[2, 0]) / s]
    else:
        s = math.sqrt(1 + m[2, 2] - m[0, 0] - m[1, 1]) * 2
        q = [(m[0, 2] + m[2, 0]) / s, (m[1, 2] + m[2, 1]) / s, s / 4, (m[1, 0] - m[0, 1]) / s]
    return [round(v, 5) + 0.0 for v in q]


def matrix(q):
    x, y, z, w = q
    n = math.sqrt(x * x + y * y + z * z + w * w)
    x, y, z, w = x / n, y / n, z / n, w / n
    return np.array([[1 - 2 * (y * y + z * z), 2 * (x * y - z * w), 2 * (x * z + y * w)], [2 * (x * y + z * w), 1 - 2 * (x * x + z * z), 2 * (y * z - x * w)], [2 * (x * z - y * w), 2 * (y * z + x * w), 1 - 2 * (x * x + y * y)]])


def unit(v):
    v = np.array(v)
    return v / np.norm(v)


def side(p, s):
    return np.array([p[0], p[1], s * p[2]])


def rounded(v, digits=5):
    return [round(c, digits) + 0.0 for c in v]


def world_loft(frame, ident, bone, stations, wide, s, exponent=2.4, extra=None):
    """🦴 Loft through world stations (point, rx along `wide`, ry across) on the frame of `bone`."""
    basis, origin = frame
    points = [side(p, s) for p, _, _ in stations]
    wide = side(wide, s)
    sections = []
    for i, (_, rx, ry) in enumerate(stations):
        ahead = unit(points[min(i + 1, len(points) - 1)] - points[max(i - 1, 0)])
        ey = unit(np.cross(ahead, wide))
        ex = np.cross(ey, ahead)
        local = basis.T @ np.column_stack([ex, ey, ahead])
        sections.append({'at': rounded(basis.T @ (points[i] - origin)), 'rx': rx, 'ry': ry, 'exponent': exponent, 'rotation': quat(local)})
    solid = {'id': ident, 'kind': 'loft', 'bone': bone, 'material': 'bone', 'role': 'body', 'tone': 0.9, 'sections': sections, 'capStart': True, 'capEnd': True, 'member': bone, 'part': 'member'}
    solid.update(extra or {})
    return solid


def local_loft(ident, bone, stations, exponent=2.4):
    sections = [{'at': list(at), 'rx': rx, 'ry': ry, 'exponent': exponent, 'rotation': [-0.5, 0.5, -0.5, 0.5]} for at, rx, ry in stations]
    return {'id': ident, 'kind': 'loft', 'bone': bone, 'material': 'bone', 'role': 'body', 'tone': 0.9, 'sections': sections, 'capStart': True, 'capEnd': True, 'member': bone, 'part': 'member'}


def ball(frame, ident, bone, at_world, radii, part, s):
    basis, origin = frame
    return {'id': ident, 'kind': 'ellipsoid', 'bone': bone, 'material': 'bone', 'role': 'body', 'tone': 0.55 if part == 'joint' else 0.9, 'radii': list(radii), 'transform': {'translation': rounded(basis.T @ (side(at_world, s) - origin)), 'rotation': quat(basis.T)}, 'member': bone, 'part': part}


def place(bone, a, b, t):
    rest = {}
    for key, value in bone.items():
        if key in ('abductionDeg', 'twistDeg'):
            continue
        rest[key] = value
        if key == 'angleDeg':
            rest['angleDeg'] = a
            if abs(b) > 1e-4:
                rest['abductionDeg'] = b
            if abs(t) > 1e-4:
                rest['twistDeg'] = t
    bone.clear()
    bone.update(rest)


def rebind(asset, bone, turn):
    """🔁 Re-expresses every solid of `bone` in its new frame so that nothing moves (`turn` = new basis transposed times old basis)."""
    count = 0
    for volume in asset['volumes'].values():
        for name in ('body', 'thermal', 'structure'):
            for solid in volume.get(name, []):
                if solid.get('bone') != bone:
                    continue
                count += 1
                if solid['kind'] == 'loft':
                    for section in solid['sections']:
                        section['at'] = rounded(turn @ np.array(section['at']), 6)
                        section['rotation'] = quat(turn @ matrix(section.get('rotation', [0, 0, 0, 1])))
                elif solid['kind'] == 'capsule':
                    solid['p0'] = rounded(turn @ np.array(solid['p0']), 6)
                    solid['p1'] = rounded(turn @ np.array(solid['p1']), 6)
                elif solid['kind'] == 'ellipsoid':
                    tr = solid.setdefault('transform', {})
                    tr['translation'] = rounded(turn @ np.array(tr.get('translation', [0, 0, 0])), 6)
                    tr['rotation'] = quat(turn @ matrix(tr.get('rotation', [0, 0, 0, 1])))
                else:
                    raise SystemExit('rebind: unhandled kind ' + solid['kind'] + ' of ' + solid['id'])
    return count


def repose(asset, tag, old_scapula, old_humerus, new_scapula, new_humerus):
    """🤸 Keeps the posed humerus in the direction the sagittal scapula carried it."""
    def solve(swing, was):
        target = old_scapula @ offset_of(swing) @ old_humerus @ offset_of(was)
        a, b, t = angles((new_scapula @ offset_of(swing) @ new_humerus).T @ target)
        now = {'angleDeg': a}
        if abs(b) > 1e-3:
            now['abductionDeg'] = b
        if abs(t) > 1e-3:
            now['twistDeg'] = t
        return now
    poses = {p['id']: p for p in asset['poses']}
    before = {p['id']: json.loads(json.dumps(p['bones'])) for p in asset['poses']}
    for pose in asset['poses']:
        swing = pose['bones'].get('scapula-' + tag)
        if swing and abs(swing.get('angleDeg', 0)) >= 0.05:
            pose['bones']['humerus-' + tag] = solve(swing, before[pose['id']].get('humerus-' + tag, {}))
    for clip in asset['clips']:
        for key in clip['keys']:
            over = key.get('bones') or {}
            if not {'scapula-' + tag, 'humerus-' + tag} & set(over):
                continue
            base = before.get(key.get('pose'), {})
            swing = over.get('scapula-' + tag, base.get('scapula-' + tag, {}))
            over['humerus-' + tag] = solve(swing, over.get('humerus-' + tag, base.get('humerus-' + tag, {})))
    return poses


def rig(asset):
    bones = asset['rig']['bones']
    if any(b['id'] == 'ilium-hn' for b in bones):
        raise SystemExit('girdles are already on the lateral rest placement')
    old = frames(bones)
    by = {b['id']: b for b in bones}
    out = []
    for bone in bones:
        tag = bone['id'][-2:]
        s = -1 if tag.endswith('n') else 1
        if bone['id'].startswith('thigh-'):
            pb, po = old['pelvis']
            start, end = side(SI_JOINT, s), side(HIP_JOINT, s)
            local = pb.T @ unit(end - start)
            at = pb.T @ (start - po)
            near = s < 0
            ilium = {'id': 'ilium-' + tag, 'parent': 'pelvis', 'title': {'en': ('Near' if near else 'Far') + ' hip bone (sacroiliac joint to hip joint)', 'de': ('Nahes' if near else 'Fernes') + ' Hüftbein (Kreuzdarmbeingelenk bis Hüftgelenk)'}, 'x': round(at[0], 5), 'y': round(at[1], 5), 'z': round(at[2], 5), 'angleDeg': round(math.degrees(math.atan2(local[1], local[0])), 4), 'abductionDeg': round(math.degrees(math.asin(local[2])), 4), 'lengthM': round(float(np.norm(end - start)), 5), 'joint': {'type': 'revolute', 'minDeg': -2, 'maxDeg': 2}}
            out.append(ilium)
            ib = pb @ rot(ilium['angleDeg'], ilium['abductionDeg'])
            a, b, t = angles(ib.T @ old[bone['id']][0])
            bone['parent'] = 'ilium-' + tag
            bone['x'], bone['y'] = ilium['lengthM'], 0.0
            bone.pop('z', None)
            place(bone, a, b, t)
        elif bone['id'].startswith('scapula-'):
            humerus = by['humerus-' + tag]
            tip = (humerus['x'], humerus.get('y', 0), humerus.get('z', 0))
            length = round(float(np.norm(np.array(tip))), 5)
            lean = round(math.degrees(math.asin(tip[2] / length)), 4)
            old_scapula = old['thorax'][0].T @ old[bone['id']][0]
            old_humerus = old[bone['id']][0].T @ old[humerus['id']][0]
            new_scapula = rot(bone['angleDeg'], lean)
            bone['mass']['comX'] = round(bone['mass']['comX'] * length / bone['lengthM'], 4)
            stretch = length / bone['lengthM']
            bone['lengthM'] = length
            place(bone, bone['angleDeg'], lean, 0.0)
            a, b, t = angles((old['thorax'][0] @ new_scapula).T @ old[humerus['id']][0])
            humerus['x'], humerus['y'] = length, 0.0
            humerus.pop('z', None)
            place(humerus, a, b, t)
            new_humerus = rot(a, b, t)
            if '--repose' in sys.argv:
                repose(asset, tag, old_scapula, old_humerus, new_scapula, new_humerus)
            print('[DEBUG] scapula-' + tag, 'length', length, 'lean', lean, 'solids rebound', rebind(asset, bone['id'], new_scapula.T @ old_scapula))
            for rep in asset['representations'].values():
                for shape in (rep.get('geometry') or {}).get('elevation', []):
                    if shape.get('bone') == bone['id']:
                        if 'points' not in shape:
                            raise SystemExit('elevation shape without points on ' + bone['id'] + ': ' + shape['id'])
                        shape['points'] = [[round(p[0] * stretch, 6), p[1]] for p in shape['points']]
        out.append(bone)
    asset['rig']['bones'] = out
    new = frames(out)
    worst = max(np.gap(new[k][0], old[k][0]) + np.gap(new[k][1], old[k][1]) for k in old if not k.startswith('scapula-'))
    print('[DEBUG] largest rest-frame change of an untouched bone:', round(worst, 6))


def pelvis_50(f):
    out = [world_loft(f['pelvis'], 'pelvic-symphysis', 'pelvis', [((-0.008, -0.029, 0), 0.010, 0.005), ((-0.034, -0.029, 0), 0.009, 0.005), ((-0.060, -0.027, 0), 0.010, 0.005)], (0, 0, 1), 1)]
    joints = []
    for name, tag, s in (('n', 'hn', -1), ('f', 'hf', 1)):
        bone = 'ilium-' + tag
        fr = f[bone]
        out.append(world_loft(fr, 'ilium-' + name, bone, [((0.092, 0.047, 0.048), 0.0045, 0.019), ((0.076, 0.043, 0.041), 0.0045, 0.022), ((0.055, 0.036, 0.033), 0.0048, 0.021), ((0.038, 0.028, 0.028), 0.005, 0.017), ((0.022, 0.018, 0.033), 0.006, 0.013), ((0.008, 0.006, 0.040), 0.008, 0.012)], (0, 0, 1), s))
        out.append(ball(fr, 'acetabulum-' + name, bone, (0.0, 0.0, 0.043), (0.015, 0.015, 0.008), 'member', s))
        out.append(world_loft(fr, 'ischium-' + name, bone, [((-0.010, -0.003, 0.042), 0.007, 0.011), ((-0.030, -0.010, 0.040), 0.006, 0.009), ((-0.052, -0.015, 0.042), 0.006, 0.010), ((-0.070, -0.018, 0.047), 0.007, 0.012), ((-0.084, -0.020, 0.053), 0.009, 0.014)], (0, 0, 1), s))
        out.append(world_loft(fr, 'pubis-' + name, bone, [((0.004, -0.010, 0.038), 0.006, 0.006), ((-0.004, -0.021, 0.022), 0.005, 0.005), ((-0.012, -0.028, 0.006), 0.005, 0.006)], (1, 0, 0), s, 2))
        out.append(world_loft(fr, 'ischial-arch-' + name, bone, [((-0.058, -0.026, 0.005), 0.005, 0.006), ((-0.066, -0.023, 0.022), 0.005, 0.005), ((-0.074, -0.019, 0.040), 0.006, 0.006)], (0, 1, 0), s, 2))
        joints.append(ball(fr, 'joint-' + bone, bone, SI_JOINT, (0.006, 0.006, 0.006), 'joint', s))
    return out, joints


def pelvis_100(f):
    out = [world_loft(f['pelvis'], 'sacrum', 'pelvis', [((0.060, 0.025, 0), 0.027, 0.019), ((0.016, 0.030, 0), 0.018, 0.0145), ((-0.066, 0.037, 0), 0.009, 0.009)], (0, 0, 1), 1),
           world_loft(f['pelvis'], 'pelvic-floor', 'pelvis', [((-0.032, -0.022, -0.042), 0.0078, 0.012), ((-0.032, -0.026, 0), 0.0078, 0.014), ((-0.032, -0.022, 0.042), 0.0078, 0.012)], (0, 1, 0), 1, 2)]
    for name, tag, s in (('n', 'hn', -1), ('f', 'hf', 1)):
        out.append(world_loft(f['ilium-' + tag], 'hip-bone-' + name, 'ilium-' + tag, [((0.090, 0.046, 0.046), 0.0078, 0.019), ((0.040, 0.029, 0.028), 0.0078, 0.018), ((0.0, 0.0, 0.041), 0.009, 0.014), ((-0.082, -0.019, 0.051), 0.0085, 0.013)], (0, 0, 1), s))
    return out, []


def scapula(scale):
    out = []
    for name, tag, s in (('n', 'fn', -1), ('f', 'ff', 1)):
        bone = 'scapula-' + tag
        if scale == '50':
            out.append(local_loft('scapula-' + name, bone, [((0.006, -0.002, 0), 0.033, 0.0045), ((0.022, -0.002, 0), 0.038, 0.0045), ((0.042, -0.002, 0), 0.040, 0.0045), ((0.068, -0.001, 0), 0.034, 0.0046), ((0.094, -0.001, 0), 0.026, 0.005), ((0.118, 0, 0), 0.019, 0.0062), ((0.139, 0, 0), 0.014, 0.008), ((0.153, 0, 0), 0.015, 0.0105), ((0.162, 0, 0), 0.017, 0.013)]))
            out.append(local_loft('scapular-spine-' + name, bone, [((0.012, 0.004, s * 0.006), 0.004, 0.0045), ((0.060, 0.004, s * 0.008), 0.004, 0.0065), ((0.115, 0.004, s * 0.010), 0.004, 0.008), ((0.143, 0.007, s * 0.011), 0.005, 0.006)], 2))
        elif scale == '100':
            out.append(local_loft('scapula-' + name, bone, [((0.010, -0.002, 0), 0.032, 0.0077), ((0.045, -0.002, 0), 0.038, 0.0077), ((0.092, -0.001, 0), 0.027, 0.0077), ((0.160, 0, 0), 0.016, 0.012)]))
        elif scale == '200':
            out.append({'id': 'scapula-' + name, 'kind': 'capsule', 'bone': bone, 'material': 'bone', 'role': 'body', 'tone': 0.9, 'p0': [0.0208, 0, 0], 'p1': [0.163, 0, 0], 'r0': 0.0152, 'r1': 0.0152, 'member': bone, 'part': 'member'})
        else:
            out.append(local_loft('scapula-' + name, bone, [((0.008, 0, 0), 0.0376, 0.0376), ((0.090, 0, 0), 0.0376, 0.0376), ((0.161, 0, 0), 0.0376, 0.0376)], 2))
    return out


def structure(asset):
    f = frames(asset['rig']['bones'])
    for scale, volume in asset['volumes'].items():
        fresh, joints = pelvis_50(f) if scale == '50' else pelvis_100(f) if scale == '100' else ([], [])
        fresh += scapula(scale)
        gone = {s['id'] for s in fresh} | ({'hip-bone-n', 'hip-bone-f'} if scale == '50' else {'pelvis-girdle'} if scale == '100' else set())
        kept = [s for s in volume['structure'] if s['id'] not in gone]
        cut = next(i for i, s in enumerate(kept) if s.get('part') != 'member')
        volume['structure'] = kept[:cut] + fresh + kept[cut:] + joints
        if scale == '500':
            volume['structure'] = [s for s in volume['structure'] if s['id'] not in ('rib-3', 'sternum')]
        if scale in ('200', '500'):
            next(s for s in volume['structure'] if s['id'] == 'pelvis-girdle')['members'] = ['ilium-hn', 'ilium-hf']


class insert:
    """🧵 The catalog formatter of volumes-insert.py (its functions above `member_end`, without running its main)."""

    def __init__(self):
        text = open(os.path.join(T, 'volumes-insert.py'), encoding='utf-8').read()
        scope = {}
        exec(text[:text.index('def member_end')], scope)
        self.dump = scope['dump']


def splice(text, key, value, dump):
    """✂️ Replaces one member of the asset in the catalog text; every other byte stays."""
    start = text.index('\n    "' + key + '": ') + len('\n    "' + key + '": ')
    _, end = json.JSONDecoder().raw_decode(text, start)
    return text[:start] + dump(value, 2) + text[end:]


def main():
    document = json.load(open(CATALOG, encoding='utf-8'))
    asset = document['asset']
    source = json.load(open(SOURCE, encoding='utf-8'))
    asset['volumes'] = source['volumes']
    rig(asset)
    structure(asset)
    next(c for c in asset['clips'] if c['id'] == 'walk-cycle')['advanceM'] = 0.5212
    dump = insert().dump
    if '--apply' in sys.argv:
        json.dump({'volumes': asset['volumes']}, open(SOURCE, 'w', encoding='utf-8'), ensure_ascii=False, separators=(',', ':'))
        text = open(CATALOG, encoding='utf-8').read()
        for key in ('rig', 'representations', 'poses', 'clips'):
            text = splice(text, key, asset[key], dump)
        open(CATALOG, 'w', encoding='utf-8').write(text)
        subprocess.run([sys.executable, os.path.join(T, 'volumes-insert.py'), SOURCE, CATALOG], check=True)
        print('applied', CATALOG)
    else:
        os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
        open(CANDIDATE, 'w', encoding='utf-8').write(dump(document, 0) + '\n')
        print(CANDIDATE)


if __name__ == '__main__':
    main()
