"""🔬 Minimize the reproduced exposed-boundary input without rounding its coordinates."""
import importlib,json,sys
from pathlib import Path
sys.path.insert(0,str(Path.cwd()))
prefix="🎓️teaching.🏛️architecture.🏗️bim.🎬️clip."
skin=importlib.import_module(prefix+"🎨️visuals.📐️drafting.🧩️surfaces.🐍️")
cache=importlib.import_module(prefix+"🧱️model.♻️cache.🐍️")
folder=Path(__file__).parent/"🗑️generated"
data=json.loads((folder/"bim-boundary-plane.json").read_text(encoding="utf-8"))
facets=cache.decode(data["facets"])
def fails(rows):
    try:skin.material_skin(rows)
    except ValueError as error:return str(error)=="Open exposed material boundary"
    return False
assert fails(facets)
changed=True
while changed:
    changed=False
    for index in range(len(facets)):
        candidate=facets[:index]+facets[index+1:]
        if candidate and fails(candidate):
            facets=candidate;changed=True;break
out={"item":data["item"],"error":data["error"],"facets":cache.encode(facets)}
(folder/"bim-boundary-minimal.json").write_text(json.dumps(out,ensure_ascii=False,indent=2),encoding="utf-8")
print("[DEBUG] minimized facets",len(facets),flush=True)
