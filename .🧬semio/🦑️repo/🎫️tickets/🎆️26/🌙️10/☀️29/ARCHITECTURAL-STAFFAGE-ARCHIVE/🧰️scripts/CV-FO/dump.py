import json,sys
a=json.load(open(sys.argv[1]))['asset']
print(a['id'],a['dimensions'],a['presentation'])
print('MATERIALS',[ (m['id'],m['emissivity'],m.get('solarAbsorptivity'),m.get('densityKgM3'),m['source'],m.get('reference')) for m in a['materials']])
print('REFS',[r['id'] for r in a['provenance']['references']])
for b in a['rig']['bones']:
    print('BONE',b['id'],b['parent'],b['x'],b['y'],b['angleDeg'],b['lengthM'],json.dumps(b['joint']),json.dumps(b.get('mass')) if b.get('mass') else '')
for x in a['anchors']: print('ANCH',json.dumps(x,ensure_ascii=False))
for p in a['poses']:
    print('POSE',p['id'],json.dumps(p['bones']),p.get('root'),p['ground'],p['contacts'],json.dumps(p['loads'],ensure_ascii=False)[:600])
for c in a['clips']:
    print('CLIP',json.dumps({k:v for k,v in c.items() if k not in('title','description')},ensure_ascii=False))
print('THERMAL',json.dumps(a['thermal'],ensure_ascii=False))
print('STRUCT',json.dumps(a['structure'],ensure_ascii=False))
if len(sys.argv)>2:
  for s in a['geometry']['elevation']:
    d={k:v for k,v in s.items() if k not in('region','material','filled')}
    print('SH',json.dumps(d))
