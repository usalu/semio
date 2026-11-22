"""🦴 Labrador 1:200 and 1:500 structure: a sternum bar under the rib hoops, so the ribcage closes between spine and breastbone instead of hanging as loose hoops (rework author 08)."""
import json

SOURCE = 'volumes/animals/labrador-dog.json'
STERNUM = {
    '200': {'after': 'rib-3', 'p0': [0.03, -0.184, 0], 'p1': [0.2, -0.158, 0], 'r': 0.015},
    '500': {'after': 'rib-8', 'p0': [0.06, -0.17, 0], 'p1': [0.19, -0.15, 0], 'r': 0.038},
}
NOTE = {
    'en': ' A sternum bar joins the rib hoops ventrally, so the ribcage is closed between spine and breastbone.',
    'de': ' Ein Brustbeinstab verbindet die Rippenreifen bauchseitig, sodass der Brustkorb zwischen Wirbelsäule und Brustbein geschlossen ist.',
}
doc = json.load(open(SOURCE))
for scale, s in STERNUM.items():
    lst = doc['volumes'][scale]['structure']
    lst[:] = [x for x in lst if x['id'] != 'sternum']
    at = next(i for i, x in enumerate(lst) if x['id'] == s['after']) + 1
    lst.insert(at, {'id': 'sternum', 'kind': 'capsule', 'bone': 'thorax', 'material': 'bone', 'role': 'body', 'tone': 0.9, 'p0': s['p0'], 'p1': s['p1'], 'r0': s['r'], 'r1': s['r'], 'member': 'thorax', 'part': 'member'})
    d = doc['volumes'][scale]['decisions'][2]
    for lang in ('en', 'de'):
        if NOTE[lang] not in d[lang]:
            d[lang] += NOTE[lang]
json.dump(doc, open(SOURCE, 'w'), ensure_ascii=False)
