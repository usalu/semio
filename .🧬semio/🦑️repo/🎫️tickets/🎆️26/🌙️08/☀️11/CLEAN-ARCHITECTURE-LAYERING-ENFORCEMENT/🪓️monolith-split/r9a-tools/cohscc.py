import json,re,sys,collections,os
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
d=json.load(open(F+"/🗑️generated/"+sys.argv[1]))
b=d['blockers'][int(os.environ.get("BI","0"))]
useT=os.environ.get("TEST","1")=="1"
par={}
def find(x):
    while par.get(x,x)!=x:
        par[x]=par.get(par[x],par[x]); x=par[x]
    return x
for c in b['causes']:
    m=re.match(r"coherence(?:\((.*)\))?: (.*) <-> (.*)",c)
    if m:
        a,cc=find(m.group(2)),find(m.group(3))
        if a!=cc: par[cc]=a
adj=collections.defaultdict(set); why=collections.defaultdict(list)
for e in b['edges']:
    m=re.match(r"(.*?) -> (.*?)( \[test\])?(?: \((.*)\))?$",e)
    a,c,t=find(m.group(1)),find(m.group(2)),bool(m.group(3))
    if t and not useT: continue
    if a==c: continue
    adj[a].add(c); adj[c]; why[(a,c)].append((m.group(1),m.group(2),t,m.group(4) or ""))
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
for v in list(adj):
    if v not in idx: visit(v)
comps.sort(key=len,reverse=True)
print("SCC sizes (coherence-contracted)",[len(c) for c in comps[:6]])
members=collections.defaultdict(list)
for x in list(par)+list(par.values()): members[find(x)].append(x)
big=set(comps[0])
if len(sys.argv)>2:
    # print union groups inside the ring
    for g in big:
        ms=members.get(g,[g])
        print(len(ms),"|"," + ".join(sorted(x[-34:] for x in ms))[:200])
