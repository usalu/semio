import json,sys
C="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/%s/🔣️.json"
a=json.load(open(C%sys.argv[1]))['asset']
print('DIM',a['dimensions'],'pose',a['presentation'])
print('MATERIALS',[m['id'] for m in a['materials']])
print('BONES')
for b in a['rig']['bones']:
    print(' ',b['id'],'parent',b['parent'],'xy',b['x'],b['y'],'ang',b['angleDeg'],'len',b.get('lengthM'),'sec',b.get('section'),'joint',b['joint'].get('type'),'mass',b.get('mass',{}).get('kg'),b.get('mass',{}).get('comX'),b.get('mass',{}).get('comY'))
print('ANCHORS',json.dumps(a.get('anchors'),ensure_ascii=False))
print('POSES',[(p['id'],p.get('values') or p.get('bones')) for p in a['poses']][:6])
t=a['thermal']
for r in t['regions']:
    print('REGION',r['id'],r['kind'],r.get('hidden'),'area',r.get('areaM2'),'mat',r.get('material'),'layers',[(l['id'],l.get('thicknessM'),l.get('material')) for l in r.get('layers',[])],'op',r.get('operating'))
print('EMITTERS',json.dumps(t.get('emitters'),ensure_ascii=False)[:1500])
print('STRUCT',json.dumps(a['structure'],ensure_ascii=False)[:1500])
for s in ['50','100','200','500']:
    r=a['representations'][s]
    if r['status']!='authored': print(s,'unavailable'); continue
    for v in ['elevation','front','plan']:
        print('--',s,v)
        for g in r['geometry'].get(v,[]):
            d={k:(round(x,4) if isinstance(x,float) else x) for k,x in g.items() if k not in('role','filled','material','regions','region','z')}
            print('  ',json.dumps(d,ensure_ascii=False))
