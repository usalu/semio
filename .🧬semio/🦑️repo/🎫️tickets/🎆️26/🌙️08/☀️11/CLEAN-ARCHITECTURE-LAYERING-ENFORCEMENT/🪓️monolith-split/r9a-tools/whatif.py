import json,re,sys,collections,os
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
d=json.load(open(F+"/🗑️generated/"+sys.argv[1]))
b=d['blockers'][int(os.environ.get('BI','0'))]
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
E=collections.defaultdict(set); why={}
nodes=set()
for e in b['edges']:
    m=re.match(r"(.*?) -> (.*?)( \[test\])?(?: \((.*)\))?$",e)
    a,c=find(m.group(1)),find(m.group(2)); nodes|={a,c}
    pk="::".join(c.split("::")[-2:])
    if (pk+"::") in a and a!=c and os.environ.get("NOPC","1")=="1": continue
    if a!=c: E[a].add(c); why[(a,c)]=m.group(4) or ""
sys.setrecursionlimit(100000)
def scc(E,nodes):
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
    for v in nodes:
        if v not in idx: visit(v)
    return comps
def biggest(E):
    return max(scc(E,nodes),key=len)
base=biggest(E)
print("SCC",len(base),"of",len(nodes))
grounded=[]
def ground(E,g):
    return {a:(set() if any(x in a for x in g) else set(c)) for a,c in E.items()}
cur=E
steps=int(sys.argv[2]) if len(sys.argv)>2 else 12
for step in range(steps):
    comp=set(biggest(cur))
    # candidates: nodes in comp with in-edges from comp
    indeg=collections.Counter()
    for a in comp:
        for c in cur.get(a,()):
            if c in comp: indeg[c]+=1
    best=None
    for cand,_ in indeg.most_common(40):
        trial={a:(set() if a==cand else set(c)) for a,c in cur.items()}
        size=len(biggest(trial))
        if best is None or size<best[0]: best=(size,cand)
    if best is None: break
    print("ground",best[1][-90:],"->",best[0], "(in-degree",indeg[best[1]],")")
    cur={a:(set() if a==best[1] else set(c)) for a,c in cur.items()}
    if best[0]<=1: break
