"""cstep scanner: methods in impls of InteractiveJob/ArtifactCommandWork/FixedOperationOwner that the trait does not declare (and required ones missing)."""
import re,sys,os
sys.path.insert(0,os.path.dirname(__file__))
from rs import mask,match_brace
from port_traits import IMPL,children,depth_array
T={'InteractiveJob':({'step','borrow_outcome','begin_close','close_step'},{'next_close_copy_byte_demand','next_close_capacity_byte_demand','next_close_release_byte_demand','next_close_depth_demand','close_demand_without_owner','register_close_wake','terminal_is_empty'}),
'ArtifactCommandWork':({'tool_id','extent','step'},{'receive_original_command','workspace_identity','work_demands','checkpoint_byte','restore','terminal_frame_release_bytes','reports_progress','begin_close','close_step','next_close_copy_byte_demand','next_close_capacity_byte_demand','next_close_release_byte_demand','next_close_depth_demand','close_demand_without_owner','terminal_is_empty'}),
'FixedOperationOwner':({'retained_bytes','cancel','begin_close','close_step','terminal_is_empty'},set())}
for p in open(sys.argv[1]).read().split('\n'):
    if not p: continue
    s=open(p,encoding='utf-8').read();m=mask(s);dep=depth_array(m)
    for im in IMPL.finditer(m):
        b=im.end()-1;e=match_brace(m,b);tr=im.group('trait')
        have={n for n,_,_,_ in children(m,dep,b,e)}
        req,opt=T[tr]
        extra=have-req-opt;missing=req-have
        if extra or missing:
            ln=s.count('\n',0,im.start())+1
            print(f"{p}:{ln}: {tr} extra={sorted(extra)} missing={sorted(missing)}")
