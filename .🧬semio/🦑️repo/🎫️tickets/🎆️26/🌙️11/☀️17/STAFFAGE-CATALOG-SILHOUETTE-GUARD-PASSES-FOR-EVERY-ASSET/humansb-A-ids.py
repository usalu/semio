"""🧾 Shape ids of one scale missing at another and the reverse. python3 humansb-A-ids.py <asset> <scaleA> <scaleB>"""
import json, sys
a, A, B = sys.argv[1:4]
d = json.load(open(f'/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/{a}/🔣️.json'))['asset']
for v in ['elevation', 'front', 'plan']:
    ia = {s['id']: s for s in d['representations'][A]['geometry'][v]}
    ib = {s['id']: s for s in d['representations'][B]['geometry'][v]}
    print(v, f'only {A}:', [(k, ia[k]['role']) for k in ia if k not in ib], f'only {B}:', [(k, ib[k]['role']) for k in ib if k not in ia])
va = {s['id'] for s in d['volumes'][A]['body']}; vb = {s['id'] for s in d['volumes'][B]['body']}
print('volumes only', A, sorted(va - vb), 'only', B, sorted(vb - va))
