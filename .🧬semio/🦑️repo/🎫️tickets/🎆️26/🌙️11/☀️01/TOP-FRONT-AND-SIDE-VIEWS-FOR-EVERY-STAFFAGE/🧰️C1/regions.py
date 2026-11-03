import json, sys, io, os
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf8')
base = r'C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog\🧱️components'
for i in sys.argv[1:]:
    a = json.load(open(os.path.join(base, i, '🔣️.json'), encoding='utf8'))['asset']
    print('==', i)
    for r in a['thermal']['regions']:
        print(' region', r['id'], r.get('model'), 'material', r.get('material'), 'area', r.get('areaM2'), 'L', r.get('characteristicLengthM'), 'layers', [(l.get('id'), l.get('material'), l.get('thicknessM')) for l in r.get('layers', [])], 'hidden', r.get('hidden'))
    print(' materials', [m['id'] for m in a['materials']])
    print(' anchors', [(x['id'], x['bone'], x['x'], x['y']) for x in a['anchors']])
    print(' summary', a['summary']['en'])
    print(' notes', a['provenance'].get('notes'))
