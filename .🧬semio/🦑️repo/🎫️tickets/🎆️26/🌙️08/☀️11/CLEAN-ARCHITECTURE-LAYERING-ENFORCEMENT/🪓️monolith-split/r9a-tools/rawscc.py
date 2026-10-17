import json,re,sys,collections,os
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
d=json.load(open(F+"/🗑️generated/"+sys.argv[1]))
b=d['blockers'][int(os.environ.get("BI","0"))]
useT=os.environ.get("TEST","1")=="1"
adj=collections.defaultdict(set); why=collections.defaultdict(list)
for e in b['edges']:
    m=re.match(r"(.*?) -> (.*?)( \[test\])?(?: \((.*)\))?$",e)
    a,c,t=m.group(1),m.group(2),bool(m.group(3))
    if t and not useT: continue
    adj[a].add(c); why[(a,c)].append(m.group(4) or "")
    adj[c]
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
print("SCC sizes",[len(c) for c in comps[:8]])
big=set(comps[int(os.environ.get("K","0"))]) if comps else set()
if len(sys.argv)>2 and sys.argv[2]=="nodes":
    for x in sorted(big): print(x)
else:
    out=collections.Counter();inn=collections.Counter()
    for a in big:
        for c in adj[a]:
            if c in big: out[a]+=1; inn[c]+=1
    print("top out:",[(k[-60:],v) for k,v in out.most_common(12)])
    print("top in:",[(k[-60:],v) for k,v in inn.most_common(12)])
if len(sys.argv)>2 and sys.argv[2]=="edges":
    for a in sorted(big):
        for c in sorted(adj[a]):
            if c in big: print(a[-48:]," -> ",c[-48:], "|", why[(a,c)][0][-150:] if why[(a,c)] else "")
