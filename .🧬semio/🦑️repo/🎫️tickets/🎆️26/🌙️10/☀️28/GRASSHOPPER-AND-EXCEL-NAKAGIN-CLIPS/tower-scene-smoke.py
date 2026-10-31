"""🎭 Exercise the complete tower scene through native animation lifecycle."""
import importlib
from pathlib import Path
import sys
sys.path.insert(0,str(Path.cwd()))
tests=importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🧪️tests.🎭️presentation.🐍️")
scene=importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🗼️tower.🐍️").Beat3_Tower
original_step=scene.step
def traced_step(self,key,*animations,**kwargs):
    print("[DEBUG] tower step "+key,flush=True)
    return original_step(self,key,*animations,**kwargs)
scene.step=traced_step
tests.GrasshopperPresentationContract().lifecycle(scene.__name__,scene,"en",Path(__file__).parent/"🗑️generated"/"tower-lifecycle")
print("[DEBUG] Tower native lifecycle passed")
