import json,sys,glob
S="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/"
slug=sys.argv[1]
views=sys.argv[2].split(',') if len(sys.argv)>2 else ['elevation','front','plan']
scale=sys.argv[3] if len(sys.argv)>3 else '50'
p=glob.glob(S+"*components/"+slug+"/🔣️.json")[0]
a=json.load(open(p)); a=a.get('asset',a)
print("DIM",a['dimensions'])
print("RIG",a['rig']['kind'])
for b in a['rig']['bones']:
    j=b.get('joint',{})
    print("  bone",b['id'],"parent",b['parent'],"x",b['x'],"y",b['y'],"ang",b['angleDeg'],"len",b['lengthM'],"joint",json.dumps(j))
print("MAT",[m['id'] for m in a['materials']])
print("REG",[(r['id'],[l['id']+':'+str(l['thicknessM']) for l in r.get('layers',[])]) for r in a['thermal']['regions']])
print("POSES",[(q['id'],q['bones']) for q in a['poses']])
print("CLIPS",[c['id'] for c in a['clips']])
print("REPS",list(a['representations'].keys()))
def sh(s):
    d={k:v for k,v in s.items() if k not in('z','filled','hatch','region','id','bone','material','role','tone')}
    return json.dumps(d,ensure_ascii=False,separators=(',',':'))
g=a['representations'][scale]['geometry']
for v in views:
    print("==",v)
    for s in g[v]:
        print(" ",s['id'],s['bone'],s['role'],s.get('material'),s.get('region'),'t',s.get('tone'),sh(s)[:260])
