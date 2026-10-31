"""🧊 Lightweight stdlib-only private-cache identity diagnostic; independent native gate remains separate."""
import importlib,json,sys
from pathlib import Path
sys.path.insert(0,str(Path.cwd()))
m=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.⚡️surface-producers')
def verify(name,result,case):
    """📜 Check declared language-neutral outputs without impersonating a third-party implementation."""
    assert json.loads(json.dumps(result))==case['reference_outputs'][name],name
m._check_private_caches(verify)
print('[DEBUG] portable private-cache8scenarios GREEN; NumPy/GEOS native gate remains required',flush=True)
