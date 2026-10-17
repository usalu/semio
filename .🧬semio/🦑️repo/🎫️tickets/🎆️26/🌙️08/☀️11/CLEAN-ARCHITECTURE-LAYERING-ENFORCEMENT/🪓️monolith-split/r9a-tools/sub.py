import json,re,sys,collections,os
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
d=json.load(open(F+"/🗑️generated/"+sys.argv[1]))
rx=re.compile(sys.argv[2]); excl=re.compile(sys.argv[3]) if len(sys.argv)>3 and sys.argv[3] else None
b=d['blockers'][0]
size={}
for g in b['groupReport']:
    m=re.match(r"slice (.*?) \[(.*?)\] files=(\d+) bytes=(\d+) lines=(\d+) -> (.*)",g)
    if m:
        for nm in m.group(1).split(" + "): size[nm]=(int(m.group(3)),int(m.group(5)))
edges=[]
for e in b['edges']:
    m=re.match(r"(.*?) -> (.*?)( \[test\])?(?: \((.*)\))?$",e)
    edges.append((m.group(1),m.group(2),bool(m.group(3)),m.group(4) or ""))
ok=lambda n: rx.search(n) and not (excl and excl.search(n))
adj=collections.defaultdict(set); nodes=set(); why={}
for a,bb,t,w in edges:
    if ok(a) and ok(bb) and a!=bb:
        adj[a].add(bb); nodes|={a,bb}; why[(a,bb)]=w
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
print("nodes",len(nodes),"edges",sum(len(v) for v in adj.values()))
for c in sorted(comps,key=len,reverse=True)[:6]:
    if len(c)<2: continue
    tot=[0,0]
    for n in c:
        s=None
        for k,v in size.items():
            if n.endswith(k) or k.endswith(n): s=v;break
        if s: tot[0]+=s[0];tot[1]+=s[1]
    print("SCC",len(c),"files~",tot[0],"lines~",tot[1])
    if os.environ.get("LIST"):
        for n in sorted(c): print("   ",n)
