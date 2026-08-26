"""🕸️ One-off derivation (ticket 26/08/23/END-TO-END-TESTING-REFACTOR, wave 22).

Reads the REAL committed IFC 4 file
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧫️fixtures/🏗️nakagin-capsule-tower.ifc` — Kisho Kurokawa's
Nakagin Capsule Tower, 2.5 MB and 24 792 entities — with **IfcOpenShell 0.8.4**, a genuine
third-party IFC implementation, and maps its real PORT-AND-CONNECTION graph onto the
`s.stdio.semio.graph` model: every `IfcBuildingElementProxy`/`IfcElementAssembly` becomes a node
carrying its real `GlobalId`, its real entity type as `kind`, its real `Name` as `label` and its real
placement translation as `position`; every `IfcDistributionPort` nested under it becomes a real port
whose `i`/`o`/`x` kind is read off the real connection graph — a port the file uses as the
`RelatingPort` of an `IfcRelConnectsPorts` is an `out`, one used as the `RelatedPort` is an `in`, and
one the file connects in neither direction is an `inOut` (the source declares `FlowDirection` as
`NOTDEFINED` throughout, so the direction is taken from what the model actually wires); every
`IfcPropertySingleValue` of its property sets becomes a real typed property; and every
`IfcRelConnectsPorts` becomes an edge between the two nodes owning the connected ports.

IfcOpenShell reads IFC, not a semio envelope, and cannot express a single one of the eleven verbs,
which is why it is the source of the ARTIFACT and never the oracle.
"""

import importlib.util
import sys
from pathlib import Path

import ifcopenshell

ROOT = Path("/Users/ueli/Documents/semio")
IFC = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧫️fixtures/🏗️nakagin-capsule-tower.ifc"
CASE = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-graph"

FLOW_PORT = {"SINK": "in", "SOURCE": "out", "SOURCEANDSINK": "inOut", "NOTDEFINED": "inOut"}


def load_oracle():
    stub = type(sys)("semio_repo_test")
    stub.Adapter = type("Adapter", (), {"__init__": lambda self, name: None, "oracle": lambda self, *a: self})
    stub.Context = object
    stub.Outcome = object
    stub.digest = lambda data: ""
    sys.modules["semio_repo_test"] = stub
    spec = importlib.util.spec_from_file_location("graph_oracle", CASE / "🐍️component.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def placement_xy(product):
    """📍️ The element's own real placement translation, in millimetres, as the node position."""
    placement = getattr(product, "ObjectPlacement", None)
    while placement is not None and placement.is_a("IfcLocalPlacement"):
        relative = placement.RelativePlacement
        if relative is not None and relative.is_a("IfcAxis2Placement3D"):
            coordinates = relative.Location.Coordinates
            return float(coordinates[0]), float(coordinates[1])
        placement = placement.PlacementRelTo
    return 0.0, 0.0


def value_of(nominal, print_number):
    """🏷️ One real `IfcPropertySingleValue` nominal value in the subset's own tagged value model."""
    if nominal is None:
        return {"kind": "null"}
    raw = nominal.wrappedValue if hasattr(nominal, "wrappedValue") else nominal
    if isinstance(raw, bool):
        return {"kind": "bool", "value": raw}
    if isinstance(raw, int):
        return {"kind": "int", "lexeme": str(raw)}
    if isinstance(raw, float):
        return {"kind": "float", "lexeme": print_number(raw)}
    return {"kind": "str", "value": str(raw)}


def main():
    model = ifcopenshell.open(str(IFC))
    oracle = load_oracle()

    relating_ports, related_ports = set(), set()
    for relation in model.by_type("IfcRelConnectsPorts"):
        relating_ports.add(relation.RelatingPort.id())
        related_ports.add(relation.RelatedPort.id())

    ports_of = {}
    for nest in model.by_type("IfcRelNests"):
        for related in nest.RelatedObjects:
            if related.is_a("IfcDistributionPort"):
                ports_of.setdefault(nest.RelatingObject.id(), []).append(related)
    owner_of_port = {port.id(): owner for owner, ports in ports_of.items() for port in ports}

    properties_of = {}
    for definition in model.by_type("IfcRelDefinesByProperties"):
        pset = definition.RelatingPropertyDefinition
        if not pset.is_a("IfcPropertySet"):
            continue
        for product in definition.RelatedObjects:
            entries = properties_of.setdefault(product.id(), [])
            for prop in pset.HasProperties or []:
                if prop.is_a("IfcPropertySingleValue"):
                    entries.append({"key": "%s.%s" % (pset.Name or "", prop.Name or ""), "value": value_of(prop.NominalValue, oracle.print_number)})

    nodes, node_id_of = [], {}
    products = [p for p in model.by_type("IfcProduct") if p.is_a("IfcBuildingElementProxy") or p.is_a("IfcElementAssembly")]
    for product in sorted(products, key=lambda p: p.id()):
        x, y = placement_xy(product)
        node_id_of[product.id()] = product.GlobalId
        nodes.append(
            {
                "id": {"value": product.GlobalId},
                "kind": product.is_a(),
                "label": product.Name or "",
                "position": {"x": x, "y": y},
                "ports": [{"name": port.Name or port.GlobalId, "kind": "out" if port.id() in relating_ports else "in" if port.id() in related_ports else "inOut"} for port in ports_of.get(product.id(), [])],
                "properties": properties_of.get(product.id(), []),
            }
        )

    edges = []
    for relation in sorted(model.by_type("IfcRelConnectsPorts"), key=lambda r: r.id()):
        source = owner_of_port.get(relation.RelatingPort.id())
        target = owner_of_port.get(relation.RelatedPort.id())
        if source is None or target is None or source not in node_id_of or target not in node_id_of:
            continue
        edges.append({"id": {"value": relation.GlobalId}, "source": {"value": node_id_of[source]}, "target": {"value": node_id_of[target]}, "kind": relation.is_a(), "label": relation.Name or ""})

    document = {"schema": "s.stdio.semio.graph", "nodes": nodes, "edges": edges}
    dsl = oracle.print_dsl(document)
    assert oracle.parse_dsl(dsl) == document, "the printed DSL does not re-parse to the same document"
    pack = oracle.pack_bytes(document)
    assert oracle.parse_pack(pack) == document, "the encoded pack does not re-decode to the same document"
    out = CASE / "🧫️fixtures"
    out.mkdir(exist_ok=True)
    (out / "🗣️nakagin-capsule-tower.dsl.semio").write_text(dsl, encoding="utf-8")
    (out / "🎒️nakagin-capsule-tower.pack.semio").write_bytes(pack)
    print("nodes=%d edges=%d ports=%d properties=%d" % (len(nodes), len(edges), sum(len(n["ports"]) for n in nodes), sum(len(n["properties"]) for n in nodes)))
    print("port kinds:", sorted({p["kind"] for n in nodes for p in n["ports"]}))
    print("value kinds:", sorted({e["value"]["kind"] for n in nodes for e in n["properties"]}))
    print("dsl bytes=%d pack bytes=%d" % (len(dsl.encode("utf-8")), len(pack)))
    print("first node:", nodes[0]["id"], nodes[0]["kind"], nodes[0]["label"], nodes[0]["position"], len(nodes[0]["ports"]), len(nodes[0]["properties"]))
    print("last node:", nodes[-1]["id"], nodes[-1]["kind"], nodes[-1]["label"], len(nodes[-1]["ports"]), len(nodes[-1]["properties"]))
    print("last edge:", edges[-1])
    print("node[1] props:", [e["key"] for e in nodes[1]["properties"]][:8])


if __name__ == "__main__":
    main()
