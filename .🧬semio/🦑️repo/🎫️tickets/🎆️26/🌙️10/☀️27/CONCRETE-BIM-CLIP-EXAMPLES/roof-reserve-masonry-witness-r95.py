"""🧱 Inventory exact finish/masonry reserve contacts before declaring physical material interfaces."""
import importlib,json,sys,time
from fractions import Fraction
from pathlib import Path
sys.dont_write_bytecode=True;sys.path.insert(0,str(Path.cwd()))
api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️");model=api.load_model();ticket=Path(__file__).parent;start=time.perf_counter();records=[]
for host in [obj for obj in model["elements"]if obj["class"]=="IfcSlab"]:
    rows=host["geometry"].get("finish_voids",[]);masonry=[row for row in rows if api._find(model,row["owner"])["class"]=="IfcChimney"]
    for row in rows:
        if api._find(model,row["owner"])["class"]!="IfcCovering":continue
        ring=api._expand(model,row["outline"]);bounds=api._surface_bounds(ring)
        for other in masonry:
            boundary=api._expand(model,other["outline"])
            if not api._surface_bounds_overlap(bounds,api._surface_bounds(boundary)):continue
            common=api._slab_surface(ring,boundary,[],exact=True)
            area=sum((abs(sum(a[0]*b[1]-a[1]*b[0]for a,b in zip(piece,piece[1:]+piece[:1])))/2 for piece in common),Fraction(0))
            if area:records.append({"host":host["tag"],"finish":row["owner"],"masonry":other["owner"],"window":row["tag"],"exact_area":str(area),"area":float(area)})
result={"scope":"Control-window contacts only; physical bodies remain independently required.","source":api._geometry_source_key(model).hex(),"records":records,"seconds":time.perf_counter()-start}
(ticket/"🗑️generated/roof-reserve-masonry-witness-r95.json").write_text(json.dumps(result,indent=2),encoding="utf8")
print("[DEBUG]",json.dumps(result),flush=True)
