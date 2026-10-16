"""📚 Topological layering of the condensation DAG with cumulative line counts and semantic group labels.
Usage: topo.py ranges.txt path:lo:hi ...   (ranges.txt: 'label start end' applying to the FIRST path only; other paths get their basename label)"""
import sys,os
from scc2 import analyse,sccs
def main():
    ranges=[]
    for l in open(sys.argv[1]):
        p=l.split()
        if len(p)==3: ranges.append((p[0],int(p[1]),int(p[2])))
    sl=[]
    for a in sys.argv[2:]:
        p,lo,hi=a.rsplit(':',2); sl.append((p,int(lo),int(hi)))
    nodes,g=analyse(sl)
    def label(k):
        path,s,e=nodes[k]['ranges'][0]
        if path==sl[0][0]:
            for lab,a,b in ranges:
                if a<=s<=b: return lab
        return os.path.basename(os.path.dirname(path))[:6]
    comps=sccs(g)
    cid={}
    for i,c in enumerate(comps):
        for x in c: cid[x]=i
    # Tarjan returns comps in reverse topological order of deps? (a comp is emitted after comps it reaches): so deps first
    order=list(range(len(comps)))
    cum=0
    runs=[]
    for i in order:
        c=comps[i]; n=sum(nodes[x]['n'] for x in c); cum+=n
        lab=label(c[0])
        if runs and runs[-1][0]==lab: runs[-1][1]+=n; runs[-1][2]+=1; runs[-1][3]=cum; runs[-1][4].append(c[0] if len(c)==1 else '{'+'+'.join(c[:3])+'}')
        else: runs.append([lab,n,1,cum,[c[0] if len(c)==1 else '{'+'+'.join(c[:3])+'}']])
    for lab,n,k,cum,names in runs: print(f'{cum:6d} {lab:10s} {n:5d} {k:3d}  {" ".join(names[:5])}')
main()
