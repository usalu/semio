import json,sys,glob
CAT='/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/'
slug,scale=sys.argv[1],sys.argv[2]
what=sys.argv[3] if len(sys.argv)>3 else 'both'
p=glob.glob(CAT+'*/'+slug+'/🔣️.json')[0]
a=json.load(open(p))['asset']
def short(s):
    d={k:v for k,v in s.items() if k not in('material','tone','region','layer','sectionFaces','role','tags','texture')}
    r=json.dumps(d,ensure_ascii=False,separators=(',',':'))
    return r if len(r)<420 else r[:420]+'...'
if what in('both','vol'):
    for k in ('body',):
        for s in a['volumes'][scale][k]: print('V',short(s))
if what in('both','draw'):
    for view in sys.argv[4].split(',') if len(sys.argv)>4 else ('elevation','front','plan'):
        print('==',view, json.dumps(a['representations'][scale].get('sections',{}).get(view)))
        for s in a['representations'][scale]['geometry'][view]:
            if s.get('kind')=='path' and len(s['d'])>200: s=dict(s,d=s['d'][:200]+'...')
            print('D',short(s))
