import sys,os,json,collections,re
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from roots2 import *
S2=S
tag=sys.argv[1] if len(sys.argv)>1 else 't5'
b=load(tag)
roots=json.load(open(S+'roots.json'));r2n={}
for k,v in roots.items():r2n.setdefault(v,[]).append(k)
n=json.load(open(S+'gen237-noncmd.json'));F=n['files']
fo=collections.defaultdict(lambda:collections.defaultdict(set))   # file -> owner -> labels
for r,s in b['own'].items():
    for name in r2n.get(r,[]):
        for f in s:fo[f][name].add('command')
for name,g in n['owners'].items():
    for gn,ids in g.items():
        for i in ids:fo[F[i]][name].add(gn)
U=[f for f,v in fo.items() if len(v)>=21]
E_=edges(U);inc=collections.Counter()
for f,ds in E_.items():
    for d in ds:
        if d!=f:inc[d]+=1
rs=[f for f in U if inc[f]==0]
def reach(r):
    seen={r};st=[r]
    while st:
        x=st.pop()
        for d in E_.get(x,()):
            if d not in seen:seen.add(d);st.append(d)
    return seen
rows=[]
for r in rs:
    g=reach(r);lab=collections.Counter()
    for f in g:
        for o,ls in list(fo[f].items())[:2]:lab.update(ls)
    rows.append((len(fo[r]),max(len(fo[f]) for f in g),len(g),r.replace('🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/','LIB/'),sorted(lab)))
rows.sort(key=lambda x:(-x[0],-x[2]))
json.dump(rows,open(S+'remaining-'+tag+'.json','w'),ensure_ascii=False)
print(len(U),'files in >=21 owners;',len(rows),'roots')
for x in rows[:45]:print(x[0],x[1],x[2],x[3][-95:],x[4][:4])
