"""🔎️ Prints the drawn shapes and the body solids of an object at a scale: show.py <id|file> <scale> [view,…] [id-substring]"""
import json, sys
a = sys.argv[1]
p = a if a.endswith('.json') else f"/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/{a}/🔣️.json"
d = json.load(open(p))['asset']
sc = sys.argv[2]
views = sys.argv[3].split(',') if len(sys.argv) > 3 else ['elevation', 'front', 'plan', 'body']
sub = sys.argv[4] if len(sys.argv) > 4 else ''
n = int(__import__('os').environ.get('N', '330'))
print(d['dimensions'], 'pose', d['presentation']['defaultPose'])
r = d['representations'][sc]
print('sections', r.get('sections'), 'pens', r.get('pens'))
for v in views:
    if v in r['geometry']:
        for s in r['geometry'][v]:
            if sub in s['id']: print(v, json.dumps(s, ensure_ascii=False)[:n])
    elif v in ('body', 'thermal', 'structure'):
        for s in d['volumes'][sc][v]:
            if sub in s['id']: print(v, json.dumps(s, ensure_ascii=False)[:n])
    elif v == 'bones':
        for b in d['rig']['bones']: print(json.dumps(b)[:n])
    elif v == 'decisions':
        print(json.dumps(r.get('decisions'), ensure_ascii=False)[:4000]); print(json.dumps(d['volumes'][sc].get('decisions'), ensure_ascii=False)[:4000])
