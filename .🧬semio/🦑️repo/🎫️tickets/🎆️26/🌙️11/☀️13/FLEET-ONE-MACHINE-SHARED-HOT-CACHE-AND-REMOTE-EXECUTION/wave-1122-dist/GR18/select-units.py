import json,sys,collections
R='/Users/ueli/Documents/semio/'
reg=json.load(open(R+'📋️build-owners.json'))['owners']
rows=[]
for o in reg:
    try: j=json.load(open(R+o['manifest']))
    except Exception: continue
    tags=j.get('tags',[])
    kind='language:rust' if 'language:rust' in tags else 'language:cargo' if 'language:cargo' in tags else None
    j['root']=o['root']
    rows.append((o['name'],kind,j))
rust=[r for r in rows if r[1]]
print(len(rows),len(rust),collections.Counter(r[1] for r in rust),file=sys.stderr)
n=int(sys.argv[1]); skip=set(sys.argv[2].split(','))
rust=[r for r in rust if r[0] not in skip]
step=len(rust)/n
pick=[rust[int(i*step)] for i in range(n)]
json.dump([{'name':a,'kind':b,'json':c} for a,b,c in pick],open(sys.argv[3],'w'))
print([p[0][12:] for p in pick],file=sys.stderr)
