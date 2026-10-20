"""🧪 Ordered carve stages P0 to P4 with chain depth and mean reach. Run with `cwd` = the generated folder."""
from sim2 import *
import json,collections,sys
H=json.load(open('hygiene.json'))
def P1(E):
    E=clone(E)
    for d,t,k,u in H:
        if k=='normal' and u in ('none','test'): E[d].discard(t)
    return E
def S3o(E):
    """🗂️ The only inversion the orphan chain allows: ordered leaves the intrinsic closure."""
    E=clone(E);E[N['intrinsic']].discard(N['ordered']);E[N['ordered']].add(N['intrinsic']);return E
def S3b(E):
    E=clone(E);E[N['retained-clone']].discard(N['paged-list']);E[N['paged-list']].add(N['retained-clone']);return E
def chain(E):
    r=revof(E)
    import sys
    sys.setrecursionlimit(1000000)
    memo={}
    def h(x):
        if x in memo: return memo[x]
        memo[x]=0
        v=1+max((h(y) for y in r.get(x,())),default=0)
        memo[x]=v;return v
    return h
def allreach(E,sample=None):
    # mean transitive dependents over all crates: use bitset approach on DAG topo order
    import itertools
    nodes=set(E)|{b for bs in E.values() for b in bs}
    r=revof(E)
    # topological: reach(x)=union of reach(y)+y for y in r[x]; use python ints as bitsets
    idx={n:i for i,n in enumerate(sorted(nodes,key=str))}
    memo={}
    sys.setrecursionlimit(1000000)
    def f(x):
        if x in memo: return memo[x]
        memo[x]=0
        b=0
        for y in r.get(x,()):
            b|=f(y)|(1<<idx[y])
        memo[x]=b;return b
    tot=0;cnt=0
    for n in nodes:
        tot+=bin(f(n)).count('1');cnt+=1
    return tot/cnt
if __name__=='__main__':
    stages=[]
    E=base;stages.append(('P0 now',E))
    E=P1(E);stages.append(('P1 hygiene',E))
    E=S1(E);stages.append(('P2 internal dead',E))
    E=S3o(E);stages.append(('P3 ordered wire',E))
    E5,unk,drop=S5(S4(E));stages.append(('P4 tiers+derive split',E5))
    keys={'control-contract':N['control-contract'],'allocation-return':N['allocation-return'],'retirement':N['retirement'],'shared-issuance':N['shared-issuance'],'native-encoding':N['native-encoding'],'native-decoding':N['native-decoding'],'intrinsic(ABI)':N['intrinsic'],'clone-source':N['clone-source'],'retained-clone':N['retained-clone'],'paged-list':N['paged-list'],'ordered':N['ordered'],'persistent':N['persistent'],'native-owned':N['native-owned'],'factory':N['factory'],'edit':N['edit'],'registry':N['registry'],'schema':N['schema'],'retained-map':N['retained-map'],'derive-expansion':N['derive-expansion'],'old derive':N['derive'],'FACADE':FACADE,'dcodec':'NEW:dcodec','dcanon':'NEW:dcanon','dretire':'NEW:dretire','dretained':'NEW:dretained','dres':'NEW:dres','TX paged/utf8':'NEW:TX','TY type':'NEW:TY','hash':HASH,'schema-registry':SREG,'snapshot-read':N['snapshot-read'],'dictionary':N['dictionary'],'paged-text':N['paged-text'],'snapshot-projection':N['snapshot-projection'],'original-aliases':N['original-aliases'],'retirement-queue':N['retirement-queue']}
    print('%-20s'%'node',*['%12s'%s[:12] for s,_ in stages])
    tab={}
    for lab,k in keys.items():
        row=[tr(E,[k])[k] if (k in E or any(k in v for v in E.values())) else 0 for _,E in stages]
        tab[lab]=row
        print('%-20s'%lab,*['%12d'%v for v in row])
    json.dump(tab,open('sim3.json','w'))
    # chain depth from control-contract
    for lab,E in stages:
        h=chain(E)
        print(lab,'longest chain from control-contract',h(N['control-contract']),'from intrinsic',h(N['intrinsic']))
    for lab,E in stages:
        print(lab,'mean reach over all nodes',round(allreach(E),1),flush=True)
