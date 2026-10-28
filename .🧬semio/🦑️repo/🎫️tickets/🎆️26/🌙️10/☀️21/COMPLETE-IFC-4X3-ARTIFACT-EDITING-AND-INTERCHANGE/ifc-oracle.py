"""🔮 Independent IfcOpenShell STEP and geometry oracle; test-only dependency interface."""
import json
import pathlib
import sys

if len(sys.argv) < 4:
    raise SystemExit("usage: ifc-oracle.py MODULE_DIRECTORY INPUT_IFC OUTPUT_DIRECTORY")
sys.path.insert(0, sys.argv[1])
import ifcopenshell
import ifcopenshell.geom
import ifcopenshell.util.unit
import ifcopenshell.validate

source = pathlib.Path(sys.argv[2])
output = pathlib.Path(sys.argv[3])
output.mkdir(parents=True, exist_ok=True)
model = ifcopenshell.file.from_string(source.read_text(encoding="utf-8"))
canonical = model.to_string()
(output / (source.stem + ".ifc")).write_text(canonical, encoding="utf-8", newline="\n")
summary = {
    "implementation": "IfcOpenShell",
    "version": ifcopenshell.version,
    "schema": model.schema_identifier,
    "entities": [[row.id(), row.is_a()] for row in model],
    "lengthScale": ifcopenshell.util.unit.calculate_unit_scale(model),
    "triangulations": [],
}
for row in model.by_type("IfcTriangulatedFaceSet"):
    summary["triangulations"].append({
        "id": row.id(),
        "coordinates": row.Coordinates.CoordList,
        "indices": row.CoordIndex,
    })
settings = ifcopenshell.geom.settings()
settings.set(settings.USE_WORLD_COORDS, True)
geometry = []
for product in model.by_type("IfcProduct"):
    if not product.Representation:
        continue
    try:
        shape = ifcopenshell.geom.create_shape(settings, product)
        vertices = list(shape.geometry.verts)
        geometry.append({
            "id": product.id(),
            "vertices": len(vertices) // 3,
            "triangles": len(shape.geometry.faces) // 3,
            "bounds": [[min(vertices[k::3]) for k in range(3)], [max(vertices[k::3]) for k in range(3)]],
        })
    except Exception as error:
        geometry.append({"id": product.id(), "unsupported": str(error)})
summary["geometry"] = geometry
logger = ifcopenshell.validate.json_logger()
ifcopenshell.validate.validate(model, logger, express_rules=False)
summary["validation"] = logger.statements
summary["properties"] = [{"name": row.Name, "value": row.NominalValue.wrappedValue if row.NominalValue else None} for row in model.by_type("IfcPropertySingleValue")]
summary["types"] = {name: len(model.by_type(name)) for name in ["IfcStructuralPointConnection", "IfcStructuralCurveMember", "IfcStructuralLoadCase", "IfcStructuralPointAction", "IfcStructuralCurveAction", "IfcSite", "IfcBuilding", "IfcBuildingStorey", "IfcPixelTexture"]}
if source.stem == "fem-export":
    assert summary["types"]["IfcStructuralPointConnection"] == 2
    assert summary["types"]["IfcStructuralCurveMember"] == 1
    assert summary["types"]["IfcStructuralLoadCase"] == 1
    assert summary["types"]["IfcStructuralPointAction"] == 1
    assert summary["types"]["IfcStructuralCurveAction"] == 1
    properties = {row["name"]: row["value"] for row in summary["properties"]}
    assert properties["YoungModulus"] == 2e11
    assert properties["CrossSectionArea"] == 0.01
    assert properties["TorsionalConstant"] == 3e-5
    assert model.by_type("IfcStructuralLoadSingleForce")[0].ForceZ == -1000
    assert model.by_type("IfcStructuralLoadLinearForce")[0].LinearForceZ == -500
    def unit_dimensions(unit):
        if unit.is_a("IfcDerivedUnit"):
            return tuple(sum(unit_dimensions(element.Unit)[index] * element.Exponent for element in unit.Elements) for index in range(7))
        return tuple(unit.Dimensions)
    expected_dimensions = {"YoungModulus": (-1, 1, -2, 0, 0, 0, 0), "MassDensity": (-3, 1, 0, 0, 0, 0, 0), "CrossSectionArea": (2, 0, 0, 0, 0, 0, 0), "TorsionalConstant": (4, 0, 0, 0, 0, 0, 0)}
    for prop in model.by_type("IfcPropertySingleValue"):
        if prop.Name in expected_dimensions:
            assert unit_dimensions(prop.Unit) == expected_dimensions[prop.Name]
if source.stem == "bim-export":
    assert summary["types"]["IfcSite"] == 1
    assert summary["types"]["IfcBuilding"] == 1
    assert summary["types"]["IfcBuildingStorey"] == 2
    assert model.by_type("IfcSite")[0].RefLatitude == (47, 30, 0, 0)
    assert sorted(row.Elevation for row in model.by_type("IfcBuildingStorey")) == [0, 3]
if source.stem == "binary":
    assert model.by_type("IfcPixelTexture")[0].Pixel == ("000000000001000100100010",)
if source.stem == "bim-mm":
    metre = next(unit for unit in model.by_type("IfcSIUnit") if unit.Name == "METRE" and unit.Prefix is None)
    for prop in model.by_type("IfcPropertySingleValue"):
        if prop.Name in ["Height", "CutHeight"]:
            assert prop.NominalValue.is_a("IfcLengthMeasure")
            assert ifcopenshell.util.unit.convert_unit(prop.NominalValue.wrappedValue, prop.Unit, metre) == (3 if prop.Name == "Height" else 1.2)
if source.stem in ["fem-shared", "fem-shared-roundtrip"]:
    members = model.by_type("IfcStructuralCurveMember")
    assert len(members) == 2
    assert len(model.by_type("IfcMaterial")) == 1
    sections = [row for row in model.by_type("IfcPropertySet") if row.Name == "semio_FemSection"]
    assert len(sections) == 1
    for member in members:
        assigned = [rel.RelatingPropertyDefinition.id() for rel in member.IsDefinedBy if rel.is_a("IfcRelDefinesByProperties") and rel.RelatingPropertyDefinition.Name == "semio_FemSection"]
        assert set(assigned) == {sections[0].id()}
    nodes = model.by_type("IfcStructuralPointConnection")
    assert len(nodes) == 2 and all(node.AppliedCondition for node in nodes)
    assert all(all(value.wrappedValue is True for value in tuple(node.AppliedCondition)[1:]) for node in nodes)
if source.stem == "native-bim":
    assert summary["types"]["IfcSite"] == 1
    assert summary["types"]["IfcBuilding"] == 1
    assert summary["types"]["IfcBuildingStorey"] == 2
    assert model.by_type("IfcSite")[0].RefLatitude == (47, 30, 0, 0)
    assert sorted(row.Elevation for row in model.by_type("IfcBuildingStorey")) == [0, 3]
if source.stem == "native-step-import":
    assert len(summary["entities"]) == 35
    assert summary["lengthScale"] == 0.001
    assert summary["geometry"][0]["triangles"] == 12
    assert summary["geometry"][0]["bounds"] == [[0.5, -0.5, 0.0], [1.5, 0.5, 2.0]]
    assert not summary["validation"], str(summary["validation"])
if source.stem == "native-fem":
    assert summary["types"]["IfcStructuralPointConnection"] == 3
    assert summary["types"]["IfcStructuralCurveMember"] == 2
    assert summary["types"]["IfcStructuralLoadCase"] == 0
    assert len(model.by_type("IfcMaterial")) == 1
    sections = [row for row in model.by_type("IfcPropertySet") if row.Name == "semio_FemSection"]
    assert len(sections) == 1
    for member in model.by_type("IfcStructuralCurveMember"):
        assert {rel.RelatingPropertyDefinition.id() for rel in member.IsDefinedBy if rel.is_a("IfcRelDefinesByProperties") and rel.RelatingPropertyDefinition.Name == "semio_FemSection"} == {sections[0].id()}
    values = {row["name"]: row["value"] for row in summary["properties"]}
    assert values["YoungModulus"] == 2e11 and values["TorsionalConstant"] == 4e-5
    assert len([node for node in model.by_type("IfcStructuralPointConnection") if node.AppliedCondition]) == 1
if source.stem in ["fem-export", "bim-export", "box-import", "binary", "bim-mm", "fem-shared", "fem-shared-roundtrip", "native-bim", "native-fem"]:
    assert not summary["validation"], str(summary["validation"])
(output / (source.stem + ".json")).write_text(json.dumps(summary, indent=2, default=str), encoding="utf-8")
print(json.dumps({"implementation": summary["implementation"], "version": summary["version"], "schema": summary["schema"], "entities": len(summary["entities"]), "file": str(source)}))
