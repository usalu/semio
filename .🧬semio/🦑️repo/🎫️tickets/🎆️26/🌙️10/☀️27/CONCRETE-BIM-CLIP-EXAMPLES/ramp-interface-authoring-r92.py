"""🛝️ Author sole physical floor ownership against exact original ramp and wall planes."""
import importlib,sys,json,copy,hashlib,itertools
from pathlib import Path
import numpy as np
sys.path.insert(0,str(Path.cwd()))
a=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️");f=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🎨️finishes.🐍️");oracle=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🛝️interfaces")
def planes(part):
    """📐 Preserve original outward face operands for exact complementary half-spaces."""
    vertices=a.part_vertices(part);result=[]
    for face in a.part_faces(part):
        anchor=vertices[face[0]]
        for i in range(1,len(face)-1):
            points=[anchor,vertices[face[i]],vertices[face[i+1]]];normal=np.cross(np.subtract(points[1],anchor),np.subtract(points[2],anchor)).tolist()
            if any(v!=0 for v in normal):break
        else:raise ValueError("A cutter face requires noncollinear original points")
        result.append({"normal":normal,"offset":sum(n*v for n,v in zip(normal,anchor)),"points":points})
    return result

def subtract(stock,cutter):
    """🪓 Retain private rational provenance through every ordered complement and inside branch."""
    current=stock;outside=[]
    for plane in planes(cutter):
        reverse={**plane,"normal":[-v for v in plane["normal"]],"offset":-plane["offset"]}
        piece=f.clip_stock(current,[reverse])
        if piece is not None:outside.append(piece)
        current=f.clip_stock(current,[plane])
        if current is None:break
    return outside

m=a.load_model();source=hashlib.sha256(Path(a.__file__).with_name("🔣️.json").read_bytes()).hexdigest();case=json.loads(Path(oracle.__file__).with_suffix(".json").read_text());rows=[]
groups={}
for tag,target in case["pairs"]:groups.setdefault(tag,[]).append(target)
for tag,targets in groups.items():
    if len(sys.argv)>1 and tag!=sys.argv[1].split(":")[0]:continue
    target="+".join(targets)
    if tag=="FIN-HALL":a._find(m,tag)["geometry"]["parts"]=json.loads((Path(__file__).parent/"hall-floor-original-stock-r92.json").read_text())["parts"]
    obj=a._find(m,tag);base=a._z(m,obj["geometry"]["base_level"]);cutters=[r for owner in targets for p in oracle.occupied(m,owner) for r in oracle.pieces(p)];out=[];removed=0.;before=0.
    for raw in oracle.occupied(m,tag):
        active=[q for q in cutters if all(max(v[k] for v in a.part_vertices(raw))>min(v[k] for v in a.part_vertices(q)) and max(v[k] for v in a.part_vertices(q))>min(v[k] for v in a.part_vertices(raw)) for k in range(3))]
        before+=sum(a._polyhedron_volume(p) for p in oracle.pieces(raw))
        if not active:
            out.append(copy.deepcopy(raw));continue
        stocks=[f.clip_stock({**p,**a._polyhedron_part(a.part_vertices(p),a.part_faces(p))},[],exact=True) for p in oracle.pieces(raw)]
        for cutter in active:stocks=[piece for stock in stocks for piece in subtract(stock,cutter)]
        for stock in stocks:
            p=f.publish_stock(stock)
            try:a._polyhedron_part(p["mesh"]["vertices"],p["mesh"]["faces"])
            except Exception:
                (Path(__file__).parent/"🗑️generated/floor-ramp-stock-failure-r92.json").write_text(json.dumps({"origin":stock._origin,"planes":stock._planes,"published":p,"source_tag":tag,"target":target},ensure_ascii=False,indent=2),encoding="utf8");raise
            p={k:p[k] for k in ("mesh","material","role")};p["mesh"]["vertices"]=[[x,y,z-base] for x,y,z in p["mesh"]["vertices"]];out.append(p)
    for p in out:
        if "mesh" not in p:
            if "z0" in p:p["z0"]-=base;p["z1"]-=base
            for key in ("outline","z0","z1","material","role"):assert key in p
    out=[{k:v for k,v in p.items() if k in (("mesh","material","role") if "mesh" in p else ("outline","z0","z1","top_outline","material","role"))} for p in out]
    n=copy.deepcopy(m);a._find(n,tag)["geometry"]["parts"]=out;after=sum(a._polyhedron_volume(p) for p in oracle.occupied(n,tag));residual=sum(oracle.common_volume(p,q) for p in oracle.occupied(n,tag) for q in cutters);expected=sum(oracle.common_volume(p,q) for p in oracle.occupied(m,tag) for q in cutters);assert abs((before-after)-expected)<1e-9,(tag,before-after,expected);assert residual<case["tolerance"],(tag,residual);m=n;rows.append({"tag":tag,"target":target,"parts":out,"before":before,"after":after,"removed":before-after,"residual":residual});print("[DEBUG] "+json.dumps({k:v for k,v in rows[-1].items() if k!="parts"}),flush=True)
(Path(__file__).parent/"🗑️generated/ramp-interface-delta-r92.json").write_text(json.dumps({"source":source,"rows":rows},ensure_ascii=False,indent=2),encoding="utf8")

ramp_slab=[]
for slab_tag in ('SL-1','SL-R'):
 slab=a._slab_geometry_entry(m,a._find(m,slab_tag));material=[r for p in slab['parts']+slab['shared_parts'] for r in oracle.pieces(p)]
 for flight in ('RF-1A','RF-1B','RF-2A','RF-2B'):
  owned=oracle.occupied(m,flight);volume=sum(oracle.common_volume(p,q) for p in owned for q in material)
  if volume>1e-10:ramp_slab.append({'flight':flight,'slab':slab_tag,'volume':volume});print('[DEBUG] actual ramp/slab '+json.dumps(ramp_slab[-1]),flush=True)
(Path(__file__).parent/'🗑️generated/ramp-slab-joints-r92.json').write_text(json.dumps(ramp_slab,ensure_ascii=False,indent=2),encoding='utf8')
