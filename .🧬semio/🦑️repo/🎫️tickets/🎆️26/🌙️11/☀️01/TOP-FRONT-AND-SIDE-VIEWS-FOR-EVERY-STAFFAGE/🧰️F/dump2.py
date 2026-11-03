import json,sys
sys.stdout.reconfigure(encoding='utf8')
S='C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/'
for n in sys.argv[1:]:
    a=json.load(open(S+n+'/🔣️.json',encoding='utf8'))['asset']
    print('==',n)
    print(' dims',a['dimensions'],'pres',json.dumps(a.get('presentation'),ensure_ascii=False))
    print(' bones',[(b['id'],b['parent'],b['x'],b['y'],b['angleDeg'],b['lengthM'],b['joint']['type']) for b in a['rig']['bones']])
    print(' poses',[(p['id']) for p in a.get('poses',[])],'clips',[(c['id'],c.get('durationS')) for c in a.get('clips',[])])
    for s,r in a['representations'].items():
        g=r.get('geometry',{})
        print(' rep',s,r.get('status'),{k:len(v) if isinstance(v,list) else v for k,v in g.items()})
