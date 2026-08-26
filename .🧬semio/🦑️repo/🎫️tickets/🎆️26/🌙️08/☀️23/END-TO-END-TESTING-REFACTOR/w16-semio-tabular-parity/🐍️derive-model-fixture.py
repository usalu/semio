"""🏗️ Derive the semio MODEL fixture from the real committed Nakagin Capsule Tower IFC, ONCE.

Source: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧫️fixtures/🏗️nakagin-capsule-tower.ifc (IFC4, 2.5 MB,
24 792 entities), read with IfcOpenShell 0.8.4 — a genuine third-party IFC implementation.

  spatial   <- IfcSite / IfcBuilding / IfcBuildingStorey (id = GlobalId, name = Name, parent from
               the IfcRelAggregates chain)
  element   <- IfcElementAssembly + 180 IfcBuildingElementProxy; class = other[<IFC type>],
               placement = IfcAxis2Placement3D (translation from Location, rotation from the
               Axis/RefDirection frame as a quaternion, rounded to 6 decimals; scale 1),
               geometry = mesh[#<IfcProductDefinitionShape id>] where one exists else none,
               spatialId = the storey for the assembly, absent for the aggregated capsules
  pset      <- IfcPropertySingleValue, typed text / number / boolean by its IFC value type
  relation  <- IfcRelAggregates (aggregates, one per related object, id "<GlobalId>-<n>"),
               IfcRelContainedInSpatialStructure (containedIn, element -> storey),
               IfcRelConnectsElements (connectsTo, 179, each with its own GlobalId)

The project-level IfcRelAggregates (IfcProject -> IfcSite) is skipped: IfcProject is neither a
SpatialKind nor an element of this subset, so a relation naming it would dangle.
"""
import importlib.util, math, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "🐍️stub.py")).read())
install()
import ifcopenshell

ROOT = "/Users/ueli/Documents/semio"
CASE = os.path.join(ROOT, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-model")
IFC = os.path.join(ROOT, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧫️fixtures/🏗️nakagin-capsule-tower.ifc")
spec = importlib.util.spec_from_file_location("modelpy", os.path.join(CASE, "🐍️component.py"))
mod = importlib.util.module_from_spec(spec); spec.loader.exec_module(mod)

def clean(v):
    v = round(float(v), 6)
    return 0.0 if v == 0 else v

def quaternion(axis, ref):
    z = [clean(c) for c in (axis or (0.0, 0.0, 1.0))]
    x = [clean(c) for c in (ref or (1.0, 0.0, 0.0))]
    def norm(v):
        n = math.sqrt(sum(c * c for c in v)) or 1.0
        return [c / n for c in v]
    z = norm(z)
    x = norm([x[i] - sum(x[j] * z[j] for j in range(3)) * z[i] for i in range(3)])
    y = [z[1] * x[2] - z[2] * x[1], z[2] * x[0] - z[0] * x[2], z[0] * x[1] - z[1] * x[0]]
    m = [[x[0], y[0], z[0]], [x[1], y[1], z[1]], [x[2], y[2], z[2]]]
    trace = m[0][0] + m[1][1] + m[2][2]
    if trace > 0:
        s = math.sqrt(trace + 1.0) * 2
        q = [(m[2][1] - m[1][2]) / s, (m[0][2] - m[2][0]) / s, (m[1][0] - m[0][1]) / s, 0.25 * s]
    elif m[0][0] > m[1][1] and m[0][0] > m[2][2]:
        s = math.sqrt(1.0 + m[0][0] - m[1][1] - m[2][2]) * 2
        q = [0.25 * s, (m[0][1] + m[1][0]) / s, (m[0][2] + m[2][0]) / s, (m[2][1] - m[1][2]) / s]
    elif m[1][1] > m[2][2]:
        s = math.sqrt(1.0 + m[1][1] - m[0][0] - m[2][2]) * 2
        q = [(m[0][1] + m[1][0]) / s, 0.25 * s, (m[1][2] + m[2][1]) / s, (m[0][2] - m[2][0]) / s]
    else:
        s = math.sqrt(1.0 + m[2][2] - m[0][0] - m[1][1]) * 2
        q = [(m[0][2] + m[2][0]) / s, (m[1][2] + m[2][1]) / s, 0.25 * s, (m[1][0] - m[0][1]) / s]
    return {"x": clean(q[0]), "y": clean(q[1]), "z": clean(q[2]), "w": clean(q[3])}

def placement_of(product):
    pl = product.ObjectPlacement
    if pl is None or not pl.is_a("IfcLocalPlacement"):
        loc, rot = (0.0, 0.0, 0.0), {"x": 0.0, "y": 0.0, "z": 0.0, "w": 1.0}
    else:
        axis = pl.RelativePlacement
        loc = axis.Location.Coordinates
        rot = quaternion(axis.Axis.DirectionRatios if axis.Axis else None, axis.RefDirection.DirectionRatios if axis.RefDirection else None)
    return {
        "translation": {"x": clean(loc[0]), "y": clean(loc[1]), "z": clean(loc[2])},
        "rotation": rot,
        "scale": {"x": 1.0, "y": 1.0, "z": 1.0},
    }

f = ifcopenshell.open(IFC)
props = {}
for rel in f.by_type("IfcRelDefinesByProperties"):
    ps = rel.RelatingPropertyDefinition
    if not ps.is_a("IfcPropertySet"):
        continue
    for o in rel.RelatedObjects:
        bucket = props.setdefault(o.id(), {})
        entry = bucket.setdefault(ps.Name, [])
        for p in ps.HasProperties:
            nominal = p.NominalValue
            if nominal is None:
                value = {"kind": "text", "value": ""}
            elif nominal.is_a() in ("IfcBoolean", "IfcLogical"):
                value = {"kind": "boolean", "value": bool(nominal.wrappedValue)}
            elif isinstance(nominal.wrappedValue, (int, float)) and not isinstance(nominal.wrappedValue, bool):
                value = {"kind": "number", "value": clean(nominal.wrappedValue)}
            else:
                value = {"kind": "text", "value": str(nominal.wrappedValue)}
            entry.append({"key": p.Name, "value": value})

def psets_of(product):
    return [{"name": name, "properties": properties} for name, properties in props.get(product.id(), {}).items()]

def geometry_of(product):
    shape = getattr(product, "Representation", None)
    if shape is None:
        return {"kind": "none"}
    return {"kind": "mesh", "mesh_id": "#%d" % shape.id()}

site = f.by_type("IfcSite")[0]
building = f.by_type("IfcBuilding")[0]
storey = f.by_type("IfcBuildingStorey")[0]
spatial = [
    {"id": site.GlobalId, "kind": "site", "name": site.Name or "", "parentId": None, "placement": placement_of(site)},
    {"id": building.GlobalId, "kind": "building", "name": building.Name or "", "parentId": site.GlobalId, "placement": placement_of(building)},
    {"id": storey.GlobalId, "kind": "storey", "name": storey.Name or "", "parentId": building.GlobalId, "placement": placement_of(storey)},
]

contained = {}
for rel in f.by_type("IfcRelContainedInSpatialStructure"):
    for o in rel.RelatedElements:
        contained[o.id()] = rel.RelatingStructure.GlobalId

products = f.by_type("IfcElementAssembly") + f.by_type("IfcBuildingElementProxy")
elements = [
    {
        "id": p.GlobalId,
        "class": {"kind": "other", "name": p.is_a()},
        "placement": placement_of(p),
        "geometry": geometry_of(p),
        "spatialId": contained.get(p.id()),
        "psets": psets_of(p),
    }
    for p in products
]

member_ids = {n["id"] for n in spatial} | {e["id"] for e in elements}
relations = []
for rel in f.by_type("IfcRelAggregates"):
    for at, o in enumerate(rel.RelatedObjects):
        if rel.RelatingObject.GlobalId not in member_ids or o.GlobalId not in member_ids:
            continue
        relations.append({"id": "%s-%d" % (rel.GlobalId, at), "kind": {"kind": "aggregates"}, "from": rel.RelatingObject.GlobalId, "to": o.GlobalId})
for rel in f.by_type("IfcRelContainedInSpatialStructure"):
    for o in rel.RelatedElements:
        relations.append({"id": rel.GlobalId, "kind": {"kind": "containedIn"}, "from": o.GlobalId, "to": rel.RelatingStructure.GlobalId})
for rel in f.by_type("IfcRelConnectsElements"):
    relations.append({"id": rel.GlobalId, "kind": {"kind": "connectsTo"}, "from": rel.RelatingElement.GlobalId, "to": rel.RelatedElement.GlobalId})

doc = {"schema": mod.ENVELOPE_ID, "spatial": spatial, "elements": elements, "relations": relations}
dsl = mod.print_dsl(doc).encode("utf-8")
pack = mod.pack_bytes(doc)
assert mod.parse_dsl(dsl.decode("utf-8")) == doc, "derived DSL does not round-trip"
assert mod.parse_pack(pack) == doc, "derived pack does not round-trip"
fx = os.path.join(CASE, "🧫️fixtures")
open(os.path.join(fx, "🏗️nakagin-capsule-tower.dsl.semio"), "wb").write(dsl)
open(os.path.join(fx, "🏗️nakagin-capsule-tower.pack.semio"), "wb").write(pack)
print("spatial", len(spatial), "elements", len(elements), "relations", len(relations))
print("psets", sum(len(e["psets"]) for e in elements), "properties", sum(len(p["properties"]) for e in elements for p in e["psets"]))
print("dsl", len(dsl), "pack", len(pack))
print("first element", elements[0]["id"], elements[0]["class"], elements[0]["geometry"], elements[0]["spatialId"])
print("last element ", elements[-1]["id"], elements[-1]["geometry"], elements[-1]["spatialId"], [p["name"] for p in elements[-1]["psets"]])
print("last spatial ", spatial[-1])
print("last relation", relations[-1])
print("first relation", relations[0])
import collections
print("rot distinct", len({tuple(sorted(e["placement"]["rotation"].items())) for e in elements}))
