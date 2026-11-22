"""😴 Red fox, pose curled-sleeping: fore legs folded forward on the ground instead of twisted out sideways and sunk into it.

Usage: python3 rework-red-fox-pose.py [--apply]
The hand-chosen targets: no humerus twist, forearm falling 4 degrees from the elbow to the carpus, carpus one forearm radius above the ground of the hind paws, metacarpus and paw in line with the forearm.
"""
import importlib.util
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location('girdle', os.path.join(T, 'rework-red-fox-girdle.py'))
girdle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(girdle)
np = girdle.np

POSE = 'curled-sleeping'
FOREARM_RADIUS = 0.014
FOREARM_FALL = 0.07


def posed(bones, offsets):
    out = {}
    for bone in bones:
        local = girdle.rot(bone['angleDeg'], bone.get('abductionDeg', 0), bone.get('twistDeg', 0)) @ girdle.offset_of(offsets.get(bone['id'], {}))
        at = np.array([bone.get('x', 0), bone.get('y', 0), bone.get('z', 0)])
        if bone.get('parent') is None:
            out[bone['id']] = (local, at)
        else:
            pb, po = out[bone['parent']]
            out[bone['id']] = (pb @ local, po + pb @ at)
    return out


def tip(frames, bones, ident):
    basis, origin = frames[ident]
    return origin + basis @ np.array([bones[ident]['lengthM'], 0, 0])


def main():
    path = girdle.CATALOG
    asset = json.load(open(path, encoding='utf-8'))['asset']
    bones = asset['rig']['bones']
    by = {b['id']: b for b in bones}
    pose = next(p for p in asset['poses'] if p['id'] == POSE)
    offsets = pose['bones']
    base = posed(bones, offsets)
    ground = min(tip(base, by, 'paw-hn')[1], tip(base, by, 'paw-hf')[1])
    print('[DEBUG] ground', round(ground, 4), 'old fore paw tips', round(tip(base, by, 'paw-fn')[1] - ground, 4), round(tip(base, by, 'paw-ff')[1] - ground, 4))
    for tag in ('fn', 'ff'):
        best = None
        sb, so = base['scapula-' + tag]
        heading = base['thorax'][0] @ np.array([1, 0, 0])
        humerus, forearm = by['humerus-' + tag], by['forearm-' + tag]
        h_rest = girdle.rot(humerus['angleDeg'], humerus.get('abductionDeg', 0), humerus.get('twistDeg', 0))
        f_rest = girdle.rot(forearm['angleDeg'])
        elbow_at = np.array([forearm['x'], forearm.get('y', 0), forearm.get('z', 0)])
        shoulder = so + sb @ np.array([humerus['x'], 0, 0])
        for span, step in ((range(-60, 1, 2), 2.0), (None, 0.25)):
            hs = [h for h in span] if span else [best[1] + i * step for i in range(-8, 9)]
            fs = [f for f in range(-50, 141, 2)] if span else [best[2] + i * step for i in range(-8, 9)]
            for a_h in hs:
                hb = sb @ h_rest @ girdle.rot(a_h)
                elbow = shoulder + hb @ elbow_at
                for a_f in fs:
                    fb = hb @ f_rest @ girdle.rot(a_f)
                    axis = fb @ np.array([1, 0, 0])
                    if sum(a * b for a, b in zip(axis, heading)) < 0.3:
                        continue
                    carpus = (elbow + fb @ np.array([forearm['lengthM'], 0, 0]))[1] - ground
                    cost = abs(axis[1] + FOREARM_FALL) * 0.12 + abs(carpus - FOREARM_RADIUS)
                    if best is None or cost < best[0]:
                        best = (cost, a_h, a_f, carpus, axis[1])
        print('[DEBUG]', tag, 'cost %.4f humerus %.2f forearm %.2f carpus %.4f slope %.3f' % best)
        offsets['humerus-' + tag] = {'angleDeg': best[1]}
        offsets['forearm-' + tag] = {'angleDeg': best[2]}
        level = None
        for tenth in range(-80, 81):
            trial = dict(offsets)
            trial['metacarpus-' + tag] = {'angleDeg': round(-by['metacarpus-' + tag]['angleDeg'] + tenth / 10, 3)}
            slope = abs((posed(bones, trial)['metacarpus-' + tag][0] @ np.array([1, 0, 0]))[1])
            if level is None or slope < level[0]:
                level = (slope, trial['metacarpus-' + tag])
        offsets['metacarpus-' + tag] = level[1]
        offsets['paw-' + tag] = {'angleDeg': -by['paw-' + tag]['angleDeg']}
    done = posed(bones, offsets)
    print('[DEBUG] new fore paw tips', round(tip(done, by, 'paw-fn')[1] - ground, 4), round(tip(done, by, 'paw-ff')[1] - ground, 4), 'elbows', round(done['forearm-fn'][1][1] - ground, 4), round(done['forearm-ff'][1][1] - ground, 4))
    if '--apply' in sys.argv:
        text = open(path, encoding='utf-8').read()
        open(path, 'w', encoding='utf-8').write(girdle.splice(text, 'poses', asset['poses']))
        print('applied')


main()
