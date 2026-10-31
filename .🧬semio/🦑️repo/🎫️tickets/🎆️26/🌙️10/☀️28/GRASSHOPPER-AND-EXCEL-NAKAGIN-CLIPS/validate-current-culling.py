"""🛡️ Compare fast visibility with exact subtraction on identical current local-grouped prototype Items."""
from contextlib import ExitStack
import hashlib
import importlib
import importlib.metadata
import json
from pathlib import Path
import platform
import sys
import time
from unittest.mock import patch
import numpy as np
sys.stdout.reconfigure(encoding='utf-8',errors='backslashreplace',line_buffering=True)
TICKET=Path(__file__).resolve().parent
ROOT=next(p for p in TICKET.parents if (p/'🧰️framework').is_dir())
sys.path.insert(0,str(ROOT))
V=importlib.import_module('🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️')
BASE=ROOT/'🎓️teaching/🏛️architecture/🏢️nakagin'
FILES=[*V.source_files(),BASE/'🐍️.py',BASE/'🔣️.json',BASE/'🧬️schema/🔣️.schema.json',Path(V._d.__file__),Path(V._art.__file__),Path(V._theme.__file__),BASE/'🧪️cases/👁️occlusion/🔣️.json',BASE/'🧪️cases/👁️occlusion/🔣️.schema.json',Path(__file__)]
def hashes():
    """🪪 Bind exact code, physical data and contract inputs at both endpoints."""
    return {str(path.resolve().relative_to(ROOT)):hashlib.sha256(path.read_bytes()).hexdigest() for path in FILES}
before=hashes()
print('[DEBUG] Preparing ONE current prototype without cull/order/draw',flush=True)
parts=V._cap_parts(V._cap(V._model.capsule_type('A')))
items=V._assembly_items(parts,base=V.CAPSULE,groups={},lod=True,glass_opacity=1)
view=V.make_view((55,24),(0,0,0),1,(0,2,1.25))
identity={id(item):index for index,item in enumerate(items)}
results=[]
for resolution in (256,512):
    padding=.45*.005+2*6/resolution
    record={'resolution':resolution,'padding':padding}
    for label,exact in (('optimized',False),('exact_subtraction',True)):
        print('[DEBUG] Current visibility '+label+' '+str(resolution)+' / '+str(len(items))+' Items',flush=True)
        V._OPAQUE_VISIBILITY.clear()
        start=time.perf_counter()
        with ExitStack() as stack:
            if exact:
                stack.enter_context(patch.object(V,'_single_cover',return_value=False))
                stack.enter_context(patch.object(V,'_coverage_samples_possible',return_value=True))
            clipping=stack.enter_context(patch.object(V,'_convex_difference',wraps=V._convex_difference))
            visible=V._opaque_visible(items,view,padding=padding)
        record[label]={'indices':[identity[id(item)] for item in visible],'seconds':time.perf_counter()-start,'clip_calls':clipping.call_count}
        print('[DEBUG] '+label+' retained='+str(len(visible))+' seconds='+str(record[label]['seconds']),flush=True)
    record['identical']=record['optimized']['indices']==record['exact_subtraction']['indices']
    results.append(record)
    (TICKET/'🗑️generated/visibility-current-culling-progress.json').write_text(json.dumps({'before':before,'complete_resolutions':results},ensure_ascii=False,indent=2),encoding='utf-8')
after=hashes()
record={'source_items':len(items),'same_current_input_items':True,'results':results,'before':before,'after':after,'unchanged':before==after,'runtime':{'python':sys.version,'executable':sys.executable,'platform':platform.platform(),'numpy':importlib.metadata.version('numpy'),'manim':importlib.metadata.version('manim')},'limits':'One current prototype at two padding scales; no historical Item-index comparison across regrouping, no Cairo capture/order, no140-slot census.'}
(TICKET/'🗑️generated/visibility-current-culling.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print('[DEBUG] Exact retained current Item identities equal:'+str(all(row['identical'] for row in results))+'; source endpoints unchanged:'+str(record['unchanged']),flush=True)
raise SystemExit(0 if record['unchanged'] and all(row['identical'] for row in results) else 1)
