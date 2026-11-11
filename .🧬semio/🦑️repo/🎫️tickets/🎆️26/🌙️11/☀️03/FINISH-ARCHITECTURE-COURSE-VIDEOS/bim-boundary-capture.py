"""🔬 Capture the exact failing BIM material facet group without changing production sources."""
import importlib
import json
import sys
from pathlib import Path
sys.path.insert(0,str(Path.cwd()))
prefix="🎓️teaching.🏛️architecture.🏗️bim.🎬️clip."
prims=importlib.import_module(prefix+"🎨️visuals.🏛️savoye.🧊️axon.🧩️primitives")
cache=importlib.import_module(prefix+"🧱️model.♻️cache.🐍️")
surface=prims._surfaces
original_skin=surface.material_skin
original_parts=prims._surface_prims
current={}
def parts(item,*args,**kwargs):
    current["tag"]=item["tag"]
    current["class"]=item.get("class")
    return original_parts(item,*args,**kwargs)
def skin(facets,*args,**kwargs):
    try:
        return original_skin(facets,*args,**kwargs)
    except ValueError as error:
        output=Path(__file__).parent/"🗑️generated"/"bim-boundary-failure.json"
        output.write_text(json.dumps({"item":current,"error":str(error),"facets":cache.encode(facets)},ensure_ascii=False,indent=2),encoding="utf-8")
        print("[DEBUG] Captured",current,str(error),"facets",len(facets),flush=True)
        raise
prims._surface_prims=parts
surface.material_skin=skin
prims.prims()
print("[DEBUG] No boundary failure reproduced",flush=True)
