"""🔎 Print poses, clips, bones, dimensions and per-scale counts of an object asset."""
import json, pathlib, sys
C = pathlib.Path('/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects')
for s in sys.argv[1:]:
    a = json.loads((C / s / '🔣️.json').read_text())['asset']
    print('##', s, a['dimensions'])
    print(' poses', [p['id'] for p in a.get('poses', [])], 'default', a['presentation'].get('defaultPose'))
    print(' clips', [(c['id'], c.get('durationS')) for c in a.get('clips', [])])
    for b in a['rig']['bones']:
        print('  bone', b['id'], b.get('parent'), b.get('x'), b.get('y'), b.get('angleDeg'), b.get('lengthM'), b.get('joint', {}).get('kind'))
    for k, v in a['volumes'].items():
        print(' scale', k, v['status'], *(len(v.get(l, [])) for l in ('body', 'thermal', 'structure')))
    print(' regions', [(r['id'], r.get('areaM2')) for r in a['thermal']['regions']])
