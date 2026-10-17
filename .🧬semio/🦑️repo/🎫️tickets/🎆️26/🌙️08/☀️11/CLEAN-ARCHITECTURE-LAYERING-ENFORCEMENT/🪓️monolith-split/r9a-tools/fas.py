import json,re,sys,collections,os
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
d=json.load(open(F+"/🗑️generated/"+sys.argv[1]))
b=d['blockers'][int(os.environ.get("BI","0"))]
skip=os.environ.get("SKIP","")
par={}
def find(x):
    while par.get(x,x)!=x:
        par[x]=par.get(par[x],par[x]); x=par[x]
    return x
def union(a,c):
    a,c=find(a),find(c)
    if a!=c: par[c]=a
for c in b['causes']:
    m=re.match(r"coherence\((.*)\): (.*) <-> (.*)",c)
    if m: union(m.group(2),m.group(3))
edges=[]
for e in b['edges']:
    m=re.match(r"(.*?) -> (.*?)( \[test\])?(?: \((.*)\))?$",e)
    edges.append((m.group(1),m.group(2),bool(m.group(3)),m.group(4) or ""))
adj=collections.defaultdict(collections.Counter); radj=collections.defaultdict(collections.Counter); why=collections.defaultdict(list)
nodes=set()
for a,c,t,w in edges:
    fa,fc=find(a),find(c)
    nodes|={fa,fc}
    pk="::".join(c.split("::")[-2:])
    if os.environ.get("NOPC","1")=="1" and (pk+"::") in a and fa!=fc: continue
    if fa!=fc:
        adj[fa][fc]+=1; radj[fc][fa]+=1; why[(fa,fc)].append((a,c,t,w))
# contract 2-cycles (parent <-> child mount couplings merge in the planner anyway)
if os.environ.get("C2","1")=="1":
    for rnd in range(6):
        pairs=[(a,c) for a in list(adj) for c in list(adj[a]) if a!=c and a in adj.get(c,{}) and find(a)!=find(c)]
        if not pairs: break
        for a,c in pairs: union(a,c)
        adj2=collections.defaultdict(collections.Counter); radj2=collections.defaultdict(collections.Counter); why2=collections.defaultdict(list)
        for a in list(adj):
            for c,n in adj[a].items():
                fa,fc=find(a),find(c)
                if fa!=fc:
                    adj2[fa][fc]+=n; radj2[fc][fa]+=n; why2[(fa,fc)]+=why[(a,c)]
        adj,radj,why=adj2,radj2,why2
        nodes={find(x) for x in nodes}
# SCC
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
big=max(comps,key=len); S=set(big)
print("SCC",len(S),"of",len(nodes))
# Eades-Lin-Smyth ordering
outd={v:sum(n for w,n in adj[v].items() if w in S) for v in S}
ind={v:sum(n for w,n in radj[v].items() if w in S) for v in S}
rem=set(S); s1=[];s2=[]
while rem:
    changed=True
    while changed:
        changed=False
        for v in list(rem):
            if outd[v]==0:
                s2.append(v);rem.discard(v);changed=True
                for u,n in radj[v].items():
                    if u in rem: outd[u]-=n
            elif ind[v]==0:
                s1.append(v);rem.discard(v);changed=True
                for u,n in adj[v].items():
                    if u in rem: ind[u]-=n
    if rem:
        v=max(rem,key=lambda x:outd[x]-ind[x])
        s1.append(v);rem.discard(v)
        for u,n in adj[v].items():
            if u in rem: ind[u]-=n
        for u,n in radj[v].items():
            if u in rem: outd[u]-=n
order=s1+s2[::-1]; pos={v:i for i,v in enumerate(order)}
back=[(a,c) for a in S for c in adj[a] if c in S and pos[c]<pos[a]]
print("back arcs",len(back),"of",sum(n for a in S for c,n in adj[a].items() if c in S))
rows=[]
for a,c in back:
    for (x,y,t,w) in why[(a,c)][:1]: rows.append((("T" if t else "L"),x[-60:],y[-60:],w[:140]))
for r in sorted(rows,key=lambda r:(r[1],r[2]))[:int(os.environ.get("N","80"))]: print(*r,sep=" | ")
