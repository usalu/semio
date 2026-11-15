import json,sys,re
slug=sys.argv[1]; scales=sys.argv[2].split(',') if len(sys.argv)>2 else ['50']
A=json.load(open(f"/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/{slug}/🔣️.json"))['asset']
def r(x):
    if isinstance(x,float): return round(x,4)
    if isinstance(x,list): return [r(i) for i in x]
    return x
for s in scales:
    rep=A['representations'][s]
    if rep['status']!='authored': print('SCALE',s,'unavailable');continue
    print('SCALE',s,{k:v for k,v in rep.items() if k not in('geometry','symbols')} if s=='x' else [k for k in rep])
    print(' extra',json.dumps({k:rep[k] for k in rep if k not in('geometry','status','denominator')},ensure_ascii=False)[:700])
    for view,shapes in rep['geometry'].items():
        print(' VIEW',view,len(shapes))
        fam={}
        for sh in shapes:
            m=re.match(r'(.*?)-?(\d+)$',sh['id'])
            f=m.group(1) if m else sh['id']
            fam.setdefault(f,[]).append(sh)
        for f,l in fam.items():
            for sh in (l if len(l)<=2 else [l[0],l[-1]]):
                d={k:v for k,v in sh.items() if k not in('pens','tone','z','title','filled','closed','material')}
                print('  ',len(l) if sh is l[0] and len(l)>2 else '',json.dumps(r(d),ensure_ascii=False)[:300])
