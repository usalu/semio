"""🔬 Compare exact visible union against complete-cell GEOS shadow projection."""
import json,sys,time
from pathlib import Path
from fractions import Fraction as F
from shapely.geometry import Polygon
from shapely.ops import unary_union
p=Path(__file__).parent;root=Path.cwd();m=json.loads((root/"🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json").read_text(encoding="utf8"));e=next(e for e in m["elements"]if e["tag"]=="FIN-PILOTIS-GRAVEL")
origin=(F(-1.25),F(19.91));scale=1000
project=lambda p:(float((F(p[0])-F(p[1])-origin[0])*scale),float((F(p[0])+F(p[1])+2*F(p[2])-origin[1])*scale))
shadows=[]
for part in e["geometry"]["parts"]:
    mesh=part["mesh"];polys=[Polygon([project(mesh["vertices"][i])for i in face])for face in mesh["faces"]];shadows.append(unary_union([x for x in polys if x.area]))
expected=unary_union(shadows)
out=json.loads((p/"🗑️generated/material-skin-silhouette-prototype-r95.json").read_text(encoding="utf8"))["regions"]["1"];actual=unary_union([Polygon([[float((F(q[i])-origin[i])*scale)for i in (0,1)]for q in r["outer"]],[[[float((F(q[i])-origin[i])*scale)for i in (0,1)]for q in h]for h in r["holes"]])for r in out])
print("holes",len(actual.interiors),len(expected.interiors),"lengths",actual.boundary.length,expected.boundary.length);print("hausdorff",actual.boundary.hausdorff_distance(expected.boundary),"lengthdiff",actual.boundary.length-expected.boundary.length,"bounds",actual.bounds,expected.bounds);print("area",actual.symmetric_difference(expected).area,"boundary",actual.boundary.symmetric_difference(expected.boundary).length,"components",actual.geom_type,expected.geom_type,flush=True)
(p/"🗑️generated/material-skin-geos-shadow-geometry-r95.json").write_text(json.dumps({"actual":actual.__geo_interface__,"expected":expected.__geo_interface__}),encoding="utf8")
(p/"🗑️generated/material-skin-geos-shadow-r95.json").write_text(json.dumps({"area":actual.symmetric_difference(expected).area,"boundary":actual.boundary.symmetric_difference(expected.boundary).length}),encoding="utf8")
