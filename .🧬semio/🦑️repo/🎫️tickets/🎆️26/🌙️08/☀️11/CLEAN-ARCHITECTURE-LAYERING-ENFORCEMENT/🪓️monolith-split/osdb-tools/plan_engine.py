from groups import *
from testassign import *
E=f'{PRISTINE}/⚙️engine/🦀️.rs'
OV={'DATABASE_CAPABILITY_OPEN_LIVE':'capopen','DATABASE_CATALOG_READ_LIVE':'catread','DATABASE_CATALOG_BOOTSTRAP_LIVE':'catboot','DATABASE_CREATE_CATALOG_LIVE':'createcat',
 'HistoryView':'handle','DatabaseCreateCatalogToken':'base','ArtifactHandleLease':'handle','ArtifactCompactionFuture':'handle','ArtifactHandleJournalSinkV1':'handle',
 'DATABASE_DOCUMENT_MOUNT_WAITERS':'base','DATABASE_DOCUMENT_MOUNT_WORK_LIVE':'base','database_document_mount_work_live_slots':'base'}
def assign(fid,it):
    nm=it.self_type if it.kind=='impl' else it.name
    if nm in OV: return OV[nm]
    if nm.startswith('DatabaseDocumentMount'): return 'base'
    h=it.header+1
    if h<=1333: return 'capopen' if h>=53 else 'core'
    if h<=2639: return 'catread'
    if h<=4040: return 'catboot'
    if h<=4638: return 'base'
    if h<=6718: return 'createcat'
    if h<=8247: return 'core'
    if h<=10066: return 'handle'
    return None
ORDER=['base','capopen','catread','catboot','createcat','handle','core']
if __name__=='__main__':
    nodes,G,g=check({'engine':E},assign,show=6)
    tp=[f'{PRISTINE}/⚙️engine/🧪️tests/🔬️unit/🦀️.rs',f'{PRISTINE}/⚙️engine/🧪️tests/⏱️throughput/🦀️.rs']
    res,allitems,direct=assign_tests(tp,G,nodes,ORDER)
    l=collections.Counter()
    for (p,i),gname in res.items():
        it=allitems[p][0][i]; l[gname]+=it.end-it.start+1
    print(l)
