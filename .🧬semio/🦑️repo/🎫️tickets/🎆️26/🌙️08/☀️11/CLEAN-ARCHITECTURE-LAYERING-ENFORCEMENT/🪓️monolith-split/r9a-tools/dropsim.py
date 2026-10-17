import json,re,collections,sys,os
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
d=json.load(open(F+"/🗑️generated/"+sys.argv[1]))
b=d['blockers'][int(os.environ.get("BI","0"))]
drops=[re.compile(x) for x in sys.argv[2:]]
adj=collections.defaultdict(set)
for e in b['edges']:
    m=re.match(r"(.*?) -> (.*?)( \[test\])?(?: \((.*)\))?$",e)
    a,c,t=m.group(1),m.group(2),bool(m.group(3))
    w=m.group(4) or ""
    if t and any(r.search(a+" "+w) for r in drops): continue
    adj[a].add(c); adj[c]
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
print([len(c) for c in comps[:6]])
size={}
for g in b['groupReport']:
    m=re.match(r"slice (.*?) \[.*?\] files=(\d+) bytes=(\d+) lines=(\d+)",g)
    if m: size[m.group(1)]=(int(m.group(2)),int(m.group(3)),int(m.group(4)))
for c in comps[:int(os.environ.get("N","4"))]:
    f=sum(size.get(x,(0,0,0))[0] for x in c); by=sum(size.get(x,(0,0,0))[1] for x in c); l=sum(size.get(x,(0,0,0))[2] for x in c)
    print("ring",len(c),"files",f,"bytes",by,"lines",l, os.environ.get("SHOW") and sorted(x[-40:] for x in c))
