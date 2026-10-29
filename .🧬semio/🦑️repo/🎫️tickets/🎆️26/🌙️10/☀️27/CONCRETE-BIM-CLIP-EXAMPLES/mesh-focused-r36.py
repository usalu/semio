"""🧪 Execute the real main mesh oracle with strict nine-decimal publication accuracy."""
import hashlib,json,runpy,traceback,inspect,importlib,time
from pathlib import Path

TICKET=Path(__file__).parent
OUTPUT=TICKET/'🗑️generated'/'mesh-focused-r36.json'
namespace=runpy.run_path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧪️tests/🐍️.py')
cls=namespace['BimAnalysisContract']
started=time.monotonic()
error=None
stage='native model and analysis setup'
OUTPUT.write_text(json.dumps({'debug':'[DEBUG] strict mesh','stage':stage}))
try:
    cls.setUpClass()
    provider=hashlib.sha256(json.dumps(cls.model,sort_keys=True,separators=(',',':')).encode()).hexdigest()
    products=len(cls.model['elements'])
    stage='independent unrounded GEOS grid and strict publication bound'
    OUTPUT.write_text(json.dumps({'debug':'[DEBUG] strict mesh','stage':stage,'provider':provider,'products':products}))
    cls('test_mesh_nodes_match_independent_shapely_clipped_grid').test_mesh_nodes_match_independent_shapely_clipped_grid()
    cls.tearDownClass()
    module=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🕸️mesh-clipping')
    functions=[fn for name,fn in inspect.getmembers(module,inspect.isfunction) if name.startswith('test_')]
    for fn in functions:
        fn()
    stage='completed'
except Exception:
    error=traceback.format_exc()
summary={'stage':stage,'products':globals().get('products'),'provider':globals().get('provider'),'error':error,'seconds':time.monotonic()-started,'main_methods':1 if error is None else 0,'portable_functions':len(globals().get('functions',[])) if error is None else 0}
OUTPUT.write_text(json.dumps(summary))
print('STRICT_NATIVE_MESH',json.dumps(summary))
