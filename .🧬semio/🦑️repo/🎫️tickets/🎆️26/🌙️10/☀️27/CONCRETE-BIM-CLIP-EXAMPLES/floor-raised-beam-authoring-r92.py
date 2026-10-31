"""🪵 Author real floor layers outside actual exclusive raised beam material."""
import importlib,json,sys,copy
from pathlib import Path
from shapely.geometry import Polygon
from shapely.ops import unary_union
from shapely import constrained_delaunay_triangles
sys.path.insert(0,str(Path.cwd()));a=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")

def trim_floor_layers(model,rows,level=3.17,beam_tags=None,bedding_material="MAT-MORTAR-PUTTY",bedding_role="bedding"):
    """🧩 Subtract only actual exclusive beam strata and preserve original unaffected source cells."""
    case=json.loads((Path(a.__file__).parent/"🧪️tests/🪵️raised-beams.json").read_text());beam_tags=beam_tags or case["beam_tags"];floor_low=min(p["z0"]+level for row in rows for p in row["parts"]);beams=[p for t in beam_tags for p in a._solid_geometry(model,a._find(model,t),resolve_embedded=False)["parts"] if p["z0"]<level and p["z1"]>floor_low];result=[];cache={}
    for row in rows:
        parts=[];removed=0.;prepared=[]
        for original in row["parts"]:
            if original.get("role")!="wearing" or original.get("material")!="MAT-PAVING":prepared.append(original);continue
            low,high=original["z0"]+level,original["z1"]+level;poly=Polygon(original["outline"]);affected=poly.intersection(unary_union([Polygon(r) for q in beams if q["z0"]<high and q["z1"]>low for r in a.part_regions(q,(max(low,q["z0"])+min(high,q["z1"]))/2)]))
            if affected.area<1e-12:prepared.append(original);continue
            for region,marked in ((poly.difference(affected),False),(affected,True)):
                if region.is_empty:continue
                triangles=[t for t in constrained_delaunay_triangles(region).geoms if t.area>1e-12 and region.covers(t)];assert abs(sum(t.area for t in triangles)-region.area)<1e-8
                prepared.extend({**original,"outline":[list(v) for v in t.exterior.coords[:-1]],"_raised_column":marked} for t in triangles)
        for original in prepared:
            p=copy.deepcopy(original);lo,hi=p["z0"]+level,p["z1"]+level;poly=Polygon(p["outline"]);active=[q for q in beams if q["z0"]<hi-1e-10 and q["z1"]>lo+1e-10];levels=sorted({lo,hi,*(max(lo,min(hi,q[k])) for q in active for k in ("z0","z1"))});pieces=[];change=False
            for bottom,top in zip(levels,levels[1:]):
                z=(bottom+top)/2;key=tuple((i,q["z0"],q["z1"]) for i,q in enumerate(beams) if q["z0"]<z<q["z1"])
                if key not in cache:
                    occupied=[]
                    for q in beams:
                        if not q["z0"]<z<q["z1"]:continue
                        mid=unary_union([Polygon(r) for r in a.part_regions(q,z)]);low=unary_union([Polygon(r) for r in a.part_regions(q,bottom+1e-9)]);high=unary_union([Polygon(r) for r in a.part_regions(q,top-1e-9)]);assert mid.symmetric_difference(low).area<1e-7 and mid.symmetric_difference(high).area<1e-7,(row["tag"],"nonprismatic owned beam in floor band",bottom,top);occupied.append(mid)
                    cache[key]=unary_union(occupied)
                cut=cache[key];common=poly.intersection(cut);removed+=common.area*(top-bottom);region=poly.difference(cut)
                if common.area>1e-12:change=True
                pieces.append((bottom,top,region))
            if not change:parts.append(p);continue
            for bottom,top,region in pieces:
                if region.is_empty or region.area<1e-12:continue
                cells=[t for t in constrained_delaunay_triangles(region).geoms if t.area>1e-12 and region.covers(t)];assert abs(sum(t.area for t in cells)-region.area)<1e-8
                for t in cells:parts.append({**p,"outline":[list(v) for v in t.exterior.coords[:-1]],"z0":round(bottom-level,12),"z1":round(top-level,12)})
        repaired=[]
        for p in parts:
            affected=p.pop("_raised_column",False)
            if affected and p.get("role")=="wearing" and p.get("material")=="MAT-PAVING":
                if p["z1"]<0.-1e-8:repaired.append({**p,"role":bedding_role,"material":bedding_material});continue
                assert p["z1"]-p["z0"]>=.015-1e-8,(row["tag"],"incompatible paver/beam reserve",p)
                if p["z0"]<-.015-1e-10:repaired.append({**p,"z1":-.015,"role":bedding_role,"material":bedding_material})
                repaired.append({**p,"z0":-.015,"material":"MAT-PAVING-UNRESOLVED"})
            else:repaired.append(p)
        before=sum(Polygon(p["outline"]).area*(p["z1"]-p["z0"]) for p in row["parts"]);after=sum(Polygon(p["outline"]).area*(p["z1"]-p["z0"]) for p in repaired);assert abs(before-after-removed)<1e-8,(row["tag"],before,after,removed);result.append({"tag":row["tag"],"parts":repaired,"before":before,"after":after,"removed":removed})
    return result

if __name__=="__main__":
    model=a.load_model();case=json.loads((Path(a.__file__).parent/"🧪️tests/🪵️raised-beams.json").read_text());rows=[{"tag":tag,"parts":a._find(model,tag)["geometry"]["parts"]} for tag in case["floor_tags"]];result=trim_floor_layers(model,rows);Path(__file__).with_name("🗑️generated").joinpath("floor-raised-beam-authored-r92.json").write_text(json.dumps(result),encoding="utf8");print("[DEBUG] "+json.dumps([{k:r[k] for k in ("tag","before","after","removed")} for r in result]),flush=True)
