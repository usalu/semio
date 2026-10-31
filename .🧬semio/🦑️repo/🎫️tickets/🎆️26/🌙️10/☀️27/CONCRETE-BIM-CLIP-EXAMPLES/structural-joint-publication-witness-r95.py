"""🔬 Retain exact source-bound rejected native joint publication for a portable regression."""
from pathlib import Path
import importlib,importlib.util,json,sys,traceback
sys.path.insert(0,str(Path.cwd()))
A=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
B=Path(__file__).parent;O=B/"🗑️generated/native-r92/structural"
spec=importlib.util.spec_from_file_location("structural_source",B/"structural-joint-source-r95.py");S=importlib.util.module_from_spec(spec);spec.loader.exec_module(S)
m=A.load_model();S.apply(m);backend=A._structural_joint_backend();original=backend._publish

def capture(api,part):
    """🧾 Preserve a failing actual operand rather than filtering it or perturbing its vertices."""
    try:return original(api,part)
    except BaseException:
        (O/"rejected-publication-exact.json").write_text(json.dumps(part,default=A._native_cache_scalar),encoding="utf-8")
        (O/"rejected-publication-float.json").write_text(json.dumps(A._native_public_value(part)),encoding="utf-8")
        raise

backend._publish=capture
A._solid_geometry(m,A._find(m,"RIB-CELLAR"))
