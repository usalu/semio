import json,re,collections,sys,os,functools
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
d=json.load(open(F+"/🗑️generated/"+sys.argv[1]))
b=d['blockers'][int(os.environ.get("BI","0"))]
adj=collections.defaultdict(set); tst={}
for e in b['edges']:
    m=re.match(r"(.*?) -> (.*?)( \[test\])?(?: \((.*)\))?$",e)
    a,c,t=m.group(1),m.group(2),bool(m.group(3))
    adj[a].add(c); adj[c]
    if t: tst[(a,c)]=m.group(4)
sys.setrecursionlimit(100000)
def scc(adjf):
    idx={};low={};st=[];on=set();cnt=[0];comps=[]
    def visit(v):
        idx[v]=low[v]=cnt[0];cnt[0]+=1;st.append(v);on.add(v)
        for w in adjf[v]:
            if w not in idx: visit(w);low[v]=min(low[v],low[w])
            elif w in on: low[v]=min(low[v],idx[w])
        if low[v]==idx[v]:
            c=[]
            while True:
                w=st.pop();on.discard(w);c.append(w)
                if w==v:break
            comps.append(c)
    for v in list(adjf):
        if v not in idx: visit(v)
    return comps
libadj=collections.defaultdict(set)
for a in adj:
    libadj[a]
    for c in adj[a]:
        if (a,c) not in tst: libadj[a].add(c)
lc=scc(libadj); comp={}
for i,c in enumerate(lc):
    for x in c: comp[x]=i
full=scc(adj); big=set(max(full,key=len))
cadj=collections.defaultdict(set)
for a in libadj:
    for c in libadj[a]:
        if comp[a]!=comp[c]: cadj[comp[a]].add(comp[c])
@functools.lru_cache(None)
def reachable(i):
    r=set()
    for j in cadj[i]: r|={j}|reachable(j)
    return frozenset(r)
print("full ring",len(big))
for (a,c),w in sorted(tst.items()):
    if a in big and c in big and (comp[a]==comp[c] or comp[a] in reachable(comp[c])):
        print(a[-52:],"->",c[-52:],"|",(w or "")[-130:])
