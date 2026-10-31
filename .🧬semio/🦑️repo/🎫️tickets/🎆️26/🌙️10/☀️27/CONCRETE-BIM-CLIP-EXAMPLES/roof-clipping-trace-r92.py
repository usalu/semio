"""✂️ Minimize the first invalid native roof residual operation without changing production geometry."""
import importlib,json,sys,traceback
from pathlib import Path
from shapely.geometry import Polygon
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True
a=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');an=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧮️analysis.🐍️');ticket=Path(__file__).parent;case=json.loads((ticket/'roof-residual-operands-r92.json').read_text(encoding='utf8'));original={name:getattr(an,name)for name in ['_half','_subtract','_loops']}
class Found(Exception):pass
def check(name,args,kwargs,result):
    """🔬 Preserve exact operands at the first positive-area invalid ring, before any repair attempt."""
    rings=[result]if name=='_half'else result
    bad=[r for r in rings if len(r)>=3 and Polygon(r).area>1e-12 and not Polygon(r).is_valid]
    if bad:
        out={'operation':name,'args':args,'kwargs':kwargs,'result':result,'invalid':bad};(ticket/'🗑️generated'/('roof-clipping-first-invalid-'+(sys.argv[1]if len(sys.argv)>1 else'rounded')+'-r92.json')).write_text(json.dumps(out,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({'operation':name,'invalid':bad}),flush=True);raise Found()
    return result
def wrap(name):
    """🪝 Keep the actual native operation and only observe its returned topology."""
    def call(*args,**kwargs):
        if len(sys.argv)>1 and sys.argv[1]=='unrounded' and name=='_subtract':
            args=list(args)
            if len(args)>2:args[2]=None
            else:kwargs={**kwargs,'digits':None}
        return check(name,args,kwargs,original[name](*args,**kwargs))
    return call
for name in original:setattr(an,name,wrap(name))
try:
    result=a._subtract_prisms(case['parts'],case['cutters']);print('[DEBUG] completed '+str(len(result)),flush=True)
except Found:pass
finally:
    for name,fn in original.items():setattr(an,name,fn)
