import json,sys
slug=sys.argv[1]
P="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/%s/🔣️.json"%slug
a=json.load(open(P))['asset']
print(slug,a['dimensions'],a['presentation'])
print('materials',[(m['id']) for m in a['materials']])
print('bones',[(b['id'],b.get('parent'),b.get('lengthM'),b.get('section'),b.get('massKg')) for b in a['rig']['bones']])
print('anchors',[(x.get('id'),x.get('bone')) for x in a.get('anchors',[])])
t=a['thermal']
for r in t['regions']:
    print('R',r['id'],r['kind'],r.get('material'),r.get('areaM2'),r.get('hidden'),[(l['id'],l.get('kind'),l.get('thicknessM'),l.get('resistanceM2KW')) for l in r.get('layers',[])])
print('E',[(e['id'],e.get('region'),e.get('diameterM'),e.get('anchor')) for e in t.get('emitters',[])])
print('supports',a['structure'].get('supports'))
for s in ['50','100','200','500']:
    r=a['representations'][s]
    if r['status']!='authored': print(s,r['status']);continue
    g=r['geometry']
    print(s,{k:(len(v) if isinstance(v,list) else '?') for k,v in g.items()})
