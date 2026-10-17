import json,collections,re,sys
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
d=json.load(open(F+"/🗑️generated/"+(sys.argv[1] if len(sys.argv)>1 else "r9a-blk0.json")))
b=d['blockers'][0]
skip=set(sys.argv[2].split("|")) if len(sys.argv)>2 and sys.argv[2] else set()
edges=[]
for e in b['edges']:
    m=re.match(r"(.*?) -> (.*?)( \[test\])?(?: \((.*)\))?$",e)
    edges.append((m.group(1),m.group(2),bool(m.group(3)),m.group(4) or ""))
par={}
def find(x):
    while par.get(x,x)!=x:
        par[x]=par.get(par[x],par[x]); x=par[x]
    return x
def union(a,b):
    a,b=find(a),find(b)
    if a!=b: par[b]=a
for c in b['causes']:
    m=re.match(r"coherence\((.*)\): (.*) <-> (.*)",c)
    if m:
        if any(s in m.group(1) for s in skip): continue
        union(m.group(2),m.group(3))
nodes=set()
adj=collections.defaultdict(set)
why={}
for a,bb,t,w in edges:
    fa,fb=find(a),find(bb)
    nodes.add(fa);nodes.add(fb)
    if fa!=fb:
        adj[fa].add(fb); why.setdefault((fa,fb),[]).append((a,bb,t,w))
sys.setrecursionlimit(100000)
idx={};low={};st=[];on=set();cnt=[0];comps=[]
def visit(v):
    idx[v]=low[v]=cnt[0];cnt[0]+=1;st.append(v);on.add(v)
    for w in adj[v]:
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
big=[c for c in comps if len(c)>1]
print("nodes",len(nodes),"sccs>1:",[len(c) for c in big])
members=collections.defaultdict(list)
for x in list(par)+list(nodes): members[find(x)].append(x)
for c in sorted(big,key=len,reverse=True)[:3]:
    print("== SCC of",len(c),"groups")
    cs=set(c)
    for g in c[:60]:
        print("  ",g[-90:],"(",len(members[g]),"slices )")
if len(sys.argv)>3:
    c=max(big,key=len); cs=set(c)
    out=collections.Counter()
    for g in c:
        for h in adj[g]:
            if h in cs:
                for (a,bb,t,w) in why[(g,h)][:1]:
                    print(("T " if t else "L ")+g[-60:]+"  ->  "+h[-60:]+"   :: "+w[:160])
