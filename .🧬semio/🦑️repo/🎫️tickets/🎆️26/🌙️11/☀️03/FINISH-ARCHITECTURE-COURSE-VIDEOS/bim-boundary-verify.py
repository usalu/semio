"""🔬 Verify the captured native surface data after boundary repair."""
import importlib,json,sys,time
from pathlib import Path
sys.path.insert(0,str(Path.cwd()))
prefix="🎓️teaching.🏛️architecture.🏗️bim.🎬️clip."
skin=importlib.import_module(prefix+"🎨️visuals.📐️drafting.🧩️surfaces.🐍️")
cache=importlib.import_module(prefix+"🧱️model.♻️cache.🐍️")
folder=Path(__file__).parent/"🗑️generated"
for name in ("bim-boundary-plane.json","bim-boundary-failure.json"):
    data=json.loads((folder/name).read_text(encoding="utf-8"))
    facets=cache.decode(data["facets"])
    start=time.monotonic()
    result=skin.material_skin(facets)
    print("[DEBUG]",name,"input",len(facets),"output",len(result),"seconds",round(time.monotonic()-start,3),flush=True)
