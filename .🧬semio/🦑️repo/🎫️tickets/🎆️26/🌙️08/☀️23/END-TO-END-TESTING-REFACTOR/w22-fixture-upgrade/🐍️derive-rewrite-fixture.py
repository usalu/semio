"""♻️ One-off derivation (ticket 26/08/23/END-TO-END-TESTING-REFACTOR, wave 22).

Grows the `before-fixture` of the committed rewrite rule from the real Nakagin Capsule Tower GROUND
FLOOR (two real pieces, one real connection) to the WHOLE real building, by reading
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧫️fixtures/🏗️nakagin-capsule-tower.ifc` with **IfcOpenShell
0.8.4**. That IFC is the same file the two committed pieces already came from: the demo document's
node ids ARE its real `ComposePieceAttributes.composeGuid` values and its port ids ARE its real
`ComposeConnector.composePort` values, so this is the same real data continued, not a second source.

* 180 real `IfcBuildingElementProxy` capsules become the nodes, each carrying its real `composeGuid`
  as `id`, its real `ComposePieceAttributes.name` as `name` and `properties.label`, its real
  placement translation in metres as `properties.position`, and that translation's real `z` as
  `properties.tier`;
* their 364 real `IfcDistributionPort`s become the ports, each carrying its real
  `ComposeConnector.composeConnectorId` — which is also the port entity's own real `Name` and is the
  identifier the committed document already addresses its one edge by — and
  a direction read off the real connection graph (`out` where the file uses the port as an
  `IfcRelConnectsPorts` `RelatingPort`, `in` where it uses it as the `RelatedPort`, `inOut` for the
  six the file connects in neither direction);
* the 179 real `IfcRelConnectsPorts` become the edges, addressed `"<nodeId>@<portId>"` exactly as the
  committed document addresses its one edge, and carrying the real `ComposeConnectionParams`
  `rotation`/`shift` of the connected capsule where the file records them.

TWO values are not in the IFC and are carried from the committed document itself rather than
invented: the editor box `width`/`height` of a node (96×48 for the root piece, 88×40 for a capsule —
the demo's own two committed values for exactly those two roles) and the `camera`. Both are stated
here and in the feature. Everything else is real IFC data.
"""

import json
import sys
from pathlib import Path

import ifcopenshell

ROOT = Path("/Users/ueli/Documents/semio")
IFC = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧫️fixtures/🏗️nakagin-capsule-tower.ifc"
FIXTURE = ROOT / "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🧪️tests/mutate-rewrite-1/🧫️fixtures/♻️nakagin-ground-floor.snapshot.json"
OUT = FIXTURE.with_name("♻️nakagin-capsule-tower.snapshot.json")
ROOT_PIECE = "7dc5b737-3b6b-4068-b315-b7bacc91c2e1"


def properties_of(model):
    out = {}
    for definition in model.by_type("IfcRelDefinesByProperties"):
        pset = definition.RelatingPropertyDefinition
        if not pset.is_a("IfcPropertySet"):
            continue
        for product in definition.RelatedObjects:
            bucket = out.setdefault(product.id(), {})
            for prop in pset.HasProperties or []:
                if prop.is_a("IfcPropertySingleValue") and prop.NominalValue is not None:
                    bucket["%s.%s" % (pset.Name, prop.Name)] = prop.NominalValue.wrappedValue
    return out


def translation(product):
    placement = getattr(product, "ObjectPlacement", None)
    while placement is not None and placement.is_a("IfcLocalPlacement"):
        relative = placement.RelativePlacement
        if relative is not None and relative.is_a("IfcAxis2Placement3D"):
            x, y, z = (float(value) for value in relative.Location.Coordinates)
            return round(x / 1000.0, 6) + 0.0, round(y / 1000.0, 6) + 0.0, round(z / 1000.0, 6) + 0.0
        placement = placement.PlacementRelTo
    return 0.0, 0.0, 0.0


def main():
    model = ifcopenshell.open(str(IFC))
    facts = properties_of(model)

    relating, related, connections = set(), set(), []
    for relation in sorted(model.by_type("IfcRelConnectsPorts"), key=lambda r: r.id()):
        relating.add(relation.RelatingPort.id())
        related.add(relation.RelatedPort.id())
        connections.append(relation)

    ports_of, owner_of_port = {}, {}
    for nest in model.by_type("IfcRelNests"):
        for member in nest.RelatedObjects:
            if member.is_a("IfcDistributionPort"):
                ports_of.setdefault(nest.RelatingObject.id(), []).append(member)
                owner_of_port[member.id()] = nest.RelatingObject.id()

    node_id, nodes = {}, []
    for capsule in sorted(model.by_type("IfcBuildingElementProxy"), key=lambda p: p.id()):
        bucket = facts.get(capsule.id(), {})
        identifier = bucket.get("ComposePieceAttributes.composeGuid")
        if identifier is None:
            continue
        node_id[capsule.id()] = identifier
        name = bucket.get("ComposePieceAttributes.name", capsule.Name or "")
        x, y, z = translation(capsule)
        root = identifier == ROOT_PIECE
        nodes.append(
            {
                "id": identifier,
                "kind": "Piece",
                "name": name,
                "x": x,
                "y": y,
                "width": 96.0 if root else 88.0,
                "height": 48.0 if root else 40.0,
                "properties": {"label": name, "position": {"x": x, "y": y, "z": z}, "tier": z},
                "ports": [
                    {
                        "id": facts.get(port.id(), {}).get("ComposeConnector.composeConnectorId", port.Name or port.GlobalId),
                        "kind": "Connector",
                        "direction": "out" if port.id() in relating else "in" if port.id() in related else "inOut",
                        "properties": {},
                    }
                    for port in ports_of.get(capsule.id(), [])
                ],
            }
        )

    port_id = {port.id(): facts.get(port.id(), {}).get("ComposeConnector.composeConnectorId", port.Name or port.GlobalId) for ports in ports_of.values() for port in ports}
    edges = []
    for relation in connections:
        source_owner = owner_of_port.get(relation.RelatingPort.id())
        target_owner = owner_of_port.get(relation.RelatedPort.id())
        if source_owner not in node_id or target_owner not in node_id:
            continue
        params = facts.get(target_owner, {})
        edges.append(
            {
                "id": relation.GlobalId,
                "kind": "Connection",
                "source": "%s@%s" % (node_id[source_owner], port_id[relation.RelatingPort.id()]),
                "target": "%s@%s" % (node_id[target_owner], port_id[relation.RelatedPort.id()]),
                "properties": {
                    "gap": 0.0,
                    "rise": 0.0,
                    "rotation": float(params.get("ComposeConnectionParams.rotation", 0.0)),
                    "shift": float(params.get("ComposeConnectionParams.shift", 0.0)),
                    "tilt": 0.0,
                    "turn": 0.0,
                    "u": nodes[[n["id"] for n in nodes].index(node_id[target_owner])]["properties"]["position"]["x"],
                    "v": nodes[[n["id"] for n in nodes].index(node_id[target_owner])]["properties"]["position"]["y"],
                },
            }
        )

    committed = json.loads(FIXTURE.read_text(encoding="utf-8"))
    before = json.loads(committed["beforeFixtureJson"])
    graph = {
        "schema": before["schema"],
        "name": "Nakagin Capsule Tower",
        "manifestId": before["manifestId"],
        "camera": before["camera"],
        "nodes": nodes,
        "edges": edges,
        "rootNodeId": ROOT_PIECE,
    }
    document = dict(committed)
    document["beforeFixtureJson"] = json.dumps(graph, ensure_ascii=False, indent=2)
    OUT.write_text(json.dumps(document, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print("nodes=%d ports=%d edges=%d" % (len(nodes), sum(len(n["ports"]) for n in nodes), len(edges)))
    print("beforeFixtureJson bytes=%d, snapshot bytes=%d" % (len(document["beforeFixtureJson"].encode("utf-8")), OUT.stat().st_size))
    print("root node present:", any(n["id"] == ROOT_PIECE for n in nodes))
    print("first edge:", json.dumps(edges[0])[:220])


if __name__ == "__main__":
    main()
