"""🔎 Capture the earliest first-party invalid polygon during shared-wall ownership."""
import importlib,json,sys,traceback,hashlib
from pathlib import Path
from shapely.geometry import Polygon
from shapely.validation import explain_validity
ROOT=next(p for p in Path(__file__).resolve().parents if (p/'package.json').is_file())
sys.path.insert(0,str(ROOT))
sys.dont_write_bytecode=True
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
analysis=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧮️analysis.🐍️')
OUT=Path(__file__).parent/'🗑️generated/native-r92/shared-invalid'
OUT.mkdir(parents=True,exist_ok=True)
def wrapper(name,original):
    """🧪 Observe exact production polygons without altering the returned geometry."""
    def run(*args,**kwargs):
        result=original(*args,**kwargs)
        rings=result if name in ('_loops','_subtract','_convex_parts') else [result]
        for ring in rings:
            if len(ring)>=3 and not Polygon(ring).is_valid:
                witness={'function':name,'args':args,'kwargs':kwargs,'result':result,'invalid':ring,'validity':explain_validity(Polygon(ring)),'stack':traceback.format_stack(),'api_sha256':hashlib.sha256(Path(api.__file__).read_bytes()).hexdigest(),'analysis_sha256':hashlib.sha256(Path(analysis.__file__).read_bytes()).hexdigest()}
                (OUT/'first-invalid.json').write_text(json.dumps(witness,indent=2),encoding='utf-8')
                raise AssertionError(f'First invalid production ring in {name}: '+witness['validity'])
        return result
    return run
for name in ('_half','_clip','_ccw','_subtract','_loops','_convex_parts'):
    setattr(analysis,name,wrapper(name,getattr(analysis,name)))
model=api.load_model()
geometry=api._wall_geometry(model,api.element('W-SHARED-BATH-N',model))
(OUT/'result.json').write_text(json.dumps(geometry,indent=2),encoding='utf-8')
print('[DEBUG] Shared wall all intermediate polygons valid',flush=True)
