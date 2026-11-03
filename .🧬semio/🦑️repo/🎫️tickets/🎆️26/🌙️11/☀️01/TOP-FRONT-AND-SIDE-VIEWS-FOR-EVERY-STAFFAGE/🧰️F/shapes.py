import json,sys
sys.stdout.reconfigure(encoding='utf8')
S='C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/'
n=sys.argv[1]; scales=sys.argv[2].split(',') ; view=sys.argv[3] if len(sys.argv)>3 else 'elevation'
a=json.load(open(S+n+'/🔣️.json',encoding='utf8'))['asset']
for s in scales:
    r=a['representations'][s]
    print('#### scale',s,r['pens'],[h['id'] for h in r['hatches']])
    for sh in r['geometry'][view]:
        d={k:v for k,v in sh.items() if k not in('id','bone','role','z','filled','tone','region','regions','material')}
        meta=f"{sh['id']} b={sh['bone']} {sh['role'][:3]} z={sh['z']} {'F' if sh['filled'] else 'o'} t={sh['tone']} reg={sh.get('region') or sh.get('regions')} mat={sh.get('material')} hatch={sh.get('hatch')}"
        print(meta,'|',json.dumps(d))
