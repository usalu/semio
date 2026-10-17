import json,re,sys,collections,os
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
d=json.load(open(F+"/🗑️generated/"+sys.argv[1]))
b=d['blockers'][int(os.environ.get("BI","0"))]
skipc=os.environ.get("SKIPCOH","visibility")  # coherence types to ignore
par={}
def find(x):
    while par.get(x,x)!=x:
        par[x]=par.get(par[x],par[x]); x=par[x]
    return x
for c in b['causes']:
    m=re.match(r"coherence\((.*)\): (.*) <-> (.*)",c)
    if m and skipc not in m.group(1):
        a,c2=find(m.group(2)),find(m.group(3))
        if a!=c2: par[c2]=a
size={}
for g in b['groupReport']:
    m=re.match(r"slice (.*?) \[.*?\] files=(\d+) bytes=(\d+) lines=(\d+)",g)
    if m: size[m.group(1).split(" + ")[0]]=(int(m.group(2)),int(m.group(4)))
rx=re.compile(os.environ.get("ONLY",".")); ex=re.compile(os.environ.get("EXCL","^$"))
E=collections.defaultdict(set); nodes=set(); lines=collections.Counter(); files=collections.Counter()
for n,(f,l) in size.items():
    k=find(n); lines[k]+=l; files[k]+=f; nodes.add(k)
for e in b['edges']:
    m=re.match(r"(.*?) -> (.*?)( \[test\])?(?: \((.*)\))?$",e)
    a,c=m.group(1),m.group(2)
    if ex.search(a) or ex.search(c): continue
    fa,fc=find(a),find(c)
    if fa!=fc and os.environ.get("NOPC","1")=="1" and ("::".join(c.split("::")[-2:])+"::") in a: continue
    if fa!=fc: E[fa].add(fc); nodes|={fa,fc}
sys.setrecursionlimit(100000)
idx={};low={};st=[];on=set();cnt=[0];comps=[]
def visit(v):
    idx[v]=low[v]=cnt[0];cnt[0]+=1;st.append(v);on.add(v)
    for w in E.get(v,()):
        if w not in idx: visit(w);low[v]=min(low[v],low[w])
        elif w in on: low[v]=min(low[v],idx[w])
    if low[v]==idx[v]:
        c=[]
        while True:
            w=st.pop();on.discard(w);c.append(w)
            if w==v:break
        comps.append(c)
for v in list(nodes):
    if v not in idx: visit(v)
rows=sorted(((sum(lines[x] for x in c),sum(files[x] for x in c),len(c),c) for c in comps),reverse=True)
for l,f,n,c in rows[:int(os.environ.get("N","8"))]:
    print("SCC nodes",n,"files",f,"lines",l, [x[-45:] for x in c[:3]])
