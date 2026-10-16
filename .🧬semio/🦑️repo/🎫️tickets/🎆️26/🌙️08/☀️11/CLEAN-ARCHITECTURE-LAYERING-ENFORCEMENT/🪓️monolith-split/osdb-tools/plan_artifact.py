from groups import *
from testassign import *
A=f'{PRISTINE}/🗿️artifact/🦀️.rs'
OV={'MAX_RECENT_TOUCHES':'command','MAX_STATE_PAGE_VALUE_BYTES':'engine','ArtifactMessage':'receipt','wire_err':'command'}
def assign(fid,it):
    nm=it.self_type if it.kind=='impl' else it.name
    if nm in OV: return OV[nm]
    h=it.header+1
    if h<=427: return 'command'
    if h<=1047: return 'receipt'
    if h<=2927: return 'engine'
    if h<=3096: return 'engine'
    if h<=4669: return 'history'
    if h<=6368: return 'actor'
    return None
ORDER=['command','receipt','history','engine','actor']
if __name__=='__main__':
    nodes,G,g=check({'artifact':A},assign,show=6)
    tp=[f'{PRISTINE}/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs',f'{PRISTINE}/🗿️artifact/🧪️tests/⚔️concurrent-write/🦀️.rs']
    res,allitems,direct=assign_tests(tp,G,nodes,ORDER)
    l=collections.Counter()
    for (p,i),gname in res.items():
        it=allitems[p][0][i]; l[gname]+=it.end-it.start+1
    print(l)
