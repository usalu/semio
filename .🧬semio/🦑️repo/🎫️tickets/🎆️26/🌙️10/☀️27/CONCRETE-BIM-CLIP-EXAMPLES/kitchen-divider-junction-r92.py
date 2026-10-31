"""🍳 Capture actual three-dimensional pantry divider to facade junction faces."""
import sys,importlib,json
from pathlib import Path
from shapely.geometry import Polygon,box
sys.path.insert(0,str(Path.cwd()));a=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️");m=a.load_model();wall=a._wall_geometry(m,a._find(m,"W-S"));parts=[dict(p,owner="W-S") for p in wall["parts"]]
for e in m["elements"]:
 if e["class"] in ("IfcWindow","IfcDoor") and a._find(m,e["geometry"]["opening"])["geometry"].get("wall")=="W-S":parts.extend(dict(p,owner=e["tag"]) for p in a._filling_geometry(m,e)["parts"])
parts.extend(dict(p) for p in a._wall_cutters(m) if a._find(m,p["owner"])["class"] in ("IfcBeam","IfcColumn"))
strip=box(2.96,-.5,3.04,.3);parts=[p for p in parts if not Polygon(p["outline"]).intersection(strip).is_empty];zs=sorted({3.17,6.34,*[max(3.17,min(6.34,p[k])) for p in parts for k in ("z0","z1")]});rows=[]
for lo,hi in zip(zs,zs[1:]):
 if hi-lo<1e-8:continue
 samples=[]
 for p in parts:
  if p["z0"]<(lo+hi)/2<p["z1"]:
   hit=Polygon(p["outline"]).intersection(strip)
   if hit.area>1e-10:samples.append({"owner":p["owner"],"role":p.get("role"),"material":p.get("material"),"bounds":list(hit.bounds)})
 rows.append({"z0":lo,"z1":hi,"faces":samples,"max_y":max((p["bounds"][3] for p in samples),default=None)})
Path(__file__).with_name("🗑️generated").joinpath("kitchen-divider-junction-r92.json").write_text(json.dumps(rows),encoding="utf8");print("[DEBUG] "+json.dumps([{k:r[k] for k in ("z0","z1","max_y")} for r in rows]),flush=True)
