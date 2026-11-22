"""🦴 Red fox skeleton: closed load path (rib heads, tail root, serratus sling) and the true thorax breadth.

Usage: python3 rework-red-fox-skeleton.py   (edits the volumes source in place, idempotent; publish with volumes-insert.py)
"""
import importlib.util
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location('girdle', os.path.join(T, 'rework-red-fox-girdle.py'))
girdle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(girdle)
np = girdle.np

THORAX_BREADTH = 0.82
RIB_HEAD_Z = 0.0045
RIB_HEAD_RISE = 0.004
SLING_TOP = (0.386, 0.010, 0.029)
SPINOUS_HEIGHT = [0.028, 0.034, 0.036, 0.034, 0.030, 0.026, 0.022, 0.018, 0.015, 0.013, 0.013, 0.013, 0.013]
SPINOUS_LEAN = [-0.35, -0.35, -0.4, -0.4, -0.45, -0.45, -0.45, -0.4, -0.35, -0.25, 0.0, 0.2, 0.2]
EAR_ROOT = {'50': 0.008, '100': 0.010}


def world(frame, at):
    basis, origin = frame
    return origin + basis @ np.array(at)


def replace(solids, fresh, before=None):
    ids = {s['id'] for s in fresh}
    kept = [s for s in solids if s['id'] not in ids]
    cut = next(i for i, s in enumerate(kept) if s.get('part') != 'member') if before is None else before
    return kept[:cut] + fresh + kept[cut:]


def main():
    asset = json.load(open(girdle.CATALOG, encoding='utf-8'))['asset']
    f = girdle.frames(asset['rig']['bones'])
    source = json.load(open(girdle.SOURCE, encoding='utf-8'))
    for scale, volume in source['volumes'].items():
        solids = volume['structure']
        for solid in solids:
            name = solid['id']
            if '-rib-' in name and len(solid['sections']) == 5:
                for section in solid['sections']:
                    section['at'][2] = round(section['at'][2] * THORAX_BREADTH, 5)
                head = json.loads(json.dumps(solid['sections'][0]))
                sign = -1 if head['at'][2] < 0 else 1
                head['at'] = [head['at'][0], round(head['at'][1] + RIB_HEAD_RISE, 5), sign * RIB_HEAD_Z]
                solid['sections'].insert(0, head)
            if name.endswith('costal-arch') and abs(abs(solid['sections'][0]['at'][2]) - 0.01) < 1e-6:
                for section in solid['sections']:
                    section['at'][2] = round(section['at'][2] * THORAX_BREADTH, 5)
        if scale == '50':
            by = {x['id']: x for x in solids}
            for n in range(1, 14):
                x, y, _ = by['t%d-body' % n]['sections'][1]['at']
                h, lean = SPINOUS_HEIGHT[n - 1], SPINOUS_LEAN[n - 1]
                path = [((x, y + 0.003), 0.0048), ((x + lean * h * 0.5, y + h * 0.55), 0.0045), ((x + lean * h, y + h), 0.0042)]
                spine = girdle.world_loft(f['thorax'], 't%d-spine' % n, 'thorax', [(list(world(f['thorax'], (px, py, 0))), 0.0038, ry) for (px, py), ry in path], (0, 0, 1), 1)
                by['t%d-spine' % n].clear()
                by['t%d-spine' % n].update(spine)
            hip = girdle.pelvis_50(f)
            solids = replace([x for x in solids if x['id'] not in {h['id'] for h in hip}], [h for h in hip if h['part'] == 'member']) + [h for h in hip if h['part'] != 'member']
        for solid in solids:
            if solid['id'].endswith('ear-cartilage') and scale in EAR_ROOT and solid['sections'][0]['at'][0] > -0.005:
                root = json.loads(json.dumps(solid['sections'][0]))
                root['at'][0] = round(root['at'][0] - EAR_ROOT[scale], 5)
                solid['sections'].insert(0, root)
        solids = [s for s in solids if not (s['id'] == 'joint-pelvis' and scale in ('50', '100'))]
        fresh = []
        if scale in ('50', '100'):
            radius = 0.0055 if scale == '50' else 0.0084
            fresh.append(girdle.world_loft(f['pelvis'], 'caudal-root', 'pelvis', [((-0.004, 0.0395, 0), radius, radius), ((-0.018, 0.0440, 0), radius, radius), ((-0.031, 0.0478, 0), radius, radius)], (0, 0, 1), 1))
            thick = 0.004 if scale == '50' else 0.0075
            for name, tag, s in (('near', 'fn', -1), ('far', 'ff', 1)):
                rib = next(x for x in solids if x['id'] == name + '-rib-3')
                foot = list(world(f['thorax'], rib['sections'][2]['at']))
                foot[2] = abs(foot[2])
                middle = [(a + b) / 2 for a, b in zip(SLING_TOP, foot)]
                sling = girdle.world_loft(f['scapula-' + tag], name + '-serratus-sling', 'scapula-' + tag, [(SLING_TOP, thick, 0.011), (middle, thick, 0.010), (foot, thick, 0.008)], (0, 0, 1), s, 2.4)
                sling['material'], sling['tone'] = 'red-fur', 0.7
                fresh.append(sling)
        else:
            girth = 0.0168 if scale == '200' else 0.016
            block = girdle.world_loft(f['pelvis'], 'pelvis-girdle', 'pelvis', [((0.058, 0.034, 0), 0.035, girth - 0.003), ((0.030, 0.026, 0), 0.030, girth - 0.002), ((0.0, 0.014, 0), 0.034, girth), ((-0.038, 0.022, 0), 0.036, girth)], (0, 0, 1), 1)
            block['members'] = ['ilium-hn', 'ilium-hf']
            where = next(i for i, x in enumerate(solids) if x['id'] == 'pelvis-girdle')
            solids = replace(solids, [block], where)
        volume['structure'] = replace(solids, fresh) if fresh else solids
    json.dump(source, open(girdle.SOURCE, 'w', encoding='utf-8'), ensure_ascii=False, separators=(',', ':'))
    print('skeleton written', girdle.SOURCE)


main()
