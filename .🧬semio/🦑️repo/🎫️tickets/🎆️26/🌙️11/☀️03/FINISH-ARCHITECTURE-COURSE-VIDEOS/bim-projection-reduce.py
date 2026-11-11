"""🔬 Minimize an actual projected-material failure without rounding inputs."""
import importlib,json,pickle,sys,time
from pathlib import Path
sys.path.insert(0,str(Path.cwd()))
p="🎓️teaching.🏛️architecture.🏗️bim.🎬️clip."
surface=importlib.import_module(p+"🎨️visuals.📐️drafting.🧩️surfaces.🐍️")
cache=importlib.import_module(p+"🧱️model.♻️cache.🐍️")
folder=Path(__file__).parent/"🗑️generated"
with (folder/"bim-projection-current.pkl").open("rb") as stream:data=pickle.load(stream)
facets=data["facets"];matrix=data["matrix"]
def fails(rows):
    try:surface.projected_material_regions(rows,matrix)
    except ValueError as e:
        if str(e)=="Open exposed material boundary":return True
        raise
    return False
assert fails(facets)
groups={}
for f in facets:groups.setdefault(json.dumps(f["material"],sort_keys=True),[]).append(f)
for rows in groups.values():
    if fails(rows):
        facets=rows;break
else:raise AssertionError("No material group reproduced")
print("[DEBUG] material group",len(facets),flush=True)
parts=2
while len(facets)>1:
    size=(len(facets)+parts-1)//parts
    reduced=False
    for start in range(0,len(facets),size):
        candidate=facets[:start]+facets[start+size:]
        if candidate and fails(candidate):
            facets=candidate;parts=max(2,parts-1);reduced=True
            print("[DEBUG] chunk reduce",len(facets),flush=True);break
    if not reduced:
        if parts>=len(facets):break
        parts=min(len(facets),parts*2)
changed=True
while changed:
    changed=False
    for index in range(len(facets)):
        candidate=facets[:index]+facets[index+1:]
        if candidate and fails(candidate):
            facets=candidate;changed=True;break
    print("[DEBUG] reduce",len(facets),flush=True)
out={"facets":cache.encode(facets),"matrix":cache.encode(matrix)}
(folder/"bim-projection-minimal.json").write_text(json.dumps(out,ensure_ascii=False),encoding="utf-8")
print("[DEBUG] minimized",len(facets),flush=True)
