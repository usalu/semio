"""🔬 Isolate the exact coplanar material group responsible for a captured boundary failure."""
import importlib,json,sys
from fractions import Fraction
from pathlib import Path
sys.path.insert(0,str(Path.cwd()))
prefix="🎓️teaching.🏛️architecture.🏗️bim.🎬️clip."
skin=importlib.import_module(prefix+"🎨️visuals.📐️drafting.🧩️surfaces.🐍️")
cache=importlib.import_module(prefix+"🧱️model.♻️cache.🐍️")
folder=Path(__file__).parent/"🗑️generated"
data=json.loads((folder/"bim-boundary-failure.json").read_text(encoding="utf-8"))
facets=cache.decode(data["facets"])
groups={};planes={}
for facet in facets:
    key,axes,plane,sign=skin.plane_chart(facet)
    if key is None:continue
    material=json.dumps(facet["material"],sort_keys=True,separators=(",",":"))
    group=groups.setdefault((material,key),{"rings":[],"facets":[],"axes":axes})
    group["facets"].append(facet)
    for i,ring in enumerate(facet["rings"]):
        group["rings"].append((tuple(tuple(Fraction(point[j]) for j in axes) for point in ring),sign if i==0 else -sign))
    planes.setdefault(key,[]).append(facet)
print("[DEBUG] plane/material groups",len(groups),flush=True)
for i,((material,key),group) in enumerate(groups.items()):
    others=[operand for (other_material,other_key),other in groups.items() if other_key==key and other_material!=material for operand in other["rings"]]
    try:
        skin._regions(group["rings"],lambda:None,lambda *args:None,others)
    except ValueError as error:
        subset=planes[key]
        output={"item":data["item"],"error":str(error),"targetMaterial":json.loads(material),"facets":cache.encode(subset)}
        (folder/"bim-boundary-plane.json").write_text(json.dumps(output,ensure_ascii=False,indent=2),encoding="utf-8")
        print("[DEBUG] failed group",i,"group facets",len(group["facets"]),"plane facets",len(subset),"error",str(error),flush=True)
        try:skin.material_skin(subset)
        except ValueError:print("[DEBUG] plane-only reproduction confirmed",flush=True)
        else:raise AssertionError("Plane-only subset did not reproduce")
        break
else:raise AssertionError("No failing planar group found")
