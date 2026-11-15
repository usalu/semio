import json,glob,sys
C="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/"
for slug in sys.argv[1:]:
    a=json.load(open(C+slug+"/🔣️.json"))['asset']
    print("=====",slug,a['dimensions'],a['presentation'])
    th=a['thermal']
    for r in th['regions']:
        print(' R',r['id'],r.get('areaM2'),r.get('kind'),r.get('hidden'),[(l['id'],l.get('thicknessM'),l.get('resistanceM2KW'),l.get('material')) for l in r.get('layers',[])])
    for e in th.get('emitters',[]): print(' E',json.dumps(e)[:300])
    print(' S',json.dumps(a['structure'].get('supports'))[:600])
    print(' bones',[ (b['id'],b['parent'],b.get('lengthM'),'sec' if 'section' in b else '') for b in a['rig']['bones']])
    print(' poses',[p['id'] for p in a['poses']])
