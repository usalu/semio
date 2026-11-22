"""🦴 Labrador 1:500 structure: every long bone is one tapering shaft from joint to joint (femur, tibia, metatarsals, humerus, radius with ulna, metacarpals) as thick as the smallest feature of the scale or the drawn limb, instead of wedge lofts that were thickest at the wrong end (rework author 08)."""
import json

SOURCE = 'volumes/animals/labrador-dog.json'
RADII = {'femur': (0.038, 0.038), 'tibia': (0.03, 0.03), 'metatarsals': (0.0205, 0.0205), 'humerus': (0.032, 0.032), 'radius-ulna': (0.029, 0.029), 'metacarpals': (0.0258, 0.0258)}
doc = json.load(open(SOURCE))
fine = {s['id']: s for s in doc['volumes']['200']['structure']}
lst = doc['volumes']['500']['structure']
for i, s in enumerate(lst):
    stem, side = s['id'][:-2], s['id'][-2:]
    if stem in RADII and side in ('-n', '-f'):
        ends = fine[s['id']]
        lst[i] = {'id': s['id'], 'kind': 'capsule', 'bone': s['bone'], 'material': 'bone', 'role': 'body', 'tone': 0.9, 'p0': ends['p0'], 'p1': ends['p1'], 'r0': RADII[stem][0], 'r1': RADII[stem][1], 'member': s['member'], 'part': 'member'}
json.dump(doc, open(SOURCE, 'w'), ensure_ascii=False)
