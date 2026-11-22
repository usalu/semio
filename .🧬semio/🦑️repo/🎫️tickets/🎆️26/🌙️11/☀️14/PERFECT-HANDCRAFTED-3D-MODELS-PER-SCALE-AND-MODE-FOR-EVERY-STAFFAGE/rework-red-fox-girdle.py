"""🦊 Red fox: shoulder and pelvic girdle on lateral rest placement, hand-chosen numbers.

Usage: python3 rework-red-fox-girdle.py [--apply]
Without --apply the candidate goes to 🗑️generated/rework-red-fox/red-fox/🔣️.json.
See rig-lateral-rest.md (sections 1.4, 1.5, 1.6).
"""
import glob
import json
import math
import os
import subprocess
import sys

import rework_red_fox_linear as np

T = os.path.dirname(os.path.abspath(__file__))
CATALOG = glob.glob('/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/red-fox/🔣️.json')[0]
SOURCE = os.path.join(T, 'volumes/animals/red-fox.json')
CANDIDATE = os.path.join(T, '🗑️generated/rework-red-fox/red-fox/🔣️.json')

SI_JOINT = (0.040, 0.027, 0.018)
HIP_JOINT = (0.0, 0.0, 0.036)
SCAPULA_TOP = (0.3720, 0.0329, 0.024)
SHOULDER_JOINT = (0.4240, -0.0470, 0.044)


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
        at = np.array([bone.get('x', 0), bone.get('y', 0), bone.get('z', 0)], dtype=float)
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


def unit(v):
    v = np.array(v, dtype=float)
    return v / np.norm(v)


def side(p, s):
    return np.array([p[0], p[1], s * p[2]], dtype=float)


def world_loft(frame, ident, bone, stations, wide, s, exponent=2, extra=None):
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
        at = basis.T @ (points[i] - origin)
        sections.append({'at': [round(v, 5) + 0.0 for v in at], 'rx': rx, 'ry': ry, 'exponent': exponent, 'rotation': quat(local)})
    solid = {'id': ident, 'kind': 'loft', 'bone': bone, 'material': 'bone', 'role': 'body', 'tone': 0.18, 'sections': sections, 'capStart': True, 'capEnd': True, 'member': bone, 'part': 'member'}
    solid.update(extra or {})
    return solid


def local_loft(ident, bone, stations, exponent=2.6):
    sections = [{'at': list(at), 'rx': rx, 'ry': ry, 'exponent': exponent, 'rotation': [0, -0.70711, 0, 0.70711]} for at, rx, ry in stations]
    return {'id': ident, 'kind': 'loft', 'bone': bone, 'material': 'bone', 'role': 'body', 'tone': 0.18, 'sections': sections, 'capStart': True, 'capEnd': True, 'member': bone, 'part': 'member'}


def ball(frame, ident, bone, at_world, radius, part='joint', s=1):
    basis, origin = frame
    at = basis.T @ (side(at_world, s) - origin)
    return {'id': ident, 'kind': 'ellipsoid', 'bone': bone, 'material': 'bone', 'role': 'body', 'tone': 0.5 if part == 'joint' else 0.18, 'radii': list(radius), 'transform': {'translation': [round(v, 5) + 0.0 for v in at], 'rotation': [0, 0, 0, 1]}, 'member': bone, 'part': part}


def rig(asset):
    bones = asset['rig']['bones']
    if any(b['id'] == 'ilium-hn' for b in bones):
        return
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
            a = math.degrees(math.atan2(local[1], local[0]))
            b = math.degrees(math.asin(local[2]))
            at = pb.T @ (start - po)
            name = 'Near' if s < 0 else 'Far'
            nah = 'Nahes' if s < 0 else 'Fernes'
            ilium = {'id': 'ilium-' + tag, 'parent': 'pelvis', 'title': {'en': name + ' hip bone (sacroiliac joint to hip joint)', 'de': nah + ' Hüftbein (Kreuzdarmbeingelenk bis Hüftgelenk)'}, 'x': round(at[0], 5), 'y': round(at[1], 5), 'z': round(at[2], 5), 'angleDeg': round(a, 4), 'abductionDeg': round(b, 4), 'lengthM': round(float(np.norm(end - start)), 5), 'joint': {'type': 'revolute', 'minDeg': -2, 'maxDeg': 2}}
            out.append(ilium)
            ib = pb @ rot(ilium['angleDeg'], ilium['abductionDeg'])
            a, b, t = angles(ib.T @ old[bone['id']][0])
            bone['parent'] = 'ilium-' + tag
            bone['x'], bone['y'] = ilium['lengthM'], 0.0
            bone.pop('z', None)
            place(bone, a, b, t)
        elif bone['id'].startswith('scapula-'):
            pb, po = old['thorax']
            start, end = side(SCAPULA_TOP, s), side(SHOULDER_JOINT, s)
            local = pb.T @ unit(end - start)
            at = pb.T @ (start - po)
            bone['x'], bone['y'], bone['z'] = round(at[0], 5), round(at[1], 5), round(at[2], 5)
            bone['lengthM'] = round(float(np.norm(end - start)), 5)
            bone['mass']['comX'] = round(bone['lengthM'] / 2, 4)
            place(bone, round(math.degrees(math.atan2(local[1], local[0])), 4), round(math.degrees(math.asin(local[2])), 4), 0.0)
        elif bone['id'].startswith('humerus-'):
            scapula = by['scapula-' + tag]
            sb = old['thorax'][0] @ rot(scapula['angleDeg'], scapula.get('abductionDeg', 0))
            a, b, t = angles(sb.T @ old[bone['id']][0])
            bone['x'], bone['y'] = scapula['lengthM'], 0.0
            bone.pop('z', None)
            place(bone, a, b, t)
            repose(asset, tag, old, scapula, bone)
        out.append(bone)
    asset['rig']['bones'] = out
    new = frames(out)
    worst = max(np.gap(new[k][0], old[k][0]) + np.gap(new[k][1], old[k][1]) for k in old if not k.startswith('scapula-'))
    print('[DEBUG] largest rest-frame change of an untouched bone:', round(worst, 6))


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


def repose(asset, tag, old, scapula, humerus):
    """🤸 Keep the posed humerus where the sagittal scapula carried it."""
    thorax = old['thorax'][0]
    old_scapula = thorax.T @ old['scapula-' + tag][0]
    old_humerus = old['scapula-' + tag][0].T @ old['humerus-' + tag][0]
    new_scapula = rot(scapula['angleDeg'], scapula.get('abductionDeg', 0))
    new_humerus = rot(humerus['angleDeg'], humerus.get('abductionDeg', 0), humerus.get('twistDeg', 0))
    for pose in asset['poses']:
        swing = pose['bones'].get('scapula-' + tag)
        if not swing or abs(swing.get('angleDeg', 0)) < 0.05:
            continue
        was = pose['bones'].get('humerus-' + tag, {})
        target = old_scapula @ offset_of(swing) @ old_humerus @ offset_of(was)
        a, b, t = angles((new_scapula @ offset_of(swing) @ new_humerus).T @ target)
        now = {'angleDeg': a}
        if abs(b) > 1e-3:
            now['abductionDeg'] = b
        if abs(t) > 1e-3:
            now['twistDeg'] = t
        pose['bones']['humerus-' + tag] = now
    for clip in asset['clips']:
        for key in clip['keys']:
            touched = set(key.get('bones') or {}) & {'scapula-' + tag, 'humerus-' + tag}
            if touched:
                raise SystemExit('clip key overrides ' + ', '.join(touched) + ': re-solve by hand')


def pelvis_50(f):
    out = []
    for name, tag, s in (('near', 'hn', -1), ('far', 'hf', 1)):
        bone = 'ilium-' + tag
        fr = f[bone]
        out.append(world_loft(fr, name + '-ilium', bone, [((0.060, 0.036, 0.028), 0.0042, 0.013), ((0.052, 0.033, 0.023), 0.0042, 0.015), ((0.040, 0.027, 0.018), 0.0042, 0.013), ((0.024, 0.016, 0.022), 0.0045, 0.009), ((0.008, 0.005, 0.029), 0.006, 0.009)], (0, 0, 1), s, 2.4))
        out.append(ball(fr, name + '-acetabulum', bone, (0.0, 0.0, 0.0305), (0.011, 0.011, 0.0065), 'member', s))
        out.append(world_loft(fr, name + '-ischium', bone, [((-0.006, 0.002, 0.030), 0.005, 0.008), ((-0.022, 0.005, 0.028), 0.0045, 0.006), ((-0.034, 0.003, 0.030), 0.005, 0.008), ((-0.041, -0.002, 0.034), 0.006, 0.010)], (0, 0, 1), s, 2.2))
        out.append(world_loft(fr, name + '-pubis', bone, [((0.002, -0.007, 0.028), 0.0045, 0.0045), ((-0.003, -0.012, 0.016), 0.004, 0.004), ((-0.007, -0.014, 0.005), 0.004, 0.0045)], (1, 0, 0), s))
        out.append(world_loft(fr, name + '-ischial-arch', bone, [((-0.030, -0.012, 0.005), 0.004, 0.0045), ((-0.036, -0.009, 0.016), 0.004, 0.004), ((-0.040, -0.005, 0.028), 0.0045, 0.0045)], (0, 1, 0), s))
        out.append(ball(fr, 'joint-' + bone, bone, SI_JOINT, (0.005, 0.005, 0.005), 'joint', s))
    out.insert(0, world_loft(f['pelvis'], 'pelvic-symphysis', 'pelvis', [((-0.004, -0.014, 0), 0.008, 0.004), ((-0.018, -0.0135, 0), 0.007, 0.004), ((-0.033, -0.012, 0), 0.008, 0.004)], (0, 0, 1), 1))
    return out


def scapula_50():
    out = []
    for name, tag, lateral in (('near', 'fn', -1), ('far', 'ff', 1)):
        bone = 'scapula-' + tag
        out.append(local_loft(name + '-scapula', bone, [((-0.004, 0, 0), 0.0042, 0.019), ((0.010, -0.001, 0), 0.0042, 0.025), ((0.045, -0.001, 0), 0.0042, 0.018), ((0.074, 0, 0), 0.005, 0.0085), ((0.088, 0, 0), 0.0055, 0.008), ((0.0974, 0.001, 0), 0.0075, 0.0105)]))
        out.append(local_loft(name + '-scapular-spine', bone, [((0.004, 0.002, lateral * 0.0045), 0.0045, 0.0038), ((0.045, 0.002, lateral * 0.0065), 0.006, 0.0038), ((0.078, 0.003, lateral * 0.0085), 0.0065, 0.0038), ((0.092, 0.006, lateral * 0.0095), 0.0045, 0.0045)], 2))
    return out


def coarse(f, scale):
    out = []
    if scale == '100':
        out.append(world_loft(f['pelvis'], 'sacrum', 'pelvis', [((0.050, 0.025, 0), 0.018, 0.0075), ((0.022, 0.033, 0), 0.013, 0.0075), ((-0.006, 0.040, 0), 0.0085, 0.0075)], (0, 0, 1), 1))
        for name, tag, s in (('near', 'hn', -1), ('far', 'hf', 1)):
            out.append(world_loft(f['ilium-' + tag], name + '-hip-bone', 'ilium-' + tag, [((0.058, 0.035, 0.026), 0.0075, 0.013), ((0.036, 0.024, 0.023), 0.0075, 0.011), ((0.0, 0.0, 0.029), 0.0075, 0.011), ((-0.040, -0.002, 0.031), 0.0075, 0.010)], (0, 0, 1), s, 2.4))
    return out


def structure(asset):
    f = frames(asset['rig']['bones'])
    for scale, volume in asset['volumes'].items():
        members = volume['structure']
        fresh = pelvis_50(f) + scapula_50() if scale == '50' else coarse(f, scale)
        ids = {s['id'] for s in fresh} | {'pubis', 'near-ilium', 'far-ilium', 'near-ischium', 'far-ischium'}
        if scale == '100':
            ids.add('pelvis-girdle')
        kept = [s for s in members if s['id'] not in ids]
        cut = next(i for i, s in enumerate(kept) if s.get('part') != 'member')
        volume['structure'] = kept[:cut] + [s for s in fresh if s['part'] == 'member'] + kept[cut:] + [s for s in fresh if s['part'] != 'member']
        for solid in volume['structure']:
            if scale != '50' and solid['id'] in ('near-scapula', 'far-scapula'):
                for section in solid['sections']:
                    section['at'][2] = 0
                    if section['at'][0] > 0.09:
                        section['at'][0] = 0.0974
            if scale in ('200', '500') and solid['id'] == 'pelvis-girdle':
                solid['sections'][0]['at'] = [0.0671, 0.0045, 0]
                solid['members'] = ['ilium-hn', 'ilium-hf']


class insert:
    """🧵 The catalog formatter of volumes-insert.py (its functions above `member_end`, without running its main)."""

    def __init__(self):
        text = open(os.path.join(T, 'volumes-insert.py'), encoding='utf-8').read()
        scope = {}
        exec(text[:text.index('def member_end')], scope)
        self.dump = scope['dump']


def splice(text, key, value):
    """✂️ Replaces one member of the asset in the catalog text; every other byte stays."""
    start = text.index('\n    "' + key + '": ') + len('\n    "' + key + '": ')
    _, end = json.JSONDecoder().raw_decode(text, start)
    body = json.dumps(value, ensure_ascii=False, indent=2).replace('\n', '\n    ')
    return text[:start] + body + text[end:]


def main():
    document = json.load(open(CATALOG, encoding='utf-8'))
    asset = document['asset']
    source = json.load(open(SOURCE, encoding='utf-8'))
    asset['volumes'] = source['volumes']
    rig(asset)
    structure(asset)
    if '--apply' in sys.argv:
        json.dump({'volumes': asset['volumes']}, open(SOURCE, 'w', encoding='utf-8'), ensure_ascii=False, separators=(',', ':'))
        text = open(CATALOG, encoding='utf-8').read()
        for key in ('rig', 'poses'):
            text = splice(text, key, asset[key])
        open(CATALOG, 'w', encoding='utf-8').write(text)
        subprocess.run([sys.executable, os.path.join(T, 'volumes-insert.py'), SOURCE, CATALOG], check=True)
        print('applied', CATALOG)
    else:
        os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
        open(CANDIDATE, 'w', encoding='utf-8').write(insert().dump(document, 0) + '\n')
        print(CANDIDATE)


if __name__ == '__main__':
    main()
