import json,os,sys
from groups import *
from testassign import *
S=f'{PRISTINE}/🗄️storage'
FILES={'core':S+'/🦀️.rs','writer':S+'/🔐️writer/🦀️.rs','release':S+'/🔐️writer/🔔️release/🦀️.rs'}
CUT1=int(os.environ.get('CUT1','2933')); CUT2=int(os.environ.get('CUT2','4636'))
def region(fid,it):
    if fid!='core': return 'kernel'
    h=it.header+1
    if h<5688: return 'kernel'
    if h<5889: return 'contract'
    if h<6567: return 'dispatch'
    if h<7698: return 'memory'
    if h<8961: return 'fs'
    return None
def kernel_layers():
    nodes=load(FILES)
    K={}
    for k,n in nodes.items():
        gs={region(f,it) for f,it in n.parts if it.kind!='impl'} or {region(*n.parts[0])}
        if 'kernel' in gs and not k.startswith('tests@') and k not in ('writer','release'): K[k]=n
    g=edges(K)
    comps=sccs(g)
    cum=0; lay={}
    for c in comps:
        n=sum(K[x].lines for x in c); 
        grp='io_core' if cum+n<=CUT1 else ('io_registry' if cum+n<=CUT2 else 'io_driver')
        for x in c: lay[x]=grp
        cum+=n
    return lay
LAY=kernel_layers()
OV={}
def assign(fid,it):
    nm=it.self_type if it.kind=='impl' else it.name
    r=region(fid,it)
    if r=='kernel': return LAY.get(nm) or 'io_driver'
    if r=='contract': return 'io_driver'
    return r
ORDER=['io_core','io_registry','io_driver','memory','fs','dispatch']
if __name__=='__main__':
    nodes,G,g=check(FILES,assign,show=5)
    tp=[S+'/🧪️tests/🔬️db-io-retained-fixtures/🦀️.rs',S+'/🧪️tests/🔬️unit/🦀️.rs',S+'/🔐️writer/🧪️tests/🔬️fence-conformance/🦀️.rs',S+'/🔐️writer/🧪️tests/🔬️unit/🦀️.rs',S+'/🧪️tests/🔬️fs-storage-directory-durability/🦀️.rs']
    res,allitems,direct=assign_tests(tp,G,nodes,ORDER)
    l=collections.Counter(); lt=collections.defaultdict(collections.Counter)
    for (p,i),gname in res.items():
        it=allitems[p][0][i]; l[gname]+=it.end-it.start+1; lt[os.path.basename(os.path.dirname(p))][gname]+=it.end-it.start+1
    print(l)
    for k,v in lt.items(): print(k,dict(v))
