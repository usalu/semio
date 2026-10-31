"""🏠 Capture minimal exact native prism-subtraction inputs for invalid roof residuals."""
import sys,importlib,json,time
from pathlib import Path
from shapely.geometry import Polygon
sys.path.insert(0,str(Path.cwd()));a=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️");m=a.load_model();original=a._subtract_prisms;witnesses=[]
def record(parts,cutters):
    """🔬 Preserve original inputs and exact invalid outputs without altering production results."""
    result=original(parts,cutters);invalid=[p for p in result if not Polygon(p["outline"]).is_valid]
    if invalid:
        for p in invalid[:2]:
            poly=Polygon(p["outline"]);bounds=poly.bounds;region=p.get("region");selected=[q for q in parts if q.get("region")==region and q["z0"]<p["z1"] and q["z1"]>p["z0"] and a._surface_bounds_overlap(a._surface_bounds(q["outline"]),bounds)];selected_bounds=[a._surface_bounds(q["outline"]) for q in selected];cuts=[c for c in cutters if any(a._surface_bounds_overlap(a._surface_bounds(c["outline"]),b) for b in selected_bounds) and c["z0"]<p["z1"] and c["z1"]>p["z0"]];witnesses.append({"input_parts":selected,"cutters":cuts,"invalid_output":p})
    return result
a._subtract_prisms=record;t=time.monotonic();a._slab_geometry(m,a._find(m,"SL-R"));Path(__file__).with_name("🗑️generated").joinpath("roof-residual-inputs-r92.json").write_text(json.dumps(witnesses),encoding="utf8");print("[DEBUG] "+json.dumps({"witnesses":len(witnesses),"seconds":time.monotonic()-t,"sizes":[[len(w["input_parts"]),len(w["cutters"])] for w in witnesses]}),flush=True)
