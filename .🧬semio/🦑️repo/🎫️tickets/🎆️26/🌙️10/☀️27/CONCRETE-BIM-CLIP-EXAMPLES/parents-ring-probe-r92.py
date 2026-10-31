"""🔬 Isolate actual invalid sanitary band rings without repairing or hiding material."""
import importlib,json,sys
from pathlib import Path
from shapely.geometry import Polygon
from shapely.validation import explain_validity
sys.path.insert(0,str(Path.cwd()));a=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️");s=a.load_model();bad=[]
for tag in ("BASIN-PARENTS","BIDET-PARENTS","FIX-BATH"):
 for i,p in enumerate(a.service_geometry(tag,s)["parts"]):
  if p["z0"]<5.27 and p["z1"]>3.17:
   for ring in a.part_band_footprints(p,max(3.17,p["z0"]),min(5.27,p["z1"])):
    poly=Polygon(ring)
    if not poly.is_valid:bad.append({"tag":tag,"part_index":i,"role":p.get("role"),"reason":explain_validity(poly),"ring":ring,"part":p,"input":a.part_outline_at(p,max(3.17,p["z0"]))+a.part_outline_at(p,min(5.27,p["z1"])) if "top_outline" in p else None})
out=Path(__file__).parent/"🗑️generated"/"parents-ring-probe-r92.json";out.write_text(json.dumps(bad,indent=2));print("[DEBUG] "+json.dumps({"invalid":len(bad),"first":bad[0] if bad else None}),flush=True)
