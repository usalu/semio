"""🔬 Capture the exact BIM axon projection failure without changing model producers."""
import importlib,json,sys
from pathlib import Path
sys.path.insert(0,str(Path.cwd()))
p="🎓️teaching.🏛️architecture.🏗️bim.🎬️clip."
model=importlib.import_module(p+"🧱️model.🐍️")
cache=importlib.import_module(p+"🧱️model.♻️cache.🐍️")
axon=importlib.import_module(p+"🎨️visuals.🏛️savoye.🧊️axon.🐍️")
scene=importlib.import_module(p+"📊️information.🐍️")
import numpy as np
from bim_projection_io import checkpoint,write
surface=axon._primitives._surfaces
original_skin=axon._Painter._skin
original_project=surface.projected_material_regions
current={}
def native(value):
    if isinstance(value,np.ndarray):return [native(x) for x in value]
    if isinstance(value,np.generic):return value.item()
    if isinstance(value,dict):return {k:native(v) for k,v in value.items()}
    if isinstance(value,tuple):return tuple(native(x) for x in value)
    if isinstance(value,list):return [native(x) for x in value]
    return value
def skin(self,prim,layer,dz):
    current.update({"item":{"tag":prim.item.get("tag"),"class":prim.item.get("class")},"layer":layer,"dz":dz})
    return original_skin(self,prim,layer,dz)
def project(facets,matrix,*args,**kwargs):
    checkpoint(Path(__file__).parent/"🗑️generated",facets,matrix,native)
    try:return original_project(facets,matrix,*args,**kwargs)
    except ValueError as error:
        out=Path(__file__).parent/"🗑️generated"/"bim-projection-failure.json"
        write(out.parent,facets,matrix,{"current":current,"error":str(error)},cache,native)
        print("[DEBUG] projection failure",current["item"].get("tag"),len(facets),str(error),flush=True)
        raise
axon._Painter._skin=skin
surface.projected_material_regions=project
m=model.load_model()
a,b=model.car_path(m)["entry_pair"]
print("[DEBUG] projecting actual BIM 114",a,b,flush=True)
scene.Beat1_DataToInformation().villa_axon((0,1.45,0),.13,model=m,highlight=(a,b),disciplines=("architecture","structure"))
raise AssertionError("Expected captured projection error did not occur")
