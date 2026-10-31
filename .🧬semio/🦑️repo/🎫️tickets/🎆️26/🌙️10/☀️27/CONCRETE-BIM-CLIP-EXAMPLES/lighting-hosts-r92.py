"""💡 Register inferred light mounts against actual current native host surfaces."""
import hashlib,importlib,json,sys
from pathlib import Path
from shapely.geometry import Point,Polygon
root=Path.cwd();ticket=Path(__file__).parent;sys.path.insert(0,str(root));api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️");source=api.load_model();result={"source_sha256":hashlib.sha256(Path(api.__file__).with_name("🔣️.json").read_bytes()).hexdigest(),"columns":{},"soffits":{}}
for tag in ("9","8","19"):
    obj=api._find(source,tag);geometry=api.column_geometry(source,obj);parts=[p for p in geometry["parts"] if p["z0"]<2.2<p["z1"]];points=[v for p in parts for v in api.part_footprint(p)];x,y=obj["geometry"]["axis"];result["columns"][tag]={"axis":[x,y],"positive_y_face":max(p[1] for p in points),"z":2.2}
for name,tag,x,y in [("hall","SL-1",6.65,11.1),("first-hall","SL-R",7.65,6.6),("salon","SL-R",11.75,3.1),("rear-west","SL-1",1.7,19.5),("rear-east","SL-1",17.3,19.5)]:
    geometry=api._slab_geometry(source,api._find(source,tag));parts=[p for p in geometry["parts"] if Polygon(api.part_footprint(p)).covers(Point(x,y))]
    if not parts:raise ValueError(("No source-backed soffit at light mount",name))
    result["soffits"][name]={"host":tag,"point":[x,y,min(p["z0"] for p in parts)]}
(ticket/"🗑️generated/lighting-hosts-r92.json").write_text(json.dumps(result,ensure_ascii=False,indent=2),encoding="utf8");print("[DEBUG] "+json.dumps(result),flush=True)
