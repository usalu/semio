"""🏠 Capture exact invalid native residual roof profiles without altering them."""
import sys,importlib,json,time
from pathlib import Path
from shapely.geometry import Polygon
from shapely.validation import explain_validity
sys.path.insert(0,str(Path.cwd()));a=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️");m=a.load_model();rows=[];t=time.monotonic()
for tag in ("SL-R","SL-KIOSK"):
 g=a._slab_geometry(m,a._find(m,tag))
 for i,p in enumerate(g["finish_parts"]):
  poly=Polygon(p["outline"])
  if not poly.is_valid:rows.append({"slab":tag,"index":i,"reason":explain_validity(poly),"part":p})
Path(__file__).with_name("🗑️generated").joinpath("roof-invalid-native-r92.json").write_text(json.dumps(rows),encoding="utf8");print("[DEBUG] "+json.dumps({"invalid":len(rows),"seconds":time.monotonic()-t,"first":rows[:2]}),flush=True)
