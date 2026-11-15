import json,sys
slug=sys.argv[1]
F=f"/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/{slug}/🔣️.json"
d=json.load(open(F))['asset']
what=sys.argv[2] if len(sys.argv)>2 else 'all'
if what in('all','meta'):
    print('DIM',d['dimensions'])
    print('PRES',d.get('presentation'))
    for b in d['rig']['bones']:
        print('BONE',b['id'],'par',b['parent'],'xy',b['x'],b['y'],'ang',b['angleDeg'],'len',b['lengthM'],json.dumps(b['joint']),json.dumps(b.get('section')), b.get('mass',{}).get('kg'))
    for r in d['thermal']['regions']:
        print('REG',r['id'],r['kind'],r.get('material'),r['areaM2'],r.get('hidden'),[(l['id'],l.get('thicknessM'),l.get('material')) for l in r.get('layers',[])])
    print('EMIT',d['thermal'].get('emitters'))
    print('SUP',d['structure']['supports'])
    print('ANCH',json.dumps([ (a['id'],a.get('bone'),a.get('x'),a.get('y')) for a in d['anchors']]))
    print('STRUCT',[k for k in d['structure']])
if what in('all','shapes'):
  for s in ['50','100','200','500']:
    rp=d['representations'][s]
    print('=== SCALE',s,rp['status'])
    if rp['status']!='authored': continue
    for view,l in rp['geometry'].items():
        print('--',view)
        for sh in l:
            sh=dict(sh)
            for k in ('hatch','filled','z','region'): sh.pop(k,None)
            print(' ',json.dumps(sh,ensure_ascii=False))
