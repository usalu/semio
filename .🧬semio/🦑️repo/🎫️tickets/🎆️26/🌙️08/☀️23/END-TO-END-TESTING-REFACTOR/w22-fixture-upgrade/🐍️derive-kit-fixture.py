"""🪑️ One-off derivation (ticket 26/08/23/END-TO-END-TESTING-REFACTOR, wave 22).

Reads the REAL committed IFC 4 file
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧫️fixtures/🏗️nakagin-capsule-tower.ifc` with
**IfcOpenShell 0.8.4** and maps the real building onto the `s.stdio.semio.kit` kit-of-parts model:
the real `IfcElementType`s the file declares become the type catalogue, the building becomes one
design whose pieces are the 180 real capsules with their real placement transforms (translation in
millimetres, orientation quaternion computed from the real `Axis`/`RefDirection` pair, unit scale)
and whose connections are the real `IfcRelConnectsPorts`, and each real type gets a representation
link addressed by its own real `GlobalId`. The owned `object`/`model`/`properties` child slots keep
the same three real handles the committed furniture kit carries, so the four composition shapes the
subset has are all still present.

IfcOpenShell reads IFC, not a semio envelope, and cannot express a single one of the fifteen verbs,
which is why it is the source of the ARTIFACT and never the oracle.
"""

import importlib.util
import math
import sys
from pathlib import Path

import ifcopenshell

ROOT = Path("/Users/ueli/Documents/semio")
IFC = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧫️fixtures/🏗️nakagin-capsule-tower.ifc"
CASE = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-kit"


def load_oracle():
    stub = type(sys)("semio_repo_test")
    stub.Adapter = type("Adapter", (), {"__init__": lambda self, name: None, "oracle": lambda self, *a: self})
    stub.Context = object
    stub.Outcome = object
    stub.digest = lambda data: ""
    sys.modules["semio_repo_test"] = stub
    spec = importlib.util.spec_from_file_location("kit_oracle", CASE / "🐍️component.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def quaternion(axis, ref):
    """🔄 The orientation quaternion of a real `IfcAxis2Placement3D`'s `Axis`/`RefDirection` pair."""
    z = axis or (0.0, 0.0, 1.0)
    x = ref or (1.0, 0.0, 0.0)
    z = normalise(z)
    x = normalise(subtract(x, scale(z, dot(x, z))))
    y = cross(z, x)
    m = [[x[0], y[0], z[0]], [x[1], y[1], z[1]], [x[2], y[2], z[2]]]
    trace = m[0][0] + m[1][1] + m[2][2]
    if trace > 0.0:
        s = math.sqrt(trace + 1.0) * 2.0
        return (m[2][1] - m[1][2]) / s, (m[0][2] - m[2][0]) / s, (m[1][0] - m[0][1]) / s, 0.25 * s
    if m[0][0] > m[1][1] and m[0][0] > m[2][2]:
        s = math.sqrt(1.0 + m[0][0] - m[1][1] - m[2][2]) * 2.0
        return 0.25 * s, (m[0][1] + m[1][0]) / s, (m[0][2] + m[2][0]) / s, (m[2][1] - m[1][2]) / s
    if m[1][1] > m[2][2]:
        s = math.sqrt(1.0 + m[1][1] - m[0][0] - m[2][2]) * 2.0
        return (m[0][1] + m[1][0]) / s, 0.25 * s, (m[1][2] + m[2][1]) / s, (m[0][2] - m[2][0]) / s
    s = math.sqrt(1.0 + m[2][2] - m[0][0] - m[1][1]) * 2.0
    return (m[0][2] + m[2][0]) / s, (m[1][2] + m[2][1]) / s, 0.25 * s, (m[1][0] - m[0][1]) / s


def normalise(v):
    length = math.sqrt(sum(component * component for component in v)) or 1.0
    return tuple(component / length for component in v)


def subtract(a, b):
    return tuple(x - y for x, y in zip(a, b))


def scale(v, factor):
    return tuple(component * factor for component in v)


def dot(a, b):
    return sum(x * y for x, y in zip(a, b))


def cross(a, b):
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def rounded(value):
    """📐️ Real coordinates rounded to the micrometre, the tolerance the source itself carries."""
    return round(float(value), 6) + 0.0


def transform_of(product):
    placement = getattr(product, "ObjectPlacement", None)
    while placement is not None and placement.is_a("IfcLocalPlacement"):
        relative = placement.RelativePlacement
        if relative is not None and relative.is_a("IfcAxis2Placement3D"):
            location = tuple(float(c) for c in relative.Location.Coordinates)
            axis = tuple(float(c) for c in relative.Axis.DirectionRatios) if relative.Axis else None
            ref = tuple(float(c) for c in relative.RefDirection.DirectionRatios) if relative.RefDirection else None
            x, y, z, w = quaternion(axis, ref)
            return {
                "translation": {"x": rounded(location[0]), "y": rounded(location[1]), "z": rounded(location[2])},
                "rotation": {"x": rounded(x), "y": rounded(y), "z": rounded(z), "w": rounded(w)},
                "scale": {"x": 1.0, "y": 1.0, "z": 1.0},
            }
        placement = placement.PlacementRelTo
    return {"translation": {"x": 0.0, "y": 0.0, "z": 0.0}, "rotation": {"x": 0.0, "y": 0.0, "z": 0.0, "w": 1.0}, "scale": {"x": 1.0, "y": 1.0, "z": 1.0}}


def ref(artifact_id, subset):
    return {"artifactId": artifact_id, "dialect": {"artifactKind": "s.stdio.semio", "standard": "v1", "subset": subset}}


def main():
    model = ifcopenshell.open(str(IFC))
    oracle = load_oracle()

    type_of_piece, types = {}, []
    for definition in model.by_type("IfcRelDefinesByType"):
        declared = definition.RelatingType
        type_id = declared.GlobalId
        if all(entry["id"] != type_id for entry in types):
            types.append({"id": type_id, "name": declared.Name or "", "category": declared.is_a()})
        for product in definition.RelatedObjects:
            type_of_piece[product.id()] = type_id
    types.sort(key=lambda entry: entry["id"])

    capsules = sorted((p for p in model.by_type("IfcBuildingElementProxy") if p.id() in type_of_piece), key=lambda p: p.id())
    pieces = [{"id": capsule.GlobalId, "typeId": type_of_piece[capsule.id()], "transform": transform_of(capsule)} for capsule in capsules]
    piece_ids = {capsule.id(): capsule.GlobalId for capsule in capsules}

    owner_of_port = {}
    for nest in model.by_type("IfcRelNests"):
        for related in nest.RelatedObjects:
            if related.is_a("IfcDistributionPort"):
                owner_of_port[related.id()] = nest.RelatingObject.id()

    connections = []
    for relation in sorted(model.by_type("IfcRelConnectsPorts"), key=lambda r: r.id()):
        source = owner_of_port.get(relation.RelatingPort.id())
        target = owner_of_port.get(relation.RelatedPort.id())
        if source not in piece_ids or target not in piece_ids:
            continue
        connections.append(
            {
                "id": relation.GlobalId,
                "connectingPieceId": piece_ids[source],
                "connectingPort": relation.RelatingPort.Name or relation.RelatingPort.GlobalId,
                "connectedPieceId": piece_ids[target],
                "connectedPort": relation.RelatedPort.Name or relation.RelatedPort.GlobalId,
            }
        )

    assembly = model.by_type("IfcElementAssembly")[0]
    document = {
        "schema": "stdio.semio.kit",
        "types": types,
        "designs": [{"id": assembly.GlobalId, "name": assembly.Name or "", "pieces": pieces, "connections": connections}],
        "objects": [{"childId": "obj-01", "target": ref("chair-instance", "object")}],
        "models": [{"childId": "model-01", "target": ref("chair-bim", "model")}],
        "properties": {"childId": "props-01", "target": ref("kit-props", "value")},
        "representations": [{"target": ref(entry["id"], "mesh"), "pin": {"kind": "head"}, "role": entry["id"]} for entry in types],
    }

    dsl = oracle.print_dsl(document)
    assert oracle.parse_dsl(dsl) == document, "the printed DSL does not re-parse to the same document"
    pack = oracle.pack_bytes(document)
    assert oracle.parse_pack(pack) == document, "the encoded pack does not re-decode to the same document"
    out = CASE / "🧫️fixtures"
    out.mkdir(exist_ok=True)
    (out / "🗣️nakagin-capsule-tower.dsl.semio").write_text(dsl, encoding="utf-8")
    (out / "🎒️nakagin-capsule-tower.pack.semio").write_bytes(pack)
    print("types=%d pieces=%d connections=%d representations=%d" % (len(types), len(pieces), len(connections), len(document["representations"])))
    print("dsl bytes=%d pack bytes=%d" % (len(dsl.encode("utf-8")), len(pack)))
    print("design id", document["designs"][0]["id"], "name", document["designs"][0]["name"])
    print("first type", types[0], "last type", types[-1])
    print("first piece", pieces[0])
    print("last piece", pieces[-1]["id"], pieces[-1]["typeId"])
    print("last connection", connections[-1])


if __name__ == "__main__":
    main()
