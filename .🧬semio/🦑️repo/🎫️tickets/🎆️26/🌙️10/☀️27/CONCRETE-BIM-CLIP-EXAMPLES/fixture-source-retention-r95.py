"""🏛️ Measure actual source fixture retention without running unrelated building geometry."""
import gc,hashlib,importlib,json,sys,time,tracemalloc
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import patch
import numpy as np
sys.path.insert(0,str(Path.cwd()))
M=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧪️tests.🐍️")
A=M.model_api();warm=A.load_model();del warm;gc.collect()
source=hashlib.sha256(A._source().encode("utf-8")).hexdigest()
tracemalloc.start();baseline=tracemalloc.get_traced_memory()[0];started=time.perf_counter()
fixture=type("ActualSourceFixture",(M.ModelCase,),{})
api=SimpleNamespace(load_model=A.load_model,building_geometry=lambda model:np.zeros(4))
with patch.object(M,"model_api",return_value=api):
    fixture.setUpClass()
    before=tracemalloc.get_traced_memory()[0]
    members={key:{"type":type(vars(fixture)[key]).__name__,"class_owns_directly":vars(fixture)[key] is getattr(fixture,key)} for key in ("model","snapshot","items","geometry")}
    assert fixture.model==fixture.snapshot and len(fixture.model["elements"])>=685
    fixture.tearDownClass();fixture.doClassCleanups()
gc.collect();after,peak=tracemalloc.get_traced_memory();tracemalloc.stop()
assert all(key not in vars(fixture) for key in ("api","model","snapshot","items","geometry"))
report={"source_sha256":source,"scope":"actual source+snapshot+registry class ownership; independent NumPy sentinel substitutes expensive building geometry to isolate source retention","before_cleanup_traced_bytes":before-baseline,"after_cleanup_traced_bytes":after-baseline,"released_traced_bytes":before-after,"peak_traced_bytes":peak,"members":members,"seconds":time.perf_counter()-started}
(Path(__file__).parent/"🗑️generated/native-r92/fixture-source-retention-r95.json").write_text(json.dumps(report,indent=2),encoding="utf-8")
print("[DEBUG] "+json.dumps(report))
