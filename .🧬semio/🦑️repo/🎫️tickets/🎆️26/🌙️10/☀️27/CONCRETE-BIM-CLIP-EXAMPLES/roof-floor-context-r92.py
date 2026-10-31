"""🏠 Capture actual roof reserve and exclusion owners for explicit finish authoring."""
import importlib,json,sys
from pathlib import Path
sys.path.insert(0,str(Path.cwd()));a=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️");m=a.load_model();out={"slabs":{},"obstacles":{},"source":m["revision"]}
for tag in ("SL-R","SL-KIOSK"):
 g=a._slab_geometry(m,a._find(m,tag));out["slabs"][tag]={"finish_parts":g["finish_parts"],"parts":g["parts"],"shared_parts":g["shared_parts"]}
for e in m["elements"]:
 if e["storey"]=="roof" and e["class"] in ("IfcWall","IfcColumn","IfcBeam","IfcChimney","IfcDoor","IfcBuildingElementProxy"):
  if e["tag"]=="SALON-CANOPY":continue
  g=a._wall_geometry(m,e) if e["class"]=="IfcWall" else a.column_geometry(m,e) if e["class"]=="IfcColumn" else a._filling_geometry(m,e) if e["class"]=="IfcDoor" else a.solid_geometry(e["tag"],m)
  profiles=[r for p in g["parts"]+g.get("cavity_parts",[]) if p["z0"]<6.62 and p["z1"]>6.6+1e-8 for r in a.part_regions(p,6.61000001)]
  if profiles:out["obstacles"][e["tag"]]=profiles
  print("[DEBUG] "+e["tag"],flush=True)
Path(__file__).with_name("🗑️generated").joinpath("roof-floor-context-r92.json").write_text(json.dumps(out),encoding="utf8");print("[DEBUG] roof context complete",flush=True)
