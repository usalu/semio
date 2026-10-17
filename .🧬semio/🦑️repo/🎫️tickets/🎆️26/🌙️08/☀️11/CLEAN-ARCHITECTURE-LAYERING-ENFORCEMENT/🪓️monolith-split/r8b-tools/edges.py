import sys,re,collections
lines=open(sys.argv[1],encoding='utf8').read().split('\n')
edges=[]
for l in lines:
    m=re.match(r'^    \{(.*?)\} => \{(.*?)\} via (.*)$',l)
    if m: edges.append((m.group(1),m.group(2),m.group(3)))
g=collections.defaultdict(list)
for a,b,v in edges: g[a].append((b,v))
nodes=sorted(set([e[0] for e in edges]+[e[1] for e in edges]))
print(len(nodes),'nodes',len(edges),'edges')
# tarjan
idx={};low={};st=[];on=set();res=[];c=[0]
sys.setrecursionlimit(10000)
def sc(v):
    idx[v]=low[v]=c[0];c[0]+=1;st.append(v);on.add(v)
    for w,_ in g[v]:
        if w not in idx: sc(w);low[v]=min(low[v],low[w])
        elif w in on: low[v]=min(low[v],idx[w])
    if low[v]==idx[v]:
        comp=[]
        while True:
            w=st.pop();on.discard(w);comp.append(w)
            if w==v:break
        res.append(comp)
for n in nodes:
    if n not in idx: sc(n)
print('sccs',[len(c) for c in res if len(c)>1])
if len(sys.argv)>2:
    for a,b,v in edges:
        if re.search(sys.argv[2],a) : print(a[:40],'=>',b[:40],'|',v[:200])
