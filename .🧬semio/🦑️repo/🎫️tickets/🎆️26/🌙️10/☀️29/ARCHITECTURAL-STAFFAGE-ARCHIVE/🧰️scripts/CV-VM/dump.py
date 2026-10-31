import json,sys
S="/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/"
a=json.load(open(S+sys.argv[1]+"/🔣️.json"))['asset']
print(a['dimensions'],a['presentation'],a['structure']['supports'])
print("MATERIALS",[(m['id'],m['emissivity'],m.get('solarAbsorptivity')) for m in a['materials']])
print("BONES")
for b in a['rig']['bones']: print(' ',b['id'],b['parent'],b['x'],b['y'],b['angleDeg'],b['lengthM'],b['joint'], (b.get('mass') or {}).get('kg'))
print("ANCHORS")
for b in a['anchors']: print(' ',json.dumps({k:v for k,v in b.items() if k!='title'}))
print("SHAPES")
for s in a['geometry']['elevation']:
    d={k:v for k,v in s.items() if k not in('id','bone','role','lod','z','filled','tone','region','material','kind')}
    print(' ',s['id'],s['bone'],s['kind'],s['role'],'lod',s.get('lod'),'z',s['z'],'f',s['filled'],s.get('region'),s.get('material'),json.dumps(d,ensure_ascii=False))
print("THERMAL",json.dumps(a['thermal'],ensure_ascii=False)[:6000])
