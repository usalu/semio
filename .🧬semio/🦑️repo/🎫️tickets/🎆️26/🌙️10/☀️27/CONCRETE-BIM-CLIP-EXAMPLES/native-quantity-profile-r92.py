"""⏱️ Observe real native composite-service quantity integration and repeated public representations."""
import importlib,json,sys,time,hashlib,inspect
from pathlib import Path
from copy import deepcopy
from numpy.testing import assert_equal
ROOT=next(p for p in Path(__file__).resolve().parents if (p/'package.json').is_file())
sys.path.insert(0,str(ROOT))
sys.dont_write_bytecode=True
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
MODE=sys.argv[1] if len(sys.argv)>1 else 'baseline'
OUT=Path(__file__).parent/'🗑️generated/native-r92'/('quantity-'+MODE)
OUT.mkdir(parents=True,exist_ok=True)
paths=[Path(api.__file__),Path(api.__file__).with_name('🔣️.json'),Path(__file__)]
def hashes():
    """🔐 Record source bytes separately from the retained loaded producer implementation."""
    return {str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}
before=hashes()
loaded={name:inspect.getsource(getattr(api,name)) for name in ('quantities','_occupied_volume','_material_quantities')}
model=api.load_model()
tag='FIX-BATH'
calls=[]
original=api._occupied_volume
def occupied(parts):
    """🔬 Time the exact underlying production integration without changing its output."""
    start=time.perf_counter()
    result=original(parts)
    calls.append({'parts':len(parts),'seconds':time.perf_counter()-start})
    return result
api._occupied_volume=occupied
rows=[]
def measure(label,callback):
    """🧾 Retain each actual public query latency and integration-count witness."""
    start=time.perf_counter()
    index=len(calls)
    result=callback()
    rows.append({'label':label,'seconds':time.perf_counter()-start,'integrations':calls[index:]})
    (OUT/'progress.json').write_text(json.dumps(rows,indent=2),encoding='utf-8')
    print('[DEBUG] '+json.dumps(rows[-1]),flush=True)
    return result
geometry=measure('cold-service-geometry',lambda:api.service_geometry(tag,model))
quantity=measure('first-quantity',lambda:api.quantities(tag,model))
again=measure('quantity-equivalent-new-dict',lambda:api.quantities(tag,deepcopy(model)))
assert_equal(quantity,again)
for kind in ('body','plan'):
    representation=measure('representation-'+kind,lambda:api.represent(tag,kind,model))
    assert_equal(quantity,{key:representation['derived'][key] for key in quantity})
    (OUT/(kind+'.json')).write_text(json.dumps(representation,sort_keys=True),encoding='utf-8')
after=hashes()
report={'tag':tag,'parts':len(geometry['parts']),'mode':MODE,'inputs_before':before,'inputs_after':after,'stable':before==after,'loaded_producers':loaded,'model_canonical_sha256':hashlib.sha256(json.dumps(model,sort_keys=True,separators=(',',':'),ensure_ascii=False).encode()).hexdigest(),'quantity':quantity,'stages':rows}
(OUT/'result.json').write_text(json.dumps(report,indent=2),encoding='utf-8')
