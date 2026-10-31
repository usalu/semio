"""🏠 Prove roof finish transfer against native strict owned-reserve protocol before publication."""
import sys,importlib,json,copy
from pathlib import Path
sys.path.insert(0,str(Path.cwd()));a=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️");m=a.load_model();r=json.loads(Path(__file__).with_name("🗑️generated").joinpath("roof-floor-authored-r92.json").read_text());before=sum(a._area(p["outline"])*(p["z1"]-p["z0"]) for t in r["reserves"] for p in a._slab_geometry(m,a._find(m,t))["finish_parts"])
for tag,row in r["floors"].items():
 e=copy.deepcopy(a._find(m,"FIN-CELLAR-CEMENT"));e.update(tag=tag,guid=a.global_id(tag),storey="roof");e["geometry"]={"kind":"flooring","base_level":"roof","parts":row["parts"]};m["elements"].append(e)
import inspect
source=inspect.getsource(a._finish_reservations).replace('raise ValueError("Finish reserve must be exactly occupied by its owner")','raise ValueError(str({"reserve":reserve,"target":target,"occupied":occupied,"expected":expected,"common":common}))');source=source.replace('raise ValueError("Finish reserves must not overlap")','raise ValueError(str({"reserve":reserve,"overlaps":[{"a":a,"b":b,"area":_area(_analysis._clip(a,b["outline"]))} for a in polygons for b in cutters if _area(_analysis._clip(a,b["outline"]))>1e-10]}))');exec(compile(source,"<diagnostic-finish-owner>","exec"),a.__dict__)
for tag,reserves in r["reserves"].items():a._find(m,tag)["geometry"].setdefault("finish_voids",[]).extend(reserves)
after=sum(a._area(p["outline"])*(p["z1"]-p["z0"]) for t in r["reserves"] for p in a._slab_geometry(m,a._find(m,t))["finish_parts"]);owned=sum(a._area(p["outline"])*(p["z1"]-p["z0"]) for row in r["floors"].values() for p in row["parts"]);assert abs(before-after-owned)<1e-6,(before,after,owned);print("[DEBUG] "+json.dumps({"before":before,"remaining":after,"floor_owned":owned,"error":before-after-owned}),flush=True)
