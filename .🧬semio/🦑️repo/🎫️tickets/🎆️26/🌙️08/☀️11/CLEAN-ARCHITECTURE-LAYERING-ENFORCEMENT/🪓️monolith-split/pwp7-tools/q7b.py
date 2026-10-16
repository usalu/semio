import sys; sys.path.insert(0,'.')
from q7 import *
OV={}
def mv(dom,names):
    for n in names.split(): OV[n]=dom
mv('hist','reprojection_status finish_recorded verb_dirties_history_panel')
mv('prod','admit_original_tool_lease mount_original_reserved_owners admit_reserved_emit_producer test_reserved_emit_admission_case test_reserved_emit_cancelled_worker_handoff test_reserved_emit_busy_completion_owner dispatch_reserved_original_producer mount_original_emit_publication commit_framework_clipboard_completion admit_framework_reserved_spawn')
def dom2(x):
    return OV.get(x['name']) or dom(x)
if __name__=='__main__':
    edges=quotient(dom2)
    doms=sorted({dom2(x) for x in nodes if dom2(x)!='APP'})
    print(collections.Counter(dom2(x) for x in nodes))
    for (a,b),v in sorted(edges.items()): print('%-8s -> %-8s %3d  %s'%(a,b,len(v),'; '.join('%s>%s'%p for p in v[:4])))
    print([s for s in sccs(edges,doms) if len(s)>1])
