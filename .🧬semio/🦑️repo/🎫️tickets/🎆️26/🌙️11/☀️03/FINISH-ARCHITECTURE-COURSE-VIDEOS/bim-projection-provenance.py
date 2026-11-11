"""🔬 Trace actual beam boundary before visual coordinates are rounded."""
import importlib,json,pickle,sys
from pathlib import Path
sys.path.insert(0,str(Path.cwd()))
p="🎓️teaching.🏛️architecture.🏗️bim.🎬️clip."
m=importlib.import_module(p+"🧱️model.🐍️")
cache=importlib.import_module(p+"🧱️model.♻️cache.🐍️")
prim=importlib.import_module(p+"🎨️visuals.🏛️savoye.🧊️axon.🧩️primitives")
s=prim._surfaces;original=s._trace
folder=Path(__file__).parent/"🗑️generated"
data=json.loads((folder/"bim-projection-minimal.json").read_text(encoding="utf8"))
target=cache.decode(data["facets"])[0]["rings"][0]
points={tuple(float(v) for v in q[:2]) for q in target};matches=[]
def trace(edges,check):
    rows=original(edges,check)
    for row in rows:
        for ring in (row["outer"],*row["holes"]):
            if len(ring)==len(target) and {tuple(float(v) for v in q) for q in ring}==points:
                matches.append(ring)
                print("[DEBUG] matched exact pre-lift ring",len(ring),flush=True)
    return rows
s._trace=trace
model=m.load_model()
print("[DEBUG] building isolated actual beam",flush=True)
item=m.solid_geometry("BEAM-PRIMARY-F-7-24",model)
print("[DEBUG] beam parts",len(item["parts"]),flush=True)
prim._surface_prims(item,{},3.17,6.6,-.15)
with (folder/"bim-projection-exact-rings.pkl").open("wb") as stream:pickle.dump(matches,stream)
print("[DEBUG] exact matches",len(matches),flush=True)
if not matches:raise AssertionError("Pre-lift ring did not match captured projection")
