"""🔬 Inspect directed-boundary balance for captured projection facets."""
import importlib,json,sys
from collections import Counter
from pathlib import Path
sys.path.insert(0,str(Path.cwd()))
p="🎓️teaching.🏛️architecture.🏗️bim.🎬️clip."
s=importlib.import_module(p+"🎨️visuals.📐️drafting.🧩️surfaces.🐍️")
c=importlib.import_module(p+"🧱️model.♻️cache.🐍️")
folder=Path(__file__).parent/"🗑️generated"
d=json.loads((folder/"bim-projection-minimal.json").read_text(encoding="utf-8"))
orig=s._trace
def trace(edges,check):
    balance=Counter()
    for a,b in edges:balance[a]+=1;balance[b]-=1
    bad=[(p,n) for p,n in balance.items() if n]
    if bad:
        print("[DEBUG] directed edges",len(edges),"imbalances",len(bad),flush=True)
        for point,n in bad[:20]:print("[DEBUG]",point,n,flush=True)
    return orig(edges,check)
s._trace=trace
try:s.projected_material_regions(c.decode(d["facets"]),c.decode(d["matrix"]))
except ValueError as e:print("[DEBUG] exception",str(e),flush=True)
