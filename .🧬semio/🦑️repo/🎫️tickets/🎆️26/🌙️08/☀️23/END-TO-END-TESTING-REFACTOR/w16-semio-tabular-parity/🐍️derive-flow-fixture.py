"""🏗️ Derive the semio FLOW fixture from the real committed Nakagin Capsule Tower IFC, ONCE.

Source: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧫️fixtures/🏗️nakagin-capsule-tower.ifc (IFC4, 2.5 MB,
24 792 entities), read with IfcOpenShell 0.8.4 — a genuine third-party IFC implementation.

  node  <- IfcBuildingElementProxy      (180 capsules/cores; id = GlobalId, label = Name)
  param <- IfcPropertySingleValue       (its property sets, "<Pset>.<Prop>" = NominalValue)
  pos   <- IfcLocalPlacement            (x, z of the placement location: a flow canvas is 2D and
                                         the tower's pieces are distributed in plan-x and elevation)
  edge  <- IfcRelConnectsPorts          (179; endpoints are the elements owning the two
                                         IfcDistributionPorts, resolved through IfcRelNests)
  kind  <- ComposeConnector.description (the relating port's own human description)
"""
import importlib.util, json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "🐍️stub.py")).read())
install()
import ifcopenshell

ROOT = "/Users/ueli/Documents/semio"
CASE = os.path.join(ROOT, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-flow")
IFC  = os.path.join(ROOT, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧫️fixtures/🏗️nakagin-capsule-tower.ifc")
spec = importlib.util.spec_from_file_location("flowpy", os.path.join(CASE, "🐍️component.py"))
mod = importlib.util.module_from_spec(spec); spec.loader.exec_module(mod)

f = ifcopenshell.open(IFC)
props = {}
for rel in f.by_type("IfcRelDefinesByProperties"):
    ps = rel.RelatingPropertyDefinition
    if not ps.is_a("IfcPropertySet"):
        continue
    for o in rel.RelatedObjects:
        for p in ps.HasProperties:
            props.setdefault(o.id(), []).append(("%s.%s" % (ps.Name, p.Name), "" if p.NominalValue is None else str(p.NominalValue.wrappedValue)))

owner = {}
for nest in f.by_type("IfcRelNests"):
    for port in nest.RelatedObjects:
        owner[port.id()] = nest.RelatingObject

nodes = []
for e in f.by_type("IfcBuildingElementProxy"):
    loc = e.ObjectPlacement.RelativePlacement.Location.Coordinates
    nodes.append({
        "id": e.GlobalId,
        "kind": e.is_a(),
        "label": e.Name or "",
        "params": [{"key": k, "value": v} for k, v in props.get(e.id(), [])],
        "position": {"x": float(loc[0]), "y": float(loc[2])},
    })

def description_of(port):
    for k, v in props.get(port.id(), []):
        if k == "ComposeConnector.description":
            return v
    return ""

edges = []
for rel in f.by_type("IfcRelConnectsPorts"):
    a, b = rel.RelatingPort, rel.RelatedPort
    edges.append({
        "id": rel.GlobalId,
        "from": {"node": owner[a.id()].GlobalId, "port": a.GlobalId},
        "to": {"node": owner[b.id()].GlobalId, "port": b.GlobalId},
        "kind": description_of(a),
    })

doc = {"schema": mod.ENVELOPE_ID, "nodes": nodes, "edges": edges}
dsl = mod.print_dsl(doc).encode("utf-8")
pack = mod.pack_bytes(doc)
assert mod.parse_dsl(dsl.decode("utf-8")) == doc, "the derived DSL does not round-trip"
assert mod.parse_pack(pack) == doc, "the derived pack does not round-trip"
fx = os.path.join(CASE, "🧫️fixtures")
open(os.path.join(fx, "🏗️nakagin-capsule-tower.dsl.semio"), "wb").write(dsl)
open(os.path.join(fx, "🏗️nakagin-capsule-tower.pack.semio"), "wb").write(pack)
print("nodes", len(nodes), "edges", len(edges), "params", sum(len(n["params"]) for n in nodes))
print("dsl bytes", len(dsl), "pack bytes", len(pack))
print("last node", nodes[-1]["id"], nodes[-1]["label"], [p["key"] for p in nodes[-1]["params"]])
print("first node", nodes[0]["id"], nodes[0]["label"], [(p["key"],p["value"]) for p in nodes[0]["params"]])
print("last edge", edges[-1])
print("edge kinds distinct", len({e["kind"] for e in edges}))
print("positions distinct", len({(n["position"]["x"], n["position"]["y"]) for n in nodes}))
