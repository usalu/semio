import sys; sys.path.insert(0,'.')
from dom7 import *
byname=collections.defaultdict(list)
for x in nodes: byname[x['name']].append(x)
def quotient(domf):
    edges=collections.defaultdict(list)
    for x in nodes:
        dx=domf(x)
        if dx=='APP': continue
        for c in x['calls']:
            for y in byname.get(c,[]):
                dy=domf(y)
                if dy!=dx: edges[(dx,dy)].append((x['name'],y['name']))
    return edges
def sccs(edges, doms):
    g=collections.defaultdict(set)
    for (a,b) in edges: g[a].add(b)
    idx={};low={};st=[];on=set();out=[];c=[0]
    def sc(v):
        idx[v]=low[v]=c[0];c[0]+=1;st.append(v);on.add(v)
        for w in g[v]:
            if w not in idx: sc(w);low[v]=min(low[v],low[w])
            elif w in on: low[v]=min(low[v],idx[w])
        if low[v]==idx[v]:
            comp=[]
            while True:
                w=st.pop();on.discard(w);comp.append(w)
                if w==v:break
            out.append(comp)
    for d in doms:
        if d not in idx: sc(d)
    return out
if __name__=='__main__':
    edges=quotient(dom)
    doms=sorted({dom(x) for x in nodes if dom(x)!='APP'})
    print(collections.Counter(dom(x) for x in nodes))
    for (a,b),v in sorted(edges.items()): print('%-8s -> %-8s %3d  %s'%(a,b,len(v),'; '.join('%s>%s'%p for p in v[:3])))
    print([s for s in sccs(edges,doms) if len(s)>1])
