"""🕊️ Pigeon wing rebuilt as feather tracts: ten primaries on the hand and ten secondaries on the forearm, each on its own bone, stacked when the wing is folded and fanned in every opened pose; the hand bone gets its skeletal length (rework author 08).
Usage: python3 rework-08-pigeon-wing.py [--candidate|--write] [TWIST]"""
import json, math, sys
sys.path.insert(0, '.')
from rework_08_root import splice

SOURCE = 'volumes/animals/pigeon.json'
HAND = 0.058
TWIST = float(next((a for a in sys.argv[1:] if not a.startswith('--')), 0))
PRIMARY = {'base': [0.003 + 0.006 * i for i in range(10)], 'length': [0.105, 0.11, 0.117, 0.124, 0.131, 0.138, 0.144, 0.148, 0.15, 0.142], 'rest': [0] * 10, 'spread': [-68, -60, -52, -44, -36, -28, -21, -14, -8, -2], 'width': 0.011}
SECONDARY = {'base': [0.05 - 0.0051 * i for i in range(10)], 'length': [0.1, 0.099, 0.097, 0.095, 0.092, 0.089, 0.086, 0.083, 0.079, 0.075], 'rest': [round(-158 + 1.33 * i, 2) for i in range(10)], 'spread': [-84 - 1.8 * i for i in range(10)], 'width': 0.013}
OPEN = {'wings-half-open': 0.5, 'takeoff-crouch': 0.3, 'takeoff-wings-up': 1, 'takeoff-push': 1, 'takeoff-liftoff': 0.9, 'airborne-wings-up': 1, 'airborne-wings-down': 1, 'airborne-climb': 1}
SIDES = {'n': ('near', -1, {'en': 'near wing', 'de': 'naher Flügel'}), 'f': ('far', 1, {'en': 'far wing', 'de': 'ferner Flügel'})}
STATIONS = [(0, 0.3), (0.12, 0.45), (0.3, 0.9), (0.55, 1), (0.8, 0.9), (0.93, 0.62), (1, 0.36)]
THICK = 0.0038
WEB = [(-0.004, 0.004, 0.003), (0.006, 0.0095, 0.0042), (0.03, 0.0105, 0.0044), (0.052, 0.008, 0.004), (0.062, 0.003, 0.0038)]
FOLD = -10
COVERT_END = -0.014
SHOULDER_END = 0.05
import os
TRIM = os.environ.get('TRIM', '50,100,200,500').split(',')
FLAT = [0.70710678, 0, -0.70710678, 0]


def feathers():
    """🪶 Every remex as (bone id, parent stem, tract, index, attach x, length, rest angle, spread angle, half width)."""
    for stem, parent, tract in (('p', 'hand', PRIMARY), ('s', 'forearm', SECONDARY)):
        for i in range(10):
            yield f'{stem}{i + 1}', parent, stem, i, tract['base'][i], tract['length'][i], tract['rest'][i], tract['spread'][i], tract['width']


def change(asset):
    """🦴 Rig bones and pose angles of the feather tracts."""
    bones = asset['rig']['bones']
    bones[:] = [b for b in bones if not (b['id'][0] in 'ps' and b['id'][1:-2].isdigit())]
    for side, (word, sign, text) in SIDES.items():
        hand = next(b for b in bones if b['id'] == f'hand-{side}')
        hand['lengthM'] = HAND
        hand['mass']['comX'] = 0.029
        hand['title'] = {'en': f'{word.capitalize()} hand (carpometacarpus and digits)', 'de': f'{"Nahe" if side == "n" else "Ferne"} Hand (Handwurzel-Mittelhand und Finger)'}
        for bone, parent, stem, i, base, length, rest, spread, width in feathers():
            name = ('Primary feather P' if stem == 'p' else 'Secondary feather S') + str(i + 1)
            german = ('Handschwinge P' if stem == 'p' else 'Armschwinge S') + str(i + 1)
            bones.append({'id': f'{bone}-{side}', 'parent': f'{parent}-{side}', 'title': {'en': f'{name} ({text["en"]})', 'de': f'{german} ({text["de"]})'}, 'x': round(base, 4), 'y': 0, 'z': round(sign * ((0.0165 if stem == 'p' else 0.008) + 0.0003 * i), 5), 'angleDeg': round(rest, 2), 'abductionDeg': FOLD * sign * (1 if stem == 'p' else 0.5), 'lengthM': length, 'joint': {'type': 'revolute', 'minDeg': -120, 'maxDeg': 120, 'twistMinDeg': -100, 'twistMaxDeg': 100}})
    for pose in asset['poses']:
        for key in [k for k in pose['bones'] if k[0] in 'ps' and k[1:-2].isdigit()]:
            del pose['bones'][key]
        share = OPEN.get(pose['id'])
        if share is None:
            continue
        for side in SIDES:
            if TWIST:
                pose['bones'][f'humerus-{side}']['twistDeg'] = TWIST * share * (1 if side == 'n' else -1)
            for bone, parent, stem, i, base, length, rest, spread, width in feathers():
                pose['bones'][f'{bone}-{side}'] = {'angleDeg': round((spread - rest) * share, 2), 'abductionDeg': round(-FOLD * (1 if side == 'f' else -1) * (1 if stem == 'p' else 0.5) * min(1, share * 2), 2)}
    return {'rig', 'poses'}


def onto_trunk(section, sign):
    """🔁 A humerus-local loft section restated in the trunk frame of the rest pose (humerus joint at 0.04698, 0.00848, lateral 0.022, turned 176 degrees)."""
    a = math.radians(176)
    x, y, z = section['at']
    w0, x0, y0, z0 = section['rotation']
    c, t = math.cos(a / 2), math.sin(a / 2)
    turned = [c * w0 - t * z0, c * x0 - t * y0, c * y0 + t * x0, c * z0 + t * w0]
    return {**section, 'at': [round(0.04698 + math.cos(a) * x - math.sin(a) * y, 5), round(0.00848 + math.sin(a) * x + math.cos(a) * y, 5), round(z + sign * 0.022, 5)], 'rotation': [round(v, 5) for v in turned]}


def solids():
    """🧊 Feather lofts at every scale in place of the single primaries blade, with their thermal references."""
    doc = json.load(open(SOURCE))
    original = json.load(open('rework-08-pigeon-coverts-original.json'))['volumes']
    for scale, v in doc['volumes'].items():
        for shield in (s for s in v['body'] if s['id'].endswith('-coverts')):
            shield['sections'] = next(o for o in original[scale]['body'] if o['id'] == shield['id'])['sections']
        gone = lambda s: s['id'].endswith(('-primaries', '-hand')) or '-primary-' in s['id'] and 'shaft' not in s['id'] or '-secondary-' in s['id']
        v['body'] = [s for s in v['body'] if not gone(s)]
        v['thermal'] = [s for s in v['thermal'] if not gone(s)]
        v['structure'] = [s for s in v['structure'] if not s['id'].endswith('-rachis')]
        for shield in (s for s in v['body'] if s['id'].endswith('-coverts') and scale in TRIM):
            order = sorted(shield['sections'], key=lambda q: -q['at'][0])
            keep = [q for q in order if q['at'][0] >= COVERT_END]
            if len(keep) < len(order):
                last = keep[-1]
                keep.append({**last, 'at': [round(last['at'][0] - 0.008, 4), last['at'][1], last['at'][2]], 'rx': round(last['rx'] * 0.35, 5), 'ry': round(last['ry'] * 0.35, 5)})
            shield['sections'] = keep
        v['body'] = [s for s in v['body'] if not s['id'].endswith('-scapulars')]
        v['thermal'] = [s for s in v['thermal'] if not s['id'].endswith('-scapulars')]
        for side, (word, sign, text) in SIDES.items():
            lobe = next((s for s in v['body'] if s['id'] == f'{word}-shoulder'), None)
            if lobe is not None:
                whole = sorted(next(o for o in original['50']['body'] if o['id'] == lobe['id'])['sections'], key=lambda q: q['at'][0])
                front = [q for q in whole if q['at'][0] <= SHOULDER_END]
                last = front[-1]
                front.append({**last, 'at': [round(last['at'][0] + 0.008, 4), last['at'][1], last['at'][2]], 'rx': round(last['rx'] * 0.4, 5), 'ry': round(last['ry'] * 0.4, 5)})
                lobe['sections'] = front
                rear = [q for q in whole if q['at'][0] >= SHOULDER_END - 0.016]
                first = rear[0]
                rear.insert(0, {**first, 'at': [round(first['at'][0] - 0.008, 4), first['at'][1], first['at'][2]], 'rx': round(first['rx'] * 0.4, 5), 'ry': round(first['ry'] * 0.4, 5)})
                v['body'].append({**{k: x for k, x in lobe.items() if k != 'sections'}, 'id': f'{word}-scapulars', 'bone': 'trunk', 'sections': [onto_trunk(q, sign) for q in rear]})
                v['thermal'].append({'id': f'th-{word}-scapulars', 'of': f'{word}-scapulars', 'region': 'plumage-wings', 'layer': 'plumage'})
        for side, (word, sign, text) in SIDES.items():
            web = {'id': f'{word}-hand', 'kind': 'loft', 'bone': f'hand-{side}', 'material': 'plumage', 'role': 'body', 'tone': 0.6, 'capStart': True, 'capEnd': True, 'sections': [{'at': [x, 0, round(sign * 0.019, 4)], 'rx': t, 'ry': w, 'exponent': 2.4, 'rotation': FLAT} for x, w, t in WEB]}
            v['body'].append(web)
            v['thermal'].append({'id': f'th-{word}-hand', 'of': f'{word}-hand', 'region': 'plumage-wings', 'layer': 'plumage'})
            for bone, parent, stem, i, base, length, rest, spread, width in feathers():
                ident = f'{word}-{"primary" if stem == "p" else "secondary"}-{i + 1}'
                sections = [{'at': [round(length * t, 5), 0, 0], 'rx': THICK, 'ry': round(max(width * w, THICK), 5), 'exponent': 2, 'rotation': FLAT} for t, w in STATIONS]
                v['body'].append({'id': ident, 'kind': 'loft', 'bone': f'{bone}-{side}', 'material': 'plumage', 'role': 'body', 'tone': 0.5 if stem == 'p' else 0.56, 'capStart': True, 'capEnd': True, 'sections': sections})
                v['thermal'].append({'id': f'th-{ident}', 'of': ident, 'region': 'plumage-wings', 'layer': 'plumage'})
                v['structure'].append({'id': f'{ident}-rachis', 'kind': 'capsule', 'bone': f'{bone}-{side}', 'material': 'plumage', 'role': 'body', 'tone': 0.9, 'p0': [0, 0, 0], 'p1': [round(length - THICK, 5), 0, 0], 'r0': THICK, 'r1': THICK, 'member': f'{bone}-{side}', 'part': 'member'})
    json.dump(doc, open(SOURCE, 'w'), ensure_ascii=False)


if '--write' in sys.argv or '--candidate' in sys.argv:
    solids()
    splice('pigeon', change, candidate='--candidate' in sys.argv)
