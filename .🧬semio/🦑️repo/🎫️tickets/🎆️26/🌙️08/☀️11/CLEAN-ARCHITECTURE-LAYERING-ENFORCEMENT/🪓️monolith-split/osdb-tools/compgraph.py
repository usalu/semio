"""🧩 Component graph from name-pattern labels. Usage: compgraph.py labels.py path:lo:hi ...
labels.py defines LABELS = [(label, regex)] first match wins; default label 'other'."""
import sys,re,collections,os
from scc2 import analyse,sccs
def main():
    ns={}
    exec(open(sys.argv[1]).read(),ns)
    labels=[(l,re.compile(r)) for l,r in ns['LABELS']]
    sl=[]
    for a in sys.argv[2:]:
        p,lo,hi=a.rsplit(':',2); sl.append((p,int(lo),int(hi)))
    nodes,g=analyse(sl)
    lab={}
    for k in nodes:
        path=nodes[k]['ranges'][0][0]
        key=k
        l='other'
        for ln,rx in labels:
            if rx.search(key) or rx.search(os.path.dirname(path)+'/'+key): l=ln;break
        lab[k]=l
    size=collections.Counter()
    for k in nodes: size[lab[k]]+=nodes[k]['n']
    edges=collections.defaultdict(lambda: collections.defaultdict(list))
    for k in g:
        for w in g[k]:
            if lab[k]!=lab[w]: edges[lab[k]][lab[w]].append((k,w))
    print('sizes',dict(size))
    for a in edges:
        for b,l in edges[a].items():
            print(f'{a} -> {b}: {len(l)}  e.g. {l[:3]}')
    # component-level SCC
    cg={l:set() for l in size}
    for a in edges:
        for b in edges[a]: cg[a].add(b)
    comps=sccs(cg)
    print('comp SCCs:',[c for c in comps if len(c)>1])
main()
