"""🧅 Hint-based dependency layering. Usage: layer.py labels.py path:lo:hi ...
labels.py: LABELS=[(label,regex)], HINT={label:level}. Prints per-level sizes and the items pulled above their hint."""
import sys,re,collections,os
from scc2 import analyse,sccs
def run(labels_py,slices,show=True):
    ns={}; exec(open(labels_py).read(),ns)
    labels=[(l,re.compile(r)) for l,r in ns['LABELS']]; HINT=ns['HINT']
    nodes,g=analyse(slices)
    lab={}
    for k in nodes:
        path=nodes[k]['ranges'][0][0]
        l='other'
        for ln,rx in labels:
            if rx.search(k) or rx.search(os.path.dirname(path)+'/'+k): l=ln;break
        lab[k]=l
    comps=sccs(g)
    cid={}
    for i,c in enumerate(comps):
        for x in c: cid[x]=i
    level={}
    # comps are emitted dependencies-first
    for i,c in enumerate(comps):
        lv=max(HINT[lab[x]] for x in c)
        for x in c:
            for w in g[x]:
                if cid[w]!=i: lv=max(lv,level[cid[w]])
        level[i]=lv
    per=collections.defaultdict(lambda:[0,collections.Counter()])
    pulled=collections.defaultdict(list)
    for k in nodes:
        lv=level[cid[k]]; per[lv][0]+=nodes[k]['n']; per[lv][1][lab[k]]+=nodes[k]['n']
        if lv!=HINT[lab[k]]: pulled[(lab[k],lv)].append((k,nodes[k]['n']))
    if show:
        for lv in sorted(per): print('LEVEL',lv,per[lv][0],dict(per[lv][1]))
        for (l,lv),items in sorted(pulled.items()):
            print('pulled',l,'->',lv,sum(n for _,n in items),sorted(items,key=lambda x:-x[1])[:8])
    return nodes,g,lab,cid,level
if __name__=='__main__':
    sl=[]
    for a in sys.argv[2:]:
        p,lo,hi=a.rsplit(':',2); sl.append((p,int(lo),int(hi)))
    run(sys.argv[1],sl)
