import json,sys
S="/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/"
a=json.load(open(S+sys.argv[1]+"/🔣️.json"))['asset']
print(a['dimensions'],a['presentation'])
print("MATERIALS",[(m['id'],m['emissivity'],m.get('solarAbsorptivity')) for m in a['materials']])
for b in a['rig']['bones']: print(' B',b['id'],b['parent'],b['x'],b['y'],b['angleDeg'],b['lengthM'],b['joint'].get('type'),b['joint'].get('minDeg',b['joint'].get('minM')),b['joint'].get('maxDeg',b['joint'].get('maxM')),(b.get('mass') or {}).get('kg'))
for b in a['anchors']: print(' A',json.dumps({k:v for k,v in b.items() if k not in('title','frictionCoefficient')}))
for s in a['geometry']['elevation']:
    if s['id'].startswith(('spoke','tread','lug')) and len(sys.argv)<3: continue
    d={k:v for k,v in s.items() if k not in('id','bone','role','lod','z','filled','tone','region','material','kind')}
    t=json.dumps(d,ensure_ascii=False); 
    print(' S',s['id'],s['bone'],s['kind'][:3],s['role'][:3],s.get('lod'),'f' if s['filled'] else '-',s.get('region'),s.get('material'),t[:230])
for r in a['thermal']['regions']: print(' R',json.dumps({k:v for k,v in r.items() if k!='title'},ensure_ascii=False), r['title']['en'])
for r in a['thermal']['emitters']: print(' E',json.dumps({k:v for k,v in r.items() if k!='title'},ensure_ascii=False))
print(' ENV',a['thermal']['environment'])
for p in a['poses']: print(' P',p['id'],p['bones'] if len(json.dumps(p['bones']))<400 else '...', p['root'], p['contacts'], json.dumps(p['thermal']))
for c in a['clips']: print(' C',c['id'],c['durationS'],c['loop'],c['rootMotion'],[e['kind']+'@'+str(e['t']) for e in c['events']],[t['target'] for t in c['tracks']])
