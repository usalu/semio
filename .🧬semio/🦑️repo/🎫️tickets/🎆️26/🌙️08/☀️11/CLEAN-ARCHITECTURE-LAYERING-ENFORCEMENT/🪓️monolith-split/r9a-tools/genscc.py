import json,re,collections,sys,os
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
d=json.load(open(F+"/🗑️generated/"+sys.argv[1]))
b=d['blockers'][int(os.environ.get("BI","2"))]
size={}
for g in b['groupReport']:
    m=re.match(r"slice (.*?) \[.*?\] files=(\d+) bytes=(\d+) lines=(\d+)",g)
    if m: size[m.group(1).split(" + ")[0]]=int(m.group(4))
par={}
def find(x):
    while par.get(x,x)!=x: x=par[x]
    return x
skip=os.environ.get("SKIPCOH","visibility")
for c in b['causes']:
    m=re.match(r"coherence\((.*)\): (.*) <-> (.*)",c)
    if m and skip not in m.group(1):
        a,c2=find(m.group(2)),find(m.group(3))
        if a!=c2: par[c2]=a
E=collections.defaultdict(lambda: collections.defaultdict(list))
for e in b['edges']:
    m=re.match(r"(.*?) -> (.*?)( \[test\])?(?: \((.*)\))?$",e)
    a,c=find(m.group(1)),find(m.group(2))
    if m.group(3) or a==c: continue
    if ("::".join(c.split("::")[-2:])+"::") in a: continue
    E[a][c].append((m.group(4) or "")[:100])
sys.setrecursionlimit(10000)
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
nodes=set(E)|{c for v in E.values() for c in v}
for v in nodes:
    if v not in idx: visit(v)
big=set(max(comps,key=len))
print(len(big),sum(size.get(n,0) for n in big))
for n in sorted(big,key=lambda x:-size.get(x,0)):
    print(size.get(n,0),n[-62:],"->",sorted({c[-34:] for c in E[n] if c in big})[:6])
if os.environ.get("WHY"):
    for n in big:
        for c in E[n]:
            if c in big and (os.environ["WHY"] in n or os.environ["WHY"] in c): print("  ",n[-40:],"->",c[-40:],E[n][c][0])
