import json,sys,re
# usage: scc.py <nodes.json> [shortlen]: SCCs of the slice graph (edges only), with intra-SCC edge reasons
nodes=json.load(open(sys.argv[1]))
def key(n): return n
# map target short names to node names by suffix match
names=list(nodes)
idx={}
for n in names:
    idx.setdefault(n,n)
def find(t):
    if t in nodes: return t
    c=[n for n in names if n.endswith(t) or t.endswith(n) or (' + ' in n and any(p.strip()==t or p.strip().endswith(t) for p in n.split(' + ')))]
    return c[0] if len(c)>=1 else None
G={n:set() for n in names}; why={}
for n,v in nodes.items():
    for t,w in v['out']:
        m=find(t)
        if m and m!=n: G[n].add(m); why[(n,m)]=w
sys.setrecursionlimit(100000)
I={};Lw={};st=[];on=set();res=[];c=[0]
def v(n):
    I[n]=Lw[n]=c[0];c[0]+=1;st.append(n);on.add(n)
    for m in G[n]:
        if m not in I: v(m);Lw[n]=min(Lw[n],Lw[m])
        elif m in on: Lw[n]=min(Lw[n],I[m])
    if Lw[n]==I[n]:
        comp=[]
        while True:
            x=st.pop();on.discard(x);comp.append(x)
            if x==n:break
        res.append(comp)
for n in names:
    if n not in I:v(n)
for comp in res:
    if len(comp)<2: continue
    print('SCC',len(comp),'lines',sum(nodes[x]['lines'] for x in comp),'files',sum(nodes[x]['files'] for x in comp))
    s=set(comp)
    for a in sorted(comp):
        for b in sorted(G[a]):
            if b in s and a!='editor::puzzle5d': print('   ',a[-45:],'->',b[-45:],'|',(why.get((a,b)) or '')[:90])
