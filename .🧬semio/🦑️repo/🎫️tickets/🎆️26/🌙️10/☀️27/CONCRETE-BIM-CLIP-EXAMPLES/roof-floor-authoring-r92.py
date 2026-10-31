"""🏠 Author source-qualified roof floor assemblies from actual native reserve owners."""
import importlib,json,sys
from pathlib import Path
from shapely.geometry import Polygon
from shapely.ops import unary_union
from shapely import constrained_delaunay_triangles,set_precision
sys.path.insert(0,str(Path.cwd()));a=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️");m=a.load_model();root=Path(__file__).parent;c=json.loads((root/"🗑️generated/roof-floor-context-r92.json").read_text());e=json.loads((root/"roof-finish-evidence-r92.json").read_text());obstacles=unary_union([Polygon(r) for tag,rings in c["obstacles"].items() if tag!="D-STAIR-ROOF" for r in rings]);indoor=Polygon(a._expand(m,a._find(m,"W-STAIR-ROOF")["geometry"]["path"]));paving=Polygon(e["zones"][0]["outline_candidate"]);floors={tag:{"parts":[],"area":0.} for tag in ("FIN-ROOF-PAVING","FIN-ROOF-GRAVEL","FIN-ROOF-STAIR")};reserves={};areas={}

def cells(region):
    """🔺 Preserve only independently contained constrained GEOS triangles."""
    if region.is_empty or region.area<1e-10:return []
    triangles=[p for p in constrained_delaunay_triangles(region).geoms if p.area>1e-10 and region.covers(p)]
    assert abs(sum(p.area for p in triangles)-region.area)<1e-7,(region.area,sum(p.area for p in triangles))
    return triangles

for slab,g in c["slabs"].items():
 reserves[slab]=[];areas[slab]=0.
 for cap in sorted({p["z0"] for p in g["finish_parts"] if abs(p["z1"]-6.6)<1e-8}):
  p={"z0":cap}
  band=set_precision(unary_union([Polygon(q["outline"]) for q in g["finish_parts"] if abs(q["z0"]-cap)<1e-8]),1e-9)
  available=band.difference(set_precision(obstacles,1e-9));inside=available.intersection(indoor);tile=available.difference(indoor).intersection(paving);gravel=available.difference(indoor).difference(paving)
  for tag,region,depth,material,bed in [("FIN-ROOF-STAIR",inside,.015,"MAT-GRES-BLACK","MAT-MORTAR-PUTTY"),("FIN-ROOF-PAVING",tile,.04 if p["z0"]<=6.56 else .015,"MAT-PAVING" if p["z0"]<=6.56 else "MAT-PAVING-UNRESOLVED","MAT-ROOF-FINISH"),("FIN-ROOF-GRAVEL",gravel,.02,"MAT-GRAVEL","MAT-ROOF-FINISH")]:
   for t in cells(region):
    ring=[[round(v,9) for v in q] for q in t.exterior.coords[:-1]];floors[tag]["parts"].extend([{"outline":ring,"z0":round(p["z0"]-6.6,9),"z1":-depth,"role":"weathering_reserve" if bed=="MAT-ROOF-FINISH" else "bedding","material":bed},{"outline":ring,"z0":-depth,"z1":0.,"role":"wearing","material":material}]);floors[tag]["area"]+=t.area;areas[slab]+=t.area;reserves[slab].append({"tag":"RES-"+tag+"-"+slab+"-"+str(len(reserves[slab])+1),"owner":tag,"outline":ring})
result={"floors":floors,"reserves":reserves,"area_by_slab":areas,"obstacle_area":obstacles.area,"limits":["Roof paver/gravel presence sourced; gravel20mm depth and weathering composition inferred.","High3cm structural bands cannot fit4cm pavers; explicit nominal15mm unresolved wearing retained.","Roof stair landing tile/bedding assignment inferred from service-stair source family."]};(root/"🗑️generated/roof-floor-authored-r92.json").write_text(json.dumps(result),encoding="utf8");print("[DEBUG] "+json.dumps({"floors":{k:{"area":v["area"],"parts":len(v["parts"])} for k,v in floors.items()},"slabs":areas}),flush=True)
