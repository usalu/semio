import importlib,sys,time,json
from pathlib import Path
t=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🪜️stairs');seen=set();records=[];start=time.monotonic()
def trace(frame,event,arg):
 if frame.f_code.co_name!='test_shared_slab_void_is_explicit_and_union_owned':return None
 if event=='line' and frame.f_lineno not in seen:
  seen.add(frame.f_lineno);records.append({'line':frame.f_lineno,'seconds':time.monotonic()-start});Path("/repo/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES/🗑️generated/shared-proof-stages.json").write_text(json.dumps(records))
 return trace
sys.settrace(trace)
t.test_shared_slab_void_is_explicit_and_union_owned()
sys.settrace(None);print('Shared void scoped proof PASS',time.monotonic()-start)
