"""🏗️ Verify independently decoded native RC column/slab interfaces without importing the model API."""
import json
from pathlib import Path
from shapely.geometry import Polygon,box
from shapely.ops import unary_union
from shapely import set_precision
TICKET=Path(__file__).parent
products={p["tag"]:p["parts"] for p in json.loads((TICKET/"🗑️generated/roof-bearing-native-profiles-r43.json").read_text())}
results=[]
for tag,axis in (("10-FIRST",[4.75,6]),("15-FIRST",[4.737,8.23])):
    domain=set_precision(box(axis[0]-.11,axis[1]-.11,axis[0]+.11,axis[1]+.11),1e-9)
    neighbourhood=domain.buffer(.05,join_style=2)
    parts=sorted(products[tag],key=lambda p:p["z0"])
    for a,b in zip(parts,parts[1:]):
        assert abs(a["z1"]-b["z0"])<1e-10
        assert Polygon(a["outline"]).intersection(Polygon(b["outline"])).area>.037
    for z in (6.341,6.5,6.54,6.58,6.599):
        column=unary_union([set_precision(Polygon(p["outline"]),1e-9) for p in parts if p["z0"]<z<p["z1"]])
        assert column.symmetric_difference(domain).area<1e-9
        local=[]
        beam_area=0
        for product,prisms in products.items():
            if product in ("10-FIRST","15-FIRST"):
                continue
            for p in prisms:
                if not p["z0"]<z<p["z1"]:
                    continue
                polygon=set_precision(Polygon(p["outline"]),1e-9)
                if not polygon.intersects(neighbourhood):
                    continue
                assert polygon.intersection(column).area<1e-9
                if product=="SL-R":
                    local.append(polygon.intersection(neighbourhood))
                else:
                    beam_area+=polygon.intersection(column).area
        slab=unary_union(local)
        contact=slab.boundary.intersection(column.boundary).length if local else 0
        if z<6.51:
            assert contact>.1
        results.append(dict(column=tag,z=z,column_area=column.area,other_material_intersection=slab.intersection(column).area+beam_area,rc_side_contact_length=contact))
(TICKET/"🗑️generated/roof-bearing-native-contact-r43.json").write_text(json.dumps(results,indent=2))
print(results)
