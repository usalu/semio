"""🧪 Diagnose source niche ring validity without changing production geometry."""
import importlib,json,sys,traceback
from pathlib import Path
root=next(p for p in Path(__file__).resolve().parents if (p/'AGENTS.md').is_file() and (p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');m=api.load_model();original=api._analysis._convex_parts

def capture(r):
    """🧾 Record the exact rejected ring before propagating its exception."""
    try:return original(r)
    except Exception:
        print('[DEBUG] rejected ring '+json.dumps(r),flush=True)
        raise
api._analysis._convex_parts=capture
try:
    result=api._wall_geometry(m,api.element('W-SHARED-BATH-N',m));print('[DEBUG] success '+str(len(result['parts'])),flush=True)
except Exception:traceback.print_exc()
