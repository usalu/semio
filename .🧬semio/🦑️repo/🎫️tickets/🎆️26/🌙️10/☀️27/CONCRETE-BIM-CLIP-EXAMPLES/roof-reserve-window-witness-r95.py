"""🔎 Isolate exact authored reserve-window overlap without materializing physical roof cells."""
import hashlib,importlib,json,sys,time
from fractions import Fraction
from pathlib import Path
sys.dont_write_bytecode=True;sys.path.insert(0,str(Path.cwd()))
api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
ticket=Path(__file__).parent;model=api.load_model();host=api._find(model,"SL-R");rows=host["geometry"]["finish_voids"];prior=[];start=time.perf_counter();result=None
def area(ring):
    """📐 Retain every exact positive control-window area without a comparison threshold."""
    return abs(sum(a[0]*b[1]-a[1]*b[0]for a,b in zip(ring,ring[1:]+ring[:1])))/2
for index,row in enumerate(rows):
    ring=api._expand(model,row["outline"]);bounds=api._surface_bounds(ring)
    for previous,other,box in prior:
        if not api._surface_bounds_overlap(bounds,box):continue
        common=api._slab_surface(ring,other,[],exact=True);volume=sum((area(piece)for piece in common),Fraction(0))
        if volume:
            result={"scope":"Authored control-window overlap only; not an occupied-material proof.","source":api._geometry_source_key(model).hex(),"api":hashlib.sha256(Path(api.__file__).read_bytes()).hexdigest(),"index":index,"current":row,"previous":previous,"exact_area":str(volume),"area":float(volume),"exact_intersection":common,"seconds":time.perf_counter()-start};break
    if result:break
    prior.append((row,ring,bounds))
    if index%100==0:print("[DEBUG] windows",index,flush=True)
if result is None:result={"scope":"Authored control-window overlap only","overlap":None,"seconds":time.perf_counter()-start}
(ticket/"🗑️generated/roof-reserve-window-witness-r95.json").write_text(json.dumps(result,ensure_ascii=False,indent=2,default=str),encoding="utf8")
print("[DEBUG]",json.dumps(result,ensure_ascii=False,default=str),flush=True)
