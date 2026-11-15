import json,sys,re
slug=sys.argv[1]; scales=sys.argv[2].split(',') if len(sys.argv)>2 else ['50']
A=json.load(open(f"/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/{slug}/🔣️.json"))['asset']
def r(x):
    if isinstance(x,float): return round(x,4)
    if isinstance(x,list): return [r(i) for i in x]
    return x
print('DIM',A['dimensions'],'default',A['presentation'])
print('MATERIALS',[(m['id'],m.get('conductivityWmK')) for m in A['materials']])
print('ANCHORS',[(a['id'],a['bone'],a['x'],a['y'],a.get('kind'),a.get('support')) for a in A['anchors']])
print('SUPPORTS',A['structure'].get('supports'))
for b in A['rig']['bones']:
    d={k:v for k,v in b.items() if k not in('title',)}
    if 'mass' in d: d['mass']=d['mass'].get('kg')
    print('BONE',json.dumps(r(d),ensure_ascii=False))
T=A['thermal']
for g in T['regions']:
    print('REG',g['id'],g['kind'],g.get('areaM2'),g.get('hidden'),[(l['id'],l.get('thicknessM'),l.get('material'),l.get('resistanceM2KW')) for l in g.get('layers',[])])
print('EMIT',[(e['id'],e.get('region'),e.get('diameterM'),e.get('anchor')) for e in T['emitters']])
for s in scales:
    rep=A['representations'][s]
    if rep['status']!='authored': print('SCALE',s,'unavailable');continue
    print('SCALE',s,[k for k in rep['geometry']])
    for view,shapes in rep['geometry'].items():
        print(' VIEW',view,len(shapes))
        seen=set()
        for sh in shapes:
            m=re.match(r'(slat|slide)-(\d+)$',sh['id'])
            if m:
                k=(m.group(1),sh.get('kind'),json.dumps(sh.get('w')),json.dumps(sh.get('h')))
                if m.group(2)!='0': 
                    if m.group(2)=='16': print('   ... slat-1..16 as slat-0 (last y/pen',sh.get('y'),sh.get('penMm'),')')
                    continue
            d={k:v for k,v in sh.items() if k not in('pens','tone','z','title')}
            print('  ',json.dumps(r(d),ensure_ascii=False)[:330])
