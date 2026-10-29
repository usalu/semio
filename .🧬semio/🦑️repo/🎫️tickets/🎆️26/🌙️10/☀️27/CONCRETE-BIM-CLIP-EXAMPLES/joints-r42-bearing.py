"""🏗️ Inspect source-only structural roof bearing beneath the two continued columns."""
from pathlib import Path
import runpy,json
from shapely.geometry import Polygon
from shapely.ops import unary_union
from shapely import set_precision
context=runpy.run_path(str(Path(__file__).with_name("joints-r42-oracle.py")))
source,objects,ring,elevation=(context[k] for k in ("source","objects","ring","elevation"))
slab=objects["SL-R"]
g=slab["geometry"]
voids=[Polygon(v["outline"]) for v in g["voids"]]+[Polygon(e["geometry"]["outline"]) for e in source["elements"] if e["class"]=="IfcOpeningElement" and e["geometry"].get("slab")=="SL-R"]
voids=[set_precision(v,1e-10) for v in voids]
results=[]
for tag in ("10-FIRST","15-FIRST"):
    obj=objects[tag]
    section=obj["geometry"]["sections"][1]
    footprint=ring(section["profile"],section.get("axis",obj["geometry"]["axis"]))
    regions=[]
    supported=[]
    for r in g["regions"]:
        actual=set_precision(Polygon(r["outline"]),1e-10).intersection(set_precision(footprint,1e-10))
        for v in voids:
            if not actual.is_empty and actual.intersects(v):
                actual=actual.difference(v)
        if actual.area<=1e-12:
            continue
        top=elevation(g["level"])+r["top_offset"]["value"]
        regions.append(dict(region=r["tag"],member=r.get("member"),material=r.get("material","composite"),area=actual.area,top=top,gap=elevation(section["base_level"])-top,compression_material=g["composite"]["concrete"]))
        supported.append(actual)
    beams=[]
    for beam in source["elements"]:
        if beam["class"]!="IfcBeam" or "section_sweep" in beam["geometry"]:
            continue
        bg=beam["geometry"]
        for index,p in enumerate(bg["parts"]):
            actual=Polygon(p["outline"]).intersection(footprint)
            top=elevation(bg["base_level"])+p["z1"]
            if actual.area>1e-12 and top<=elevation(section["base_level"])+1e-10:
                beams.append(dict(beam=beam["tag"],part=index,area=actual.area,top=top,gap=elevation(section["base_level"])-top))
    union=unary_union(supported)
    results.append(dict(column=tag,base=elevation(section["base_level"]),top=elevation(section["top_level"]),footprint_area=footprint.area,structural_support_coverage=union.area,unsupported_plan_area=footprint.difference(union).area,regions=regions,beams=beams))
Path(__file__).with_name("🗑️generated").joinpath("joints-r42-bearing.json").write_text(json.dumps(results,indent=2))
print('BEARING',results)
