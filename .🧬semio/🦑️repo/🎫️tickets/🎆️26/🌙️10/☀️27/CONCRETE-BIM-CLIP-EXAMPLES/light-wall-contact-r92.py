"""📍 Probe actual kernel wall contact at source light elevation."""
import importlib,sys,json
from pathlib import Path
from shapely.geometry import Polygon,LineString
sys.path.insert(0,str(Path.cwd()))
a=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️");s=a.load_model()
for tag in ("W-W","W-KITCHEN"):
 g=a._wall_geometry(s,a._find(s,tag));hits=[]
 for p in g["parts"]:
  if p["z0"]<4.9<p["z1"]:
   for r in a.part_band_footprints(p,4.9,4.900001):
    hit=Polygon(r).intersection(LineString([[-1,.5],[6,.5]]))
    if not hit.is_empty:hits.append(hit.bounds)
 print("[DEBUG] "+json.dumps([tag,hits]),flush=True)
