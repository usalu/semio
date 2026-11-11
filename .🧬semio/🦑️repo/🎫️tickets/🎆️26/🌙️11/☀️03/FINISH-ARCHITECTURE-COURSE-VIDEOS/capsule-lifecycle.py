"""🎞️ Run only the complete capsule lesson lifecycle in both languages."""
import importlib
import sys
from pathlib import Path
sys.path.insert(0,str(Path.cwd()))
package="🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip"
tests=importlib.import_module(package+".🧪️tests.🎭️presentation.🐍️")
scene=importlib.import_module(package+".💊️capsule.🐍️").Beat2_Capsule
output=Path(__file__).parent/"🗑️generated"/"capsule-lifecycle"
output.mkdir(parents=True,exist_ok=True)
case=tests.GrasshopperPresentationContract()
for language in ("en","de"):
    case.lifecycle(scene.__name__,scene,language,output)
print("Capsule lifecycle passed in English and German",flush=True)
