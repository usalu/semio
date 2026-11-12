import json,sys,glob
S="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/"
slug=sys.argv[1]
p=glob.glob(S+"*humans/"+slug+"/🔣️.json")[0]
a=json.load(open(p))['asset']
print("DIM",json.dumps(a['dimensions']))
print("RIG")
for b in a['rig']['bones']:
    print(" ",json.dumps(b,ensure_ascii=False))
print("MATERIALS",[ (m['id']) for m in a['materials']])
print("REGIONS",[ (r['id']) for r in a['thermal']['regions']])
for p in a['poses']:
    print("POSE",p['id'],json.dumps(p.get('bones'),ensure_ascii=False), "root",json.dumps({k:p[k] for k in p if k in('root','ground')}))
for c in a['clips']: print("CLIP",c['id'],json.dumps(c)[:300])
print("REPS",list(a['representations'].keys()))
g=a['representations']["50"]['geometry']
def ex(s):
    s=dict(s)
    return json.dumps(s,ensure_ascii=False,separators=(',',':'))
for v in ('elevation','front','plan'):
    print("==",v)
    for s in g[v]:
        print(" ",ex(s)[:260])
for d in ("200","500","20","10","100"):
    if d in a['representations']:
        gg=a['representations'][d]['geometry']
        print("REP",d,{v:[s['id'] for s in gg[v]] for v in gg if isinstance(gg[v],list)})
