"""🐕 Hand-solved sitting pose of the labrador: rump and hocks on the ground, forelegs straight (rework author 08)."""
import json, os, sys
sys.path.insert(0, '.')
from rework_08_fk import load, fk, point, path

doc = load('labrador-dog'); a = doc['asset']
bones = {b['id']: b for b in a['rig']['bones']}
pose = next(p for p in a['poses'] if p['id'] == 'sitting-waiting')
ROOT = float(sys.argv[1]) if len(sys.argv) > 1 else 53
WORLD = {'thigh': float(os.environ.get('THIGH', 7)), 'shank': float(os.environ.get('SHANK', -146.7)), 'metatarsus': -8, 'paw-h': -8, 'scapula': float(sys.argv[6]), 'humerus': -118, 'forearm': -86, 'metacarpus': -80, 'paw-f': -16}
CHAIN = [('thigh', 'thigh-h'), ('shank', 'shank-h'), ('metatarsus', 'metatarsus-h'), ('paw-h', 'paw-h'), ('scapula', 'scapula-f'), ('humerus', 'humerus-f'), ('forearm', 'forearm-f'), ('metacarpus', 'metacarpus-f'), ('paw-f', 'paw-f')]
ov = {k: v.get('angleDeg', 0) for k, v in pose['bones'].items()}
ov['__root'] = ROOT
ov.update({'neck': -37.5, 'head': -23.5, 'tail-1': -63, 'tail-2': -17, 'tail-3': 6})
ov['thorax'] = float(sys.argv[2])
ov['lumbar'] = float(sys.argv[3])
WORLD['humerus'], WORLD['forearm'] = float(sys.argv[4]), float(sys.argv[5])
for key, stem in CHAIN:
    for side in 'nf':
        i = stem + side
        ov[i] = 0
        f = fk(a, None, ov)
        ov[i] = round(WORLD[key] - f[i][2], 2)
f = fk(a, None, ov)
for an in a['anchors']:
    if an.get('kind') == 'contact':
        print(an['id'], [round(v, 3) for v in point(a, f, an['bone'], an['x'], an['y'])])
for i in ['pelvis', 'thigh-hn', 'shank-hn', 'metatarsus-hn', 'paw-hn', 'scapula-fn', 'humerus-fn', 'forearm-fn', 'metacarpus-fn', 'paw-fn']:
    j = bones[i]['joint']
    print(i, [round(v, 3) for v in f[i][0]], 'pose', ov.get(i), 'limits', j['minDeg'], j['maxDeg'])
if '--write' in sys.argv or '--candidate' in sys.argv:
    from rework_08_root import splice
    root = ov.pop('__root')

    def change(asset):
        target = next(p for p in asset['poses'] if p['id'] == 'sitting-waiting')
        for k, v in ov.items():
            target['bones'][k] = {'angleDeg': v}
        target['root'] = {'angleDeg': root}
        target['description'] = {'en': 'Sitting on the sitting bones with the hocks and hind paws flat on the ground beside the haunches, the stifles folded forward, the back at 60 degrees and the forelegs straight under the shoulders; the tail rests behind.', 'de': 'Sitzen auf den Sitzbeinhöckern, Sprunggelenke und Hinterpfoten flach am Boden neben der Hinterhand, die Knie nach vorn gefaltet, der Rücken um 60 Grad aufgerichtet und die Vorderbeine gestreckt unter den Schultern; die Rute ruht hinten.'}
        for b in asset['rig']['bones']:
            if b['id'] in ('paw-hn', 'paw-hf'):
                b['joint']['minDeg'] = -80
            if b['id'] == 'tail-1':
                b['joint']['minDeg'] = -75
        return {'poses', 'rig'}

    splice('labrador-dog', change, candidate='--candidate' in sys.argv)
