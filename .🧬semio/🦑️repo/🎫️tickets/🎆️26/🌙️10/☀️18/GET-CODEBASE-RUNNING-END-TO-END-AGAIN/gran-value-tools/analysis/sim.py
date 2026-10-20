"""🧪 Graph surgery simulator: removes and adds edges and recomputes transitive dependents (stages S1 to S4). Run with `cwd` = the generated folder."""
from graph import *
import copy,collections,random,sys,json
def nm(s): return byname[s]
FACADE=nm('semio-framework-value')
N={k:nm('semio-framework-value-'+k) for k in ['clone-source','native-decoding','native-encoding','paged-list','allocation-return','edit','schema','registry','retirement','intrinsic','ordered','shared-issuance','control-contract','derive','persistent','original-aliases','retained-clone','native-owned','factory','retained-map','retirement-queue','paged-text','snapshot-projection','derive-expansion','snapshot-read','dictionary']}
HASH=nm('semio-framework-hash'); SREG=nm('semio-framework-schema-registry')
base=collections.defaultdict(set)
for a,bs in E_.items(): base[a]|=set(bs)
def revof(E):
    r=collections.defaultdict(set)
    for a,bs in E.items():
        for b in bs: r[b].add(a)
    return r
def tr(E,starts):
    r=revof(E);out={}
    for s in starts:
        seen=set();st=[s]
        while st:
            x=st.pop()
            for y in r.get(x,()):
                if y not in seen: seen.add(y);st.append(y)
        out[s]=len(seen)
    return out
def height(E):
    import functools
    sys.setrecursionlimit(100000)
    h={}
    def f(x):
        if x in h: return h[x]
        h[x]=0  # cycle guard
        v=1+max((f(y) for y in rev_cache.get(x,())),default=0)
        h[x]=v;return v
    return f
def clone(E): return collections.defaultdict(set,{k:set(v) for k,v in E.items()})
def S1(E):
    E=clone(E)
    for k in ['clone-source','native-decoding','paged-list','allocation-return','edit','schema','registry']: E[FACADE].discard(N[k])
    E[FACADE].discard(HASH);E[FACADE].discard(SREG)
    E[N['derive']].discard(HASH)
    E[N['edit']].discard(N['native-encoding'])
    E[N['persistent']].discard(N['original-aliases'])
    return E
def S2(E):
    E=clone(E);E[N['retirement']].discard(N['paged-list']);E[N['paged-list']].add(N['retirement']);return E
def S3(E):
    E=clone(E);E[N['intrinsic']].discard(N['ordered']);E[N['intrinsic']].discard(N['paged-list'])
    E[N['ordered']].add(N['intrinsic']);E[N['paged-list']].add(N['intrinsic']);return E
if __name__=='__main__':
    fam_names=['control-contract','allocation-return','paged-list','retirement','shared-issuance','native-encoding','native-decoding','ordered','intrinsic','clone-source','derive','retained-clone','persistent','native-owned','factory','schema','registry','edit','retained-map']
    starts=[N[k] for k in fam_names]+[FACADE,HASH,SREG,N['derive-expansion']]
    labels=fam_names+['FACADE','hash','schema-registry','derive-expansion']
    stages=[('S0',base)]
    E1=S1(base);stages.append(('S1 dead edges',E1))
    E2=S2(E1);stages.append(('S2 +ret/paged inv',E2))
    E3=S3(E2);stages.append(('S3 +intr/ord/pl inv',E3))
    rows=[tr(E,starts) for _,E in stages]
    print('%-20s'%'crate',*['%8s'%s[0] for s in stages])
    for i,l in enumerate(labels):
        print('%-20s'%l,*['%8d'%r[starts[i]] for r in rows])

def S4(E):
    E=clone(E)
    keep={'derive','intrinsic','retirement','shared-issuance','native-encoding','native-decoding','retained-clone','clone-source','control-contract','allocation-return'}
    E[FACADE]={t for t in E[FACADE] if t not in {N[k] for k in ['persistent','native-owned','factory','retained-map','ordered','paged-text','original-aliases','retirement-queue','snapshot-projection','paged-list','edit','schema','registry']}}
    E[FACADE].discard(HASH);E[FACADE].discard(SREG)
    return E
def run():
    fam_names=['control-contract','allocation-return','paged-list','retirement','shared-issuance','native-encoding','native-decoding','ordered','intrinsic','clone-source','derive','retained-clone','persistent','native-owned','factory','schema','registry','edit','retained-map','derive-expansion','paged-text','snapshot-projection','original-aliases','retirement-queue']
    starts=[N[k] for k in fam_names]+[FACADE,HASH,SREG]
    labels=fam_names+['FACADE','hash','schema-registry']
    stages=[('S0',base)]
    E=base
    for lab,f in [('S1',S1),('S2',S2),('S3',S3),('S4',S4)]:
        E=f(E);stages.append((lab,E))
    rows=[tr(E,starts) for _,E in stages]
    print('%-20s'%'crate',*['%7s'%s[0] for s in stages])
    for i,l in enumerate(labels):
        print('%-20s'%l,*['%7d'%r[starts[i]] for r in rows])
    return stages
if __name__=='__main__' and len(sys.argv)>1: run()
